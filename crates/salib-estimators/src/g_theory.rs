//! Crossed `p × i × r` generalizability-theory G-study and D-study projection.
//!
//! [`estimate_g_theory_pir`] returns the balanced random-effects ANOVA
//! (expected-mean-square) estimates of the seven variance components. These
//! estimates are **signed**: a sampling-negative estimate is reported as it is.
//! A G or Phi coefficient is computed only from nonnegative components, so every
//! available coefficient satisfies `0 <= Phi <= G <= 1`. When a component it
//! uses is negative, the coefficient is
//! `Err(GTheoryCoefficientError::NegativeComponent)`. Callers who want the
//! conventional truncation must request it with
//! [`GTheoryResult::with_negative_components_zeroed`].
//!
//! The coefficients do not depend on the measurement unit: scaling every
//! observation by a constant leaves them unchanged whenever the variance
//! components are representable as normal `f64` values, roughly for data
//! spreads between 1e-150 and 1e150. Outside that range estimation returns
//! [`GTheoryError::ScaleOutOfRange`]. An estimate whose magnitude lies within
//! the rounding resolution of the data is reported as exactly zero (see
//! [`estimate_g_theory_pir`]).
//!
//! Callers can set the public result fields directly, or deserialize a
//! result, without these checks. Projection re-evaluates the coefficients
//! from whatever components it is given.
//!
//! [`bootstrap_g_theory_pir`] reports Tong & Brennan component standard errors
//! with Brennan's exact bias correction, parametric-bootstrap intervals for
//! the components and Phi, and a closed-form F interval for G. Their
//! calibration is established only where salib's coverage study says so. [crolib](https://github.com/antimemeai/crolib) develops
//! G-theory beyond this module: designs, fitting, uncertainty and allocation.

#![allow(
    clippy::cast_precision_loss,
    clippy::similar_names,
    clippy::too_many_lines
)]

use std::fmt;

use ndarray::{Array3, ArrayView3};
use rand::RngCore;
use salib_core::RngState;
use statrs::distribution::{ContinuousCDF, FisherSnedecor};

use crate::bootstrap::percentile_value;
use crate::bootstrap_given_data::BootstrapGivenDataError;

/// Measurement design to decompose. Only `Crossed` is implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum GTheoryDesign {
    /// Fully crossed persons × items × raters design; implemented.
    Crossed,
    /// Nested design; currently rejected as unsupported.
    Nested,
    /// Mixed crossed/nested design; currently rejected as unsupported.
    Mixed,
}

/// One of the seven variance components of the crossed `p × i × r` design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GTheoryComponent {
    /// Persons (objects of measurement).
    P,
    /// Items.
    I,
    /// Raters.
    R,
    /// Person × item interaction.
    Pi,
    /// Person × rater interaction.
    Pr,
    /// Item × rater interaction.
    Ir,
    /// Three-way interaction confounded with residual error.
    Pir,
}

impl GTheoryComponent {
    /// All components in the order used by result fields and interval vectors.
    pub const ALL: [Self; 7] = [
        Self::P,
        Self::I,
        Self::R,
        Self::Pi,
        Self::Pr,
        Self::Ir,
        Self::Pir,
    ];
}

impl fmt::Display for GTheoryComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::P => "p",
            Self::I => "i",
            Self::R => "r",
            Self::Pi => "pi",
            Self::Pr => "pr",
            Self::Ir => "ir",
            Self::Pir => "pir",
        };
        write!(f, "\u{03c3}\u{00b2}({name})")
    }
}

/// Reason a G or Phi coefficient is unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum GTheoryCoefficientError {
    /// A variance component used by the coefficient is negative.
    #[error("g-theory: coefficient uses negative variance component {component}")]
    NegativeComponent {
        /// First negative component in [`GTheoryComponent::ALL`] order.
        component: GTheoryComponent,
    },
    /// Universe-score and error variance are both zero, so the ratio is 0/0.
    #[error("g-theory: coefficient is 0/0: universe-score and error variance are both zero")]
    ZeroDenominator,
    /// A component used by the coefficient, or the error sum, is not finite.
    #[error("g-theory: coefficient uses a nonfinite variance or error sum")]
    NonFinite,
}

/// Invalid grids, counts or searches.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum GTheoryError {
    /// Only the fully crossed design is implemented.
    #[error("g-theory: only crossed design is implemented in V1")]
    UnsupportedDesign,
    /// A grid axis or a projected count has too few levels.
    #[error("g-theory: axis {axis} must have length >= {min}, got {len}")]
    DegenerateAxis {
        /// Name of the grid axis or count that failed validation.
        axis: &'static str,
        /// Length or count supplied.
        len: usize,
        /// Minimum accepted length or count.
        min: usize,
    },
    /// Every observation is identical, so no variance can be decomposed.
    #[error("g-theory: all observations are identical")]
    ZeroVariance,
    /// An observation is NaN or infinite.
    #[error("g-theory: observations must be finite")]
    NonFiniteObservation,
    /// A supplied variance component is NaN or infinite.
    #[error("g-theory: variance component {component} must be finite")]
    NonFiniteComponent {
        /// The offending component.
        component: GTheoryComponent,
    },
    /// Variance components fall outside the normal `f64` range, or their sum
    /// overflows; rescale the observations.
    #[error(
        "g-theory: variance components are outside the normal f64 range; rescale the observations"
    )]
    ScaleOutOfRange,
    /// A coefficient required by the request is unavailable.
    #[error(transparent)]
    Coefficient(#[from] GTheoryCoefficientError),
}

/// Errors from G-theory estimation or bootstrap resampling.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum GTheoryBootstrapError {
    /// Generalizability-theory estimation failed.
    #[error("g-theory bootstrap: estimator failed: {0}")]
    GTheory(#[from] GTheoryError),
    /// The bootstrap arguments are invalid.
    #[error("g-theory bootstrap: invalid bootstrap params: {0}")]
    Bootstrap(#[from] BootstrapGivenDataError),
    /// Every bootstrap draw failed; no interval can be computed.
    #[error("g-theory bootstrap: every resample failed")]
    AllResamplesFailed,
}

/// A G or Phi coefficient, or the reason it is unavailable.
pub type GTheoryCoefficient = Result<f64, GTheoryCoefficientError>;

/// Seven crossed-design variance components and their G/Phi coefficients.
///
/// Components are signed moment estimates unless
/// [`negative_components_zeroed`](Self::negative_components_zeroed) is set.
/// The coefficients refer to [`n_items`](Self::n_items) items and
/// [`n_raters`](Self::n_raters) raters, which are the observed counts for a
/// G-study. [`bootstrap`](Self::bootstrap) is set only by
/// [`estimate_g_theory_pir_with_bootstrap`]; read its intervals through
/// [`g_coefficient_interval`](Self::g_coefficient_interval) and
/// [`phi_coefficient_interval`](Self::phi_coefficient_interval), which apply
/// the attachment rules.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct GTheoryResult {
    /// Item count the coefficients refer to.
    pub n_items: usize,
    /// Rater count the coefficients refer to.
    pub n_raters: usize,
    /// Person variance component.
    pub sigma_p: f64,
    /// Item variance component.
    pub sigma_i: f64,
    /// Rater variance component.
    pub sigma_r: f64,
    /// Person × item interaction variance component.
    pub sigma_pi: f64,
    /// Person × rater interaction variance component.
    pub sigma_pr: f64,
    /// Item × rater interaction variance component.
    pub sigma_ir: f64,
    /// Three-way interaction plus residual variance component.
    pub sigma_pir: f64,
    /// Relative-decision coefficient G (Eρ²), or why it is unavailable.
    pub g_coefficient: GTheoryCoefficient,
    /// Absolute-decision coefficient Phi, or why it is unavailable.
    pub phi_coefficient: GTheoryCoefficient,
    /// Whether negative estimates were replaced by zero via
    /// [`with_negative_components_zeroed`](Self::with_negative_components_zeroed).
    pub negative_components_zeroed: bool,
    /// Bootstrap and interval results; `None` without bootstrap.
    pub bootstrap: Option<GTheoryBootstrap>,
}

impl GTheoryResult {
    /// Build a result from supplied variance components.
    ///
    /// G and Phi are computed for `n_items` items and `n_raters` raters under
    /// the same rules as [`estimate_g_theory_pir`]. Signed components are
    /// accepted and kept as given; nothing is snapped or zeroed.
    ///
    /// # Errors
    ///
    /// [`GTheoryError::DegenerateAxis`] when either count is zero,
    /// [`GTheoryError::NonFiniteComponent`] for a NaN or infinite component, and
    /// [`GTheoryError::ScaleOutOfRange`] for a subnormal component or when the
    /// components' magnitudes sum past `f64::MAX`.
    #[allow(clippy::too_many_arguments)]
    pub fn from_components(
        sigma_p: f64,
        sigma_i: f64,
        sigma_r: f64,
        sigma_pi: f64,
        sigma_pr: f64,
        sigma_ir: f64,
        sigma_pir: f64,
        n_items: usize,
        n_raters: usize,
    ) -> Result<Self, GTheoryError> {
        check_counts(n_items, n_raters)?;
        let components = [
            sigma_p, sigma_i, sigma_r, sigma_pi, sigma_pr, sigma_ir, sigma_pir,
        ];
        for (component, value) in GTheoryComponent::ALL.into_iter().zip(components) {
            if !value.is_finite() {
                return Err(GTheoryError::NonFiniteComponent { component });
            }
        }
        let subnormal = components
            .iter()
            .any(|c| *c != 0.0 && c.abs() < f64::MIN_POSITIVE);
        if subnormal || !components.iter().map(|c| c.abs()).sum::<f64>().is_finite() {
            return Err(GTheoryError::ScaleOutOfRange);
        }
        Ok(Self::new(components, n_items, n_raters))
    }

    /// Copy with negative components replaced by zero and coefficients recomputed.
    ///
    /// Each negative estimate is clipped after solving the moment equations;
    /// the other estimates are unchanged. This is the common D-study
    /// convention. It is not the Cronbach et al. (1972) procedure, which feeds
    /// the zeros back into the remaining equations, and not a constrained
    /// (e.g. REML) fit, which would reallocate the variance. The copy sets
    /// [`negative_components_zeroed`](Self::negative_components_zeroed), even
    /// when nothing was negative, because the flag records the procedure.
    ///
    /// Bootstrap results carry over except the component standard errors,
    /// which describe the signed estimates. The G interval is a function of
    /// the signed mean squares, so the clipped G can lie outside it.
    #[must_use]
    pub fn with_negative_components_zeroed(&self) -> Self {
        let zeroed = self.components().map(|c| if c < 0.0 { 0.0 } else { c });
        let mut result = Self::new(zeroed, self.n_items, self.n_raters);
        result.negative_components_zeroed = true;
        result.bootstrap = self.bootstrap.clone().map(|mut boot| {
            boot.component_se = [None; 7];
            boot
        });
        result
    }

    /// Interval for G from [`GTheoryBootstrap::g_coefficient_ci`], whenever it exists.
    #[must_use]
    pub fn g_coefficient_interval(&self) -> Option<(f64, f64)> {
        self.bootstrap.as_ref()?.g_coefficient_ci
    }

    /// Interval for Phi from [`GTheoryBootstrap::phi_coefficient_ci`], only
    /// when this result's Phi is available. That interval describes the
    /// clipped estimator, which equals an available Phi.
    #[must_use]
    pub fn phi_coefficient_interval(&self) -> Option<(f64, f64)> {
        self.phi_coefficient
            .is_ok()
            .then(|| self.bootstrap.as_ref()?.phi_coefficient_ci)
            .flatten()
    }

    /// Components in [`GTheoryComponent::ALL`] order.
    #[must_use]
    pub fn components(&self) -> [f64; 7] {
        [
            self.sigma_p,
            self.sigma_i,
            self.sigma_r,
            self.sigma_pi,
            self.sigma_pr,
            self.sigma_ir,
            self.sigma_pir,
        ]
    }

    fn new(components: [f64; 7], n_items: usize, n_raters: usize) -> Self {
        // Adding +0.0 turns -0.0 into +0.0 and leaves every other value unchanged.
        let components = components.map(|c| c + 0.0);
        let (g_coefficient, phi_coefficient) = coefficients(&components, n_items, n_raters);
        let [sigma_p, sigma_i, sigma_r, sigma_pi, sigma_pr, sigma_ir, sigma_pir] = components;
        Self {
            n_items,
            n_raters,
            sigma_p,
            sigma_i,
            sigma_r,
            sigma_pi,
            sigma_pr,
            sigma_ir,
            sigma_pir,
            g_coefficient,
            phi_coefficient,
            negative_components_zeroed: false,
            bootstrap: None,
        }
    }
}

/// Projected G and Phi for a specified item/rater count.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DStudyPoint {
    /// Number of items in the projected design (at least one).
    pub n_items: usize,
    /// Number of raters in the projected design (at least one).
    pub n_raters: usize,
    /// Relative-decision coefficient G, or why it is unavailable.
    pub g_coefficient: GTheoryCoefficient,
    /// Absolute-decision coefficient Phi, or why it is unavailable.
    pub phi_coefficient: GTheoryCoefficient,
}

fn fmt_coefficient(f: &mut fmt::Formatter<'_>, value: &GTheoryCoefficient) -> fmt::Result {
    match value {
        Ok(v) => write!(f, "{v:.4}"),
        Err(GTheoryCoefficientError::NegativeComponent { component }) => {
            write!(f, "unavailable (negative {component})")
        }
        Err(GTheoryCoefficientError::ZeroDenominator) => f.write_str("undefined (0/0)"),
        Err(GTheoryCoefficientError::NonFinite) => f.write_str("unavailable (nonfinite)"),
    }
}

impl fmt::Display for GTheoryResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "G-theory (p x i x r)")?;
        writeln!(f)?;
        if self.negative_components_zeroed {
            writeln!(f, "  Variance components (negative estimates set to zero):")?;
        } else {
            writeln!(f, "  Variance components (signed estimates):")?;
        }
        for (component, value) in GTheoryComponent::ALL.iter().zip(self.components()) {
            writeln!(f, "    {:<8} = {value:.4}", component.to_string())?;
        }
        writeln!(f)?;
        write!(
            f,
            "  {} items, {} raters: G = ",
            self.n_items, self.n_raters
        )?;
        fmt_coefficient(f, &self.g_coefficient)?;
        f.write_str("  \u{03a6} = ")?;
        fmt_coefficient(f, &self.phi_coefficient)?;
        writeln!(f)
    }
}

/// Safety factor on the summation-error bounds used to snap estimates to zero.
const SNAP_FACTOR: f64 = 8.0;

/// Binary exponent `e` with `2^e <= value < 2^(e+1)`, for finite `value > 0`.
fn binary_exponent(value: f64) -> i32 {
    let bits = value.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    if biased == 0 {
        // Subnormal: value = mantissa · 2^-1074.
        63 - bits.leading_zeros() as i32 - 1074
    } else {
        biased - 1023
    }
}

/// `value · 2^k`, exact whenever the result is a normal float.
fn scale_by_pow2(mut value: f64, mut k: i32) -> f64 {
    let step = |k: i32| f64::from_bits(((k + 1023) as u64) << 52);
    while k > 1023 {
        value *= step(1023);
        k -= 1023;
    }
    while k < -1022 {
        value *= step(-1022);
        k += 1022;
    }
    value * step(k)
}

/// Estimate variance components for a balanced `(persons, items, raters)` grid.
///
/// All axes require at least two levels. Only [`GTheoryDesign::Crossed`] is
/// implemented. Components are the signed expected-mean-square estimates;
/// G and Phi refer to the grid's item and rater counts and are unavailable
/// when a component they use is negative (see the [module docs](self)).
///
/// The observations are first scaled by an exact power of two so that
/// `max|y|` lies in `[1, 2)`, then centered on a two-pass mean (`z`); no square
/// can overflow. An estimate `σ̂_k = Σ c_j MS_j / d_k`, with
/// `M_k = Σ |c_j| MS_j / d_k`, is reported as exactly zero when
///
/// ```text
/// |σ̂_k| <= δ² + δ √M_k + K N ε M_k + (4 ε max|y|)²,   δ = K N ε max|z|
/// ```
///
/// with `K = 8`, `N` the cell count and `ε` the `f64` epsilon. `δ` bounds the
/// rounding of the centered means and contrasts. The first three terms bound
/// the resulting error in a zero component, a nonzero one, and a difference of
/// nearly equal mean squares. The last is the resolution of the observations,
/// each known only to within a few units in the last place (ulp) of
/// `max|y|`. Every term scales with the square of the data, so the rule does
/// not depend on units. The bounds are worst-case, not typical: a real
/// component far below its own sampling error can be zeroed. That happens when
/// it is below about `δ √M_k`, for example next to a single huge outlier, or
/// below about 16 ulp² of the largest observation. The rule cannot tell exact
/// values from rounded ones.
///
/// # Errors
///
/// Rejects unsupported designs, axes shorter than two levels, nonfinite
/// observations, grids whose observations are all identical, and data whose
/// variance components cannot be represented as normal `f64` values
/// ([`GTheoryError::ScaleOutOfRange`]). An unavailable coefficient is not an
/// error: it is reported in the result.
///
/// # Examples
///
/// ```rust
/// use ndarray::Array3;
/// use salib_estimators::{estimate_g_theory_pir, GTheoryDesign};
/// // ±1 effect codes: every component is positive.
/// let grid = Array3::from_shape_fn((2, 2, 2), |(p, i, r)| {
///     let [p, i, r] = [p, i, r].map(|k| if k == 0 { -1.0 } else { 1.0 });
///     50.0 + 6.0 * p + 4.0 * i + 2.0 * r + 3.0 * p * i + 1.5 * p * r + i * r + 0.5 * p * i * r
/// });
/// let result = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
/// let g = result.g_coefficient.expect("G components are nonnegative");
/// assert!((g - 25.0 / 36.0).abs() < 1e-12);
/// ```
pub fn estimate_g_theory_pir(
    grid: ArrayView3<'_, f64>,
    design: GTheoryDesign,
) -> Result<GTheoryResult, GTheoryError> {
    if !matches!(design, GTheoryDesign::Crossed) {
        return Err(GTheoryError::UnsupportedDesign);
    }

    let p = grid.shape()[0];
    let i = grid.shape()[1];
    let r = grid.shape()[2];
    for (axis, len) in [("p", p), ("i", i), ("r", r)] {
        if len < 2 {
            return Err(GTheoryError::DegenerateAxis { axis, len, min: 2 });
        }
    }
    if grid.iter().any(|y| !y.is_finite()) {
        return Err(GTheoryError::NonFiniteObservation);
    }
    let first = grid[[0, 0, 0]];
    if grid.iter().all(|&y| y == first) {
        return Err(GTheoryError::ZeroVariance);
    }

    // Work in units of 2^e, with max|y| in [1, 2). Values far below max|y|
    // may round in this step; they are below the data's resolution anyway.
    let exponent = binary_exponent(grid.iter().fold(0.0_f64, |m, y| m.max(y.abs())));
    let scaled = grid.mapv(|y| scale_by_pow2(y, -exponent));
    let max_abs_y = scaled.iter().fold(0.0_f64, |m, y| m.max(y.abs()));
    let n_cells = (p * i * r) as f64;
    let rough = scaled.iter().sum::<f64>() / n_cells;
    let center = rough + scaled.iter().map(|y| y - rough).sum::<f64>() / n_cells;
    let z = scaled.mapv(|y| y - center);

    let grand = z.iter().sum::<f64>() / n_cells;
    let mean_p: Vec<f64> = (0..p)
        .map(|pp| {
            let mut sum = 0.0;
            for ii in 0..i {
                for rr in 0..r {
                    sum += z[[pp, ii, rr]];
                }
            }
            sum / (i * r) as f64
        })
        .collect();
    let mean_i: Vec<f64> = (0..i)
        .map(|ii| {
            let mut sum = 0.0;
            for pp in 0..p {
                for rr in 0..r {
                    sum += z[[pp, ii, rr]];
                }
            }
            sum / (p * r) as f64
        })
        .collect();
    let mean_r: Vec<f64> = (0..r)
        .map(|rr| {
            let mut sum = 0.0;
            for pp in 0..p {
                for ii in 0..i {
                    sum += z[[pp, ii, rr]];
                }
            }
            sum / (p * i) as f64
        })
        .collect();

    let mean_pi: Vec<Vec<f64>> = (0..p)
        .map(|pp| {
            (0..i)
                .map(|ii| {
                    let mut sum = 0.0;
                    for rr in 0..r {
                        sum += z[[pp, ii, rr]];
                    }
                    sum / r as f64
                })
                .collect()
        })
        .collect();
    let mean_pr: Vec<Vec<f64>> = (0..p)
        .map(|pp| {
            (0..r)
                .map(|rr| {
                    let mut sum = 0.0;
                    for ii in 0..i {
                        sum += z[[pp, ii, rr]];
                    }
                    sum / i as f64
                })
                .collect()
        })
        .collect();
    let mean_ir: Vec<Vec<f64>> = (0..i)
        .map(|ii| {
            (0..r)
                .map(|rr| {
                    let mut sum = 0.0;
                    for pp in 0..p {
                        sum += z[[pp, ii, rr]];
                    }
                    sum / p as f64
                })
                .collect()
        })
        .collect();

    let ss_p = (i * r) as f64 * mean_p.iter().map(|m| (m - grand).powi(2)).sum::<f64>();
    let ss_i = (p * r) as f64 * mean_i.iter().map(|m| (m - grand).powi(2)).sum::<f64>();
    let ss_r = (p * i) as f64 * mean_r.iter().map(|m| (m - grand).powi(2)).sum::<f64>();

    let mut ss_pi = 0.0;
    for pp in 0..p {
        for ii in 0..i {
            let term = mean_pi[pp][ii] - mean_p[pp] - mean_i[ii] + grand;
            ss_pi += (r as f64) * term.powi(2);
        }
    }
    let mut ss_pr = 0.0;
    for pp in 0..p {
        for rr in 0..r {
            let term = mean_pr[pp][rr] - mean_p[pp] - mean_r[rr] + grand;
            ss_pr += (i as f64) * term.powi(2);
        }
    }
    let mut ss_ir = 0.0;
    for ii in 0..i {
        for rr in 0..r {
            let term = mean_ir[ii][rr] - mean_i[ii] - mean_r[rr] + grand;
            ss_ir += (p as f64) * term.powi(2);
        }
    }

    let mut ss_pir = 0.0;
    for pp in 0..p {
        for ii in 0..i {
            for rr in 0..r {
                let term = z[[pp, ii, rr]] - mean_pi[pp][ii] - mean_pr[pp][rr] - mean_ir[ii][rr]
                    + mean_p[pp]
                    + mean_i[ii]
                    + mean_r[rr]
                    - grand;
                ss_pir += term.powi(2);
            }
        }
    }

    let ms_p = ss_p / ((p - 1) as f64);
    let ms_i = ss_i / ((i - 1) as f64);
    let ms_r = ss_r / ((r - 1) as f64);
    let ms_pi = ss_pi / (((p - 1) * (i - 1)) as f64);
    let ms_pr = ss_pr / (((p - 1) * (r - 1)) as f64);
    let ms_ir = ss_ir / (((i - 1) * (r - 1)) as f64);
    let ms_pir = ss_pir / (((p - 1) * (i - 1) * (r - 1)) as f64);

    // Each component is (Σ sign_j MS_j) / divisor, from the expected mean squares.
    // Rounding in the centered contrasts has absolute size about `delta`; a
    // zero component then carries residue of order delta², and a real one an
    // error of order delta·√magnitude plus cancellation among mean squares.
    let max_abs_z = z.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
    let delta = SNAP_FACTOR * n_cells * f64::EPSILON * max_abs_z;
    let floor = delta * delta + (4.0 * f64::EPSILON * max_abs_y).powi(2);
    let estimate = |positive: &[f64], negative: &[f64], divisor: usize| -> f64 {
        let divisor = divisor as f64;
        let value = (positive.iter().sum::<f64>() - negative.iter().sum::<f64>()) / divisor;
        let magnitude = positive.iter().chain(negative).sum::<f64>() / divisor;
        let bound =
            floor + delta * magnitude.sqrt() + SNAP_FACTOR * n_cells * f64::EPSILON * magnitude;
        if value.abs() <= bound {
            0.0
        } else {
            value
        }
    };
    let scaled_components = [
        estimate(&[ms_p, ms_pir], &[ms_pi, ms_pr], i * r),
        estimate(&[ms_i, ms_pir], &[ms_pi, ms_ir], p * r),
        estimate(&[ms_r, ms_pir], &[ms_pr, ms_ir], p * i),
        estimate(&[ms_pi], &[ms_pir], r),
        estimate(&[ms_pr], &[ms_pir], i),
        estimate(&[ms_ir], &[ms_pir], p),
        estimate(&[ms_pir], &[], 1),
    ];
    // Undo the power-of-two scaling; exact unless a component leaves the
    // normal range, in which case it cannot be stored faithfully.
    let components = scaled_components.map(|c| scale_by_pow2(c, 2 * exponent));
    let representable = scaled_components
        .iter()
        .zip(&components)
        .all(|(s, c)| *s == 0.0 || (c.is_finite() && c.abs() >= f64::MIN_POSITIVE));
    if !representable || !components.iter().map(|c| c.abs()).sum::<f64>().is_finite() {
        return Err(GTheoryError::ScaleOutOfRange);
    }
    Ok(GTheoryResult::new(components, i, r))
}

/// Estimate G-theory components/coefficients and attach bootstrap results.
///
/// Combines [`estimate_g_theory_pir`] with [`bootstrap_g_theory_pir`]; read the
/// latter for what each interval and standard error means and what it does not.
///
/// # Errors
///
/// Returns [`GTheoryBootstrapError`] on an invalid original grid or arguments,
/// or if every draw of a stream fails.
pub fn estimate_g_theory_pir_with_bootstrap(
    grid: ArrayView3<'_, f64>,
    design: GTheoryDesign,
    n_resamples: usize,
    alpha: f64,
    rng: &mut RngState,
) -> Result<GTheoryResult, GTheoryBootstrapError> {
    let mut result = estimate_g_theory_pir(grid, design)?;
    result.bootstrap = Some(bootstrap_g_theory_pir(
        grid,
        design,
        n_resamples,
        alpha,
        rng,
    )?);
    Ok(result)
}

/// A facet of the crossed `p × i × r` design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GTheoryFacet {
    /// Persons.
    P,
    /// Items.
    I,
    /// Raters.
    R,
}

impl GTheoryFacet {
    const ALL: [Self; 3] = [Self::P, Self::I, Self::R];

    fn axis(self) -> usize {
        match self {
            Self::P => 0,
            Self::I => 1,
            Self::R => 2,
        }
    }
}

/// Facets of each component as a bit mask over (p, i, r), in
/// [`GTheoryComponent::ALL`] order.
const COMPONENT_FACETS: [u8; 7] = [0b001, 0b010, 0b100, 0b011, 0b101, 0b110, 0b111];

fn component_index(mask: u8) -> usize {
    COMPONENT_FACETS
        .iter()
        .position(|&m| m == mask)
        .unwrap_or(usize::MAX)
}

/// Tong & Brennan's resampling plan for each component's standard error.
///
/// A main effect uses its own facet (CASMA 5, Rule 1). An interaction uses its
/// facet with the most levels (Rule 2); ties are broken in p, i, r order,
/// which is salib's choice, not the source's. The discretionary Rule 3 is not
/// applied. The rules were "supported by normal-data simulations" at
/// 100 × 20 × 2 only.
#[must_use]
pub fn g_theory_resampling_plan(sizes: [usize; 3]) -> [GTheoryFacet; 7] {
    COMPONENT_FACETS.map(|mask| {
        let mut best: Option<GTheoryFacet> = None;
        for facet in GTheoryFacet::ALL {
            if mask & (1 << facet.axis()) != 0
                && best.is_none_or(|b| sizes[facet.axis()] > sizes[b.axis()])
            {
                best = Some(facet);
            }
        }
        best.unwrap_or(GTheoryFacet::P)
    })
}

/// Correct one single-facet replicate (Brennan 2006, CASMA 21, Eq. 73 with
/// λ = {j}): `σ̂²(α|j)·n_j/(n_j − 1)` when `j ∈ α`, otherwise
/// `σ̂²(α|j) − σ̂²(α ∪ j | j)/(n_j − 1)`. Conditional on the data, corrected
/// replicates average exactly to the observed estimates.
fn correct_single_facet(replicate: [f64; 7], facet: GTheoryFacet, sizes: [usize; 3]) -> [f64; 7] {
    let n = sizes[facet.axis()] as f64;
    let bit = 1_u8 << facet.axis();
    std::array::from_fn(|k| {
        let mask = COMPONENT_FACETS[k];
        if mask & bit != 0 {
            replicate[k] * n / (n - 1.0)
        } else {
            replicate[k] - replicate[component_index(mask | bit)] / (n - 1.0)
        }
    })
}

/// Fit a resample, treating a constant one as having every component zero.
fn fit_replicate(
    grid: ArrayView3<'_, f64>,
    design: GTheoryDesign,
) -> Result<GTheoryResult, GTheoryError> {
    let (i, r) = (grid.shape()[1], grid.shape()[2]);
    match estimate_g_theory_pir(grid, design) {
        Err(GTheoryError::ZeroVariance) => Ok(GTheoryResult::new([0.0; 7], i, r)),
        other => other,
    }
}

/// Standard uniform on the open interval (0, 1), from 53 random bits.
fn open_unit(rng: &mut impl RngCore) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1_u64 << 53) as f64
}

/// Balanced normal random-effects arrays: seven independent effect arrays,
/// each `N(0, σ²)` with the given standard deviations, summed per cell.
struct ParametricGenerator {
    sizes: [usize; 3],
    effects: [Vec<f64>; 7],
    normal: salib_core::Distribution,
}

impl ParametricGenerator {
    fn new(sizes: [usize; 3]) -> Self {
        let [np, ni, nr] = sizes;
        Self {
            sizes,
            effects: [
                vec![0.0; np],
                vec![0.0; ni],
                vec![0.0; nr],
                vec![0.0; np * ni],
                vec![0.0; np * nr],
                vec![0.0; ni * nr],
                vec![0.0; np * ni * nr],
            ],
            normal: salib_core::Distribution::Normal {
                mu: 0.0,
                sigma: 1.0,
            },
        }
    }

    fn fill(&mut self, out: &mut Array3<f64>, scale: &[f64; 7], rng: &mut impl RngCore) {
        let [_, ni, nr] = self.sizes;
        for (k, effect) in self.effects.iter_mut().enumerate() {
            for value in effect.iter_mut() {
                *value = scale[k] * self.normal.quantile(open_unit(rng));
            }
        }
        let e = &self.effects;
        for ((a, b, c), value) in out.indexed_iter_mut() {
            *value = e[0][a]
                + e[1][b]
                + e[2][c]
                + e[3][a * ni + b]
                + e[4][a * nr + c]
                + e[5][b * nr + c]
                + e[6][(a * ni + b) * nr + c];
        }
    }
}

/// Closed-form interval for G at the G-study counts.
///
/// `1 − G = (EMS_pi + EMS_pr − EMS_pir)/EMS_p`, so under normality
/// `(1 − Ĝ)/(1 − G) ≈ F(ν̂, n_p − 1)`, where `L = MS_pi + MS_pr − MS_pir` and
/// `ν̂ = L²/(MS_pi²/df_pi + MS_pr²/df_pr + MS_pir²/df_pir)` (Satterthwaite).
/// This generalizes Feldt's p × i interval. The identity and `ν̂` are checked
/// symbolically in `verify/sage/g_theory_f_interval.sage`; the F law is an
/// approximation. Unavailable when `L ≤ 0` or `ν̂ < 0.5`. Both bounds are
/// clipped to `[0, 1]`, so a near-zero Ĝ can give `(0, 0)`.
fn g_f_interval(components: [f64; 7], sizes: [usize; 3], alpha: f64) -> Option<(f64, f64)> {
    let (one_minus_g, nu) = g_f_parameters(components, sizes)?;
    if nu < 0.5 {
        return None;
    }
    let f = FisherSnedecor::new(nu, sizes[0] as f64 - 1.0).ok()?;
    let (q_low, q_high) = (
        quantile_by_bisection(|x| f.cdf(x), alpha / 2.0)?,
        quantile_by_bisection(|x| f.cdf(x), 1.0 - alpha / 2.0)?,
    );
    Some((
        (1.0 - one_minus_g / q_low).max(0.0),
        (1.0 - one_minus_g / q_high).max(0.0),
    ))
}

/// `(1 − Ĝ, ν̂)` for [`g_f_interval`], or `None` when `L ≤ 0`.
fn g_f_parameters(components: [f64; 7], sizes: [usize; 3]) -> Option<(f64, f64)> {
    let [p, _, _, pi, pr, _, pir] = components;
    let (np, ni, nr) = (sizes[0] as f64, sizes[1] as f64, sizes[2] as f64);
    // Mean squares, recovered exactly from the expected-mean-square equations.
    let ms_pi = pir + nr * pi;
    let ms_pr = pir + ni * pr;
    let ms_pir = pir;
    let ms_p = ms_pi + ms_pr - ms_pir + ni * nr * p;
    let l = ms_pi + ms_pr - ms_pir;
    if !(l > 0.0 && ms_p > 0.0) {
        return None;
    }
    let df_pi = (np - 1.0) * (ni - 1.0);
    let df_pr = (np - 1.0) * (nr - 1.0);
    let df_pir = df_pi * (nr - 1.0);
    let nu = l * l / (ms_pi * ms_pi / df_pi + ms_pr * ms_pr / df_pr + ms_pir * ms_pir / df_pir);
    (nu.is_finite() && nu > 0.0).then_some((l / ms_p, nu))
}

/// The `prob` quantile of a continuous distribution on `(0, ∞)`, by
/// bisection on its cdf. statrs' own `inverse_cdf` is wrong for F and χ² at
/// small degrees of freedom; its cdf is reliable there. `None` if the cdf
/// returns a nonfinite value or the quantile lies outside `(0, 1e300)`.
fn quantile_by_bisection(cdf: impl Fn(f64) -> f64, prob: f64) -> Option<f64> {
    let (mut low, mut high) = (0.0_f64, 1.0_f64);
    loop {
        let c = cdf(high);
        if !c.is_finite() {
            return None;
        }
        if c >= prob {
            break;
        }
        low = high;
        high *= 2.0;
        if high > 1e300 {
            return None;
        }
    }
    for _ in 0..2000 {
        let mid = 0.5 * (low + high);
        if mid <= low || mid >= high {
            break;
        }
        let c = cdf(mid);
        if !c.is_finite() {
            return None;
        }
        if c < prob {
            low = mid;
        } else {
            high = mid;
        }
    }
    (high > 0.0).then_some(high)
}

/// Results of [`bootstrap_g_theory_pir`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct GTheoryBootstrap {
    /// Resampling facet behind each component's standard error, in
    /// [`GTheoryComponent::ALL`] order ([`g_theory_resampling_plan`]).
    pub plan: [GTheoryFacet; 7],
    /// Standard error of each signed component: the SD of its plan's corrected
    /// replicates. `None` when that stream had a failed fit, or when the
    /// result's components were zeroed.
    pub component_se: [Option<f64>; 7],
    /// Parametric-bootstrap percentile interval for each component; `None`
    /// where failed fits make a bound unidentifiable.
    pub component_ci: [Option<(f64, f64)>; 7],
    /// Closed-form F interval for G at the grid's counts; `None` when unavailable.
    pub g_coefficient_ci: Option<(f64, f64)>,
    /// Parametric-bootstrap percentile interval for the clipped Phi estimator.
    pub phi_coefficient_ci: Option<(f64, f64)>,
    /// Draws per stream: three facet streams and one parametric stream.
    pub n_resamples: usize,
    /// Two-sided level: bounds are the `alpha/2` and `1 − alpha/2` points.
    pub alpha: f64,
    /// Failed fits in the p, i and r facet streams.
    pub facet_skipped: [usize; 3],
    /// Failed fits in the parametric stream.
    pub parametric_skipped: usize,
    /// Parametric replicates whose clipped Phi was 0/0.
    pub phi_coefficient_failed: usize,
}

/// Bootstrap standard errors and intervals for the components, G and Phi.
///
/// Four procedures, each matched to the evidence for it (salib #5; research
/// notes `papers/2026-10-23-gtheory-bootstrap-performance.md` and
/// `papers/2026-10-23-pigeonhole-bootstrap-theory.md`):
///
/// - **Component standard errors.** Each component resamples one facet,
///   chosen by [`g_theory_resampling_plan`] (Tong & Brennan, CASMA 5/15). Each
///   replicate is corrected with Brennan's CASMA 21 formula, so the corrected
///   replicates centre exactly on the observed estimates, and the standard
///   error is their SD. Tong & Brennan validated these standard errors for
///   normal data at 100 × 20 × 2 only; with few rater levels the rater
///   standard error is too small (SD ratio about 0.6–0.8 at 20 × 8 × 2–4).
/// - **Component intervals.** Parametric bootstrap: the point components,
///   with negatives zeroed, generate normal effect arrays at the grid's
///   counts. Each array is refitted, and the interval is the percentile range
///   of the signed replicate estimates.
/// - **G interval.** Closed-form F interval from the observed mean squares
///   (see `g_f_interval` in the source). It holds at the grid's counts only.
/// - **Phi interval.** Percentile interval of the clipped Phi over the
///   parametric replicates.
///
/// None of these intervals is claimed to be calibrated beyond the coverage
/// study recorded in salib's `papers/`. Non-normal data, near-zero components,
/// facets with two or three levels and D-study counts are unsupported.
///
/// A failed replicate is not dropped: each percentile bound holds for any
/// value the failed replicates could have had, and it is unavailable once
/// failures reach the tail fraction. A constant resample counts as having
/// every component zero.
///
/// Every call draws one word from `rng` as a nonce and uses child streams
/// forked from it: `g-theory/boot-p`, `/boot-i`, `/boot-r` and `/parametric`.
/// `n_resamples` applies to each stream. Results are bit-reproducible for a
/// given `rng`, platform and statrs version.
///
/// # Errors
///
/// Returns [`GTheoryBootstrapError`] if the original grid or arguments are
/// invalid, or if every draw of a stream fails.
pub fn bootstrap_g_theory_pir(
    grid: ArrayView3<'_, f64>,
    design: GTheoryDesign,
    n_resamples: usize,
    alpha: f64,
    rng: &mut RngState,
) -> Result<GTheoryBootstrap, GTheoryBootstrapError> {
    let point = estimate_g_theory_pir(grid, design)?;
    if n_resamples == 0 {
        return Err(GTheoryBootstrapError::Bootstrap(
            BootstrapGivenDataError::ZeroResamples,
        ));
    }
    if !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
        return Err(GTheoryBootstrapError::Bootstrap(
            BootstrapGivenDataError::OutOfRangeAlpha { alpha },
        ));
    }
    let sizes = [grid.shape()[0], grid.shape()[1], grid.shape()[2]];
    // Each call consumes one parent word as a nonce, before any error return,
    // so a retry after a failure draws fresh streams.
    let mut parent = rng.clone().into_chacha();
    let nonce = parent.next_u64().to_le_bytes();
    let base = rng.clone();
    *rng = RngState::snapshot(&parent, rng);
    let child = |name: &[u8]| base.fork(&[name, &nonce].concat()).into_chacha();

    // Component standard errors: one corrected single-facet stream per facet.
    let plan = g_theory_resampling_plan(sizes);
    let mut component_se = [None; 7];
    let mut facet_skipped = [0usize; 3];
    let mut resampled = Array3::<f64>::zeros(grid.raw_dim());
    for facet in GTheoryFacet::ALL {
        let axis = facet.axis();
        let mut stream = child(match facet {
            GTheoryFacet::P => b"g-theory/boot-p".as_slice(),
            GTheoryFacet::I => b"g-theory/boot-i".as_slice(),
            GTheoryFacet::R => b"g-theory/boot-r".as_slice(),
        });
        let mut draw = vec![0usize; sizes[axis]];
        let mut replicates: Vec<[f64; 7]> = Vec::with_capacity(n_resamples);
        for _ in 0..n_resamples {
            for level in &mut draw {
                *level = (stream.next_u32() as usize) % sizes[axis];
            }
            for ((a, b, c), value) in resampled.indexed_iter_mut() {
                let mut src = [a, b, c];
                src[axis] = draw[src[axis]];
                *value = grid[src];
            }
            match fit_replicate(resampled.view(), design) {
                Ok(fit) => replicates.push(correct_single_facet(fit.components(), facet, sizes)),
                Err(_) => facet_skipped[axis] += 1,
            }
        }
        if replicates.is_empty() {
            return Err(GTheoryBootstrapError::AllResamplesFailed);
        }
        if facet_skipped[axis] == 0 && n_resamples >= 2 {
            let m = n_resamples as f64;
            for k in (0..7).filter(|&k| plan[k] == facet) {
                // Two-pass: the mean can be large relative to the spread.
                let mean = replicates.iter().map(|x| x[k]).sum::<f64>() / m;
                let ss = replicates
                    .iter()
                    .map(|x| (x[k] - mean).powi(2))
                    .sum::<f64>();
                component_se[k] = Some((ss / (m - 1.0)).sqrt());
            }
        }
    }

    // Parametric stream: normal effect arrays from the zeroed point components.
    let generating = point.with_negative_components_zeroed().components();
    let scale = generating.map(f64::sqrt);
    let mut stream = child(b"g-theory/parametric");
    let mut component_samples: Vec<Vec<f64>> =
        (0..7).map(|_| Vec::with_capacity(n_resamples)).collect();
    let mut phi_samples = Vec::with_capacity(n_resamples);
    let mut parametric_skipped = 0usize;
    let mut phi_failed = 0usize;
    let mut generator = ParametricGenerator::new(sizes);
    for _ in 0..n_resamples {
        generator.fill(&mut resampled, &scale, &mut stream);
        match fit_replicate(resampled.view(), design) {
            Ok(fit) => {
                for (samples, value) in component_samples.iter_mut().zip(fit.components()) {
                    samples.push(value);
                }
                match fit.with_negative_components_zeroed().phi_coefficient {
                    Ok(phi) => phi_samples.push(phi),
                    Err(_) => phi_failed += 1,
                }
            }
            Err(_) => parametric_skipped += 1,
        }
    }
    if parametric_skipped == n_resamples {
        return Err(GTheoryBootstrapError::AllResamplesFailed);
    }
    Ok(GTheoryBootstrap {
        plan,
        component_se,
        component_ci: std::array::from_fn(|k| {
            bounded_percentiles(&component_samples[k], parametric_skipped, alpha)
        }),
        g_coefficient_ci: g_f_interval(point.components(), sizes, alpha),
        phi_coefficient_ci: bounded_percentiles(
            &phi_samples,
            n_resamples - phi_samples.len(),
            alpha,
        ),
        n_resamples,
        alpha,
        facet_skipped,
        parametric_skipped,
        phi_coefficient_failed: phi_failed,
    })
}

/// Project G and Phi to new item/rater counts from a result's components.
///
/// No model evaluations are performed. The components are used as stored:
/// signed estimates make a coefficient unavailable exactly as in
/// [`estimate_g_theory_pir`], and a
/// [`with_negative_components_zeroed`](GTheoryResult::with_negative_components_zeroed)
/// copy projects the truncated components.
///
/// # Errors
///
/// [`GTheoryError::DegenerateAxis`] when either count is zero. An unavailable
/// coefficient is reported in the returned point, not as an error.
pub fn project_g_theory_d_study(
    result: &GTheoryResult,
    n_items: usize,
    n_raters: usize,
) -> Result<DStudyPoint, GTheoryError> {
    check_counts(n_items, n_raters)?;
    let (g_coefficient, phi_coefficient) = coefficients(&result.components(), n_items, n_raters);
    Ok(DStudyPoint {
        n_items,
        n_raters,
        g_coefficient,
        phi_coefficient,
    })
}

/// A grid of D-study projections over item-count and rater-count
/// combinations.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DStudySurface {
    /// Projected reliability points, one per (n_items, n_raters) pair.
    pub points: Vec<DStudyPoint>,
}

/// Compute a D-study surface: project G and Phi for every combination
/// of `item_counts` and `rater_counts`.
///
/// Returns `item_counts.len() * rater_counts.len()` projected points
/// in row-major order (items vary slowest, raters vary fastest).
///
/// # Errors
///
/// [`GTheoryError::DegenerateAxis`] when any count is zero.
pub fn d_study_surface(
    result: &GTheoryResult,
    item_counts: &[usize],
    rater_counts: &[usize],
) -> Result<DStudySurface, GTheoryError> {
    let mut points = Vec::with_capacity(item_counts.len() * rater_counts.len());
    for &ni in item_counts {
        for &nr in rater_counts {
            points.push(project_g_theory_d_study(result, ni, nr)?);
        }
    }
    Ok(DStudySurface { points })
}

/// Find the minimum-cost design that achieves at least `target_phi`.
///
/// Searches all `(n_items, n_raters)` combinations with
/// `1 <= n_items <= max_items` and `1 <= n_raters <= max_raters`.
/// Returns the design with the lowest `cost_fn(n_items, n_raters)`
/// that achieves `phi_coefficient >= target_phi`, or `None` if no
/// feasible design exists in the search grid. Ties keep the first design
/// found, in items-then-raters order.
///
/// # Cost function
///
/// The cost function maps `(n_items, n_raters) -> cost`. A typical
/// cost function for eval campaigns is `n_items * n_raters * cost_per_cell`.
///
/// # Errors
///
/// [`GTheoryError::Coefficient`] when Phi is unavailable at a searched design.
/// A negative component or a 0/0 Phi does not depend on the counts, so the
/// search stops at the first one. Use
/// [`GTheoryResult::with_negative_components_zeroed`] first to search under
/// the truncation convention. With σ²(p) = 0 and counts near 1e16 or more,
/// error terms can underflow to zero and make Phi 0/0 at those counts only.
pub fn find_minimum_design<F>(
    result: &GTheoryResult,
    target_phi: f64,
    max_items: usize,
    max_raters: usize,
    cost_fn: F,
) -> Result<Option<DStudyPoint>, GTheoryError>
where
    F: Fn(usize, usize) -> f64,
{
    let mut best: Option<(DStudyPoint, f64)> = None;
    for ni in 1..=max_items {
        for nr in 1..=max_raters {
            let point = project_g_theory_d_study(result, ni, nr)?;
            let phi = point.phi_coefficient?;
            if phi >= target_phi {
                let cost = cost_fn(ni, nr);
                if best.as_ref().is_none_or(|(_, bc)| cost < *bc) {
                    best = Some((point, cost));
                }
            }
        }
    }
    Ok(best.map(|(p, _)| p))
}

fn check_counts(n_items: usize, n_raters: usize) -> Result<(), GTheoryError> {
    for (axis, len) in [("n_items", n_items), ("n_raters", n_raters)] {
        if len < 1 {
            return Err(GTheoryError::DegenerateAxis { axis, len, min: 1 });
        }
    }
    Ok(())
}

/// G and Phi for nonnegative components at positive counts.
///
/// Phi's denominator extends G's by nonnegative terms, so the rounded
/// coefficients keep `0 <= Phi <= G <= 1`.
fn coefficients(
    components: &[f64; 7],
    n_items: usize,
    n_raters: usize,
) -> (GTheoryCoefficient, GTheoryCoefficient) {
    // Adding +0.0 turns a hand-set -0.0 into +0.0.
    let [p, i, r, pi, pr, ir, pir] = components.map(|c| c + 0.0);
    let ni = n_items as f64;
    let nr = n_raters as f64;
    let admissible = |used: &[usize]| -> Result<(), GTheoryCoefficientError> {
        if used.iter().any(|&k| !components[k].is_finite()) {
            return Err(GTheoryCoefficientError::NonFinite);
        }
        match used.iter().find(|&&k| components[k] < 0.0) {
            Some(&k) => Err(GTheoryCoefficientError::NegativeComponent {
                component: GTheoryComponent::ALL[k],
            }),
            None => Ok(()),
        }
    };
    let ratio = |denominator: f64| -> GTheoryCoefficient {
        if !denominator.is_finite() {
            Err(GTheoryCoefficientError::NonFinite)
        } else if denominator == 0.0 {
            Err(GTheoryCoefficientError::ZeroDenominator)
        } else {
            Ok(p / denominator)
        }
    };
    let relative = p + pi / ni + pr / nr + pir / (ni * nr);
    let g = admissible(&[0, 3, 4, 6]).and_then(|()| ratio(relative));
    let absolute = relative + i / ni + r / nr + ir / (ni * nr);
    let phi = admissible(&[0, 1, 2, 3, 4, 5, 6]).and_then(|()| ratio(absolute));
    (g, phi)
}

/// Percentile bounds of `values` plus `n_failed` replicates of unknown value.
///
/// With all `n` replicates sorted, the `alpha/2` and `1 − alpha/2` percentiles
/// sit at positions `alpha/2 · (n − 1)` and `(1 − alpha/2) · (n − 1)`. Placing
/// every failure below the survivors gives the lowest possible lower bound;
/// placing them above gives the highest possible upper bound. `None` when
/// either bound would fall on a failed replicate. With no failures this is
/// the ordinary percentile interval.
fn bounded_percentiles(values: &[f64], n_failed: usize, alpha: f64) -> Option<(f64, f64)> {
    let m = values.len();
    if m == 0 {
        return None;
    }
    let last = (m + n_failed - 1) as f64;
    let low = alpha / 2.0 * last - n_failed as f64;
    let high = (1.0 - alpha / 2.0) * last;
    let top = (m - 1) as f64;
    if low < 0.0 || high > top {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    if m == 1 {
        return Some((sorted[0], sorted[0]));
    }
    Some((
        percentile_value(&sorted, low / top),
        percentile_value(&sorted, high / top),
    ))
}

#[cfg(test)]
mod tests {
    use ndarray::Array3;
    use salib_core::RngState;

    use super::*;

    fn grid() -> Array3<f64> {
        let mut grid = Array3::<f64>::zeros((2, 2, 2));
        let levels = [-1.0_f64, 1.0_f64];
        for (ip, &p) in levels.iter().enumerate() {
            for (ii, &i) in levels.iter().enumerate() {
                for (ir, &r) in levels.iter().enumerate() {
                    grid[[ip, ii, ir]] = 50.0
                        + 6.0 * p
                        + 4.0 * i
                        + 2.0 * r
                        + 3.0 * p * i
                        + 1.5 * p * r
                        + 1.0 * i * r
                        + 0.5 * p * i * r;
                }
            }
        }
        grid
    }

    #[test]
    fn nonfinite_observations_error() {
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut grid = grid();
            grid[[1, 0, 1]] = invalid;
            assert_eq!(
                estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap_err(),
                GTheoryError::NonFiniteObservation
            );
        }
    }

    #[test]
    fn identical_observations_error_at_any_magnitude() {
        for value in [1.0, 0.1, 1e-300, 1e300] {
            let grid = Array3::<f64>::from_elem((2, 2, 2), value);
            assert_eq!(
                estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap_err(),
                GTheoryError::ZeroVariance
            );
        }
    }

    #[test]
    fn unrepresentable_components_error_and_extreme_finite_data_fit() {
        for scale in [1e300, 1e-300] {
            let grid = grid().mapv(|y| (y - 50.0) * scale);
            assert_eq!(
                estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap_err(),
                GTheoryError::ScaleOutOfRange
            );
        }
        // Sums of observations near f64::MAX would overflow without rescaling.
        let near_max = grid().mapv(|y| y * 1e305);
        let r = estimate_g_theory_pir(near_max.view(), GTheoryDesign::Crossed);
        assert_eq!(r.unwrap_err(), GTheoryError::ScaleOutOfRange);
        let base = estimate_g_theory_pir(grid().view(), GTheoryDesign::Crossed).unwrap();
        let wide = estimate_g_theory_pir(grid().mapv(|y| y * 1e140).view(), GTheoryDesign::Crossed)
            .unwrap();
        // 1e140 is not a power of two, so the scaled observations are rounded.
        for (a, b) in [
            (wide.g_coefficient, base.g_coefficient),
            (wide.phi_coefficient, base.phi_coefficient),
        ] {
            assert!((a.unwrap() - b.unwrap()).abs() <= 4.0 * f64::EPSILON);
        }
    }

    #[test]
    fn binary_scaling_is_exact() {
        for v in [
            1.0,
            1.5,
            3.0,
            f64::MAX,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            1e-310,
        ] {
            let e = binary_exponent(v);
            let unit = scale_by_pow2(v, -e);
            assert!((1.0..2.0).contains(&unit), "{v:e} -> {unit}");
            assert_eq!(scale_by_pow2(unit, e), v);
        }
    }

    #[test]
    fn failed_replicates_widen_bounds_and_can_withhold_them() {
        let values: Vec<f64> = (0..100).map(f64::from).collect();
        let (low, high) = bounded_percentiles(&values, 0, 0.1).unwrap();
        assert!((low - 4.95).abs() < 1e-12 && (high - 94.05).abs() < 1e-12);
        let (wlow, whigh) = bounded_percentiles(&values, 3, 0.1).unwrap();
        assert!(wlow < low && whigh > high, "{wlow} {whigh}");
        // 103 replicates: lower position 0.05 * 102 = 5.1, minus 3 failures.
        assert!((wlow - 2.1).abs() < 1e-12);
        assert!((whigh - 96.9).abs() < 1e-12);
        assert_eq!(bounded_percentiles(&values, 6, 0.1), None);
        assert_eq!(bounded_percentiles(&[], 0, 0.1), None);
    }

    #[test]
    fn unsupported_design_errors() {
        let err = estimate_g_theory_pir(grid().view(), GTheoryDesign::Nested).unwrap_err();
        assert_eq!(err, GTheoryError::UnsupportedDesign);
    }

    #[test]
    fn zero_counts_error() {
        let base = estimate_g_theory_pir(grid().view(), GTheoryDesign::Crossed).unwrap();
        assert!(matches!(
            project_g_theory_d_study(&base, 0, 2),
            Err(GTheoryError::DegenerateAxis {
                axis: "n_items",
                ..
            })
        ));
        assert!(matches!(
            GTheoryResult::from_components(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2, 0),
            Err(GTheoryError::DegenerateAxis {
                axis: "n_raters",
                ..
            })
        ));
    }

    #[test]
    fn supplied_components_are_validated() {
        assert_eq!(
            GTheoryResult::from_components(1.0, f64::NAN, 1.0, 1.0, 1.0, 1.0, 1.0, 2, 2)
                .unwrap_err(),
            GTheoryError::NonFiniteComponent {
                component: GTheoryComponent::I
            }
        );
        assert_eq!(
            GTheoryResult::from_components(f64::MAX, f64::MAX, 0.0, 0.0, 0.0, 0.0, 0.0, 1, 1)
                .unwrap_err(),
            GTheoryError::ScaleOutOfRange
        );
        let r = GTheoryResult::from_components(-0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1, 1).unwrap();
        assert!(r.sigma_p.is_sign_positive());
        assert_eq!(
            r.g_coefficient,
            Err(GTheoryCoefficientError::ZeroDenominator)
        );
        assert_eq!(r.phi_coefficient, Ok(0.0));
    }

    #[test]
    fn tampered_nonfinite_component_makes_coefficient_unavailable() {
        let mut r =
            GTheoryResult::from_components(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2, 2).unwrap();
        r.sigma_i = f64::NAN;
        let d = project_g_theory_d_study(&r, 2, 2).unwrap();
        assert!(d.g_coefficient.is_ok(), "G does not use sigma_i");
        assert_eq!(d.phi_coefficient, Err(GTheoryCoefficientError::NonFinite));

        // A hand-set -0.0 must not leak into a coefficient as -0.0.
        r.sigma_i = 1.0;
        r.sigma_p = -0.0;
        let d = project_g_theory_d_study(&r, 2, 2).unwrap();
        assert!(d.g_coefficient.unwrap().is_sign_positive());
    }

    #[test]
    fn subnormal_supplied_component_is_out_of_range() {
        // Otherwise Phi would be Ok(0) at (1, 1) and 0/0 at (1, 2).
        assert_eq!(
            GTheoryResult::from_components(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5e-324, 1, 1).unwrap_err(),
            GTheoryError::ScaleOutOfRange
        );
    }

    #[test]
    fn bootstrap_invalid_params_error() {
        let mut rng = RngState::from_seed([0x55; 32]);
        let err = bootstrap_g_theory_pir(grid().view(), GTheoryDesign::Crossed, 0, 0.05, &mut rng)
            .unwrap_err();
        assert_eq!(
            err,
            GTheoryBootstrapError::Bootstrap(BootstrapGivenDataError::ZeroResamples)
        );
    }

    mod sage_oracle {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/oracles/g_theory_single_facet.rs"
        ));
    }

    /// Every single-facet resample of each Sage case, through salib's own
    /// resampling, fit and correction: the corrected mean must equal the exact
    /// observed estimates and the variance the exact replicate variance
    /// (CASMA 21 boot-λ T-term system, solved in Sage).
    #[test]
    fn single_facet_corrections_match_exact_enumeration() {
        for case in sage_oracle::SINGLE_FACET_CASES {
            let (np, ni, nr) = case.shape;
            let sizes = [np, ni, nr];
            let grid = Array3::from_shape_vec(case.shape, case.data.to_vec()).unwrap();
            let facet = match case.facet {
                'p' => GTheoryFacet::P,
                'i' => GTheoryFacet::I,
                _ => GTheoryFacet::R,
            };
            let axis = facet.axis();
            let n = sizes[axis];
            let total = n.pow(u32::try_from(n).unwrap());
            assert_eq!(total, case.resamples);
            let mut replicates = Vec::with_capacity(total);
            let mut resampled = Array3::<f64>::zeros(grid.raw_dim());
            for code in 0..total {
                let draw: Vec<usize> = (0..n)
                    .map(|d| code / n.pow(u32::try_from(d).unwrap()) % n)
                    .collect();
                for ((a, b, c), value) in resampled.indexed_iter_mut() {
                    let mut src = [a, b, c];
                    src[axis] = draw[src[axis]];
                    *value = grid[src];
                }
                let fit = fit_replicate(resampled.view(), GTheoryDesign::Crossed).unwrap();
                replicates.push(correct_single_facet(fit.components(), facet, sizes));
            }
            let m = total as f64;
            // Each replicate carries rounding of order N·ε·scale²; summing m of
            // them adds m·ε·scale². The bound is a generous multiple of both.
            let scale = replicates
                .iter()
                .flatten()
                .fold(0.0_f64, |a, v| a.max(v.abs()));
            let n_cells = (np * ni * nr) as f64;
            let tol = 64.0 * (n_cells + m) * f64::EPSILON * scale.max(1.0);
            for k in 0..7 {
                let mean = replicates.iter().map(|x| x[k]).sum::<f64>() / m;
                let variance = replicates
                    .iter()
                    .map(|x| (x[k] - mean).powi(2))
                    .sum::<f64>()
                    / m;
                assert!(
                    (mean - case.estimates[k]).abs() <= tol,
                    "{:?} {} component {k}: mean {mean}, exact {}",
                    case.shape,
                    case.facet,
                    case.estimates[k]
                );
                assert!(
                    (variance - case.replicate_variance[k]).abs() <= tol * scale.max(1.0),
                    "{:?} {} component {k}: variance {variance}, exact {}",
                    case.shape,
                    case.facet,
                    case.replicate_variance[k]
                );
            }
        }
    }

    mod sage_f_oracle {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/oracles/g_theory_f_interval.rs"
        ));
    }

    /// Pinned against 50-digit F quantiles computed in Sage. Catches a swapped
    /// degrees-of-freedom pair or a wrong quantile; statrs' cdf is accurate to
    /// about 1e-12, which bounds the error of the bisected quantiles.
    #[test]
    fn f_interval_matches_high_precision_oracle() {
        for case in sage_f_oracle::F_INTERVAL_CASES {
            let (_, nu) = g_f_parameters(case.components, case.sizes).unwrap();
            assert!(
                (nu - case.nu).abs() <= 1e-12 * case.nu,
                "{:?}: nu {nu} vs {}",
                case.sizes,
                case.nu
            );
            let got = g_f_interval(case.components, case.sizes, case.alpha).unwrap();
            for (a, b) in [(got.0, case.interval.0), (got.1, case.interval.1)] {
                assert!(
                    (a - b).abs() <= 1e-9,
                    "{:?}: got {got:?}, exact {:?}",
                    case.sizes,
                    case.interval
                );
            }
        }
    }

    #[test]
    fn resampling_plan_follows_tong_brennan_rules() {
        use GTheoryFacet::{I, P, R};
        assert_eq!(g_theory_resampling_plan([20, 8, 4]), [P, I, R, P, P, I, P]);
        // Ties fall to p, then i (salib's choice).
        assert_eq!(g_theory_resampling_plan([5, 5, 5]), [P, I, R, P, P, I, P]);
        assert_eq!(g_theory_resampling_plan([2, 10, 3]), [P, I, R, I, R, I, I]);
    }

    #[test]
    fn f_interval_contains_point_and_guards_its_domain() {
        let point = estimate_g_theory_pir(
            Array3::from_shape_fn((3, 3, 2), |(p, i, r)| {
                (p * 3 + i * 2 + r + (p * i) % 2) as f64
            })
            .view(),
            GTheoryDesign::Crossed,
        )
        .unwrap();
        let g = point.g_coefficient.unwrap();
        let wide = g_f_interval(point.components(), [3, 3, 2], 0.05).unwrap();
        let narrow = g_f_interval(point.components(), [3, 3, 2], 0.2).unwrap();
        assert!(
            0.0 <= wide.0 && wide.0 < narrow.0 && narrow.0 < g,
            "{wide:?} {narrow:?} {g}"
        );
        assert!(
            g < narrow.1 && narrow.1 < wide.1 && wide.1 < 1.0,
            "{wide:?} {narrow:?} {g}"
        );
        // L = MS_pi + MS_pr - MS_pir <= 0: no interval.
        assert_eq!(
            g_f_interval([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], [5, 4, 3], 0.05),
            None
        );
        assert_eq!(
            g_f_interval([1.0, 0.0, 0.0, -0.5, 0.0, 0.0, 0.1], [5, 4, 3], 0.05),
            None
        );
        // With σ²(pi) = σ²(pr) = 0 at 2×2×2, ν̂ = 1/3 < 0.5: no interval.
        assert_eq!(
            g_f_interval([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], [2, 2, 2], 0.05),
            None
        );
        // A tiny person component pushes the lower bound below 0, where it is clipped.
        let low = g_f_interval([0.001, 0.1, 0.1, 1.0, 1.0, 0.1, 1.0], [20, 5, 2], 0.05).unwrap();
        assert_eq!(low.0, 0.0);
        // With no person variance both raw bounds can fall below 0; clipping
        // keeps the interval ordered within [0, 1].
        let zero = g_f_interval([0.0, 0.1, 0.1, 1.0, 1.0, 0.1, 1.0], [20, 5, 2], 0.05).unwrap();
        assert!(
            0.0 <= zero.0 && zero.0 <= zero.1 && zero.1 <= 1.0,
            "{zero:?}"
        );
    }

    /// The parametric generator's mean squares have the balanced normal-theory
    /// law: E MS_e = EMS_e and Var MS_e = 2 EMS_e²/df_e (within 4 MCSE).
    #[test]
    fn parametric_generator_has_normal_theory_mean_square_moments() {
        let sizes = [6, 4, 3];
        let [np, ni, nr] = sizes.map(|v| v as f64);
        let theta = [1.0, 0.5, 0.3, 0.8, 0.2, 0.4, 1.0];
        let [p, i, r, pi, pr, ir, pir] = theta;
        let ems = [
            pir + nr * pi + ni * pr + ni * nr * p,
            pir + nr * pi + np * ir + np * nr * i,
            pir + ni * pr + np * ir + np * ni * r,
            pir + nr * pi,
            pir + ni * pr,
            pir + np * ir,
            pir,
        ];
        let df = [
            np - 1.0,
            ni - 1.0,
            nr - 1.0,
            (np - 1.0) * (ni - 1.0),
            (np - 1.0) * (nr - 1.0),
            (ni - 1.0) * (nr - 1.0),
            (np - 1.0) * (ni - 1.0) * (nr - 1.0),
        ];
        let mut generator = ParametricGenerator::new(sizes);
        let mut rng = RngState::from_seed([0x5a; 32]).into_chacha();
        let mut out = Array3::<f64>::zeros((6, 4, 3));
        let reps = 20_000;
        let mut ms: Vec<[f64; 7]> = Vec::with_capacity(reps);
        for _ in 0..reps {
            generator.fill(&mut out, &theta.map(f64::sqrt), &mut rng);
            let c = estimate_g_theory_pir(out.view(), GTheoryDesign::Crossed)
                .unwrap()
                .components();
            let [p, i, r, pi, pr, ir, pir] = c;
            ms.push([
                pir + nr * pi + ni * pr + ni * nr * p,
                pir + nr * pi + np * ir + np * nr * i,
                pir + ni * pr + np * ir + np * ni * r,
                pir + nr * pi,
                pir + ni * pr,
                pir + np * ir,
                pir,
            ]);
        }
        let n = reps as f64;
        for k in 0..7 {
            let variance = 2.0 * ems[k] * ems[k] / df[k];
            let mean = ms.iter().map(|m| m[k]).sum::<f64>() / n;
            let sample_var = ms.iter().map(|m| (m[k] - mean).powi(2)).sum::<f64>() / (n - 1.0);
            assert!(
                (mean - ems[k]).abs() <= 4.0 * (variance / n).sqrt(),
                "MS {k} mean {mean} vs {}",
                ems[k]
            );
            // Scaled χ²_df/df has excess kurtosis 12/df.
            let se_var = variance * ((2.0 + 12.0 / df[k]) / n).sqrt();
            assert!(
                (sample_var - variance).abs() <= 4.0 * se_var,
                "MS {k} var {sample_var} vs {variance}"
            );
        }
    }

    #[test]
    fn bootstrap_is_reproducible_and_advances_its_state() {
        let mut a = RngState::from_seed([0x44; 32]);
        let mut b = RngState::from_seed([0x44; 32]);
        let first = bootstrap_g_theory_pir(grid().view(), GTheoryDesign::Crossed, 64, 0.05, &mut a)
            .unwrap();
        let same = bootstrap_g_theory_pir(grid().view(), GTheoryDesign::Crossed, 64, 0.05, &mut b)
            .unwrap();
        assert_eq!(first, same);
        assert_eq!(a, b);
        let second =
            bootstrap_g_theory_pir(grid().view(), GTheoryDesign::Crossed, 64, 0.05, &mut a)
                .unwrap();
        assert_ne!(first.component_ci, second.component_ci);
    }

    #[test]
    fn d_study_projection_increases_with_more_items_and_raters() {
        let base = estimate_g_theory_pir(grid().view(), GTheoryDesign::Crossed).unwrap();
        let current = project_g_theory_d_study(&base, 2, 2).unwrap();
        let expanded = project_g_theory_d_study(&base, 4, 4).unwrap();
        let (g0, phi0) = (
            current.g_coefficient.unwrap(),
            current.phi_coefficient.unwrap(),
        );
        let (g1, phi1) = (
            expanded.g_coefficient.unwrap(),
            expanded.phi_coefficient.unwrap(),
        );
        assert!(g1 > g0 && phi1 > phi0);
        assert!(g0 > phi0 && g1 > phi1);
    }
}
