//! Owen's Correlation 2 estimator for first-order Sobol' indices.
//!
//! [Owen (2013)](https://doi.org/10.1145/2457459.2457460) uses three
//! independent input vectors. The Correlation 2 formula appears in
//! [the preprint, Section 3](https://arxiv.org/abs/1204.4763).
//! For each input `i`, this implementation computes
//!
//! ```text
//! S_i = mean((f(A) - f(A_Cⁱ)) * (f(B_Aⁱ) - f(B))) / D,
//! ```
//!
//! where `A_Cⁱ` replaces column `i` of `A` with `C`, `B_Aⁱ` replaces
//! column `i` of `B` with `A`, and `D` is the pooled `A,B` output variance
//! with divisor `2N`. The ratio uses an estimated denominator; the
//! paper's unbiasedness result for the numerator does not make the ratio
//! unbiased.
//!
//! Random centering can reduce variance when an input has little total
//! influence. A small first-order index alone does not guarantee this:
//! the input may still have large interaction effects. Performance
//! depends on the model and sampling design.
//!
//! The estimator evaluates `A`, `B`, and both hybrid families, for
//! `N(2+2d)` calls. It does not evaluate `C` directly. `OwenMatrix`'s
//! `total_evaluations()` includes `C` and reports `N(3+2d)`.
//! The result has first-order indices only. For total effects, use
//! [`crate::estimate_saltelli2010`] with a separate Saltelli design.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::expect_used
)]

use std::fmt;

use ndarray::Array2;
use salib_core::tree_sum;
use salib_samplers::OwenMatrix;

/// First-order Sobol' indices via Owen Correlation 2.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct OwenIndices {
    /// First-order Sobol' indices, length `d`.
    pub first_order: Vec<f64>,
    /// Total variance estimated from the joint `(A, B)` samples.
    pub total_variance: f64,
    /// Second-order indices; always `None` in this implementation.
    pub second_order: Option<Vec<Vec<f64>>>,
}

impl OwenIndices {
    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.first_order.len()
    }
}

impl fmt::Display for OwenIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Owen indices (d={})", self.d())?;
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
/// Estimate first-order Sobol' indices via Owen 2013 Correlation 2.
///
/// Pure function; no RNG.
pub fn estimate_owen<F>(matrix: &OwenMatrix, model: F) -> OwenIndices
where
    F: Fn(&[f64]) -> f64,
{
    let n = matrix.n;
    let d = matrix.dim;
    let n_f = n as f64;

    // Base evaluations.
    let fa = evaluate_rows(&matrix.a, &model);
    let fb = evaluate_rows(&matrix.b, &model);
    // Hybrid evaluations per factor.
    let fac: Vec<Vec<f64>> = matrix
        .a_c
        .iter()
        .map(|m| evaluate_rows(m, &model))
        .collect();
    let fba: Vec<Vec<f64>> = matrix
        .b_a
        .iter()
        .map(|m| evaluate_rows(m, &model))
        .collect();

    // This implementation estimates the denominator from pooled A,B outputs.
    let mut combined = Vec::with_capacity(2 * n);
    combined.extend_from_slice(&fa);
    combined.extend_from_slice(&fb);
    let mean_combined = tree_sum(&combined) / (2.0 * n_f);
    let sq: Vec<f64> = combined
        .iter()
        .map(|v| (v - mean_combined).powi(2))
        .collect();
    let total_variance = tree_sum(&sq) / (2.0 * n_f);

    let mut first_order = Vec::with_capacity(d);
    for i in 0..d {
        // Owen Correlation 2:
        //   S_i ∝ (1/N) Σ (f(A) − f(A_Cⁱ)) · (f(B_Aⁱ) − f(B)).
        let terms: Vec<f64> = (0..n)
            .map(|j| (fa[j] - fac[i][j]) * (fba[i][j] - fb[j]))
            .collect();
        let num = tree_sum(&terms) / n_f;
        let s_i = if total_variance.is_finite() && total_variance > 1e-15 && num.is_finite() {
            num / total_variance
        } else {
            0.0
        };
        first_order.push(s_i);
    }

    OwenIndices {
        first_order,
        total_variance: if total_variance.is_finite() {
            total_variance
        } else {
            0.0
        },
        second_order: None,
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
    use salib_samplers::{build_owen_matrix, LhsSampler};

    #[test]
    fn phase3_nonfinite_variance_yields_zero_indices() {
        let mut rng = RngState::from_seed([0x42; 32]);
        let matrix = build_owen_matrix(&LhsSampler::classic(6), 4, &mut rng).unwrap();
        for invalid in [f64::NAN, f64::INFINITY] {
            let result = estimate_owen(&matrix, |_| invalid);
            assert!(result.total_variance.is_finite());
            for value in result.first_order {
                assert!(value.is_finite());
                assert!(value.abs() < 1e-12);
            }
        }
        // Finite outputs with overflowing variance: infinity must not be divided into.
        let result = estimate_owen(&matrix, |x| if x[0] < 0.5 { -1e200 } else { 1e200 });
        for value in result.first_order {
            assert!(value.is_finite());
            assert!(value.abs() < 1e-12);
        }
    }

    #[test]
    fn output_length_matches_d() {
        let s = LhsSampler::classic(9); // 3d = 9, d = 3
        let mut rng = RngState::from_seed([0x42; 32]);
        let m = build_owen_matrix(&s, 64, &mut rng).unwrap();
        let est = estimate_owen(&m, |x| x[0]);
        assert_eq!(est.d(), 3);
    }

    #[test]
    fn constant_model_yields_zero_indices() {
        let s = LhsSampler::classic(6);
        let mut rng = RngState::from_seed([0x42; 32]);
        let m = build_owen_matrix(&s, 64, &mut rng).unwrap();
        let est = estimate_owen(&m, |_| 7.0);
        for &v in &est.first_order {
            assert_eq!(v, 0.0);
        }
    }

    #[test]
    fn linear_single_factor_concentrates_first_order() {
        let s = LhsSampler::classic(9);
        let mut rng = RngState::from_seed([0x42; 32]);
        let m = build_owen_matrix(&s, 4096, &mut rng).unwrap();
        let est = estimate_owen(&m, |x| x[0]);
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

    #[test]
    fn small_factor_index_is_well_bounded() {
        // Y = 2·X[0] + 0.001·X[1]: factor 1 has tiny effect (S_1 ≈ 1e-6).
        // Owen should give near-zero S_1 with low variance even at small N.
        let s = LhsSampler::classic(9);
        let mut rng = RngState::from_seed([0x42; 32]);
        let m = build_owen_matrix(&s, 1024, &mut rng).unwrap();
        let est = estimate_owen(&m, |x| 2.0 * x[0] + 0.001 * x[1]);
        // S_0 dominates.
        assert!(est.first_order[0] > 0.9);
        // S_1 is "Owen-suppressed near zero" (~1e-6 analytic).
        assert!(
            est.first_order[1].abs() < 0.01,
            "S_1 = {} should be near 0 (analytic ~1e-6)",
            est.first_order[1]
        );
    }

    #[test]
    fn same_matrix_yields_identical_estimates() {
        let s = LhsSampler::classic(9);
        let mut rng = RngState::from_seed([0x42; 32]);
        let m = build_owen_matrix(&s, 256, &mut rng).unwrap();
        let model = |x: &[f64]| x[0] + x[1] * x[2];
        let a = estimate_owen(&m, model);
        let b = estimate_owen(&m, model);
        assert_eq!(a.first_order, b.first_order);
    }
}
