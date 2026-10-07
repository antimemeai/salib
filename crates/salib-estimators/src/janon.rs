//! Janon's symmetrized first-order Sobol' estimator (`T_N^X`).
//!
//! [Janon et al. (2014)](https://doi.org/10.1051/ps/2013040), Eq. (2.6),
//! pairs `Y = f(B)` with `Y^X = f(A_Bⁱ)`, sharing input `i`. With
//! `m = (mean(Y) + mean(Y^X))/2`, the estimate is
//!
//! ```text
//! T_N^X = mean((Y - m) * (Y^X - m))
//!         / mean(((Y - m)^2 + (Y^X - m)^2)/2).
//! ```
//!
//! This is algebraically equivalent to Eqs. (2.6) and (2.8) in the
//! published paper. Proposition 3.5 establishes asymptotic efficiency
//! among regular estimators for the exchangeable pair model; the normal
//! limit in Proposition 3.2 assumes a finite fourth output moment and
//! independent, identically distributed pairs. These results do not give
//! a finite-sample or deterministic QMC ranking of estimators.
//!
//! The implementation uses `N(d+2)` model calls, or `N(2d+2)` with
//! second-order blocks. Its optional pairwise estimates use a separate
//! cross-product formula; the Janon efficiency result does not cover them.
//! For total effects, use [`crate::estimate_saltelli2010`].

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::expect_used
)]

use std::fmt;

use ndarray::Array2;
use salib_core::tree_sum;
use salib_samplers::SaltelliMatrix;

/// First-order Sobol' indices via Janon `T_N^X`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct JanonIndices {
    /// First-order Sobol' indices, length `d`.
    pub first_order: Vec<f64>,
    /// Output variance estimated from `f(B)` with divisor `N`.
    pub total_variance: f64,
    /// Second-order indices `S2_{i,j}` for all `i < j`, laid out as
    /// `second_order[i][k] = S2_{i, i+k+1}` (upper triangle, row-major).
    /// `Some` only when the `SaltelliMatrix` supplies `b_a` matrices.
    pub second_order: Option<Vec<Vec<f64>>>,
}

impl JanonIndices {
    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.first_order.len()
    }
}

impl fmt::Display for JanonIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Janon indices (d={})", self.d())?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S1")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.first_order[i])?;
        }
        Ok(())
    }
}

/// Nonfinite variance or numerators yield zero indices; nonfinite total variance is reported as zero.
///
/// Estimate first-order Sobol' indices via Janon 2014 `T_N^X`.
///
/// Consumes the same `SaltelliMatrix` as
/// `saltelli2010::estimate_saltelli2010`. Pure function; no RNG.
pub fn estimate_janon<F>(matrix: &SaltelliMatrix, model: F) -> JanonIndices
where
    F: Fn(&[f64]) -> f64,
{
    let n = matrix.n;
    let d = matrix.dim;
    let n_f = n as f64;

    // Y = f(B), Y^X_i = f(A_B^i). The pair (Y, Y^X_i) shares column i
    // (both come from B's col i) and differs in everything else.
    let fa = evaluate_rows(&matrix.a, &model);
    let y = evaluate_rows(&matrix.b, &model);
    let y_x: Vec<Vec<f64>> = matrix
        .a_b
        .iter()
        .map(|m| evaluate_rows(m, &model))
        .collect();

    // Diagnostic variance from Y=f(B). Each first-order estimate uses
    // its own joint variance denominator below.
    let mean_y = tree_sum(&y) / n_f;
    let y_sq: Vec<f64> = y.iter().map(|v| (v - mean_y).powi(2)).collect();
    let total_variance = tree_sum(&y_sq) / n_f;

    let mut first_order = Vec::with_capacity(d);
    for y_xi in &y_x {
        let mean_yx = tree_sum(y_xi) / n_f;
        let mean_joint = 0.5 * mean_y + 0.5 * mean_yx;

        // Numerator (Janon Eq. (2.6)), centered about the joint mean.
        let yy_xi: Vec<f64> = y
            .iter()
            .zip(y_xi.iter())
            .map(|(yj, yxj)| (yj - mean_joint) * (yxj - mean_joint))
            .collect();
        let mean_y_yx = tree_sum(&yy_xi) / n_f;
        let num = mean_y_yx;

        // Denominator (Janon Eq. (2.6)), centered about the joint mean.
        // This is the joint second-moment estimator that gives
        // Janon's asymptotic-efficiency property.
        let half_sq_sum: Vec<f64> = y
            .iter()
            .zip(y_xi.iter())
            .map(|(yj, yxj)| 0.5 * (yj - mean_joint).powi(2) + 0.5 * (yxj - mean_joint).powi(2))
            .collect();
        let mean_half_sq = tree_sum(&half_sq_sum) / n_f;
        let denom = mean_half_sq;

        let s_i = if denom.is_finite() && denom > 1e-15 && num.is_finite() {
            num / denom
        } else {
            0.0
        };
        first_order.push(s_i);
    }

    // Pairwise indices from complementary hybrids.
    //
    // When B_Aⁱ matrices are available, compute S2_{ij} for i < j:
    //   V_{ij}  = (1/N) Σ_k [ fba[j][k] · fab[i][k] - fa[k] · fb[k] ]
    //   S2_{ij} = V_{ij} / D  - S_i - S_j
    //
    // Layout: second_order[i][k] = S2_{i, i+k+1} (upper triangle).
    let second_order = matrix.b_a.as_ref().map(|b_a_matrices| {
        let fba: Vec<Vec<f64>> = b_a_matrices
            .iter()
            .map(|m| evaluate_rows(m, &model))
            .collect();

        let fa_fb: Vec<f64> = fa.iter().zip(y.iter()).map(|(a, b)| a * b).collect();

        let mut s2: Vec<Vec<f64>> = Vec::with_capacity(d);
        for i in 0..d {
            let mut row = Vec::with_capacity(d - i - 1);
            for j in (i + 1)..d {
                let cross: Vec<f64> = (0..n).map(|k| fba[j][k] * y_x[i][k] - fa_fb[k]).collect();
                let vij = tree_sum(&cross) / n_f;
                let s2_ij =
                    if total_variance.is_finite() && total_variance > 1e-15 && vij.is_finite() {
                        vij / total_variance - first_order[i] - first_order[j]
                    } else {
                        0.0
                    };
                row.push(s2_ij);
            }
            s2.push(row);
        }
        s2
    });

    JanonIndices {
        first_order,
        total_variance: if total_variance.is_finite() {
            total_variance
        } else {
            0.0
        },
        second_order,
    }
}

fn evaluate_rows<F: Fn(&[f64]) -> f64>(matrix: &Array2<f64>, model: &F) -> Vec<f64> {
    let n = matrix.shape()[0];
    let mut out = Vec::with_capacity(n);
    for row in matrix.rows() {
        let slice = row
            .as_slice()
            .expect("Array2 row should be contiguous (row-major)");
        out.push(model(slice));
    }
    out
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {
    use super::*;
    use salib_core::RngState;
    use salib_samplers::{build_saltelli_matrix, LhsSampler};

    #[test]
    fn phase3_nonfinite_variance_yields_zero_indices() {
        let mut rng = RngState::from_seed([0x42; 32]);
        let matrix = build_saltelli_matrix(&LhsSampler::classic(4), 4, true, &mut rng).unwrap();
        for invalid in [f64::NAN, f64::INFINITY] {
            let result = estimate_janon(&matrix, |_| invalid);
            assert!(result.total_variance.is_finite());
            for value in result
                .first_order
                .iter()
                .chain(result.second_order.as_ref().unwrap().iter().flatten())
            {
                assert!(value.is_finite());
                assert!(value.abs() < 1e-12);
            }
        }
    }

    #[test]
    fn phase3_centered_variance_large_offset() {
        let values = [1e9, 1e9 + 1.0, 1e9 + 2.0, 1e9 + 3.0];
        let mut rng = RngState::from_seed([0x42; 32]);
        let mut matrix =
            build_saltelli_matrix(&LhsSampler::classic(4), 4, false, &mut rng).unwrap();
        for (row, value) in values.iter().enumerate() {
            matrix.a[[row, 0]] = *value;
            matrix.b[[row, 0]] = *value;
            for hybrid in &mut matrix.a_b {
                hybrid[[row, 0]] = *value;
            }
        }
        let result = estimate_janon(&matrix, |x| x[0]);
        assert!(result.total_variance.is_finite());
        assert!((result.total_variance - 1.25).abs() < 1e-12);
        for value in result.first_order {
            assert!(value.is_finite());
            assert!((value - 1.0).abs() < 1e-12);
        }
    }

    // ── Output shape ──────────────────────────────────────────────

    #[test]
    fn output_length_matches_d() {
        let s = LhsSampler::classic(6); // d=3 (Saltelli matrix takes 2d-dim sampler)
        let mut rng = RngState::from_seed([0x42; 32]);
        let m = build_saltelli_matrix(&s, 64, false, &mut rng).unwrap();
        let est = estimate_janon(&m, |x| x[0] + x[1]);
        assert_eq!(est.d(), 3);
    }

    // ── Constant model: zero variance, indices clamp to zero ──────

    #[test]
    fn constant_model_yields_zero_indices() {
        let s = LhsSampler::classic(4);
        let mut rng = RngState::from_seed([0x42; 32]);
        let m = build_saltelli_matrix(&s, 64, false, &mut rng).unwrap();
        let est = estimate_janon(&m, |_| 7.0);
        for &v in &est.first_order {
            assert_eq!(v, 0.0, "constant model should give S_i = 0");
        }
    }

    // ── Linear single-factor: factor 0 dominant ───────────────────

    #[test]
    fn linear_single_factor_concentrates_first_order() {
        // Y = X[0]; S_0 ≈ 1, others ≈ 0.
        let s = LhsSampler::classic(6);
        let mut rng = RngState::from_seed([0x42; 32]);
        let m = build_saltelli_matrix(&s, 4096, false, &mut rng).unwrap();
        let est = estimate_janon(&m, |x| x[0]);
        assert!(
            est.first_order[0] > 0.85,
            "S_0 = {} should be near 1",
            est.first_order[0]
        );
        assert!(
            est.first_order[1].abs() < 0.1,
            "S_1 = {} should be near 0",
            est.first_order[1]
        );
        assert!(
            est.first_order[2].abs() < 0.1,
            "S_2 = {} should be near 0",
            est.first_order[2]
        );
    }

    // ── Determinism ───────────────────────────────────────────────

    #[test]
    fn same_matrix_yields_identical_estimates() {
        let s = LhsSampler::classic(6);
        let mut rng = RngState::from_seed([0x42; 32]);
        let m = build_saltelli_matrix(&s, 256, false, &mut rng).unwrap();
        let model = |x: &[f64]| x[0] + 0.5 * x[1] * x[2];
        let a = estimate_janon(&m, model);
        let b = estimate_janon(&m, model);
        assert_eq!(a.first_order, b.first_order);
        assert_eq!(a.total_variance, b.total_variance);
    }
}
