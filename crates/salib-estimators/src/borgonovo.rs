//! Borgonovo's δ: expected total variation between unconditional and
//! conditional output distributions, estimated from paired observations.
//!
//! [Borgonovo (2007)](https://doi.org/10.1016/j.ress.2006.04.015) defines
//! the index, written here for distributions with the required densities:
//!
//! ```text
//! δᵢ = (1/2) E_{Xᵢ}[∫ |f_Y(y) − f_{Y|Xᵢ}(y)| dy].
//! ```
//!
//! The population index lies in `[0, 1]` and is zero exactly when
//! `Y` and `Xᵢ` are independent. It can detect changes in location,
//! scale, or shape. It does not decompose output variance.
//!
//! # Estimator
//!
//! This follows the class-conditional density approach of
//! [Plischke, Borgonovo and Smith (2013)](https://doi.org/10.1016/j.ejor.2012.11.047).
//!
//! 1. Estimate the unconditional output density with Gaussian KDE.
//! 2. Split each input into approximately equal-frequency classes by
//!    sorted input values, keeping ties in one class. The requested
//!    class count is `ceil(N^exp)`, clamped to
//!    `[2, 48]`, with `exp = 2 / (7 + tanh((1500 − N)/500))`.
//! 3. Estimate each class's output density with Gaussian KDE.
//! 4. Integrate the absolute density difference by the trapezoidal
//!    rule on 100 points between `min(Y)` and `max(Y)`, then weight
//!    each integral by its class size divided by `2N`.
//!
//! The bandwidth is `(3N/4)^(−1/5)` times the population standard
//! deviation of the density's sample. Outputs are affinely normalized
//! to `[0,1]` before KDE to avoid scale overflow; the integrated density
//! distance is unchanged by this coordinate transformation. No bootstrap bias correction
//! or confidence interval is computed.
//!
//! The 48-class cap and fixed integration grid leave approximation
//! error that more observations alone need not remove. The paper's
//! partition-refinement convergence results are not guarantees for
//! these fixed limits. There is no universal sufficient sample size.
//! Ties are assigned by the midpoint of their sorted rank block;
//! empty classes are omitted. Thus discrete inputs can yield fewer
//! classes, while a constant input produces one unconditional class.
//! The KDE still assumes continuous output densities.
//!
//! Computation is deterministic for the same ordered `(X, Y)` data.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::items_after_statements
)]

use std::f64::consts::PI;
use std::fmt;

use crate::conditioning::classes;
#[cfg(test)]
use ndarray::Array2;
use ndarray::ArrayView2;
use salib_core::tree_sum;

/// Borgonovo `δ` estimates per factor.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct BorgonovoIndices {
    /// Borgonovo δ per factor, length `d`. `δᵢ ∈ [0, 1]`.
    pub delta: Vec<f64>,
}

impl BorgonovoIndices {
    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.delta.len()
    }
}

impl fmt::Display for BorgonovoIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Borgonovo \u{03b4} indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "\u{03b4}")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.delta[i])?;
        }
        Ok(())
    }
}

/// Errors from [`estimate_borgonovo_delta`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum BorgonovoError {
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
    #[error("Borgonovo δ: shape mismatch — X has {x_rows} rows, y has {y_len} elements")]
    ShapeMismatch {
        /// Number of rows in the input matrix.
        x_rows: usize,
        /// Number of output observations.
        y_len: usize,
    },
    /// At least one input dimension is required.
    #[error("Borgonovo δ: d must be ≥ 1, got 0")]
    ZeroD,
    /// `N < 16` — too few samples for meaningful KDE + partitioning.
    /// `SALib`'s default `M ≥ 2` requires at least a few samples per
    /// class; we floor at `N = 16` to keep the estimator stable.
    #[error("Borgonovo δ: N must be ≥ 16, got {n}")]
    InsufficientSamples {
        /// Sample count supplied by the caller.
        n: usize,
    },
    /// Y has zero range (constant model).
    #[error("Borgonovo δ: Y has zero range (model output is constant)")]
    ZeroVariance,
}

/// Estimate Borgonovo `δ` per factor from generic `(X, Y)` data.
///
/// `x` is the `(N, d)` input matrix; `y` is the `N`-element model
/// output vector. The estimator is sampler-agnostic — LHS, Sobol',
/// Saltelli matrix, user data all work.
///
/// # Errors
///
/// - [`BorgonovoError::NonfiniteInput`] for any NaN or infinite input.
/// - [`BorgonovoError::NonfiniteOutput`] for any NaN or infinite output.
///
/// - [`BorgonovoError::ShapeMismatch`] if `x.nrows() != y.len()`.
/// - [`BorgonovoError::ZeroD`] if `x.ncols() == 0`.
/// - [`BorgonovoError::InsufficientSamples`] if `N < 16`.
/// - [`BorgonovoError::ZeroVariance`] if `Y` has zero range.
pub fn estimate_borgonovo_delta(
    x: ArrayView2<'_, f64>,
    y: &[f64],
) -> Result<BorgonovoIndices, BorgonovoError> {
    let n = x.nrows();
    let d = x.ncols();
    if d == 0 {
        return Err(BorgonovoError::ZeroD);
    }
    if y.len() != n {
        return Err(BorgonovoError::ShapeMismatch {
            x_rows: n,
            y_len: y.len(),
        });
    }
    if n < 16 {
        return Err(BorgonovoError::InsufficientSamples { n });
    }

    if let Some(((row, column), _)) = x.indexed_iter().find(|(_, v)| !v.is_finite()) {
        return Err(BorgonovoError::NonfiniteInput { row, column });
    }
    if let Some(index) = y.iter().position(|v| !v.is_finite()) {
        return Err(BorgonovoError::NonfiniteOutput { index });
    }

    let (y_min, y_max) = min_max(y);
    if y_min == y_max {
        return Err(BorgonovoError::ZeroVariance);
    }

    // Density distance is invariant under a common affine output coordinate.
    // Work on [0,1] to keep bandwidth residual squares and KDE arithmetic bounded.
    let range = y_max - y_min;
    let normalized: Vec<_> = if range.is_finite() {
        y.iter().map(|&value| (value - y_min) / range).collect()
    } else {
        // Opposite-sign finite extrema can have an unrepresentable raw range.
        let scale = y_min.abs().max(y_max.abs());
        let lo = y_min / scale;
        let span = y_max / scale - lo;
        y.iter().map(|&value| (value / scale - lo) / span).collect()
    };
    let y = normalized.as_slice();
    const Y_GRID_POINTS: usize = 100;
    let y_grid: Vec<_> = (0..Y_GRID_POINTS)
        .map(|k| k as f64 / (Y_GRID_POINTS - 1) as f64)
        .collect();

    // Adaptive class count per `SALib` / Plischke 2013.
    let n_classes = class_count(n);

    // Pre-compute the unconditional KDE — it's reused across factors.
    let h_y = silverman_bandwidth(y);
    let fy = gaussian_kde(y, h_y, &y_grid);

    let mut delta = vec![0.0_f64; d];
    let mut x_col_buf = vec![0.0_f64; n];
    for i in 0..d {
        for k in 0..n {
            x_col_buf[k] = x[[k, i]];
        }
        delta[i] = calc_delta(y, &y_grid, &fy, &x_col_buf, n_classes);
    }

    Ok(BorgonovoIndices { delta })
}

/// Plischke 2013 Eq 26 estimator for a single factor's `δ`.
fn calc_delta(y: &[f64], y_grid: &[f64], fy: &[f64], x_col: &[f64], n_classes: usize) -> f64 {
    let n = y.len();
    let groups = classes(x_col, n_classes);

    let n_f = n as f64;
    let mut d_hat = 0.0_f64;
    let mut class_y_buf: Vec<f64> = Vec::with_capacity(n);
    let mut diff_buf = vec![0.0_f64; y_grid.len()];

    for group in groups {
        class_y_buf.clear();
        class_y_buf.extend(group.iter().map(|&k| y[k]));
        let nm = class_y_buf.len();

        // Peak-to-peak: if the class's Y is constant, the conditional
        // density collapses to a δ-distribution; treat the divergence
        // as |fy| (matches `SALib`'s degenerate-class fallback).
        let (cy_min, cy_max) = min_max(&class_y_buf);
        if (cy_max - cy_min) > 0.0 {
            let h_yc = silverman_bandwidth(&class_y_buf);
            // Re-use diff_buf as the conditional KDE then convert
            // in-place to |fy − fyc|.
            gaussian_kde_into(&class_y_buf, h_yc, y_grid, &mut diff_buf);
            for (k, slot) in diff_buf.iter_mut().enumerate() {
                *slot = (fy[k] - *slot).abs();
            }
        } else {
            for (k, slot) in diff_buf.iter_mut().enumerate() {
                *slot = fy[k].abs();
            }
        }

        let integral = trapz(&diff_buf, y_grid);
        d_hat += (nm as f64 / (2.0 * n_f)) * integral;
    }
    d_hat
}

/// `SALib` / Plischke 2013 adaptive partition count.
///
/// `M = round(min(⌈N^exp⌉, 48))` where
/// `exp = 2 / (7 + tanh((1500 − N) / 500))`. For `N = 1024`, `M = 6`;
/// for `N = 4096`, `M = 16`; capped at `48` for very large `N`.
///
/// `pub(crate)` so sibling partition-based estimators (e.g. `qosa`)
/// can share the same heuristic without re-deriving it.
pub(crate) fn class_count(n: usize) -> usize {
    let n_f = n as f64;
    let tanh_arg = (1500.0 - n_f) / 500.0;
    let exp = 2.0 / (7.0 + tanh_arg.tanh());
    let raw = n_f.powf(exp).ceil() as usize;
    raw.clamp(2, 48)
}

/// Silverman's normal-reference bandwidth for univariate Gaussian KDE:
///
/// `h = (n · 3/4)^(−1/5) · σ`
///
/// where `σ` uses the population variance divisor `n`.
fn silverman_bandwidth(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    let mean = tree_sum(data) / n;
    let var: f64 = tree_sum(
        &data
            .iter()
            .map(|&v| (v - mean).powi(2))
            .collect::<Vec<f64>>(),
    ) / n;
    let sigma = var.sqrt();
    let factor = (n * 3.0 / 4.0).powf(-0.2);
    let h = factor * sigma;
    // Numerical safety: KDE bandwidth must be strictly positive. If
    // the data is exactly constant (caught upstream by ZeroVariance
    // or per-class fallback), this code path shouldn't fire — but
    // guard against denormal-tiny σ from numerical noise.
    h.max(1e-15)
}

/// Evaluate Gaussian KDE at each grid point, writing into `out`.
///
/// `f̂(y) = (1 / (n · h · √(2π))) · Σᵢ exp(− ½ ((y − xᵢ) / h)²)`
fn gaussian_kde_into(data: &[f64], h: f64, grid: &[f64], out: &mut [f64]) {
    debug_assert_eq!(grid.len(), out.len());
    let n = data.len() as f64;
    let norm = 1.0 / (n * h * (2.0 * PI).sqrt());
    for (k, &y) in grid.iter().enumerate() {
        let mut acc = 0.0;
        for &xi in data {
            let z = (y - xi) / h;
            acc += (-0.5 * z * z).exp();
        }
        out[k] = norm * acc;
    }
}

/// Convenience wrapper: allocate and return.
fn gaussian_kde(data: &[f64], h: f64, grid: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; grid.len()];
    gaussian_kde_into(data, h, grid, &mut out);
    out
}

/// Trapezoidal integration of `y` against `x` (both length `n`).
fn trapz(y: &[f64], x: &[f64]) -> f64 {
    debug_assert_eq!(y.len(), x.len());
    if y.len() < 2 {
        return 0.0;
    }
    let mut sum = 0.0;
    for i in 1..y.len() {
        sum += 0.5 * (y[i] + y[i - 1]) * (x[i] - x[i - 1]);
    }
    sum
}

fn min_max(data: &[f64]) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &v in data {
        if v < lo {
            lo = v;
        }
        if v > hi {
            hi = v;
        }
    }
    (lo, hi)
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {
    use super::*;

    fn synthetic_x(n: usize, d: usize) -> Array2<f64> {
        // Per-column independent permutations of (k+0.5)/n.
        let mut x = Array2::<f64>::zeros((n, d));
        for j in 0..d {
            let mut perm: Vec<usize> = (0..n).collect();
            let mut state: u64 = 0x9E37_79B9_7F4A_7C15_u64.wrapping_mul((j as u64).wrapping_add(1));
            for i in (1..n).rev() {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1);
                let k = (state >> 33) as usize % (i + 1);
                perm.swap(i, k);
            }
            for i in 0..n {
                x[[i, j]] = (perm[i] as f64 + 0.5) / (n as f64);
            }
        }
        x
    }

    // ── Validation ────────────────────────────────────────────────

    #[test]
    fn zero_d_errors() {
        let x = Array2::<f64>::zeros((100, 0));
        let y = vec![0.0; 100];
        assert_eq!(
            estimate_borgonovo_delta(x.view(), &y).unwrap_err(),
            BorgonovoError::ZeroD
        );
    }

    #[test]
    fn shape_mismatch_errors() {
        let x = Array2::<f64>::zeros((100, 3));
        let y = vec![0.0; 50];
        let err = estimate_borgonovo_delta(x.view(), &y).unwrap_err();
        assert!(matches!(err, BorgonovoError::ShapeMismatch { .. }));
    }

    #[test]
    fn insufficient_samples_errors() {
        let x = synthetic_x(10, 3);
        let y = vec![0.0; 10];
        let err = estimate_borgonovo_delta(x.view(), &y).unwrap_err();
        assert!(matches!(err, BorgonovoError::InsufficientSamples { .. }));
    }

    #[test]
    fn constant_model_errors() {
        let x = synthetic_x(64, 3);
        let y = vec![1.0; 64];
        let err = estimate_borgonovo_delta(x.view(), &y).unwrap_err();
        assert_eq!(err, BorgonovoError::ZeroVariance);
    }

    // ── Class-count formula matches `SALib` ───────────────────────

    #[test]
    fn class_count_matches_salib_table() {
        // SALib `np.round(min(int(np.ceil(N**exp)), 48))` for the
        // same N → M values:
        //   N=1024  M=6
        //   N=4096  M=16
        assert_eq!(class_count(1024), 6);
        assert_eq!(class_count(4096), 16);
    }

    #[test]
    fn class_count_capped_at_48() {
        assert_eq!(class_count(1_000_000), 48);
    }

    #[test]
    fn class_count_floors_at_2() {
        assert!(class_count(16) >= 2);
    }

    // ── Output shape ──────────────────────────────────────────────

    #[test]
    fn output_length_matches_d() {
        let n = 256;
        let x = synthetic_x(n, 5);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let est = estimate_borgonovo_delta(x.view(), &y).unwrap();
        assert_eq!(est.d(), 5);
    }

    // ── δ ∈ [0, 1] ────────────────────────────────────────────────

    #[test]
    fn delta_within_unit_interval_with_slack() {
        // δ is bounded in [0, 1] by definition. Empirical KDE-based
        // estimates can slip slightly outside due to numerical
        // integration error; allow ε slack.
        let n = 512;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n)
            .map(|k| x[[k, 0]] + 0.5 * x[[k, 1]] * x[[k, 2]])
            .collect();
        let est = estimate_borgonovo_delta(x.view(), &y).unwrap();
        for &v in &est.delta {
            assert!((-0.05..=1.05).contains(&v), "δ = {v} outside [-0.05, 1.05]");
        }
    }

    // ── Linear single-factor: factor 0 has highest δ ─────────────

    #[test]
    fn linear_single_factor_dominates_delta() {
        // Y = X[:, 0] — factor 0 should have the largest δ.
        let n = 512;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let est = estimate_borgonovo_delta(x.view(), &y).unwrap();
        assert!(
            est.delta[0] > est.delta[1],
            "δ_0 = {} should exceed δ_1 = {}",
            est.delta[0],
            est.delta[1]
        );
        assert!(
            est.delta[0] > est.delta[2],
            "δ_0 = {} should exceed δ_2 = {}",
            est.delta[0],
            est.delta[2]
        );
    }

    // ── Determinism ───────────────────────────────────────────────

    #[test]
    fn same_input_yields_identical_output() {
        let n = 64;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + x[[k, 1]] * x[[k, 2]]).collect();
        let a = estimate_borgonovo_delta(x.view(), &y).unwrap();
        let b = estimate_borgonovo_delta(x.view(), &y).unwrap();
        assert_eq!(a.delta, b.delta);
    }

    // ── Helpers: KDE / trapz / ranks ──────────────────────────────

    #[test]
    fn silverman_bandwidth_matches_formula() {
        // For data with σ = 1, n = 100: h = (75)^(−1/5) · 1 ≈ 0.4156.
        let data: Vec<f64> = (0..100_i32)
            .map(|i| (f64::from(i) - 49.5) / 28.866_07)
            .collect();
        // ^ pre-scaled so σ ≈ 1.
        let h = silverman_bandwidth(&data);
        let expected = 75.0_f64.powf(-0.2);
        assert!(
            (h - expected).abs() < 0.01,
            "h = {h}, expected ≈ {expected}"
        );
    }

    #[test]
    fn gaussian_kde_normalizes_to_unity() {
        // ∫ f̂(y) dy ≈ 1 for any KDE with sufficient grid coverage.
        let data: Vec<f64> = (-50..50).map(|i| f64::from(i) * 0.1).collect();
        let h = silverman_bandwidth(&data);
        let grid: Vec<f64> = (-200..200).map(|i| f64::from(i) * 0.05).collect();
        let pdf = gaussian_kde(&data, h, &grid);
        let area = trapz(&pdf, &grid);
        assert!(
            (area - 1.0).abs() < 0.01,
            "KDE integral = {area}, expected ≈ 1"
        );
    }

    #[test]
    fn trapz_simple_cases() {
        // ∫₀² 2x dx = 4.
        let x = [0.0, 1.0, 2.0];
        let y = [0.0, 2.0, 4.0];
        assert!((trapz(&y, &x) - 4.0).abs() < 1e-12);
    }
}
