//! Quantile-oriented sensitivity analysis (QOSA) using a
//! partition-based approximation.
//!
//! # Population index
//!
//! QOSA measures the relative reduction in expected quantile loss
//! when an input is known. For `ψ_α(y, θ) = (y − θ)(α − 1_{y≤θ})`,
//!
//! ```text
//! S_i^α = 1 − E[min_θ E[ψ_α(Y, θ) | X_i]] / min_θ E[ψ_α(Y, θ)].
//! ```
//!
//! The index lies in `[0, 1]` when the denominator is positive and
//! the expectations exist. Independence gives zero; an input that
//! determines the output gives one. Changing `α` can change factor
//! rankings. Rankings need not agree with Sobol' indices at `α = 0.5`.
//!
//! [Maume-Deschamps and Niang (2018)](https://doi.org/10.1016/j.spl.2017.10.019)
//! express the index through conditional tail expectations:
//!
//! ```text
//! S_i^α = 1 − (E[Y | Y > q_α(Y|X_i)] − E[Y])
//!             / (CTE_α(Y) − E[Y]).
//! ```
//!
//! This identity uses tail probability `1 − α`. It requires the
//! relevant marginal and conditional distributions to have no atom
//! at their quantiles, together with a finite first moment and a
//! nonzero denominator. The contrast definition above also covers
//! cases for which this strict-tail expression is unsuitable.
//!
//! # Implemented approximation and limitations
//!
//! The paper estimates conditional quantiles with kernels and uses
//! two independent samples. This implementation instead minimizes
//! empirical pinball loss globally and within input classes using
//! the same observations for fitting and evaluation. It therefore
//! is not the paper's estimator. In-sample fitting can bias the
//! estimated loss reduction upward.
//!
//! Quantiles use the `ceil(alpha * class_size)`-th order statistic.
//! The loss formulation handles atoms and is invariant to output
//! translation and positive scaling, subject to rounding. A constant
//! output has zero loss, so its index is undefined.
//!
//! Tied input values stay together, assigned by their sorted rank
//! block midpoint. Empty classes are omitted. A constant input gives
//! one class and zero index. Class count uses [`crate::borgonovo`]'s
//! heuristic, capped at 48. The cap leaves conditioning error as
//! sample size grows; the paper's consistency result does not cover
//! this estimator. Check sample-size and partition sensitivity rather
//! than assuming more observations remove all approximation error.

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::similar_names,
    clippy::many_single_char_names,
    clippy::doc_lazy_continuation,
    clippy::doc_markdown,
    clippy::doc_overindented_list_items,
    clippy::needless_range_loop
)]

use std::fmt;

#[cfg(test)]
use ndarray::Array2;
use ndarray::ArrayView2;
use salib_core::tree_sum;

use crate::borgonovo::class_count;
use crate::conditioning::classes;

/// QOSA index estimates for a fixed α.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct QosaIndices {
    /// QOSA estimate per factor, length `d`, clamped to `[0, 1]`.
    /// Clamping handles rounding, not sampling or partition error.
    pub s: Vec<f64>,
    /// Quantile level α used for the estimate. Echo of input.
    pub alpha: f64,
    /// Empirical α-quantile of the marginal output. Diagnostic.
    pub global_quantile: f64,
    /// Empirical expected shortfall: `q + mean((Y-q)_+) / (1-alpha)`.
    /// This assigns fractional mass at the quantile when the empirical
    /// distribution has an atom, rather than averaging only strict-tail rows.
    pub global_cte: f64,
    /// Minimum empirical mean pinball loss; the index denominator.
    pub global_loss: f64,
}

impl QosaIndices {
    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.s.len()
    }
}

impl fmt::Display for QosaIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "QOSA indices (d={}, \u{03b1}={:.2})",
            self.d(),
            self.alpha
        )?;
        writeln!(
            f,
            "  quantile = {:.4}  CTE = {:.4}",
            self.global_quantile, self.global_cte
        )?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.s[i])?;
        }
        Ok(())
    }
}

/// Errors from [`estimate_qosa`].
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum QosaError {
    /// An input observation is NaN or infinite.
    #[error("input at row {row}, column {column} must be finite")]
    NonfiniteInput {
        /// Zero-based input row.
        row: usize,
        /// Zero-based input column.
        column: usize,
    },
    /// An output observation is NaN or infinite.
    #[error("output at index {index} must be finite")]
    NonfiniteOutput {
        /// Zero-based output index.
        index: usize,
    },
    /// Input and output shapes are incompatible.
    #[error("qosa: shape mismatch — x has {x_rows} rows, y has {y_len} elements")]
    ShapeMismatch {
        /// Number of rows in the input matrix.
        x_rows: usize,
        /// Number of output observations.
        y_len: usize,
    },
    /// At least one input dimension is required.
    #[error("qosa: d must be ≥ 1, got 0")]
    ZeroD,
    /// The sample count is too small for the requested analysis.
    #[error("qosa: insufficient samples — need N ≥ 16, got {n}")]
    InsufficientSamples {
        /// Sample count supplied by the caller.
        n: usize,
    },
    /// The significance or quantile level must be finite and strictly between zero and one.
    #[error("qosa: alpha must lie in (0, 1), got {alpha}")]
    InvalidAlpha {
        /// Significance or quantile level supplied; must be finite and in `(0,1)`.
        alpha: f64,
    },
    /// Empirical quantile loss is zero or cannot be represented finitely.
    #[error("qosa: empirical quantile loss is zero or nonfinite")]
    ZeroVariance,
    /// Legacy strict-tail error retained for compatibility.
    /// The pinball-loss estimator does not emit this variant.
    #[error("qosa: degenerate strict tail (legacy estimator error)")]
    DegenerateTail,
}

/// Estimate quantile-oriented sensitivity indices on generic
/// `(X, Y)` data by empirical quantile-loss reduction within classes.
///
/// `x` is the `(N, d)` input matrix; `y` is the `N`-element model
/// output. `alpha ∈ (0, 1)` is the quantile level; common choices
/// are `0.5` (median), `0.9` / `0.95` (tail), `0.99` (extreme tail).
///
/// Rows must be aligned observations representative of the input
/// distribution of interest. See the module documentation for
/// in-sample fitting and partition limitations. Atomic outputs are supported.
///
/// # Errors
///
/// - [`QosaError::ShapeMismatch`] if `x.nrows() != y.len()`.
/// - [`QosaError::ZeroD`] if `x.ncols() == 0`.
/// - [`QosaError::InsufficientSamples`] if `N < 16` (matches the
///   sibling given-data estimators' floor).
/// - [`QosaError::InvalidAlpha`] if `alpha ∉ (0, 1)`.
/// - [`QosaError::NonfiniteInput`] or [`QosaError::NonfiniteOutput`] for NaN/infinity.
/// - [`QosaError::ZeroVariance`] if the minimum empirical quantile loss
///   is zero (constant output), or nonfinite from numerical overflow.
pub fn estimate_qosa(
    x: ArrayView2<'_, f64>,
    y: &[f64],
    alpha: f64,
) -> Result<QosaIndices, QosaError> {
    let n = x.nrows();
    let d = x.ncols();
    if d == 0 {
        return Err(QosaError::ZeroD);
    }
    if y.len() != n {
        return Err(QosaError::ShapeMismatch {
            x_rows: n,
            y_len: y.len(),
        });
    }
    if n < 16 {
        return Err(QosaError::InsufficientSamples { n });
    }
    if !(alpha.is_finite() && alpha > 0.0 && alpha < 1.0) {
        return Err(QosaError::InvalidAlpha { alpha });
    }

    if let Some(((row, column), _)) = x.indexed_iter().find(|(_, v)| !v.is_finite()) {
        return Err(QosaError::NonfiniteInput { row, column });
    }
    if let Some(index) = y.iter().position(|v| !v.is_finite()) {
        return Err(QosaError::NonfiniteOutput { index });
    }

    let n_f = n as f64;
    let mut y_sorted = y.to_vec();
    y_sorted.sort_by(f64::total_cmp);
    let global_quantile = empirical_quantile(&y_sorted, alpha);
    let global_loss = pinball_sum(&y_sorted, global_quantile, alpha) / n_f;
    if !global_loss.is_finite() || global_loss <= 0.0 {
        return Err(QosaError::ZeroVariance);
    }
    // Average the largest empirical tail mass, with a fractional final row.
    // Applying weights before summation avoids overflowing an excess sum.
    let tail_mass = n_f * (1.0 - alpha);
    let whole = tail_mass.floor() as usize;
    let fraction = tail_mass - whole as f64;
    let mut tail_terms = Vec::with_capacity(whole.saturating_add(1));
    for &value in y_sorted.iter().rev().take(whole) {
        tail_terms.push(value / tail_mass);
    }
    if fraction > 0.0 && whole < n {
        tail_terms.push(y_sorted[n - whole - 1] * (fraction / tail_mass));
    }
    // A convex average lies within the observed range; contain endpoint rounding.
    let global_cte = tree_sum(&tail_terms).clamp(y_sorted[0], y_sorted[n - 1]);
    let n_classes = class_count(n);
    let mut s = vec![0.0; d];
    let mut x_col_buf = vec![0.0; n];
    let mut class_y_buf = Vec::with_capacity(n);
    for i in 0..d {
        for k in 0..n {
            x_col_buf[k] = x[[k, i]];
        }
        let groups = classes(&x_col_buf, n_classes);
        let mut class_losses = Vec::with_capacity(groups.len());
        for group in groups {
            class_y_buf.clear();
            class_y_buf.extend(group.iter().map(|&k| y[k]));
            class_y_buf.sort_by(f64::total_cmp);
            let quantile = empirical_quantile(&class_y_buf, alpha);
            class_losses.push(pinball_sum(&class_y_buf, quantile, alpha));
        }
        let conditional_loss = tree_sum(&class_losses) / n_f;
        // Optimizing each class separately cannot increase the in-sample loss.
        // Clamping handles floating-point rounding at the interval endpoints.
        s[i] = (1.0 - conditional_loss / global_loss).clamp(0.0, 1.0);
    }

    Ok(QosaIndices {
        s,
        alpha,
        global_quantile,
        global_cte,
        global_loss,
    })
}

/// A minimizing empirical alpha-quantile, including laws with atoms.
fn empirical_quantile(sorted: &[f64], alpha: f64) -> f64 {
    let index = ((alpha * sorted.len() as f64).ceil() as usize)
        .saturating_sub(1)
        .min(sorted.len() - 1);
    sorted[index]
}

fn pinball_sum(sorted: &[f64], quantile: f64, alpha: f64) -> f64 {
    let losses: Vec<_> = sorted
        .iter()
        .map(|&value| {
            if value >= quantile {
                weighted_difference(value, quantile, alpha)
            } else {
                weighted_difference(quantile, value, 1.0 - alpha)
            }
        })
        .collect();
    tree_sum(&losses)
}

/// Scale before subtraction if a finite pair's difference overflows.
fn weighted_difference(upper: f64, lower: f64, weight: f64) -> f64 {
    let difference = upper - lower;
    if difference.is_finite() {
        weight * difference
    } else {
        weight * upper - weight * lower
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {
    use super::*;

    fn synthetic_uniform(n: usize, d: usize) -> Array2<f64> {
        let mut x = Array2::<f64>::zeros((n, d));
        for j in 0..d {
            let mut state: u64 = 0x9E37_79B9_7F4A_7C15_u64.wrapping_mul((j as u64).wrapping_add(1));
            for k in 0..n {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1);
                let u = (state >> 33) as f64 / ((u64::MAX >> 33) as f64 + 1.0);
                x[[k, j]] = u;
            }
        }
        x
    }

    // ── Validation ────────────────────────────────────────────────

    #[test]
    fn shape_mismatch_errors() {
        let x = Array2::<f64>::zeros((10, 3));
        let y = vec![0.0; 5];
        let err = estimate_qosa(x.view(), &y, 0.5).unwrap_err();
        assert!(matches!(err, QosaError::ShapeMismatch { .. }));
    }

    #[test]
    fn zero_d_errors() {
        let x = Array2::<f64>::zeros((100, 0));
        let y = vec![0.0; 100];
        assert_eq!(
            estimate_qosa(x.view(), &y, 0.5).unwrap_err(),
            QosaError::ZeroD
        );
    }

    #[test]
    fn insufficient_samples_errors() {
        let x = Array2::<f64>::zeros((8, 3));
        let y = vec![0.0; 8];
        assert!(matches!(
            estimate_qosa(x.view(), &y, 0.5).unwrap_err(),
            QosaError::InsufficientSamples { .. }
        ));
    }

    #[test]
    fn invalid_alpha_errors() {
        let x = Array2::<f64>::zeros((100, 3));
        let y: Vec<f64> = (0..100).map(|k| k as f64).collect();
        for bad in [0.0, 1.0, -0.1, 1.5, f64::NAN] {
            assert!(matches!(
                estimate_qosa(x.view(), &y, bad).unwrap_err(),
                QosaError::InvalidAlpha { .. }
            ));
        }
    }

    #[test]
    fn zero_variance_errors_on_constant_y() {
        let x = synthetic_uniform(64, 3);
        let y = vec![5.0; 64];
        assert_eq!(
            estimate_qosa(x.view(), &y, 0.5).unwrap_err(),
            QosaError::ZeroVariance
        );
    }

    // ── Sanity properties ────────────────────────────────────────

    #[test]
    fn independent_factor_yields_near_zero_index() {
        // Y depends only on X_0; X_1, X_2 are independent of Y.
        // Sanity property: S^α_1 ≈ S^α_2 ≈ 0.
        let n = 1024;
        let x = synthetic_uniform(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let result = estimate_qosa(x.view(), &y, 0.5).unwrap();
        assert!(
            result.s[1] < 0.1,
            "S^α_1 = {} should be ≈ 0 (X_1 ⊥ Y)",
            result.s[1]
        );
        assert!(
            result.s[2] < 0.1,
            "S^α_2 = {} should be ≈ 0 (X_2 ⊥ Y)",
            result.s[2]
        );
        // X_0 fully determines Y → S^α_0 should be substantial.
        assert!(
            result.s[0] > 0.3,
            "S^α_0 = {} should be substantial (X_0 ⇒ Y)",
            result.s[0]
        );
    }

    #[test]
    fn fully_determining_factor_yields_index_near_one() {
        // Y = X_0 exactly. Maume-Deschamps Remark: S^α = 1 if Y is
        // X_i-measurable. Finite input classes leave conditioning error;
        // this fixture only checks a substantial estimate at its chosen N.
        let n = 2048;
        let x = synthetic_uniform(n, 2);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let result = estimate_qosa(x.view(), &y, 0.5).unwrap();
        assert!(
            result.s[0] > 0.85,
            "S^α_0 = {} should be near 1 (Y = X_0)",
            result.s[0]
        );
    }

    // ── Output shape ──────────────────────────────────────────────

    #[test]
    fn output_dimensions_match_inputs() {
        let n = 256;
        let x = synthetic_uniform(n, 5);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + x[[k, 2]]).collect();
        let result = estimate_qosa(x.view(), &y, 0.75).unwrap();
        assert_eq!(result.d(), 5);
        assert_eq!(result.alpha, 0.75);
        assert!(result.global_cte > result.global_quantile);
    }

    // ── Tail-vs-median demonstrates QOSA's distinguishing feature ─

    #[test]
    fn tail_alpha_emphasizes_tail_driving_factor() {
        // Y = X_0 + 5 · X_1 · 1_{X_2 > 0.95}.
        // At α = 0.5 (median), X_0 dominates (the indicator fires
        // only 5% of the time, contributing little to the median).
        // At α = 0.95 (tail), X_1 and X_2 dominate (when the
        // indicator fires, the 5·X_1 term swamps X_0).
        let n = 4096;
        let x = synthetic_uniform(n, 3);
        let y: Vec<f64> = (0..n)
            .map(|k| {
                let base = x[[k, 0]];
                let tail = if x[[k, 2]] > 0.95 {
                    5.0 * x[[k, 1]]
                } else {
                    0.0
                };
                base + tail
            })
            .collect();

        let median = estimate_qosa(x.view(), &y, 0.5).unwrap();
        let tail = estimate_qosa(x.view(), &y, 0.95).unwrap();

        // At median, X_0 is the dominant driver.
        assert!(
            median.s[0] > median.s[1],
            "at α=0.5: S_0 = {} should exceed S_1 = {}",
            median.s[0],
            median.s[1]
        );
        // At tail, X_2 should dominate over X_0 (it's the gate
        // variable that triggers the tail-driving term).
        assert!(
            tail.s[2] > tail.s[0],
            "at α=0.95: S_2 = {} should exceed S_0 = {}",
            tail.s[2],
            tail.s[0]
        );
    }

    // ── Determinism ──────────────────────────────────────────────

    #[test]
    fn same_input_yields_identical_output() {
        let n = 256;
        let x = synthetic_uniform(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] * x[[k, 1]]).collect();
        let a = estimate_qosa(x.view(), &y, 0.7).unwrap();
        let b = estimate_qosa(x.view(), &y, 0.7).unwrap();
        assert_eq!(a.s, b.s);
        assert_eq!(a.global_quantile, b.global_quantile);
        assert_eq!(a.global_cte, b.global_cte);
    }
}
