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
//! with Brennan's exact bias correction, Satterthwaite component intervals, a
//! closed-form F interval for G and a fiducial (generalized pivotal quantity)
//! interval for Phi.
//! Their calibration is established only where salib's coverage study says so. [crolib](https://github.com/antimemeai/crolib) develops
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
use statrs::distribution::{ChiSquared, ContinuousCDF, FisherSnedecor};
use statrs::function::gamma::ln_gamma;

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

    /// Interval for Phi from [`GTheoryBootstrap::phi_coefficient_ci`], whenever
    /// it exists. It is an interval for the parameter, so it does not depend on
    /// whether this result's own Phi is available. Read
    /// [`bootstrap_g_theory_pir`] for where it is reliable.
    #[must_use]
    pub fn phi_coefficient_interval(&self) -> Option<(f64, f64)> {
        self.bootstrap.as_ref()?.phi_coefficient_ci
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

/// Binary exponent of the largest component magnitude, or 0 when all are zero.
fn unit_exponent(components: &[f64; 7]) -> i32 {
    let max = components.iter().fold(0.0_f64, |m, c| m.max(c.abs()));
    if max > 0.0 {
        binary_exponent(max)
    } else {
        0
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

/// Standard uniform on the open interval (0, 1), from 52 random bits: the
/// values (k + ½)·2⁻⁵² lie in [2⁻⁵³, 1 − 2⁻⁵³] and are exact. With 53 bits,
/// k + ½ rounds up to 2⁵³ at the top word and the result would be 1.
fn open_unit(rng: &mut impl RngCore) -> f64 {
    ((rng.next_u64() >> 12) as f64 + 0.5) / (1_u64 << 52) as f64
}

/// Standard normal deviate, by inverting the normal cdf.
fn standard_normal(rng: &mut impl RngCore) -> f64 {
    salib_core::Distribution::Normal {
        mu: 0.0,
        sigma: 1.0,
    }
    .quantile(open_unit(rng))
}

/// χ²(df) deviate: twice a Gamma(df/2) deviate from Marsaglia & Tsang (2000),
/// with the `U^(1/a)` boost for shape a < 1.
fn chi_square(df: f64, rng: &mut impl RngCore) -> f64 {
    fn gamma(shape: f64, rng: &mut impl RngCore) -> f64 {
        if shape < 1.0 {
            return gamma(shape + 1.0, rng) * open_unit(rng).powf(1.0 / shape);
        }
        let d = shape - 1.0 / 3.0;
        let c = 1.0 / (9.0 * d).sqrt();
        loop {
            let x = standard_normal(rng);
            let v = (1.0 + c * x).powi(3);
            if v > 0.0 && open_unit(rng).ln() < 0.5 * x * x + d - d * v + d * v.ln() {
                return d * v;
            }
        }
    }
    2.0 * gamma(df / 2.0, rng)
}

/// Expected mean squares at the given counts, in [`GTheoryComponent::ALL`]
/// order (Brennan 2001). Given the estimates, they are the observed mean squares.
fn expected_mean_squares(components: &[f64; 7], sizes: [usize; 3]) -> [f64; 7] {
    let [p, i, r, pi, pr, ir, pir] = *components;
    let [np, ni, nr] = sizes.map(|v| v as f64);
    [
        pir + nr * pi + ni * pr + ni * nr * p,
        pir + nr * pi + np * ir + np * nr * i,
        pir + ni * pr + np * ir + np * ni * r,
        pir + nr * pi,
        pir + ni * pr,
        pir + np * ir,
        pir,
    ]
}

/// Mean-square degrees of freedom, in [`GTheoryComponent::ALL`] order.
fn degrees_of_freedom(sizes: [usize; 3]) -> [f64; 7] {
    let [fp, fi, fr] = sizes.map(|v| v as f64 - 1.0);
    [fp, fi, fr, fp * fi, fp * fr, fi * fr, fp * fi * fr]
}

/// The expected-mean-square equations solved for the components.
fn components_from_mean_squares(ms: &[f64; 7], sizes: [usize; 3]) -> [f64; 7] {
    let [p, i, r, pi, pr, ir, pir] = *ms;
    let [np, ni, nr] = sizes.map(|v| v as f64);
    [
        (p - pi - pr + pir) / (ni * nr),
        (i - pi - ir + pir) / (np * nr),
        (r - pr - ir + pir) / (np * ni),
        (pi - pir) / nr,
        (pr - pir) / ni,
        (ir - pir) / np,
        pir,
    ]
}

/// Fiducial generalized pivotal quantity for the components (Hannig, Iyer &
/// Patterson 2006, JASA 101:254, Theorem 2). Under the balanced normal model
/// `SS_j/EMS_j ~ χ²(df_j)` independently, so `R_j = MS_j df_j / U_j` with
/// `U_j ~ χ²(df_j)` is a fiducial quantity for `EMS_j`; the components follow
/// from the EMS equations. With `U_j = df_j` it returns the estimates.
fn fiducial_components(
    ms: &[f64; 7],
    df: &[f64; 7],
    pivots: &[f64; 7],
    sizes: [usize; 3],
) -> [f64; 7] {
    components_from_mean_squares(&std::array::from_fn(|j| ms[j] * df[j] / pivots[j]), sizes)
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
    // Both outputs are scale-free. At unit scale the mean squares stay
    // finite, and ν is formed from ratios so that no square under- or overflows.
    let exponent = unit_exponent(&components);
    let [p, _, _, pi, pr, _, pir] = components.map(|c| scale_by_pow2(c, -exponent));
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
    let nu = 1.0
        / ((ms_pi / l).powi(2) / df_pi
            + (ms_pr / l).powi(2) / df_pr
            + (ms_pir / l).powi(2) / df_pir);
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

/// χ² cdf that stays accurate near zero. statrs' `ChiSquared::cdf` returns
/// exactly 0 for every x ≤ 1e-20 (and its `Gamma::cdf` panics once x/2
/// underflows), but matches 60-digit values to about 1e-15 from x = 1e-10 up.
/// Below 1e-10 the lower regularized gamma series
/// `P(a, y) = y^a e^(-y)/Γ(a+1) · Σ_k y^k/((a+1)…(a+k))`, with a = ν/2 and
/// y = x/2, converges within three terms.
fn chi_square_cdf(chi: &ChiSquared, nu: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1e-10 {
        return chi.cdf(x);
    }
    // ln y is taken as ln x − ln 2: x/2 rounds to 0 for the smallest subnormals.
    let (a, y) = (nu / 2.0, x / 2.0);
    let series = 1.0 + y / (a + 1.0) * (1.0 + y / (a + 2.0));
    (a * (x.ln() - std::f64::consts::LN_2) - y - ln_gamma(a + 1.0)).exp() * series
}

/// Satterthwaite interval for each component (Brennan, Harris & Hanson 1987,
/// ACT RR 87-7, App. A): `[σ̂ ν/χ²_{1−α/2}(ν), σ̂ ν/χ²_{α/2}(ν)]` with
/// `ν = 2 σ̂²/Σ_j 2 (c_j MS_j)²/df_j`, the mean squares recovered from the
/// components through the expected-mean-square equations. `None` when
/// `σ̂ ≤ 0`. When ν is tiny the interval can lie entirely above σ̂, and
/// either bound can be infinite (both, for ν below about 1e-4 at α = 0.05). Normal theory: in salib's coverage study it is calibrated for
/// σ²(pir), and for σ²(pi), σ²(pr) and σ²(ir) with n_p ≥ 50 and a
/// non-negligible component, but not for σ²(p), σ²(i), σ²(r) or binary data.
fn satterthwaite_intervals(
    components: [f64; 7],
    sizes: [usize; 3],
    alpha: f64,
) -> [Option<(f64, f64)>; 7] {
    // The intervals are scale-equivariant. They are solved at unit scale,
    // where the mean squares stay finite, with ν formed from ratios so that
    // no square under- or overflows; the bounds are then scaled back.
    let exponent = unit_exponent(&components);
    let components = components.map(|c| scale_by_pow2(c, -exponent));
    let [np, ni, nr] = sizes.map(|v| v as f64);
    let ms = expected_mean_squares(&components, sizes);
    let df = degrees_of_freedom(sizes);
    // Mean squares and divisor of each component's EMS estimator.
    let terms: [(&[usize], f64); 7] = [
        (&[0, 3, 4, 6], ni * nr),
        (&[1, 3, 5, 6], np * nr),
        (&[2, 4, 5, 6], np * ni),
        (&[3, 6], nr),
        (&[4, 6], ni),
        (&[5, 6], np),
        (&[6], 1.0),
    ];
    std::array::from_fn(|k| {
        let (used, divisor) = terms[k];
        let estimate = components[k];
        if estimate <= 0.0 {
            return None;
        }
        // ν = 2σ̂²/Σ_j 2(MS_j/d)²/df_j.
        let nu = 1.0
            / used
                .iter()
                .map(|&j| (ms[j] / (divisor * estimate)).powi(2) / df[j])
                .sum::<f64>();
        if !nu.is_finite() {
            return None;
        }
        let chi = ChiSquared::new(nu).ok()?;
        let cdf = |x: f64| chi_square_cdf(&chi, nu, x);
        // A χ² quantile below the normal f64 range is solved in logs from
        // P(χ²_ν ≤ x) = (x/2)^(ν/2) / Γ(ν/2 + 1), exact to relative error O(x)
        // there; the bound σ̂ν/q then overflows to ∞ when it should.
        let bound = |prob: f64| -> Option<f64> {
            if cdf(f64::MIN_POSITIVE) >= prob {
                let a = nu / 2.0;
                let ln_q = std::f64::consts::LN_2 + (prob.ln() + ln_gamma(a + 1.0)) / a;
                Some(scale_by_pow2(((estimate * nu).ln() - ln_q).exp(), exponent))
            } else {
                Some(scale_by_pow2(
                    estimate * nu / quantile_by_bisection(cdf, prob)?,
                    exponent,
                ))
            }
        };
        Some((bound(1.0 - alpha / 2.0)?, bound(alpha / 2.0)?))
    })
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
    /// Satterthwaite interval for each component; `None` when the estimate is
    /// not positive. See [`bootstrap_g_theory_pir`] for where it is calibrated.
    pub component_ci: [Option<(f64, f64)>; 7],
    /// Closed-form F interval for G at the grid's counts; `None` when unavailable.
    pub g_coefficient_ci: Option<(f64, f64)>,
    /// Fiducial (generalized pivotal quantity) percentile interval for Phi.
    pub phi_coefficient_ci: Option<(f64, f64)>,
    /// Draws per stream: three facet streams and one fiducial stream.
    pub n_resamples: usize,
    /// Two-sided level: bounds are the `alpha/2` and `1 − alpha/2` points.
    pub alpha: f64,
    /// Failed fits in the p, i and r facet streams.
    pub facet_skipped: [usize; 3],
    /// Fiducial draws whose clipped Phi was 0/0, which happens only when
    /// every estimated component is zero.
    pub phi_coefficient_failed: usize,
}

/// Bootstrap standard errors and intervals for the components, G and Phi.
///
/// Each quantity uses the procedure the evidence supports (salib #5). The
/// coverage study (90 cells, n_sim 1900, `papers/2026-10-25-g-theory-coverage-study.md`)
/// scopes every claim below.
///
/// - **Component standard errors.** Each component resamples one facet,
///   chosen by [`g_theory_resampling_plan`] (Tong & Brennan, CASMA 5/15), and
///   each replicate is corrected with Brennan's CASMA 21 formula, so the
///   corrected replicates centre exactly on the observed estimates. The
///   standard error is their SD. In the study, the SE ratio to the true SD has
///   a median of 0.96–1.00 for every component on normal data. On binary data
///   the interaction standard errors are too small (0.57–0.84).
/// - **Component intervals.** Satterthwaite, closed form (see
///   `satterthwaite_intervals` in the source). Calibrated for σ²(pir) on
///   normal data, and for σ²(pi), σ²(pr) and σ²(ir) with n_p ≥ 50 when the
///   component is not near zero. Not calibrated for σ²(p) (37/90 cells),
///   σ²(i), σ²(r) or binary data. It replaced parametric-bootstrap
///   percentiles, which sat too low for every component.
/// - **G interval.** Closed-form F interval from the observed mean squares
///   (see `g_f_interval` in the source), at the grid's counts only.
///   Calibrated in 86 of 90 study cells, including all binary cells.
/// - **Phi interval.** Fiducial generalized pivotal quantity (Hannig, Iyer &
///   Patterson 2006): each draw replaces every mean square `MS_j` by
///   `MS_j df_j / U_j` with `U_j ~ χ²(df_j)`, solves the expected-mean-square
///   equations, clips the components at zero and evaluates Phi. The interval
///   is the percentile range of the draws. No data are refitted. Unlike a
///   plug-in bootstrap it carries the uncertainty of every mean square,
///   including a rater mean square with one or two degrees of freedom, so it
///   is wide when raters are few. On normal data its two-sided coverage is
///   .938–.967 in all 81 study cells (median .952). Its misses are lopsided:
///   per cell, the interval lies below Phi in 2.4–6.0% of samples and above
///   it in 0–2.8% (4.3% and 0.6% on average). The lower bound is usually
///   conservative but only about nominal in some large-n_p, few-rater cells.
///   The upper bound behaves as a 94–97.6% one-sided bound. On binary data
///   coverage is .918–.947 (salib #10,
///   `papers/2026-10-25-phi-fiducial-interval.md`).
///
/// Not established: D-study counts, finite universes, near-zero components,
/// and simultaneous coverage.
///
/// A failed replicate is not dropped: each percentile bound holds for any
/// value the failed replicates could have had, and it is unavailable once
/// failures reach the tail fraction. A constant resample counts as having
/// every component zero.
///
/// Every call draws one word from `rng` as a nonce and uses child streams
/// forked from it: `g-theory/boot-p`, `/boot-i`, `/boot-r` and `/fiducial`.
/// `n_resamples` applies to each stream. Results are bit-reproducible for a
/// given `rng`, platform and statrs version.
///
/// # Errors
///
/// Returns [`GTheoryBootstrapError`] if the original grid or arguments are
/// invalid, or if every draw of a facet stream fails. When every fiducial Phi
/// draw is 0/0 (all components zero), the Phi interval is `None`.
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

    // Fiducial stream for Phi. Phi is scale-free, so the draws are made at
    // unit scale, where the mean squares cannot overflow.
    let exponent = unit_exponent(&point.components());
    let unit = point.components().map(|c| scale_by_pow2(c, -exponent));
    let ms = expected_mean_squares(&unit, sizes);
    let df = degrees_of_freedom(sizes);
    let mut stream = child(b"g-theory/fiducial");
    let mut phi_samples = Vec::with_capacity(n_resamples);
    let mut phi_failed = 0usize;
    for _ in 0..n_resamples {
        let pivots = df.map(|d| chi_square(d, &mut stream));
        let draw = fiducial_components(&ms, &df, &pivots, sizes).map(|c| c.max(0.0));
        match coefficients(&draw, sizes[1], sizes[2]).1 {
            Ok(phi) => phi_samples.push(phi),
            Err(_) => phi_failed += 1,
        }
    }
    Ok(GTheoryBootstrap {
        plan,
        component_se,
        component_ci: satterthwaite_intervals(point.components(), sizes, alpha),
        g_coefficient_ci: g_f_interval(point.components(), sizes, alpha),
        phi_coefficient_ci: bounded_percentiles(
            &phi_samples,
            n_resamples - phi_samples.len(),
            alpha,
        ),
        n_resamples,
        alpha,
        facet_skipped,
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

    mod sage_satterthwaite_oracle {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/oracles/g_theory_satterthwaite.rs"
        ));
    }

    /// Pinned against 50-digit χ² quantiles computed in Sage, including a tiny-ν
    /// case whose lower quantile is deep in the tail.
    #[test]
    fn satterthwaite_matches_high_precision_oracle() {
        let mut worst = 0.0_f64;
        for case in sage_satterthwaite_oracle::SATTERTHWAITE_CASES {
            let got = satterthwaite_intervals(case.components, case.sizes, case.alpha);
            for (k, (g, e)) in got.iter().zip(case.intervals).enumerate() {
                match (g, e) {
                    (None, None) => {}
                    (Some(g), Some(e)) => {
                        for (a, b) in [(g.0, e.0), (g.1, e.1)] {
                            if a == b {
                                continue;
                            }
                            let rel = (a - b).abs() / b.abs();
                            worst = worst.max(rel);
                            assert!(
                                rel <= 1e-9,
                                "{:?} component {k}: got {g:?}, exact {e:?}",
                                case.sizes
                            );
                        }
                    }
                    _ => panic!("{:?} component {k}: got {g:?}, exact {e:?}", case.sizes),
                }
            }
        }
        eprintln!("worst relative error {worst:e}");
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
        // Component intervals are closed form; the stochastic outputs differ.
        assert_eq!(first.component_ci, second.component_ci);
        assert_ne!(first.component_se, second.component_se);
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

    /// Exact components of an integer grid. `N·T_x = G_x Σ_g S_g²` is an
    /// integer for every effect x with G_x groups of sums S_g, so each
    /// component is one correctly rounded quotient of integers.
    fn exact_components(k: &Array3<i64>) -> [f64; 7] {
        let (np, ni, nr) = k.dim();
        let n = (np * ni * nr) as i128;
        let t = |key: &dyn Fn(usize, usize, usize) -> usize, groups: usize| -> i128 {
            let mut sums = vec![0_i128; groups];
            for ((a, b, c), &v) in k.indexed_iter() {
                sums[key(a, b, c)] += i128::from(v);
            }
            groups as i128 * sums.iter().map(|s| s * s).sum::<i128>()
        };
        let t0 = t(&|_, _, _| 0, 1);
        let tp = t(&|a, _, _| a, np);
        let ti = t(&|_, b, _| b, ni);
        let tr = t(&|_, _, c| c, nr);
        let tpi = t(&|a, b, _| a * ni + b, np * ni);
        let tpr = t(&|a, _, c| a * nr + c, np * nr);
        let tir = t(&|_, b, c| b * nr + c, ni * nr);
        let tpir = t(&|a, b, c| (a * ni + b) * nr + c, np * ni * nr);
        let ss = [
            tp - t0,
            ti - t0,
            tr - t0,
            tpi - tp - ti + t0,
            tpr - tp - tr + t0,
            tir - ti - tr + t0,
            tpir - tpi - tpr - tir + tp + ti + tr - t0,
        ];
        let (fp, fi, fr) = (np as i128 - 1, ni as i128 - 1, nr as i128 - 1);
        let df = [fp, fi, fr, fp * fi, fp * fr, fi * fr, fp * fi * fr];
        // Every df divides fp·fi·fr, so m_j = N·D·MS_j is an integer.
        let d = fp * fi * fr;
        let m: Vec<i128> = ss.iter().zip(df).map(|(s, f)| s * (d / f)).collect();
        let (pn, inn, rn) = (np as i128, ni as i128, nr as i128);
        let numerators = [
            (m[0] - m[3] - m[4] + m[6], inn * rn),
            (m[1] - m[3] - m[5] + m[6], pn * rn),
            (m[2] - m[4] - m[5] + m[6], pn * inn),
            (m[3] - m[6], rn),
            (m[4] - m[6], inn),
            (m[5] - m[6], pn),
            (m[6], 1),
        ];
        numerators.map(|(num, divisor)| num as f64 / (n * d * divisor) as f64)
    }

    /// Integer scores far from zero: every centred contrast must keep full
    /// precision, and the grand mean of the centred data (not exactly zero
    /// once the centre rounds) must enter each sum of squares with its sign.
    #[test]
    fn large_offset_integer_data_match_exact_components() {
        let k = Array3::from_shape_fn((7, 5, 3), |(a, b, c)| {
            let (a, b, c) = (a as i64, b as i64, c as i64);
            (3 * a * a + 7 * b + 5 * c * c + 2 * a * b * c + 14 * (a + 2 * b + 3 * c + a * c)) % 17
                - 8
        });
        let exact = exact_components(&k);
        let fit = estimate_g_theory_pir(k.mapv(|v| 1e14 + v as f64).view(), GTheoryDesign::Crossed)
            .unwrap()
            .components();
        for (j, (got, want)) in fit.iter().zip(exact).enumerate() {
            assert!(
                (got - want).abs() <= 1e-14 * want.abs(),
                "component {j}: {got:e} vs exact {want:e}"
            );
        }
    }

    /// Observations two ulps either side of 1.75: σ̂²(pir) = 32ε² is below the
    /// resolution term (4ε·max|y|)² = 49ε², so every component snaps to zero.
    #[test]
    fn components_within_observation_resolution_snap_to_zero() {
        let grid = Array3::from_shape_fn((2, 2, 2), |(p, i, r)| {
            let sign = if (p + i + r) % 2 == 0 { 1.0 } else { -1.0 };
            1.75 + sign * 2.0 * f64::EPSILON
        });
        let fit = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
        assert_eq!(fit.components(), [0.0; 7]);
    }

    /// z = a·(pi code) + ½·(pir code) with a = ½ + mε on ±1 codes, so
    /// σ̂²(pi) = 4(a² − ¼) ≈ 4mε. The documented bound for σ²(pi) is about
    /// 218ε here; its √M_k and K N ε M_k terms are 91ε and 128ε.
    #[test]
    fn snap_rule_uses_the_documented_bound() {
        let sigma_pi = |m: f64| {
            let a = 0.5 + m * f64::EPSILON;
            let grid = Array3::from_shape_fn((2, 2, 2), |(p, i, r)| {
                let [p, i, r] = [p, i, r].map(|v| if v == 0 { -1.0 } else { 1.0 });
                a * p * i + 0.5 * p * i * r
            });
            let fit = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
            let (ms_pi, ms_pir) = (8.0 * a * a, 2.0);
            let magnitude = (ms_pi + ms_pir) / 2.0;
            let delta = 8.0 * 8.0 * f64::EPSILON * (1.0 + m * f64::EPSILON);
            let bound = delta * delta
                + (4.0 * f64::EPSILON * (1.0 + m * f64::EPSILON)).powi(2)
                + delta * magnitude.sqrt()
                + 8.0 * 8.0 * f64::EPSILON * magnitude;
            (fit.sigma_pi, (ms_pi - ms_pir) / 2.0, bound)
        };
        // Inside the bound, but above either of its two main terms alone.
        let (got, value, bound) = sigma_pi(40.0);
        assert!(
            value <= bound && value > 0.73 * bound,
            "{value:e} {bound:e}"
        );
        assert_eq!(got, 0.0);
        // Just outside it, the estimate survives.
        let (got, value, bound) = sigma_pi(60.0);
        assert!(value > bound, "{value:e} {bound:e}");
        assert!((got - value).abs() <= 1e-3 * value, "{got:e} {value:e}");
    }

    /// Every component snaps to zero, so every mean square is zero and each
    /// fiducial draw's Phi is 0/0: counted as failed.
    #[test]
    fn bootstrap_counts_undefined_phi_replicates() {
        let grid = Array3::from_shape_fn((2, 2, 2), |(p, i, r)| {
            let sign = if (p + i + r) % 2 == 0 { 1.0 } else { -1.0 };
            1.75 + sign * 2.0 * f64::EPSILON
        });
        let mut rng = RngState::from_seed([0x31; 32]);
        let boot = bootstrap_g_theory_pir(grid.view(), GTheoryDesign::Crossed, 32, 0.05, &mut rng)
            .unwrap();
        assert_eq!(boot.phi_coefficient_failed, 32);
        assert_eq!(boot.phi_coefficient_ci, None);
    }

    /// At a scale where σ²(pir) is barely normal, resamples that repeat a
    /// person push components below the normal range, so those fits fail.
    /// The standard errors from a stream with failures are withheld.
    #[test]
    fn failed_fits_are_counted_and_withhold_standard_errors() {
        let scale = 0.8 * 2.0_f64.powi(-511);
        let grid = grid().mapv(|y| y * scale);
        let mut rng = RngState::from_seed([0x32; 32]);
        let boot = bootstrap_g_theory_pir(grid.view(), GTheoryDesign::Crossed, 200, 0.05, &mut rng)
            .unwrap();
        assert!(boot.facet_skipped[0] > 0, "{:?}", boot.facet_skipped);
        for k in 0..7 {
            let skipped = boot.facet_skipped[boot.plan[k].axis()] > 0;
            assert_eq!(boot.component_se[k].is_none(), skipped, "component {k}");
        }
        // Phi is scale-free and needs no refits, so its interval survives.
        assert!(boot.phi_coefficient_ci.is_some());
    }

    /// One resample per stream: no standard error, and the Phi interval is
    /// the single replicate.
    #[test]
    fn single_resample_gives_no_standard_errors_and_a_point_interval() {
        let mut rng = RngState::from_seed([0x33; 32]);
        let boot = bootstrap_g_theory_pir(grid().view(), GTheoryDesign::Crossed, 1, 0.05, &mut rng)
            .unwrap();
        assert_eq!(boot.component_se, [None; 7]);
        let (low, high) = boot.phi_coefficient_ci.unwrap();
        assert_eq!(low, high);
    }

    /// Replays the documented streams and checks each standard error against
    /// the sample SD (n − 1) of its plan's corrected replicates, accumulated
    /// with Welford's method rather than the code's two passes.
    #[test]
    fn standard_errors_are_replicate_sds_of_their_plan_streams() {
        let shape = (4, 3, 2);
        let sizes = [4, 3, 2];
        let data = Array3::from_shape_fn(shape, |(p, i, r)| {
            ((p * 7 + i * 3 + r * 5 + p * i * r) % 11) as f64
        });
        let seed = RngState::from_seed([0x34; 32]);
        let m = 50;
        let mut rng = seed.clone();
        let boot =
            bootstrap_g_theory_pir(data.view(), GTheoryDesign::Crossed, m, 0.05, &mut rng).unwrap();
        assert_eq!(boot.facet_skipped, [0; 3]);
        let nonce = seed.clone().into_chacha().next_u64().to_le_bytes();
        for (facet, name) in GTheoryFacet::ALL.into_iter().zip([
            b"g-theory/boot-p".as_slice(),
            b"g-theory/boot-i",
            b"g-theory/boot-r",
        ]) {
            let axis = facet.axis();
            let mut stream = seed.fork(&[name, &nonce].concat()).into_chacha();
            let (mut mean, mut m2) = ([0.0_f64; 7], [0.0_f64; 7]);
            for count in 1..=m {
                let draw: Vec<usize> = (0..sizes[axis])
                    .map(|_| stream.next_u32() as usize % sizes[axis])
                    .collect();
                let resampled = Array3::from_shape_fn(shape, |(a, b, c)| {
                    let mut src = [a, b, c];
                    src[axis] = draw[src[axis]];
                    data[src]
                });
                let fit = fit_replicate(resampled.view(), GTheoryDesign::Crossed).unwrap();
                let x = correct_single_facet(fit.components(), facet, sizes);
                for k in 0..7 {
                    let d = x[k] - mean[k];
                    mean[k] += d / count as f64;
                    m2[k] += d * (x[k] - mean[k]);
                }
            }
            for k in (0..7).filter(|&k| boot.plan[k] == facet) {
                let sd = (m2[k] / (m - 1) as f64).sqrt();
                let se = boot.component_se[k].unwrap();
                assert!(
                    (se - sd).abs() <= 1e-12 * sd,
                    "component {k} ({facet:?}): {se} vs {sd}"
                );
            }
        }
    }

    #[test]
    fn bootstrap_rejects_alpha_outside_the_open_unit_interval() {
        for alpha in [f64::NAN, 0.0, -0.1, 1.0, f64::INFINITY] {
            let mut rng = RngState::from_seed([0x35; 32]);
            let err =
                bootstrap_g_theory_pir(grid().view(), GTheoryDesign::Crossed, 8, alpha, &mut rng)
                    .unwrap_err();
            assert!(
                matches!(
                    err,
                    GTheoryBootstrapError::Bootstrap(
                        BootstrapGivenDataError::OutOfRangeAlpha { .. }
                    )
                ),
                "{alpha}: {err:?}"
            );
        }
    }

    struct ConstantWord(u64);

    impl RngCore for ConstantWord {
        fn next_u32(&mut self) -> u32 {
            self.0 as u32
        }
        fn next_u64(&mut self) -> u64 {
            self.0
        }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            dst.fill(0);
        }
    }

    /// The extreme words must stay strictly inside (0, 1): the normal
    /// quantile of 0 or 1 is infinite.
    #[test]
    fn open_unit_excludes_both_endpoints() {
        assert_eq!(open_unit(&mut ConstantWord(0)), 2.0_f64.powi(-53));
        assert_eq!(
            open_unit(&mut ConstantWord(u64::MAX)),
            1.0 - 2.0_f64.powi(-53)
        );
    }

    /// χ²(df) has mean df and variance 2·df, checked within 5 MCSE on both
    /// sides of the shape-1 boost.
    #[test]
    fn chi_square_sampler_has_the_right_moments() {
        let mut rng = RngState::from_seed([0x38; 32]).into_chacha();
        let n = 100_000;
        for df in [0.5, 1.0, 2.0, 3.0, 19.0, 3781.0] {
            let x: Vec<f64> = (0..n).map(|_| chi_square(df, &mut rng)).collect();
            let mean = x.iter().sum::<f64>() / f64::from(n);
            let var = x.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / f64::from(n - 1);
            let se_mean = (2.0 * df / f64::from(n)).sqrt();
            // Var of the sample variance is (μ₄ − σ⁴)/n, with μ₄ = 12df + 12df².
            let se_var = ((12.0 * df + 8.0 * df * df) / f64::from(n)).sqrt();
            assert!((mean - df).abs() < 5.0 * se_mean, "df {df}: mean {mean}");
            assert!((var - 2.0 * df).abs() < 5.0 * se_var, "df {df}: var {var}");
        }
    }

    /// With every pivot at its degrees of freedom the fiducial quantity returns
    /// the estimates (Hannig et al.'s FGPQ2: R(s, s) = θ).
    #[test]
    fn fiducial_quantity_at_its_centre_returns_the_estimates() {
        // Unequal counts, so a divisor taken from the wrong facet shows.
        let c = [0.8, 0.3, 0.2, 1.0, 0.4, 0.3, 1.0];
        let sizes = [7, 5, 3];
        let df = degrees_of_freedom(sizes);
        let back = fiducial_components(&expected_mean_squares(&c, sizes), &df, &df, sizes);
        for k in 0..7 {
            assert!(
                (back[k] - c[k]).abs() <= 1e-14,
                "{k}: {} vs {}",
                back[k],
                c[k]
            );
        }
    }

    /// Replays the documented fiducial stream at unequal counts. The
    /// components come from Gaussian elimination on the EMS coefficient
    /// matrix, and Phi from its definition with items and raters named, so
    /// a wrong inversion or swapped counts in the library would show.
    #[test]
    fn phi_interval_replays_the_fiducial_stream() {
        let (np, ni, nr) = (6, 4, 3);
        let sizes = [np, ni, nr];
        let data = Array3::from_shape_fn((np, ni, nr), |(p, i, r)| {
            ((3 * p * p + 7 * i + 5 * r * r + 2 * p * i * r) % 13) as f64
        });
        let seed = RngState::from_seed([0x39; 32]);
        let mut rng = seed.clone();
        let (m, alpha) = (400, 0.1);
        let boot = bootstrap_g_theory_pir(data.view(), GTheoryDesign::Crossed, m, alpha, &mut rng)
            .unwrap();
        let point = estimate_g_theory_pir(data.view(), GTheoryDesign::Crossed)
            .unwrap()
            .components();
        let ms = expected_mean_squares(&point, sizes);
        let df = degrees_of_freedom(sizes);
        // Column k of the EMS map is the image of the k-th unit component.
        let columns: Vec<[f64; 7]> = (0..7)
            .map(|k| {
                let mut e = [0.0; 7];
                e[k] = 1.0;
                expected_mean_squares(&e, sizes)
            })
            .collect();
        let solve = |rhs: [f64; 7]| -> [f64; 7] {
            let mut a: Vec<[f64; 8]> = (0..7)
                .map(|j| std::array::from_fn(|k| if k < 7 { columns[k][j] } else { rhs[j] }))
                .collect();
            for col in 0..7 {
                let pivot = (col..7)
                    .max_by(|&x, &y| a[x][col].abs().total_cmp(&a[y][col].abs()))
                    .unwrap();
                a.swap(col, pivot);
                for row in 0..7 {
                    if row != col {
                        let f = a[row][col] / a[col][col];
                        for k in col..8 {
                            a[row][k] -= f * a[col][k];
                        }
                    }
                }
            }
            std::array::from_fn(|k| a[k][7] / a[k][k])
        };
        let nonce = seed.clone().into_chacha().next_u64().to_le_bytes();
        let mut stream = seed
            .fork(&[b"g-theory/fiducial".as_slice(), &nonce].concat())
            .into_chacha();
        let (fi, fr) = (ni as f64, nr as f64);
        let phis: Vec<f64> = (0..m)
            .map(|_| {
                let pivots = df.map(|d| chi_square(d, &mut stream));
                let c =
                    solve(std::array::from_fn(|j| ms[j] * df[j] / pivots[j])).map(|v| v.max(0.0));
                let [p, i, r, pi, pr, ir, pir] = c;
                let absolute_error = i / fi + r / fr + pi / fi + pr / fr + (ir + pir) / (fi * fr);
                p / (p + absolute_error)
            })
            .collect();
        let want = bounded_percentiles(&phis, 0, alpha).unwrap();
        let got = boot.phi_coefficient_ci.unwrap();
        assert!(
            (got.0 - want.0).abs() <= 1e-12 && (got.1 - want.1).abs() <= 1e-12,
            "{got:?} vs {want:?}"
        );
    }

    mod sage_fiducial_oracle {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/oracles/g_theory_fiducial.rs"
        ));
    }

    /// Pinned against exact quantiles of the fiducial distributions of σ²(pir)
    /// and σ²(pi) from Sage. The share of draws at or below each exact quantile
    /// must be within 5 binomial standard errors of its level.
    #[test]
    fn fiducial_draws_match_exact_quantiles() {
        let n = 100_000;
        for case in sage_fiducial_oracle::FIDUCIAL_CASES {
            let ms = expected_mean_squares(&case.components, case.sizes);
            let df = degrees_of_freedom(case.sizes);
            let mut rng = RngState::from_seed([0x37; 32]).into_chacha();
            let draws: Vec<[f64; 7]> = (0..n)
                .map(|_| {
                    let pivots = df.map(|d| chi_square(d, &mut rng));
                    fiducial_components(&ms, &df, &pivots, case.sizes)
                })
                .collect();
            for &(k, level, value) in case.quantiles {
                let share = draws.iter().filter(|d| d[k] <= value).count() as f64 / f64::from(n);
                let se = (level * (1.0 - level) / f64::from(n)).sqrt();
                assert!(
                    (share - level).abs() <= 5.0 * se,
                    "{:?} component {k}: share {share} at the exact {level} quantile {value}",
                    case.sizes
                );
            }
        }
    }

    /// Pinned against 50-digit regularized incomplete gamma values from Sage,
    /// including the point just below statrs' range.
    #[test]
    fn chi_square_cdf_matches_high_precision_oracle_below_statrs_range() {
        for &(nu, x, exact) in sage_satterthwaite_oracle::CHI_SQUARE_CDF_CASES {
            let got = chi_square_cdf(&ChiSquared::new(nu).unwrap(), nu, x);
            if exact == 0.0 || exact < f64::MIN_POSITIVE {
                assert!(
                    got <= f64::MIN_POSITIVE,
                    "nu {nu}, x {x:e}: {got:e} vs {exact:e}"
                );
                continue;
            }
            // exp of a large negative argument carries |ln P|·ε relative error.
            let tol = 8.0 * f64::EPSILON * (1.0 + exact.ln().abs());
            assert!(
                (got / exact - 1.0).abs() <= tol,
                "nu {nu}, x {x:e}: {got:e} vs {exact:e}"
            );
        }
    }

    /// Both closed-form intervals are scale-equivariant: multiplying every
    /// component by c multiplies each Satterthwaite bound by c and leaves the
    /// G interval unchanged, across the whole normal range. At 2^1022 the
    /// mean squares would overflow without working at unit scale.
    #[test]
    fn closed_form_intervals_do_not_depend_on_units() {
        let base = [0.5528, 0.4417, 0.0074, 0.575, 0.1009, 0.1565, 0.9352];
        let sizes = [10, 4, 2];
        let unit_ci = satterthwaite_intervals(base, sizes, 0.05);
        let unit_g = g_f_interval(base, sizes, 0.05).unwrap();
        let close = |a: f64, b: f64| a == b || (a - b).abs() <= 1e-12 * b.abs();
        for c in [
            1e-200,
            1e200,
            2.0_f64.powi(-900),
            2.0_f64.powi(900),
            2.0_f64.powi(1022),
        ] {
            let scaled = base.map(|v| v * c);
            for (k, (got, unit)) in satterthwaite_intervals(scaled, sizes, 0.05)
                .iter()
                .zip(unit_ci)
                .enumerate()
            {
                let (got, unit) = (got.unwrap(), unit.unwrap());
                assert!(
                    close(got.0, unit.0 * c) && close(got.1, unit.1 * c),
                    "scale {c:e}, component {k}: {got:?} vs {unit:?}"
                );
            }
            let g = g_f_interval(scaled, sizes, 0.05).unwrap();
            assert!(
                close(g.0, unit_g.0) && close(g.1, unit_g.1),
                "scale {c:e}: {g:?}"
            );
        }
    }

    #[test]
    fn f_interval_domain_edges() {
        // MS_p = 0 exactly: no interval.
        assert_eq!(
            g_f_interval([-0.25, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0], [5, 4, 3], 0.05),
            None
        );
        // ν̂ = 1/(1 + 2/L²) at 2×2×2 with σ²(pr) = 0 and σ²(pir) = 1; this σ²(pi)
        // gives exactly ν̂ = 0.5, the smallest accepted value.
        let pi = f64::from_bits(0x3fca_8279_99fc_ef2b);
        let at_edge = [1.0, 0.0, 0.0, pi, 0.0, 0.0, 1.0];
        assert_eq!(g_f_parameters(at_edge, [2, 2, 2]).unwrap().1, 0.5);
        assert!(g_f_interval(at_edge, [2, 2, 2], 0.05).is_some());
        let below = [
            1.0,
            0.0,
            0.0,
            f64::from_bits(0x3fca_8279_99fc_ef2a),
            0.0,
            0.0,
            1.0,
        ];
        assert!(g_f_parameters(below, [2, 2, 2]).unwrap().1 < 0.5);
        assert_eq!(g_f_interval(below, [2, 2, 2], 0.05), None);
    }

    #[test]
    fn quantile_search_covers_its_documented_range() {
        // Exponential with mean 1e200: the median is 1e200·ln 2.
        let q = quantile_by_bisection(|x| -(-x / 1e200).exp_m1(), 0.5).unwrap();
        assert!(
            (q / (1e200 * std::f64::consts::LN_2) - 1.0).abs() <= 1e-12,
            "{q:e}"
        );
        // A quantile above 1e300 is outside the documented range.
        assert_eq!(quantile_by_bisection(|x| (x / 1e305).min(1.0), 0.5), None);
    }

    #[test]
    fn coefficients_match_exact_rationals_at_unequal_counts() {
        // Unit components at 3 items, 4 raters: G = 1/(1 + 1/3 + 1/4 + 1/12)
        // = 3/5 and Phi = 1/(5/3 + 1/3 + 1/4 + 1/12) = 3/7.
        let r = GTheoryResult::from_components(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 3, 4).unwrap();
        assert!((r.g_coefficient.unwrap() - 0.6).abs() <= 2.0 * f64::EPSILON);
        assert!((r.phi_coefficient.unwrap() - 3.0 / 7.0).abs() <= 2.0 * f64::EPSILON);
    }

    #[test]
    fn minimum_design_ties_keep_the_first_design() {
        let r = GTheoryResult::from_components(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2, 2).unwrap();
        let best = find_minimum_design(&r, 0.0, 3, 3, |_, _| 1.0)
            .unwrap()
            .unwrap();
        assert_eq!((best.n_items, best.n_raters), (1, 1));
    }

    #[test]
    fn display_reports_components_and_each_coefficient_state() {
        assert_eq!(GTheoryComponent::Pi.to_string(), "\u{03c3}\u{00b2}(pi)");
        let available =
            GTheoryResult::from_components(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 3, 4).unwrap();
        assert_eq!(
            available.to_string(),
            "G-theory (p x i x r)\n\n  Variance components (signed estimates):\n    \
             σ²(p)    = 1.0000\n    σ²(i)    = 1.0000\n    σ²(r)    = 1.0000\n    \
             σ²(pi)   = 1.0000\n    σ²(pr)   = 1.0000\n    σ²(ir)   = 1.0000\n    \
             σ²(pir)  = 1.0000\n\n  3 items, 4 raters: G = 0.6000  Φ = 0.4286\n"
        );
        let negative =
            GTheoryResult::from_components(1.0, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 1, 1).unwrap();
        assert!(negative
            .to_string()
            .ends_with("G = 1.0000  Φ = unavailable (negative σ²(i))\n"));
        let zeroed = negative.with_negative_components_zeroed();
        assert!(zeroed
            .to_string()
            .contains("Variance components (negative estimates set to zero):"));
        let undefined =
            GTheoryResult::from_components(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1, 1).unwrap();
        assert!(undefined
            .to_string()
            .ends_with("G = undefined (0/0)  Φ = undefined (0/0)\n"));
        let mut nonfinite = available;
        nonfinite.phi_coefficient = Err(GTheoryCoefficientError::NonFinite);
        assert!(nonfinite
            .to_string()
            .ends_with("Φ = unavailable (nonfinite)\n"));
    }

    #[test]
    fn smallest_normal_component_is_accepted() {
        assert!(GTheoryResult::from_components(
            f64::MIN_POSITIVE,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            1,
            1
        )
        .is_ok());
    }
}
