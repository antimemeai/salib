//! Approximate first-order Sobol' indices from paired input/output data.
//!
//! The partition approach is described by
//! [Plischke, Borgonovo and Smith (2013)](https://doi.org/10.1016/j.ejor.2012.11.047).
//! For each input, sort observations by value and split them into
//! nearly equal-size classes without separating tied values. Compute
//!
//! ```text
//! S_i = 1 - sum_m (n_m/N) Var_Nm(Y in class m) / Var_N(Y).
//! ```
//!
//! Variances use the population divisor. This is the empirical
//! between-class variance divided by total variance, clamped to `[0,1]`
//! for rounding error. It has no bias correction.
//!
//! Classes approximate conditioning on an input value. Their count is
//! automatic and capped at 48, so increasing `N` cannot generally remove
//! the remaining binning error. Ties are assigned by their sorted rank
//! block midpoint; empty classes are omitted. A constant input gives
//! one class and zero index.
//! With dependent inputs, the measure includes input association.
//!
//! Sorting and class accumulation cost `O(d N log N)`, with `M <= 48`.
//! No additional model evaluations or paired sampling design are needed.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::needless_range_loop
)]

use std::fmt;

use crate::borgonovo::class_count;
use crate::conditioning::classes;
#[cfg(test)]
use ndarray::Array2;
use ndarray::ArrayView2;
use salib_core::tree_sum;

/// First-order Sobol' index estimates from given-data partition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct GivenDataSobolIndices {
    /// Approximate first-order index per factor, length `d`.
    /// Clamped to `[0, 1]` for floating-point rounding error.
    pub s1: Vec<f64>,
}

impl GivenDataSobolIndices {
    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.s1.len()
    }
}

impl fmt::Display for GivenDataSobolIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Given-data Sobol' indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S1")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.s1[i])?;
        }
        Ok(())
    }
}

/// Errors from [`estimate_given_data_sobol`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum GivenDataSobolError {
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
    #[error("given-data Sobol': shape mismatch — X has {x_rows} rows, y has {y_len} elements")]
    ShapeMismatch {
        /// Number of rows in the input matrix.
        x_rows: usize,
        /// Number of output observations.
        y_len: usize,
    },
    /// At least one input dimension is required.
    #[error("given-data Sobol': d must be ≥ 1, got 0")]
    ZeroD,
    /// `N < 16`, below this implementation's minimum sample count.
    #[error("given-data Sobol': N must be ≥ 16, got {n}")]
    InsufficientSamples {
        /// Sample count supplied by the caller.
        n: usize,
    },
    /// Output variance is zero or below the estimator threshold.
    #[error("given-data Sobol': Var(Y) is zero (model output is constant)")]
    ZeroVariance,
}

/// Estimate first-order Sobol' indices from generic `(X, Y)` data
/// via the Plischke-Borgonovo-Smith 2013 partition estimator.
///
/// `x` is the `(N, d)` input matrix; `y` is the `N`-element model
/// output. Rows must be aligned and represent the input distribution
/// of interest. The estimator does not require a paired design.
///
/// # Errors
///
/// - [`GivenDataSobolError::NonfiniteInput`] for any NaN or infinite input.
/// - [`GivenDataSobolError::NonfiniteOutput`] for any NaN or infinite output.
///
/// - [`GivenDataSobolError::ShapeMismatch`] if `x.nrows() != y.len()`.
/// - [`GivenDataSobolError::ZeroD`] if `x.ncols() == 0`.
/// - [`GivenDataSobolError::InsufficientSamples`] if `N < 16`.
/// - [`GivenDataSobolError::ZeroVariance`] if `Var(Y) ≈ 0`.
pub fn estimate_given_data_sobol(
    x: ArrayView2<'_, f64>,
    y: &[f64],
) -> Result<GivenDataSobolIndices, GivenDataSobolError> {
    let n = x.nrows();
    let d = x.ncols();
    if d == 0 {
        return Err(GivenDataSobolError::ZeroD);
    }
    if y.len() != n {
        return Err(GivenDataSobolError::ShapeMismatch {
            x_rows: n,
            y_len: y.len(),
        });
    }
    if n < 16 {
        return Err(GivenDataSobolError::InsufficientSamples { n });
    }

    if let Some(((row, column), _)) = x.indexed_iter().find(|(_, v)| !v.is_finite()) {
        return Err(GivenDataSobolError::NonfiniteInput { row, column });
    }
    if let Some(index) = y.iter().position(|v| !v.is_finite()) {
        return Err(GivenDataSobolError::NonfiniteOutput { index });
    }

    let var_y = population_variance(y);
    if !var_y.is_finite() || var_y < 1e-15 {
        return Err(GivenDataSobolError::ZeroVariance);
    }

    let n_classes = class_count(n);
    let n_f = n as f64;
    let mut s1 = vec![0.0_f64; d];

    let mut x_col_buf = vec![0.0_f64; n];
    let mut class_y_buf: Vec<f64> = Vec::with_capacity(n);

    for i in 0..d {
        for k in 0..n {
            x_col_buf[k] = x[[k, i]];
        }
        let groups = classes(&x_col_buf, n_classes);
        let mut weighted_intra = Vec::with_capacity(groups.len());
        for group in groups {
            class_y_buf.clear();
            class_y_buf.extend(group.iter().map(|&k| y[k]));
            let weight = class_y_buf.len() as f64 / n_f;
            weighted_intra.push(weight * population_variance(&class_y_buf));
        }
        let e_var_given_x = tree_sum(&weighted_intra);
        // Law of total variance: S_1 = 1 - E[Var(Y|X_i)] / Var(Y).
        let raw = 1.0 - e_var_given_x / var_y;
        s1[i] = raw.clamp(0.0, 1.0);
    }

    Ok(GivenDataSobolIndices { s1 })
}

/// Population variance (1/n divisor; not Bessel) — matches the
/// scaling used by `SALib`'s `delta.sobol_first`.
fn population_variance(v: &[f64]) -> f64 {
    let n = v.len() as f64;
    let mean = tree_sum(v) / n;
    let sq_sum: f64 = v.iter().map(|x| (x - mean).powi(2)).sum();
    sq_sum / n
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {
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
                #[allow(clippy::cast_possible_truncation)]
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
            estimate_given_data_sobol(x.view(), &y).unwrap_err(),
            GivenDataSobolError::ZeroD
        );
    }

    #[test]
    fn shape_mismatch_errors() {
        let x = Array2::<f64>::zeros((100, 3));
        let y = vec![0.0; 50];
        let err = estimate_given_data_sobol(x.view(), &y).unwrap_err();
        assert!(matches!(err, GivenDataSobolError::ShapeMismatch { .. }));
    }

    #[test]
    fn insufficient_samples_errors() {
        let x = synthetic_x(10, 3);
        let y = vec![0.0; 10];
        let err = estimate_given_data_sobol(x.view(), &y).unwrap_err();
        assert!(matches!(
            err,
            GivenDataSobolError::InsufficientSamples { .. }
        ));
    }

    #[test]
    fn constant_model_errors() {
        let x = synthetic_x(64, 3);
        let y = vec![1.0; 64];
        let err = estimate_given_data_sobol(x.view(), &y).unwrap_err();
        assert_eq!(err, GivenDataSobolError::ZeroVariance);
    }

    // ── Output shape ──────────────────────────────────────────────

    #[test]
    fn output_length_matches_d() {
        let n = 256;
        let x = synthetic_x(n, 5);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let est = estimate_given_data_sobol(x.view(), &y).unwrap();
        assert_eq!(est.d(), 5);
        assert_eq!(est.s1.len(), 5);
    }

    // ── S_1 in [0, 1] ─────────────────────────────────────────────

    #[test]
    fn indices_in_unit_interval() {
        let n = 512;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n)
            .map(|k| x[[k, 0]] + 0.5 * x[[k, 1]] * x[[k, 2]])
            .collect();
        let est = estimate_given_data_sobol(x.view(), &y).unwrap();
        for &v in &est.s1 {
            assert!((0.0..=1.0).contains(&v), "S_1 = {v} not in [0, 1]");
        }
    }

    // ── Linear single-factor: S_1[0] should dominate ─────────────

    #[test]
    fn linear_single_factor_dominates() {
        // Y = X[:, 0] — first-order S_1[0] should be near 1.
        let n = 512;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let est = estimate_given_data_sobol(x.view(), &y).unwrap();
        assert!(
            est.s1[0] > 0.7,
            "S_1[0] = {} should dominate for Y = X_0",
            est.s1[0]
        );
        assert!(
            est.s1[1] < 0.3,
            "S_1[1] = {} should be small (factor 1 not in model)",
            est.s1[1]
        );
    }

    // ── Additive sum of two: S_1 should split proportionally ─────

    #[test]
    fn additive_two_factors_split_proportionally() {
        // Y = 2·X[:, 0] + X[:, 1] over independent uniform X.
        // Var(Y) = 4·Var(X_0) + Var(X_1) = 5·(1/12) ≈ 0.417.
        // Var(E[Y|X_0]) = Var(2·X_0) = 4/12 = 1/3.
        // Var(E[Y|X_1]) = Var(X_1) = 1/12.
        // S_1[0] = (1/3) / (5/12) = 4/5 = 0.8.
        // S_1[1] = (1/12) / (5/12) = 1/5 = 0.2.
        let n = 4096;
        let x = synthetic_x(n, 2);
        let y: Vec<f64> = (0..n).map(|k| 2.0 * x[[k, 0]] + x[[k, 1]]).collect();
        let est = estimate_given_data_sobol(x.view(), &y).unwrap();
        assert!(
            (est.s1[0] - 0.8).abs() < 0.05,
            "S_1[0] = {} should ≈ 0.8",
            est.s1[0]
        );
        assert!(
            (est.s1[1] - 0.2).abs() < 0.05,
            "S_1[1] = {} should ≈ 0.2",
            est.s1[1]
        );
    }

    // ── Determinism ───────────────────────────────────────────────

    #[test]
    fn same_input_yields_identical_output() {
        let n = 64;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + x[[k, 1]] * x[[k, 2]]).collect();
        let a = estimate_given_data_sobol(x.view(), &y).unwrap();
        let b = estimate_given_data_sobol(x.view(), &y).unwrap();
        assert_eq!(a.s1, b.s1);
    }

    // ── Helpers ───────────────────────────────────────────────────

    #[test]
    fn class_count_matches_borgonovo_table() {
        // Same formula as borgonovo::class_count.
        assert_eq!(class_count(1024), 6);
        assert_eq!(class_count(4096), 16);
        assert_eq!(class_count(1_000_000), 48);
    }

    #[test]
    fn population_variance_zero_for_constant() {
        assert_eq!(population_variance(&[1.0, 1.0, 1.0]), 0.0);
    }

    #[test]
    fn population_variance_matches_formula() {
        // Var([1, 2, 3]) population = ((1-2)² + (2-2)² + (3-2)²)/3
        //                           = 2/3 ≈ 0.6667.
        let v = vec![1.0, 2.0, 3.0];
        let expected = 2.0 / 3.0;
        assert!((population_variance(&v) - expected).abs() < 1e-12);
    }
}
