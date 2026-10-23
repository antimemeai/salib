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
//! [`bootstrap_g_theory_pir`] implements only the plain resampling plan
//! described on that function. Its replicate distribution is biased for
//! crossed designs. [crolib](https://github.com/antimemeai/crolib) develops
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
/// G-study. Interval fields are populated only by bootstrap estimation.
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
    /// Lower percentile bounds in [`GTheoryComponent::ALL`] order; `None` without
    /// bootstrap, and a `None` entry where failed resamples leave a bound unidentifiable.
    pub variance_component_ci_low: Option<Vec<Option<f64>>>,
    /// Upper percentile bounds in [`GTheoryComponent::ALL`] order; `None` without
    /// bootstrap, and a `None` entry where failed resamples leave a bound unidentifiable.
    pub variance_component_ci_high: Option<Vec<Option<f64>>>,
    /// Lower percentile bound for G; `None` without bootstrap or when unavailable.
    pub g_coefficient_ci_low: Option<f64>,
    /// Upper percentile bound for G; `None` without bootstrap or when unavailable.
    pub g_coefficient_ci_high: Option<f64>,
    /// Lower percentile bound for Phi; `None` without bootstrap or when unavailable.
    pub phi_coefficient_ci_low: Option<f64>,
    /// Upper percentile bound for Phi; `None` without bootstrap or when unavailable.
    pub phi_coefficient_ci_high: Option<f64>,
    /// Number of requested bootstrap resamples; `None` without bootstrap.
    pub bootstrap_iterations: Option<usize>,
    /// Significance level for intervals of nominal coverage `1-alpha`; `None` without bootstrap.
    pub bootstrap_alpha: Option<f64>,
    /// Resamples whose fit failed, bounded as unknown values in every interval; `None` without bootstrap.
    pub bootstrap_skipped: Option<usize>,
    /// Fitted resamples whose clipped G was 0/0, bounded as unknown values; `None` without bootstrap.
    pub g_coefficient_failed: Option<usize>,
    /// Fitted resamples whose clipped Phi was 0/0, bounded as unknown values; `None` without bootstrap.
    pub phi_coefficient_failed: Option<usize>,
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
    /// when nothing was negative, because the flag records the procedure. It
    /// drops all bootstrap fields, which describe the signed procedure.
    #[must_use]
    pub fn with_negative_components_zeroed(&self) -> Self {
        let zeroed = self.components().map(|c| if c < 0.0 { 0.0 } else { c });
        let mut result = Self::new(zeroed, self.n_items, self.n_raters);
        result.negative_components_zeroed = true;
        result
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
            variance_component_ci_low: None,
            variance_component_ci_high: None,
            g_coefficient_ci_low: None,
            g_coefficient_ci_high: None,
            phi_coefficient_ci_low: None,
            phi_coefficient_ci_high: None,
            bootstrap_iterations: None,
            bootstrap_alpha: None,
            bootstrap_skipped: None,
            g_coefficient_failed: None,
            phi_coefficient_failed: None,
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

/// Estimate G-theory components/coefficients and attach bootstrap intervals.
///
/// Combines [`estimate_g_theory_pir`] with [`bootstrap_g_theory_pir`]; read the
/// latter for what the intervals mean and what they do not.
///
/// # Errors
///
/// Returns [`GTheoryBootstrapError`] on an invalid original grid or arguments,
/// or if every draw fails.
pub fn estimate_g_theory_pir_with_bootstrap(
    grid: ArrayView3<'_, f64>,
    design: GTheoryDesign,
    n_resamples: usize,
    alpha: f64,
    rng: &mut RngState,
) -> Result<GTheoryResult, GTheoryBootstrapError> {
    let mut result = estimate_g_theory_pir(grid, design)?;
    let boot = bootstrap_g_theory_pir(grid, design, n_resamples, alpha, rng)?;
    result.variance_component_ci_low =
        Some(boot.component_ci.iter().map(|ci| ci.map(|c| c.0)).collect());
    result.variance_component_ci_high =
        Some(boot.component_ci.iter().map(|ci| ci.map(|c| c.1)).collect());
    // The bootstrap's coefficient intervals describe the clipped estimator,
    // which equals the point coefficient exactly when that is available.
    let g_ci = boot
        .g_coefficient_ci
        .filter(|_| result.g_coefficient.is_ok());
    let phi_ci = boot
        .phi_coefficient_ci
        .filter(|_| result.phi_coefficient.is_ok());
    result.g_coefficient_ci_low = g_ci.map(|ci| ci.0);
    result.g_coefficient_ci_high = g_ci.map(|ci| ci.1);
    result.phi_coefficient_ci_low = phi_ci.map(|ci| ci.0);
    result.phi_coefficient_ci_high = phi_ci.map(|ci| ci.1);
    result.bootstrap_iterations = Some(boot.n_resamples);
    result.bootstrap_alpha = Some(boot.alpha);
    result.bootstrap_skipped = Some(boot.n_skipped);
    result.g_coefficient_failed = Some(boot.g_coefficient_failed);
    result.phi_coefficient_failed = Some(boot.phi_coefficient_failed);
    Ok(result)
}

/// Percentile intervals from [`bootstrap_g_theory_pir`].
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GTheoryBootstrap {
    /// `(low, high)` for each signed component in [`GTheoryComponent::ALL`]
    /// order; `None` where failed fits make a bound unidentifiable.
    pub component_ci: [Option<(f64, f64)>; 7],
    /// `(low, high)` for the clipped G estimator; `None` when failed
    /// replicates make a bound unidentifiable.
    pub g_coefficient_ci: Option<(f64, f64)>,
    /// `(low, high)` for the clipped Phi estimator; `None` when failed
    /// replicates make a bound unidentifiable.
    pub phi_coefficient_ci: Option<(f64, f64)>,
    /// Number of resamples drawn.
    pub n_resamples: usize,
    /// Significance level: bounds are the `alpha/2` and `1 − alpha/2` percentiles.
    pub alpha: f64,
    /// Resamples whose fit failed.
    pub n_skipped: usize,
    /// Fitted resamples whose clipped G was 0/0.
    pub g_coefficient_failed: usize,
    /// Fitted resamples whose clipped Phi was 0/0.
    pub phi_coefficient_failed: usize,
}

/// Bootstrap percentile intervals for the components, G and Phi.
///
/// **Plan:** each replicate resamples persons, items and raters independently,
/// with replacement, refits the signed moment estimates, and evaluates G and
/// Phi at the grid's counts. No bias correction is applied. For crossed
/// designs this plan's replicates are biased: in a 20 × 8 × 4 normal simulation
/// with every true component equal to 1, replicates averaged +0.33 above the
/// estimate for σ²(p) and −0.38 below it for σ²(pir) (salib #5). Treat the
/// intervals as descriptive, not as calibrated confidence intervals.
///
/// Component intervals use the signed replicate estimates. Replicate G and Phi
/// use [`GTheoryResult::with_negative_components_zeroed`]: a percentile
/// interval needs an estimator defined on every resample, and the signed rule
/// would leave a coefficient undefined on many of them. In Brennan's Synthetic
/// Data Set No. 3, roughly 40% of resamples have a negative σ²(i) or σ²(r).
/// When the point coefficient is available, its components are nonnegative,
/// so it equals the clipped estimator at the sample.
///
/// A resample whose observations are all identical has every component exactly
/// zero; its G and Phi are 0/0. A replicate *fails* for a quantity when its fit
/// fails or its clipped coefficient is 0/0. Failed replicates are not dropped: each bound is the most
/// conservative value of the `alpha/2` or `1 − alpha/2` percentile of all
/// `n_resamples` replicates, over every value the failed ones could have had.
/// The bound is therefore unavailable once failures reach the tail
/// fraction. The coefficient intervals here always describe the clipped
/// estimator, so they exist even when the signed point coefficient does not;
/// [`estimate_g_theory_pir_with_bootstrap`] attaches them only where the point
/// coefficient is available.
///
/// Requires a valid crossed grid, positive `n_resamples` and finite
/// `0 < alpha < 1`; advances `rng`.
///
/// # Errors
///
/// Returns [`GTheoryBootstrapError`] if the original grid or arguments are
/// invalid or every draw fails.
pub fn bootstrap_g_theory_pir(
    grid: ArrayView3<'_, f64>,
    design: GTheoryDesign,
    n_resamples: usize,
    alpha: f64,
    rng: &mut RngState,
) -> Result<GTheoryBootstrap, GTheoryBootstrapError> {
    // Fitting the original grid validates it; its errors are the caller's.
    estimate_g_theory_pir(grid, design)?;
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

    let p = grid.shape()[0];
    let i = grid.shape()[1];
    let r = grid.shape()[2];
    let mut chacha = rng.clone().into_chacha();
    let mut p_idx = vec![0usize; p];
    let mut i_idx = vec![0usize; i];
    let mut r_idx = vec![0usize; r];
    let mut resampled = Array3::<f64>::zeros((p, i, r));
    let mut component_samples = (0..7)
        .map(|_| Vec::with_capacity(n_resamples))
        .collect::<Vec<_>>();
    let mut g_samples = Vec::with_capacity(n_resamples);
    let mut phi_samples = Vec::with_capacity(n_resamples);
    let mut n_skipped = 0usize;
    let mut g_failed = 0usize;
    let mut phi_failed = 0usize;

    for _ in 0..n_resamples {
        for idx in &mut p_idx {
            *idx = (chacha.next_u32() as usize) % p;
        }
        for idx in &mut i_idx {
            *idx = (chacha.next_u32() as usize) % i;
        }
        for idx in &mut r_idx {
            *idx = (chacha.next_u32() as usize) % r;
        }
        for out_p in 0..p {
            for out_i in 0..i {
                for out_r in 0..r {
                    resampled[[out_p, out_i, out_r]] =
                        grid[[p_idx[out_p], i_idx[out_i], r_idx[out_r]]];
                }
            }
        }
        let fit = match estimate_g_theory_pir(resampled.view(), design) {
            Err(GTheoryError::ZeroVariance) => Ok(GTheoryResult::new([0.0; 7], i, r)),
            other => other,
        };
        match fit {
            Ok(est) => {
                for (samples, value) in component_samples.iter_mut().zip(est.components()) {
                    samples.push(value);
                }
                // Replicate coefficients use the clip-after-solving estimator,
                // which is defined on every resample and equals the point
                // coefficient whenever that is available.
                let clipped = est.with_negative_components_zeroed();
                match clipped.g_coefficient {
                    Ok(g) => g_samples.push(g),
                    Err(_) => g_failed += 1,
                }
                match clipped.phi_coefficient {
                    Ok(phi) => phi_samples.push(phi),
                    Err(_) => phi_failed += 1,
                }
            }
            Err(_) => n_skipped += 1,
        }
    }
    *rng = RngState::snapshot(&chacha, rng);
    if n_skipped == n_resamples {
        return Err(GTheoryBootstrapError::AllResamplesFailed);
    }
    let component_ci =
        std::array::from_fn(|k| bounded_percentiles(&component_samples[k], n_skipped, alpha));
    Ok(GTheoryBootstrap {
        component_ci,
        g_coefficient_ci: bounded_percentiles(&g_samples, n_resamples - g_samples.len(), alpha),
        phi_coefficient_ci: bounded_percentiles(
            &phi_samples,
            n_resamples - phi_samples.len(),
            alpha,
        ),
        n_resamples,
        alpha,
        n_skipped,
        g_coefficient_failed: g_failed,
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
