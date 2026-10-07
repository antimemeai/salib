//! Polynomial chaos expansion (PCE) fitted by ordinary least squares.
//!
//! For independent inputs, choose a polynomial family orthogonal under
//! each input's probability measure and form the tensor-product basis:
//!
//! ```text
//! Y ≈ Σ_α β_α Ψ_α(ξ),    |α| ≤ p
//! Ψ_α(ξ) = ∏ᵢ Ψ_{αᵢ}(ξᵢ)
//! ```
//!
//! The coefficients solve an OLS problem on the `(N, P)` basis matrix.
//! Sobol' indices of the fitted surrogate follow from coefficient sums
//! ([Sudret 2008](https://doi.org/10.1016/j.ress.2007.04.002), §5.4, Eqs. 51 and 53):
//!
//! ```text
//! D_PCE = Σ_{α ≠ 0} β_α² ⟨Ψ_α, Ψ_α⟩
//! S_i   = Σ_{α : αᵢ>0, αⱼ=0 ∀j≠i} β_α² ⟨Ψ_α, Ψ_α⟩ / D_PCE
//! S_Ti  = Σ_{α : αᵢ>0} β_α² ⟨Ψ_α, Ψ_α⟩ / D_PCE
//! ```
//!
//! These formulas are exact for the fitted polynomial, apart from
//! numerical error. Its indices can differ from the original model's
//! because of basis truncation and coefficient estimation error.
//!
//! # Input convention
//!
//! `samples_canonical` has shape `(N, d)`. The caller maps each input
//! to its family's reference distribution, not just its domain:
//!
//! - Legendre: uniform on `[-1, 1]`.
//! - Hermite: standard Normal `N(0, 1)`.
//! - Laguerre: Exponential with rate 1.
//! - Jacobi: density proportional to `(1-ξ)^alpha (1+ξ)^beta`
//!   on `[-1, 1]`.
//!
//! See [`PolynomialFamily`] for the parameter conventions. Domain
//! validation cannot check independence or a match to the input measure.
//!
//! # Cost
//!
//! There are `P = (d+p)! / (d! p!)` basis terms. Building the basis
//! matrix makes `O(N P d)` polynomial evaluations, each costing at most
//! `O(p)`. The normal-equation solve costs `O(N P² + P³)`; coefficient
//! grouping for Sobol' indices costs `O(P d)` at fixed degree.
//!
//! A sparse fit can help when only a small subset of a large candidate
//! basis is needed. Both paths still construct the candidate basis matrix.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::many_single_char_names
)]

use std::fmt;

use nalgebra::{DMatrix, DVector};
#[cfg(test)]
use ndarray::Array2;
use ndarray::ArrayView2;
use salib_core::tree_sum;

use crate::multi_index::{enumerate_total_degree, total_degree_basis_size, MultiIndex};
use crate::polynomial::{evaluate, is_in_canonical_domain, norm_squared, PolynomialFamily};

/// A fitted Polynomial Chaos Expansion.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct PolynomialChaos {
    /// Coefficients `{β_α}`, length `P`. Aligned with `multi_indices`.
    pub coefficients: Vec<f64>,
    /// Multi-indices `{α}`, length `P`. Aligned with `coefficients`.
    pub multi_indices: Vec<MultiIndex>,
    /// Per-factor polynomial family. Length `d`.
    pub families: Vec<PolynomialFamily>,
    /// Truncation order `p`. Echo of input.
    pub max_degree: usize,
}

impl PolynomialChaos {
    /// Factor count `d`.
    #[must_use]
    pub fn d(&self) -> usize {
        self.families.len()
    }

    /// Basis size `P`.
    #[must_use]
    pub fn basis_size(&self) -> usize {
        self.coefficients.len()
    }

    /// Mean of the fitted polynomial under its reference input measure:
    /// the constant coefficient `β_0`. This estimates the model's mean.
    #[must_use]
    pub fn mean(&self) -> f64 {
        // The first multi-index in lex order is α = (0, ..., 0).
        debug_assert!(!self.multi_indices.is_empty() && self.multi_indices[0].is_zero());
        self.coefficients[0]
    }

    /// Evaluate the fitted PCE at a single canonical-domain point
    /// `ξ`. `ŷ = Σ_α β_α · Ψ_α(ξ)`.
    ///
    /// Each component `ξ[k]` must lie in the canonical domain of
    /// `families[k]` (Legendre / Jacobi: `[-1, 1]`; Hermite: `ℝ`;
    /// Laguerre: `[0, ∞)`). Debug builds assert this condition.
    /// Release builds permit extrapolation, whose accuracy is not
    /// established by the fit.
    #[must_use]
    pub fn evaluate(&self, xi: &[f64]) -> f64 {
        debug_assert_eq!(xi.len(), self.d());
        #[cfg(debug_assertions)]
        {
            for (k, family) in self.families.iter().enumerate() {
                debug_assert!(
                    is_in_canonical_domain(*family, xi[k]),
                    "PolynomialChaos::evaluate: ξ[{k}] = {} outside {:?}'s canonical domain",
                    xi[k],
                    family
                );
            }
        }
        let mut acc = 0.0;
        for (alpha, &beta) in self.multi_indices.iter().zip(self.coefficients.iter()) {
            let mut psi = 1.0;
            for (k, &deg) in alpha.indices.iter().enumerate() {
                psi *= evaluate(self.families[k], deg, xi[k]);
            }
            acc += beta * psi;
        }
        acc
    }
}

/// Sobol' indices derived analytically from PCE coefficients
/// (Sudret 2008, §5.4, Eqs. 51 and 53).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct SobolFromPce {
    /// First-order Sobol' indices, length `d`. `S_i ∈ [0, 1]` by
    /// construction (sum of squared coefficients ≥ 0; numerator
    /// ≤ denominator).
    pub first_order: Vec<f64>,
    /// Total-order Sobol' indices, length `d`. `S_{T_i} ∈ [0, 1]`.
    pub total_order: Vec<f64>,
    /// Total variance `D_PCE` from the expansion.
    pub total_variance: f64,
}

impl SobolFromPce {
    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.first_order.len()
    }
}

impl fmt::Display for SobolFromPce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Sobol' indices from PCE (d={})", self.d())?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S1", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..self.d() {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}",
                i, self.first_order[i], self.total_order[i]
            )?;
        }
        Ok(())
    }
}

/// Errors from PCE fitting and coefficient analysis.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum PceError {
    /// Input and output shapes are incompatible.
    #[error("PCE: shape mismatch — samples has {x_rows} rows, y has {y_len} elements")]
    ShapeMismatch {
        /// Number of rows in the input matrix.
        x_rows: usize,
        /// Number of output observations.
        y_len: usize,
    },
    /// At least one input dimension is required.
    #[error("PCE: d must be ≥ 1, got 0")]
    ZeroD,
    /// Each input dimension must have one polynomial family.
    #[error("PCE: families.len() ({families_len}) must equal samples.ncols() ({d})")]
    FamiliesDimMismatch {
        /// Number of polynomial families supplied.
        families_len: usize,
        /// Number of input factors.
        d: usize,
    },
    /// The sample count is too small for the requested analysis.
    #[error(
        "PCE: insufficient samples — need N ≥ P (got N={n}, P={basis_size}); \
         a healthy fit needs N ≈ 2·P"
    )]
    InsufficientSamples {
        /// Sample count supplied by the caller.
        n: usize,
        /// Number of candidate polynomial terms.
        basis_size: usize,
    },
    /// A supplied sample or response is NaN or infinite.
    #[error("PCE: samples and responses must contain only finite values")]
    NonFiniteInput,
    /// Basis evaluation, fitting, or variance arithmetic exceeded finite range.
    #[error("PCE: basis, fit, or variance arithmetic produced a nonfinite value")]
    NonFiniteFit,
    /// The polynomial design matrix could not be factorized.
    #[error("PCE: design matrix XᵀX is singular (Cholesky failed)")]
    SingularDesignMatrix,
    /// Every sparse fit candidate has an undefined or nonfinite PRESS score.
    #[error("PCE: no candidate has a finite leave-one-out error")]
    UndefinedLooError,
    /// Output variance is zero or below the estimator threshold.
    #[error("PCE: Var(Y) is zero (model output is constant)")]
    ZeroVariance,
}

/// Fit a full (non-sparse) Polynomial Chaos Expansion of total
/// degree `max_degree` over the given polynomial families.
///
/// `samples_canonical` is the `(N, d)` input matrix in each
/// factor's polynomial-canonical domain (see module doc). `y` is
/// the corresponding `N`-element model output. `families` length
/// must equal `d`.
///
/// # Errors
///
/// - [`PceError::ShapeMismatch`] / [`PceError::ZeroD`] /
///   [`PceError::FamiliesDimMismatch`] — input shape errors.
/// - [`PceError::InsufficientSamples`] if `N < P`.
/// - [`PceError::NonFiniteInput`] if a sample or response is not finite.
/// - [`PceError::NonFiniteFit`] if basis evaluation or fitting overflows.
/// - [`PceError::SingularDesignMatrix`] if Cholesky on `Ψᵀ Ψ`
///   fails (typically near-collinearity in the basis).
///
/// Canonical coordinates must match each family’s weight: `[-1,1]` for Legendre,
/// standard-normal coordinates for Hermite. Validate predictions on held-out data
/// before treating coefficient-derived indices as simulator sensitivities.
///
/// # Examples
///
/// ```rust
/// use ndarray::array;
/// use salib_surrogate::{fit_full_pce, PolynomialFamily};
/// let x = array![[-1.0], [0.0], [1.0]];
/// let pce = fit_full_pce(x.view(), &[-1.0, 1.0, 3.0],
///     &[PolynomialFamily::Legendre], 1).unwrap();
/// assert!((pce.evaluate(&[0.25]) - 1.5).abs() < 1e-12);
/// ```
pub fn fit_full_pce(
    samples_canonical: ArrayView2<'_, f64>,
    y: &[f64],
    families: &[PolynomialFamily],
    max_degree: usize,
) -> Result<PolynomialChaos, PceError> {
    let n = samples_canonical.nrows();
    let d = samples_canonical.ncols();
    if d == 0 {
        return Err(PceError::ZeroD);
    }
    if y.len() != n {
        return Err(PceError::ShapeMismatch {
            x_rows: n,
            y_len: y.len(),
        });
    }
    if families.len() != d {
        return Err(PceError::FamiliesDimMismatch {
            families_len: families.len(),
            d,
        });
    }
    validate_finite_data(samples_canonical, y)?;
    let basis_size = total_degree_basis_size(d, max_degree);
    if n < basis_size {
        return Err(PceError::InsufficientSamples { n, basis_size });
    }

    // Enumerate the basis. multi_indices[0] is the zero multi-index.
    let multi_indices = enumerate_total_degree(d, max_degree).map_err(|_| PceError::ZeroD)?;

    // Caller-side mistake: feeding values outside each family's
    // polynomial-canonical domain (e.g. ξ=5 to a Legendre basis) is
    // silently accepted by the polynomial recurrences but produces
    // garbage Sobol' indices. Trip a debug-only assert as the lone
    // signal a caller will get; release builds skip the check on
    // the assumption that production callers route through
    // `Distribution::quantile`.
    #[cfg(debug_assertions)]
    {
        for (k, family) in families.iter().enumerate() {
            let xs = samples_canonical.column(k);
            let in_domain = xs.iter().all(|&x| is_in_canonical_domain(*family, x));
            debug_assert!(
                in_domain,
                "PCE: column {k} contains values outside {family:?}'s canonical domain"
            );
        }
    }

    // Build basis matrix Ψ ∈ R^{N × P}.
    let mut psi = DMatrix::<f64>::zeros(n, basis_size);
    for i in 0..n {
        for (j, alpha) in multi_indices.iter().enumerate() {
            let mut value = 1.0;
            for (k, &deg) in alpha.indices.iter().enumerate() {
                value *= evaluate(families[k], deg, samples_canonical[[i, k]]);
            }
            psi[(i, j)] = value;
        }
    }

    if psi.iter().any(|value| !value.is_finite()) {
        return Err(PceError::NonFiniteFit);
    }

    // Solve OLS via Cholesky on the normal equations.
    let psi_t = psi.transpose();
    let xtx = &psi_t * &psi;
    let y_vec = DVector::from_iterator(n, y.iter().copied());
    let xty = &psi_t * &y_vec;
    if xtx.iter().chain(xty.iter()).any(|value| !value.is_finite()) {
        return Err(PceError::NonFiniteFit);
    }
    let beta = xtx
        .cholesky()
        .ok_or(PceError::SingularDesignMatrix)?
        .solve(&xty);

    if beta.iter().any(|value| !value.is_finite()) {
        return Err(PceError::NonFiniteFit);
    }
    let coefficients = beta.iter().copied().collect();

    Ok(PolynomialChaos {
        coefficients,
        multi_indices,
        families: families.to_vec(),
        max_degree,
    })
}

/// Validate observations before basis evaluation or canonical-domain assertions.
pub(crate) fn validate_finite_data(
    samples: ArrayView2<'_, f64>,
    y: &[f64],
) -> Result<(), PceError> {
    if samples
        .iter()
        .chain(y.iter())
        .any(|value| !value.is_finite())
    {
        return Err(PceError::NonFiniteInput);
    }
    Ok(())
}

/// Compute Sobol' indices analytically from a fitted PCE per
/// Sudret 2008, §5.4, Eqs. 51 and 53.
///
/// # Errors
///
/// - [`PceError::ZeroVariance`] if the PCE's total variance is
///   numerically zero (e.g., a constant-fit edge case).
pub fn sobol_indices_from_pce(pce: &PolynomialChaos) -> Result<SobolFromPce, PceError> {
    let d = pce.d();

    // Per-multi-index variance contribution: β_α² · ⟨Ψ_α, Ψ_α⟩
    // where ⟨Ψ_α, Ψ_α⟩ = ∏_k norm_squared(family_k, α_k).
    let contributions: Vec<f64> = pce
        .multi_indices
        .iter()
        .zip(pce.coefficients.iter())
        .map(|(alpha, &beta)| {
            let mut norm_sq = 1.0;
            for (k, &deg) in alpha.indices.iter().enumerate() {
                norm_sq *= norm_squared(pce.families[k], deg);
            }
            beta * beta * norm_sq
        })
        .collect();

    // Total variance: sum over all NON-zero multi-indices.
    let nonzero_contribs: Vec<f64> = pce
        .multi_indices
        .iter()
        .zip(contributions.iter())
        .filter(|(alpha, _)| !alpha.is_zero())
        .map(|(_, &c)| c)
        .collect();
    let total_variance = tree_sum(&nonzero_contribs);
    // The `1e-15` is an absolute cutoff: this assumes the caller has
    // *not* rescaled `Y` by something exotic (typical PCE workloads
    // are `O(1)` outputs). For physically-tiny `Var(Y)` (say
    // `< 1e-20`) a relative cutoff against `(Σ β²).max(1.0)` would
    // be more honest; a potential future optimization.
    // The `is_finite()` guard catches the path where Cholesky
    // reported success but the solve produced NaN/Inf coefficients
    // (subnormal-pivot edge case in `nalgebra`); without it, NaN
    // contributions would silently zero out the indices via clamp.
    if !total_variance.is_finite() || total_variance < 1e-15 {
        return Err(PceError::ZeroVariance);
    }

    let mut first_order = vec![0.0_f64; d];
    let mut total_order = vec![0.0_f64; d];

    for (alpha, &c) in pce.multi_indices.iter().zip(contributions.iter()) {
        if alpha.is_zero() {
            continue;
        }
        let active = alpha.active_factors();
        // First-order: only factor i is active (active = [i]).
        if active.len() == 1 {
            first_order[active[0]] += c;
        }
        // Total-order: factor i is in the active set, regardless
        // of others.
        for &i in &active {
            total_order[i] += c;
        }
    }

    for i in 0..d {
        first_order[i] = (first_order[i] / total_variance).clamp(0.0, 1.0);
        total_order[i] = (total_order[i] / total_variance).clamp(0.0, 1.0);
    }

    Ok(SobolFromPce {
        first_order,
        total_order,
        total_variance,
    })
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {
    use super::*;

    fn linspace_unit_to_canonical(n: usize, d: usize) -> Array2<f64> {
        // Per-column independent permutations of (k+0.5)/n,
        // mapped to [-1, 1] (Legendre canonical).
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
                #[allow(clippy::cast_precision_loss)]
                let unit = (perm[i] as f64 + 0.5) / (n as f64);
                x[[i, j]] = 2.0 * unit - 1.0;
            }
        }
        x
    }

    // ── Validation ────────────────────────────────────────────────

    #[test]
    fn full_pce_rejects_nonfinite_observations_and_ols_overflow() {
        let mut x = Array2::from_shape_fn((16, 1), |(i, _)| i as f64);
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(
                fit_full_pce(x.view(), &[invalid; 16], &[PolynomialFamily::Hermite], 1).is_err()
            );
            x[[0, 0]] = invalid;
            assert!(fit_full_pce(x.view(), &[1.0; 16], &[PolynomialFamily::Hermite], 1).is_err());
        }
        x[[0, 0]] = 0.0;
        // The exact constant coefficient is 1e308, but naive X^T y overflows.
        // Until a scaled solve is used, this must fail rather than return Inf/NaN.
        assert!(fit_full_pce(x.view(), &[1e308; 16], &[PolynomialFamily::Hermite], 0).is_err());
    }

    #[test]
    fn zero_d_errors() {
        let x = Array2::<f64>::zeros((10, 0));
        let y = vec![0.0; 10];
        let err = fit_full_pce(x.view(), &y, &[], 3).unwrap_err();
        assert_eq!(err, PceError::ZeroD);
    }

    #[test]
    fn shape_mismatch_errors() {
        let x = Array2::<f64>::zeros((10, 3));
        let y = vec![0.0; 5];
        let err = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 3], 3).unwrap_err();
        assert!(matches!(err, PceError::ShapeMismatch { .. }));
    }

    #[test]
    fn families_dim_mismatch_errors() {
        let x = Array2::<f64>::zeros((20, 3));
        let y = vec![0.0; 20];
        let err = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 2).unwrap_err();
        assert!(matches!(err, PceError::FamiliesDimMismatch { .. }));
    }

    #[test]
    fn insufficient_samples_errors() {
        // d=3, p=4 → P=35. N=20 is below.
        let x = Array2::<f64>::zeros((20, 3));
        let y = vec![0.0; 20];
        let err = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 3], 4).unwrap_err();
        assert!(matches!(err, PceError::InsufficientSamples { .. }));
    }

    #[test]
    fn singular_design_matrix_errors_on_collinear_inputs() {
        // Two factors with identical sample columns → ξ_0 = ξ_1 → the
        // basis has perfectly-collinear rows for any α with the same
        // sum-of-degrees; XᵀX is rank-deficient; Cholesky fails.
        // d=2, p=3 → P=10; need N ≥ 10.
        let n = 16;
        let mut x = Array2::<f64>::zeros((n, 2));
        for i in 0..n {
            #[allow(clippy::cast_precision_loss)]
            let v = -1.0 + 2.0 * (i as f64 + 0.5) / (n as f64);
            x[[i, 0]] = v;
            x[[i, 1]] = v; // identical second column
        }
        let y: Vec<f64> = (0..n).map(|i| x[[i, 0]] + x[[i, 1]]).collect();
        let err = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 3)
            .expect_err("expected SingularDesignMatrix");
        assert_eq!(err, PceError::SingularDesignMatrix);
    }

    // ── Recovery on closed-form polynomial models ────────────────

    #[test]
    fn fits_constant_perfectly() {
        // Y = 7. PCE β_0 = 7, all others = 0.
        let n = 64;
        let x = linspace_unit_to_canonical(n, 2);
        let y = vec![7.0; n];
        let pce = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 3).unwrap();
        assert!(
            (pce.mean() - 7.0).abs() < 1e-10,
            "β_0 = {}, expected 7",
            pce.mean()
        );
        // All non-constant coefficients should be ~0.
        for (alpha, &beta) in pce.multi_indices.iter().zip(pce.coefficients.iter()) {
            if !alpha.is_zero() {
                assert!(
                    beta.abs() < 1e-9,
                    "α={:?}: β={beta}, expected ~0",
                    alpha.indices
                );
            }
        }
    }

    #[test]
    fn fits_linear_function_in_one_factor() {
        // Y = 3·ξ_0 (Legendre P_1(ξ) = ξ → β_{(1, 0)} = 3, others = 0).
        let n = 128;
        let x = linspace_unit_to_canonical(n, 2);
        let y: Vec<f64> = (0..n).map(|i| 3.0 * x[[i, 0]]).collect();
        let pce = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 3).unwrap();
        // Find β at α = (1, 0).
        let target_idx = pce
            .multi_indices
            .iter()
            .position(|a| a.indices == vec![1, 0])
            .unwrap();
        assert!(
            (pce.coefficients[target_idx] - 3.0).abs() < 1e-9,
            "β_{{(1,0)}} = {}, expected 3",
            pce.coefficients[target_idx]
        );
    }

    #[test]
    fn fits_quadratic_via_legendre() {
        // Y = ξ_0² has PCE expansion ξ² = (1/3)·1 + (2/3)·P_2(ξ)
        // since P_2(x) = (3x² − 1)/2, so x² = (2 P_2 + 1)/3.
        // → β_{(0,0)} = 1/3, β_{(2,0)} = 2/3.
        let n = 256;
        let x = linspace_unit_to_canonical(n, 2);
        let y: Vec<f64> = (0..n).map(|i| x[[i, 0]] * x[[i, 0]]).collect();
        let pce = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 3).unwrap();
        let beta_00 = pce.coefficients[pce
            .multi_indices
            .iter()
            .position(|a| a.indices == vec![0, 0])
            .unwrap()];
        let beta_20 = pce.coefficients[pce
            .multi_indices
            .iter()
            .position(|a| a.indices == vec![2, 0])
            .unwrap()];
        assert!(
            (beta_00 - 1.0 / 3.0).abs() < 1e-6,
            "β_{{(0,0)}} = {beta_00}, expected 1/3"
        );
        assert!(
            (beta_20 - 2.0 / 3.0).abs() < 1e-6,
            "β_{{(2,0)}} = {beta_20}, expected 2/3"
        );
    }

    // ── Sobol' indices from coefficients ─────────────────────────

    #[test]
    fn sobol_recovers_expected_split_for_additive_model() {
        // Y = ξ_0 + 2·ξ_1: PCE perfect at p≥1. Var(ξ) = 1/3 for
        // each factor (Legendre canonical [-1, 1] uniform).
        // Var(Y) = Var(ξ_0) + 4·Var(ξ_1) = 1/3 + 4/3 = 5/3.
        // S_0 = (1/3)/(5/3) = 0.2, S_1 = (4/3)/(5/3) = 0.8.
        let n = 256;
        let x = linspace_unit_to_canonical(n, 2);
        let y: Vec<f64> = (0..n).map(|i| x[[i, 0]] + 2.0 * x[[i, 1]]).collect();
        let pce = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 3).unwrap();
        let sobol = sobol_indices_from_pce(&pce).unwrap();
        assert!(
            (sobol.first_order[0] - 0.2).abs() < 1e-6,
            "S_0 = {}, expected 0.2",
            sobol.first_order[0]
        );
        assert!(
            (sobol.first_order[1] - 0.8).abs() < 1e-6,
            "S_1 = {}, expected 0.8",
            sobol.first_order[1]
        );
        // No interactions in additive model → S_T_i = S_i.
        assert!(
            (sobol.total_order[0] - sobol.first_order[0]).abs() < 1e-6,
            "S_T_0 should equal S_0 (additive)"
        );
        assert!((sobol.total_order[1] - sobol.first_order[1]).abs() < 1e-6);
    }

    #[test]
    fn sobol_total_at_least_first_order_with_interaction() {
        // Y = ξ_0 + ξ_1 + 0.5·ξ_0·ξ_1 — has factor 0 ↔ 1
        // interaction. S_T_i > S_i for both factors.
        let n = 512;
        let x = linspace_unit_to_canonical(n, 2);
        let y: Vec<f64> = (0..n)
            .map(|i| x[[i, 0]] + x[[i, 1]] + 0.5 * x[[i, 0]] * x[[i, 1]])
            .collect();
        let pce = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 3).unwrap();
        let sobol = sobol_indices_from_pce(&pce).unwrap();
        for i in 0..2 {
            assert!(
                sobol.total_order[i] > sobol.first_order[i] - 1e-9,
                "S_T_{i} = {} should ≥ S_{i} = {}",
                sobol.total_order[i],
                sobol.first_order[i]
            );
        }
    }

    #[test]
    fn sobol_indices_in_unit_interval() {
        let n = 256;
        let x = linspace_unit_to_canonical(n, 3);
        let y: Vec<f64> = (0..n).map(|i| x[[i, 0]] + x[[i, 1]] * x[[i, 2]]).collect();
        let pce = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 3], 3).unwrap();
        let sobol = sobol_indices_from_pce(&pce).unwrap();
        for i in 0..3 {
            assert!((0.0..=1.0).contains(&sobol.first_order[i]));
            assert!((0.0..=1.0).contains(&sobol.total_order[i]));
        }
    }

    // ── Determinism ───────────────────────────────────────────────

    #[test]
    fn same_input_yields_identical_pce() {
        let n = 64;
        let x = linspace_unit_to_canonical(n, 2);
        let y: Vec<f64> = (0..n).map(|i| x[[i, 0]] + x[[i, 1]]).collect();
        let a = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 3).unwrap();
        let b = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 3).unwrap();
        assert_eq!(a.coefficients, b.coefficients);
    }

    // ── PolynomialChaos::evaluate sanity ─────────────────────────

    #[test]
    fn pce_evaluate_returns_fitted_y_at_training_points() {
        let n = 128;
        let x = linspace_unit_to_canonical(n, 2);
        let y: Vec<f64> = (0..n).map(|i| x[[i, 0]] + 2.0 * x[[i, 1]]).collect();
        let pce = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre; 2], 3).unwrap();
        // Evaluate at training points should recover y to fit
        // tolerance (additive linear model is in span of degree-3
        // Legendre basis).
        for i in 0..10 {
            let xi = [x[[i, 0]], x[[i, 1]]];
            let y_hat = pce.evaluate(&xi);
            assert!(
                (y_hat - y[i]).abs() < 1e-6,
                "i={i}: PCE({:?})={y_hat}, y={}",
                xi,
                y[i]
            );
        }
    }
}
