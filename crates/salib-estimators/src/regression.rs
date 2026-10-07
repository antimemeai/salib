//! Regression coefficients and partial correlations on raw and ranked data.
//!
//! SRC standardizes an OLS coefficient by the input/output standard deviations.
//! SRRC applies SRC to ranks. PCC correlates the residuals obtained by regressing
//! an input and the output on the other inputs; PRCC applies that procedure to
//! ranks. Inspect fit diagnostics and residuals. No fixed R² threshold proves
//! the indices adequate, and rank coefficients are not generally Sobol' indices.
//!
//! # Tied observations
//!
//! Equal values receive the average of their occupied one-based ranks, so ties
//! do not introduce a row-order association. This is the usual rank treatment;
//! see [Marino et al. (2008), §2.1 and footnote 3](https://pmc.ncbi.nlm.nih.gov/articles/PMC2570191/).
//!
//! # Cost and reproducibility
//!
//! PCC and PRCC refit two regressions per factor. Dense normal equations and
//! Cholesky solves cost O(N d³ + d⁴) overall, plus rank sorting. Results are
//! reproducible for the same ordered data, binary, and platform.
//!
//! [Saltelli and Marivoet (1990)](https://doi.org/10.1016/0951-8320%2890%2990065-U)
//! discusses regression and rank-based sensitivity measures.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::many_single_char_names,
    clippy::needless_range_loop
)]

use std::cmp::Ordering;
use std::fmt;

use nalgebra::{DMatrix, DVector};
use ndarray::{Array2, ArrayView2};
use salib_core::tree_sum;

/// Regression-based sensitivity indices and `R²` diagnostics.
///
/// Fields include both linear and rank-regression fit diagnostics.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct RegressionIndices {
    /// Standardized regression coefficients, length `d`. Inspect `r2_linear`.
    pub src: Vec<f64>,
    /// Standardized rank regression coefficients, length `d`.
    /// Inspect `r2_rank`; tied values receive average ranks.
    pub srrc: Vec<f64>,
    /// Partial correlation coefficients, length `d`.
    pub pcc: Vec<f64>,
    /// Partial rank correlation coefficients, length `d`.
    pub prcc: Vec<f64>,
    /// `R²` of the linear OLS fit `Y ≈ β₀ + β·X`. Diagnostic for
    /// SRC and PCC trustworthiness.
    pub r2_linear: f64,
    /// `R²` of the rank-linear OLS fit. Diagnostic for SRRC/PRCC.
    pub r2_rank: f64,
}

impl RegressionIndices {
    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.src.len()
    }
}

impl fmt::Display for RegressionIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Regression indices (d={})", self.d())?;
        writeln!(
            f,
            "  R\u{00b2}(linear) = {:.4}  R\u{00b2}(rank) = {:.4}",
            self.r2_linear, self.r2_rank
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}",
            "Factor", "SRC", "SRRC", "PCC", "PRCC"
        )?;
        writeln!(
            f,
            "  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}",
            "------", "------", "------", "------", "------"
        )?;
        for i in 0..self.d() {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}  {:>8.4}  {:>8.4}",
                i, self.src[i], self.srrc[i], self.pcc[i], self.prcc[i]
            )?;
        }
        Ok(())
    }
}

/// Errors from [`estimate_regression_indices`].
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum RegressionError {
    /// Input and output shapes are incompatible.
    #[error("regression: shape mismatch — X has {x_rows} rows, y has {y_len} elements")]
    ShapeMismatch {
        /// Number of rows in the input matrix.
        x_rows: usize,
        /// Number of output observations.
        y_len: usize,
    },
    /// At least one input dimension is required.
    #[error("regression: d must be ≥ 1, got 0")]
    ZeroD,
    /// Need at least `d + 2` samples to fit `d` regression
    /// coefficients + intercept and have ≥ 1 residual DOF.
    #[error("regression: N must be ≥ d + 2 (got N={n}, d={d}, minimum={minimum})")]
    InsufficientSamples {
        /// Sample count supplied by the caller.
        n: usize,
        /// Number of input factors.
        d: usize,
        /// Minimum sample count required by this configuration.
        minimum: usize,
    },
    /// Total variance of `Y` is zero — model is constant; no
    /// regression signal to recover.
    #[error("regression: Var(Y) is zero (model output is constant)")]
    ZeroVariance,
    /// Total variance of `Xᵢ` is zero for some factor — design
    /// matrix is rank-deficient.
    #[error("regression: Var(X[:, {factor}]) is zero — design matrix rank-deficient")]
    ZeroFactorVariance {
        /// Zero-based factor index that failed validation.
        factor: usize,
    },
    /// `(XᵀX)` is singular — design matrix is rank-deficient
    /// despite per-factor variance checks. Possible collinearity.
    #[error("regression: design matrix XᵀX is singular (factor collinearity?)")]
    SingularDesignMatrix,
}

/// Estimate SRC / SRRC / PCC / PRCC plus `R²` diagnostics.
///
/// `x` is the `(N, d)` input matrix; `y` is the `N`-element model
/// output. Sampler-agnostic.
///
/// # Errors
///
/// - [`RegressionError::ShapeMismatch`] if `x.nrows() != y.len()`.
/// - [`RegressionError::ZeroD`] if `x.ncols() == 0`.
/// - [`RegressionError::InsufficientSamples`] if `N < d + 2`.
/// - [`RegressionError::ZeroVariance`] if `Var(Y) ≈ 0`.
/// - [`RegressionError::ZeroFactorVariance`] if any column of `X`
///   is constant.
/// - [`RegressionError::SingularDesignMatrix`] if `XᵀX` is
///   non-invertible.
pub fn estimate_regression_indices(
    x: ArrayView2<'_, f64>,
    y: &[f64],
) -> Result<RegressionIndices, RegressionError> {
    let n = x.nrows();
    let d = x.ncols();
    if d == 0 {
        return Err(RegressionError::ZeroD);
    }
    if y.len() != n {
        return Err(RegressionError::ShapeMismatch {
            x_rows: n,
            y_len: y.len(),
        });
    }
    let minimum = d + 2;
    if n < minimum {
        return Err(RegressionError::InsufficientSamples { n, d, minimum });
    }

    // Per-column variance checks.
    let var_y = sample_variance(y);
    if !var_y.is_finite() || var_y < 1e-15 {
        return Err(RegressionError::ZeroVariance);
    }
    for j in 0..d {
        let col: Vec<f64> = (0..n).map(|k| x[[k, j]]).collect();
        let variance = sample_variance(&col);
        if !variance.is_finite() || variance < 1e-15 {
            return Err(RegressionError::ZeroFactorVariance { factor: j });
        }
    }

    // ── Linear regression: SRC + PCC + R²_linear ──────────────────
    let (src, pcc, r2_linear) = compute_src_pcc_r2(x, y)?;

    // ── Rank regression: SRRC + PRCC + R²_rank ────────────────────
    // Build rank-transformed X and Y.
    let mut x_rank = Array2::<f64>::zeros((n, d));
    for j in 0..d {
        let col: Vec<f64> = (0..n).map(|k| x[[k, j]]).collect();
        let r = average_ranks_f64(&col);
        for k in 0..n {
            x_rank[[k, j]] = r[k];
        }
    }
    let y_rank = average_ranks_f64(y);

    let (srrc, prcc, r2_rank) = compute_src_pcc_r2(x_rank.view(), &y_rank)?;

    Ok(RegressionIndices {
        src,
        srrc,
        pcc,
        prcc,
        r2_linear,
        r2_rank,
    })
}

/// Compute SRC, PCC, and `R²` for the regression of `y` on `x`.
///
/// SRC: `β̂ᵢ · σ_{Xᵢ} / σ_y` from `Y ≈ β₀ + β·X`.
/// PCC: Pearson correlation between residuals of `Xᵢ` and `y`
///      regressed on the *other* X columns.
fn compute_src_pcc_r2(
    x: ArrayView2<'_, f64>,
    y: &[f64],
) -> Result<(Vec<f64>, Vec<f64>, f64), RegressionError> {
    let n = x.nrows();
    let d = x.ncols();

    // OLS: Y = β₀ + β·X. Build augmented design matrix [1, X].
    let design = build_design_matrix(x, n, d);
    let y_vec = DVector::from_iterator(n, y.iter().copied());

    let beta = solve_ols(&design, &y_vec)?;

    // Residuals and R².
    let y_hat = &design * &beta;
    let residuals = &y_vec - &y_hat;
    let ss_res = residuals.dot(&residuals);
    let y_mean = y_vec.mean();
    let centered = y_vec.map(|v| v - y_mean);
    let ss_tot = centered.dot(&centered);
    let r2 = if ss_tot > 1e-15 {
        1.0 - ss_res / ss_tot
    } else {
        0.0
    };

    // SRC = β̂_j · σ_X_j / σ_Y for j = 1..=d (skipping intercept).
    let sigma_y = (ss_tot / (n as f64)).sqrt();
    let mut src = vec![0.0_f64; d];
    for j in 0..d {
        let col: Vec<f64> = (0..n).map(|k| x[[k, j]]).collect();
        let sigma_xj = sample_variance(&col).sqrt();
        // β[0] is intercept; β[j+1] is coefficient on X_j.
        src[j] = beta[j + 1] * sigma_xj / sigma_y;
    }

    // PCC: for each factor j, regress X_j on the *other* factors,
    // regress y on the other factors, then correlate residuals.
    let mut pcc = vec![0.0_f64; d];
    for j in 0..d {
        pcc[j] = partial_correlation(x, y, j)?;
    }

    Ok((src, pcc, r2))
}

/// `(N, d+1)` design matrix `[1, X]`.
fn build_design_matrix(x: ArrayView2<'_, f64>, n: usize, d: usize) -> DMatrix<f64> {
    let mut design = DMatrix::<f64>::zeros(n, d + 1);
    for k in 0..n {
        design[(k, 0)] = 1.0;
        for j in 0..d {
            design[(k, j + 1)] = x[[k, j]];
        }
    }
    design
}

/// Solve `(XᵀX) β = Xᵀ y` via Cholesky.
fn solve_ols(design: &DMatrix<f64>, y: &DVector<f64>) -> Result<DVector<f64>, RegressionError> {
    let xt = design.transpose();
    let xtx = &xt * design;
    let xty = &xt * y;
    xtx.cholesky()
        .ok_or(RegressionError::SingularDesignMatrix)
        .map(|chol| chol.solve(&xty))
}

/// Partial correlation between `X[:, j]` and `y`, controlling for
/// the other columns of `X`.
fn partial_correlation(
    x: ArrayView2<'_, f64>,
    y: &[f64],
    j: usize,
) -> Result<f64, RegressionError> {
    let n = x.nrows();
    let d = x.ncols();

    // Build "other factors" matrix Z = X with column j removed.
    let z = if d > 1 {
        let mut z_arr = Array2::<f64>::zeros((n, d - 1));
        let mut col_in_z = 0;
        for j_other in 0..d {
            if j_other == j {
                continue;
            }
            for k in 0..n {
                z_arr[[k, col_in_z]] = x[[k, j_other]];
            }
            col_in_z += 1;
        }
        z_arr
    } else {
        Array2::<f64>::zeros((n, 0))
    };

    // Residuals from regressing X[:, j] on Z (with intercept) and
    // y on Z.
    let xj: Vec<f64> = (0..n).map(|k| x[[k, j]]).collect();
    let xj_resid = if d > 1 {
        residuals_from_regression(z.view(), &xj, n, d - 1)?
    } else {
        // d = 1: no other factors → residuals = X_j - mean(X_j).
        let mean_xj = tree_sum(&xj) / (n as f64);
        xj.iter().map(|&v| v - mean_xj).collect()
    };
    let y_resid = if d > 1 {
        residuals_from_regression(z.view(), y, n, d - 1)?
    } else {
        let mean_y = tree_sum(y) / (n as f64);
        y.iter().map(|&v| v - mean_y).collect()
    };

    Ok(pearson_correlation(&xj_resid, &y_resid))
}

/// Fit `target ≈ α₀ + α·z` via OLS and return the residuals.
fn residuals_from_regression(
    z: ArrayView2<'_, f64>,
    target: &[f64],
    n: usize,
    d_z: usize,
) -> Result<Vec<f64>, RegressionError> {
    let design = build_design_matrix(z, n, d_z);
    let y_vec = DVector::from_iterator(n, target.iter().copied());
    let beta = solve_ols(&design, &y_vec)?;
    let predicted = &design * &beta;
    let resid = (0..n).map(|k| y_vec[k] - predicted[k]).collect();
    Ok(resid)
}

/// Pearson correlation. Returns 0.0 if either input has zero variance.
fn pearson_correlation(a: &[f64], b: &[f64]) -> f64 {
    debug_assert_eq!(a.len(), b.len());
    let n = a.len() as f64;
    let mean_a = tree_sum(a) / n;
    let mean_b = tree_sum(b) / n;
    let mut cov = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;
    for (av, bv) in a.iter().zip(b.iter()) {
        let da = av - mean_a;
        let db = bv - mean_b;
        cov += da * db;
        var_a += da * da;
        var_b += db * db;
    }
    let denom = (var_a * var_b).sqrt();
    if denom < 1e-15 {
        0.0
    } else {
        cov / denom
    }
}

/// Population variance with divisor `N`, matching the output scaling used here.
fn sample_variance(v: &[f64]) -> f64 {
    let n = v.len() as f64;
    let mean = tree_sum(v) / n;
    let sq_sum: f64 = v.iter().map(|x| (x - mean).powi(2)).sum();
    sq_sum / n
}

/// One-based ranks; each equal-value block receives its mean occupied rank.
fn average_ranks_f64(data: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..data.len()).collect();
    idx.sort_by(|&a, &b| data[a].partial_cmp(&data[b]).unwrap_or(Ordering::Equal));
    let mut ranks = vec![0.0_f64; data.len()];
    let mut start = 0;
    while start < idx.len() {
        let mut end = start + 1;
        while end < idx.len() && data[idx[end]] == data[idx[start]] {
            end += 1;
        }
        let mean_rank = (start as f64 + 1.0 + end as f64) / 2.0;
        for &i in &idx[start..end] {
            ranks[i] = mean_rank;
        }
        start = end;
    }
    ranks
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {

    #[test]
    fn phase3_regression_rejects_nonfinite_variance() {
        let mut x = ndarray::array![[0.0], [1.0], [2.0], [3.0]];
        let y = [0.0, 1.0, 2.0, 3.0];
        for invalid in [f64::NAN, f64::INFINITY] {
            let mut invalid_y = y;
            invalid_y[0] = invalid;
            assert!(matches!(
                estimate_regression_indices(x.view(), &invalid_y),
                Err(RegressionError::ZeroVariance)
            ));
            x[[0, 0]] = invalid;
            assert!(matches!(
                estimate_regression_indices(x.view(), &y),
                Err(RegressionError::ZeroFactorVariance { factor: 0 })
            ));
        }
    }

    use super::*;

    fn synthetic_x(n: usize, d: usize) -> Array2<f64> {
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
            estimate_regression_indices(x.view(), &y).unwrap_err(),
            RegressionError::ZeroD
        );
    }

    #[test]
    fn shape_mismatch_errors() {
        let x = Array2::<f64>::zeros((100, 3));
        let y = vec![0.0; 50];
        let err = estimate_regression_indices(x.view(), &y).unwrap_err();
        assert!(matches!(err, RegressionError::ShapeMismatch { .. }));
    }

    #[test]
    fn insufficient_samples_errors() {
        // d=3, need N ≥ 5.
        let x = synthetic_x(4, 3);
        let y = vec![0.0; 4];
        let err = estimate_regression_indices(x.view(), &y).unwrap_err();
        assert!(matches!(err, RegressionError::InsufficientSamples { .. }));
    }

    #[test]
    fn constant_model_errors() {
        let x = synthetic_x(64, 3);
        let y = vec![1.0; 64];
        let err = estimate_regression_indices(x.view(), &y).unwrap_err();
        assert_eq!(err, RegressionError::ZeroVariance);
    }

    #[test]
    fn zero_factor_variance_errors() {
        let mut x = synthetic_x(64, 3);
        for k in 0..64 {
            x[[k, 1]] = 0.5; // factor 1 constant
        }
        let y: Vec<f64> = (0..64).map(|k| x[[k, 0]] + x[[k, 2]]).collect();
        let err = estimate_regression_indices(x.view(), &y).unwrap_err();
        assert_eq!(err, RegressionError::ZeroFactorVariance { factor: 1 });
    }

    // ── Output shape ──────────────────────────────────────────────

    #[test]
    fn output_lengths_match_d() {
        let n = 64;
        let x = synthetic_x(n, 4);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + 2.0 * x[[k, 1]]).collect();
        let est = estimate_regression_indices(x.view(), &y).unwrap();
        assert_eq!(est.d(), 4);
        assert_eq!(est.src.len(), 4);
        assert_eq!(est.srrc.len(), 4);
        assert_eq!(est.pcc.len(), 4);
        assert_eq!(est.prcc.len(), 4);
    }

    // ── Linear model: SRC + PCC near 1 for active factor ─────────

    #[test]
    fn linear_model_recovers_high_r2_and_dominant_src() {
        // Y = X[:, 0] — perfectly linear, factor 0 dominant.
        let n = 256;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let est = estimate_regression_indices(x.view(), &y).unwrap();
        assert!(est.r2_linear > 0.99, "R²_linear = {}", est.r2_linear);
        assert!(
            est.src[0].abs() > 0.95,
            "|SRC_0| = {} should dominate for Y = X_0",
            est.src[0].abs()
        );
        assert!(
            est.src[1].abs() < 0.1,
            "|SRC_1| = {} should be small",
            est.src[1].abs()
        );
        assert!(
            est.pcc[0].abs() > 0.95,
            "|PCC_0| = {} should dominate",
            est.pcc[0].abs()
        );
    }

    // ── Linear with multiple factors ─────────────────────────────

    #[test]
    fn linear_combination_yields_proportional_src() {
        // Y = 2·X[:, 0] + X[:, 1]. With independent uniform X,
        // SRC scales with the coefficient times std ratio. Both
        // factors have the same std, so SRC_0 should be ~2× SRC_1.
        let n = 1024;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| 2.0 * x[[k, 0]] + x[[k, 1]]).collect();
        let est = estimate_regression_indices(x.view(), &y).unwrap();
        assert!(est.r2_linear > 0.99);
        // Ratio SRC_0 / SRC_1 should be ≈ 2 within MC noise.
        let ratio = est.src[0].abs() / est.src[1].abs();
        assert!(
            (ratio - 2.0).abs() < 0.2,
            "SRC ratio = {ratio}, expected ≈ 2"
        );
    }

    // ── Non-linear: low R² flags untrustworthy SRC ────────────────

    #[test]
    fn nonlinear_model_yields_low_r2_linear() {
        // Y = X[:, 0]² is non-linear. R²_linear should be lower
        // than R²_rank since rank captures monotonic-in-X²
        // structure.
        let n = 512;
        let x = synthetic_x(n, 2);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]].powi(2)).collect();
        let est = estimate_regression_indices(x.view(), &y).unwrap();
        // R²_linear is reduced because Y vs X is not linear.
        // (It's still > 0 since X² is correlated with X over [0, 1].)
        assert!(est.r2_linear < 0.99, "R²_linear = {}", est.r2_linear);
        // R²_rank should be high — Y is monotonic in X over [0, 1].
        assert!(est.r2_rank > 0.95, "R²_rank = {}", est.r2_rank);
    }

    // ── PRCC catches monotonicity that PCC misses on cubed input ─

    #[test]
    fn prcc_catches_strong_monotonic_relationship_with_high_magnitude() {
        // Y = (X[:, 0] - 0.5)³ — strictly monotonic in X_0 over
        // [0, 1] (cube preserves order). PRCC_0 should be ~1.
        let n = 1024;
        let x = synthetic_x(n, 2);
        let y: Vec<f64> = (0..n).map(|k| (x[[k, 0]] - 0.5).powi(3)).collect();
        let est = estimate_regression_indices(x.view(), &y).unwrap();
        assert!(
            est.prcc[0].abs() > 0.95,
            "|PRCC_0| = {} should be near 1 (strict monotonic)",
            est.prcc[0].abs()
        );
    }

    // ── Determinism ───────────────────────────────────────────────

    #[test]
    fn same_input_yields_identical_output() {
        let n = 64;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + x[[k, 1]] * x[[k, 2]]).collect();
        let a = estimate_regression_indices(x.view(), &y).unwrap();
        let b = estimate_regression_indices(x.view(), &y).unwrap();
        assert_eq!(a.src, b.src);
        assert_eq!(a.srrc, b.srrc);
        assert_eq!(a.pcc, b.pcc);
        assert_eq!(a.prcc, b.prcc);
        assert_eq!(a.r2_linear, b.r2_linear);
        assert_eq!(a.r2_rank, b.r2_rank);
    }

    // ── Pearson + ranking unit ────────────────────────────────────

    #[test]
    fn pearson_correlation_perfect_positive() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let b = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        assert!((pearson_correlation(&a, &b) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn pearson_correlation_perfect_negative() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let b = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        assert!((pearson_correlation(&a, &b) + 1.0).abs() < 1e-12);
    }

    #[test]
    fn pearson_correlation_uncorrelated() {
        // Constant b → zero variance → returns 0.
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![5.0, 5.0, 5.0];
        assert_eq!(pearson_correlation(&a, &b), 0.0);
    }

    #[test]
    fn average_ranks_one_indexed() {
        let data = [3.0, 1.0, 2.0];
        assert_eq!(average_ranks_f64(&data), vec![3.0, 1.0, 2.0]);
    }

    #[test]
    fn tied_ranks_use_the_mean_of_occupied_positions() {
        // The zeros occupy ranks 1 and 2, the twos ranks 4 through 6.
        assert_eq!(
            average_ranks_f64(&[2.0, -0.0, 1.0, 2.0, 0.0, 2.0]),
            vec![5.0, 1.5, 3.0, 5.0, 1.5, 5.0]
        );
    }

    #[test]
    fn independent_tied_binary_data_has_zero_rank_association() {
        // Balanced 2x2 table: each value of X sees the same distribution of Y.
        let x = ndarray::array![[0.0], [0.0], [1.0], [1.0]];
        let y = [0.0, 1.0, 0.0, 1.0];
        for order in [[0, 1, 2, 3], [1, 0, 3, 2], [3, 1, 0, 2]] {
            let permuted_x = Array2::from_shape_fn((4, 1), |(row, _)| x[[order[row], 0]]);
            let permuted_y: Vec<_> = order.iter().map(|&row| y[row]).collect();
            let result = estimate_regression_indices(permuted_x.view(), &permuted_y).unwrap();
            for coefficient in [result.src[0], result.pcc[0], result.srrc[0], result.prcc[0]] {
                assert!(
                    coefficient.abs() < 1e-12,
                    "spurious association: {coefficient}"
                );
            }
        }
    }

    // ── d=1 special case (no "other factors" for partial corr) ───

    #[test]
    fn d_one_pcc_equals_pearson_with_y() {
        // For d=1, PCC reduces to Pearson(X_0, Y).
        let n = 100;
        let x = synthetic_x(n, 1);
        let y: Vec<f64> = (0..n).map(|k| 2.0 * x[[k, 0]]).collect();
        let est = estimate_regression_indices(x.view(), &y).unwrap();
        // Y is perfectly linear in X_0 → PCC ≈ 1.
        assert!(
            (est.pcc[0].abs() - 1.0).abs() < 1e-6,
            "|PCC_0| = {}",
            est.pcc[0].abs()
        );
    }
}
