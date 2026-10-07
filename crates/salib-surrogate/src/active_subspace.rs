//! Active subspaces from sampled gradients.
//!
//! For a differentiable scalar model with square-integrable gradients,
//! define the uncentered gradient second-moment matrix:
//!
//! ```text
//! C = E[(∇f)(∇f)ᵀ] = W Λ Wᵀ,    λ_1 ≥ … ≥ λ_d ≥ 0
//! ```
//!
//! Each eigenvalue is the mean squared directional derivative along
//! its eigenvector ([Constantine, Dow and Wang
//! 2014](https://arxiv.org/pdf/1304.2070), Eq. 2.3 and Lemma 2.1).
//! The leading eigenvectors span the chosen active subspace. A spectral
//! gap helps choose a dimension; it does not establish surrogate accuracy.
//! The conditional-mean approximation bound in their Theorem 3.1 depends
//! on the sum of discarded eigenvalues and a domain/measure constant.
//!
//! # Estimation
//!
//! Given gradient rows sampled under the intended input distribution,
//! the estimator forms `C̃ = gradientsᵀ gradients / M` (Eq. 2.16) and
//! eigendecomposes it. Cost is `O(M d² + d³)`. Choose input units or
//! scaling before computing gradients: changing coordinates changes C.
//!
//! Callers supply analytic or numerical gradients. The result contains
//! eigenvalues, eigenvectors, and a suggested dimension; it does not fit
//! a response surface or quantify gradient and sampling uncertainty.
//!
//! # Dimension heuristic
//!
//! The largest adjacent eigenvalue ratio and the relative numerical-zero
//! tolerance are choices made by this implementation. They are not an
//! error bound. With fewer gradient samples than inputs, sample rank is
//! at most `M`; zero sample eigenvalues need not imply inactive directions
//! in the model.
//!
//! For a nonconstant ridge function `f(x) = h(aᵀx)`, C has rank one
//! when `E[h'(aᵀx)²] > 0`. Its leading eigenvector is `±a/||a||`.
//! A nonzero sampled gradient identifies that direction, but gradients
//! sampled where `h'` vanishes provide no such information.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::many_single_char_names,
    clippy::collapsible_if
)]

use nalgebra::{DMatrix, SymmetricEigen};
use ndarray::{Array2, ArrayView2};

/// Result of an active-subspace computation.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct ActiveSubspace {
    /// Eigenvalues `λ_1 ≥ … ≥ λ_d ≥ 0`, length `d`. Sorted descending.
    pub eigenvalues: Vec<f64>,
    /// Eigenvectors as `(d, d)` matrix; columns are eigenvectors
    /// aligned with `eigenvalues` (column `i` corresponds to `λ_i`).
    /// Each column is unit-norm.
    pub eigenvectors: Array2<f64>,
    /// Active-subspace dimension via the largest-gap heuristic. The
    /// top `k_active` columns of `eigenvectors` span the active
    /// subspace. `1 ≤ k_active ≤ d`.
    pub k_active: usize,
}

impl ActiveSubspace {
    /// Factor count `d`.
    #[must_use]
    pub fn d(&self) -> usize {
        self.eigenvalues.len()
    }
}

/// Errors from [`compute_active_subspace`].
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum ActiveSubspaceError {
    /// The gradient matrix has no observations.
    #[error("active-subspace: gradients must have ≥ 1 sample, got 0 rows")]
    EmptyGradients,
    /// At least one input dimension is required.
    #[error("active-subspace: d must be ≥ 1, got 0 cols")]
    ZeroD,
    /// The eigenvalue gap threshold must be finite and greater than one.
    #[error("active-subspace: gap_threshold must be > 1 if supplied, got {threshold}")]
    InvalidGapThreshold {
        /// Eigenvalue gap threshold supplied.
        threshold: f64,
    },
    /// The gradient covariance produced a nonfinite eigenspectrum.
    #[error("active-subspace: all eigenvalues are non-finite (NaN/Inf in gradients?)")]
    NonFiniteSpectrum,
}

/// Compute the active-subspace eigendecomposition from a sample of
/// gradients.
///
/// `gradients` is shape `(M, d)`: row `j` is the gradient `∇f(x_j)`
/// at sample `x_j`. The MC approximation `C̃ = (1/M) gradientsᵀ ·
/// gradients` is eigendecomposed in descending eigenvalue order.
///
/// `gap_threshold` controls active-subspace dimension detection:
///
/// - `None` — take `k = argmax_j (λ_j / λ_{j+1})`.
/// - `Some(t)` (`t > 1`) — require the largest eigenvalue ratio
///   to be ≥ `t` to qualify as a gap. If no ratio meets the
///   threshold, `k_active = d` (no detected active subspace, all
///   directions retained).
///
/// Perfect-gap short-circuit: if `λ_{j+1} ≤ 1e-12 · λ_max` (a
/// numerically-zero eigenvalue follows a non-zero one), `k_active`
/// is committed at that `j+1` regardless of `gap_threshold`. This
/// avoids dividing by a nearly zero eigenvalue. It identifies numerical
/// rank in the sample matrix, not a proven rank of the population matrix.
///
/// # Errors
///
/// - [`ActiveSubspaceError::EmptyGradients`] if `gradients.nrows() == 0`.
/// - [`ActiveSubspaceError::ZeroD`] if `gradients.ncols() == 0`.
/// - [`ActiveSubspaceError::InvalidGapThreshold`] if `gap_threshold = Some(t)` with `t ≤ 1`.
/// - [`ActiveSubspaceError::NonFiniteSpectrum`] if any computed eigenvalue is NaN/Inf.
pub fn compute_active_subspace(
    gradients: ArrayView2<'_, f64>,
    gap_threshold: Option<f64>,
) -> Result<ActiveSubspace, ActiveSubspaceError> {
    let m = gradients.nrows();
    let d = gradients.ncols();
    if m == 0 {
        return Err(ActiveSubspaceError::EmptyGradients);
    }
    if d == 0 {
        return Err(ActiveSubspaceError::ZeroD);
    }
    if let Some(t) = gap_threshold {
        if !(t.is_finite() && t > 1.0) {
            return Err(ActiveSubspaceError::InvalidGapThreshold { threshold: t });
        }
    }

    // Build C̃ = (1/M) gradientsᵀ · gradients via column dot-products.
    // We iterate over upper triangle and mirror — symmetric PSD.
    let m_inv = 1.0_f64 / m as f64;
    let mut c_tilde = DMatrix::<f64>::zeros(d, d);
    for i in 0..d {
        for j in i..d {
            // Dot product of columns i and j.
            let mut acc = 0.0_f64;
            for row in 0..m {
                acc += gradients[[row, i]] * gradients[[row, j]];
            }
            let v = acc * m_inv;
            c_tilde[(i, j)] = v;
            if i != j {
                c_tilde[(j, i)] = v;
            }
        }
    }

    let eig = SymmetricEigen::new(c_tilde);
    // nalgebra returns eigenvalues in arbitrary order — sort descending.
    let mut idx_perm: Vec<usize> = (0..d).collect();
    idx_perm.sort_by(|&a, &b| {
        eig.eigenvalues[b]
            .partial_cmp(&eig.eigenvalues[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut eigenvalues = vec![0.0_f64; d];
    let mut eigenvectors = Array2::<f64>::zeros((d, d));
    for (out_col, &src_col) in idx_perm.iter().enumerate() {
        eigenvalues[out_col] = eig.eigenvalues[src_col];
        for row in 0..d {
            eigenvectors[[row, out_col]] = eig.eigenvectors[(row, src_col)];
        }
    }

    // Catch any NaN/Inf in the spectrum — a partial-NaN result would
    // pass an `.any(is_finite)` check while still corrupting the
    // gap-detection scan and downstream caller logic. Require every
    // eigenvalue to be finite.
    if !eigenvalues.iter().all(|v| v.is_finite()) {
        return Err(ActiveSubspaceError::NonFiniteSpectrum);
    }

    // Symmetric PSD theory says λᵢ ≥ 0, but f64 roundoff can produce
    // λᵢ ≈ -1e-15 on a true-zero eigenvalue. Clamp at 0 — anything
    // larger surfaces unmodified for the caller to audit.
    let lambda_max = eigenvalues[0].abs().max(1.0);
    for v in &mut eigenvalues {
        if *v < 0.0 && v.abs() < 1e-12 * lambda_max {
            *v = 0.0;
        }
    }

    let k_active = detect_active_dimension(&eigenvalues, gap_threshold);

    Ok(ActiveSubspace {
        eigenvalues,
        eigenvectors,
        k_active,
    })
}

/// Compute the active-subspace dimension from a descending eigenvalue
/// list via the largest-gap heuristic. `λ_max = eigenvalues[0]`;
/// "perfect gap" = `λ_{j+1} ≤ 1e-12 · λ_max` (numerical zero); we
/// commit `k_active = j+1` at the first such j regardless of
/// `gap_threshold`.
fn detect_active_dimension(eigenvalues: &[f64], gap_threshold: Option<f64>) -> usize {
    let d = eigenvalues.len();
    if d == 0 {
        return 0;
    }
    let lambda_max = eigenvalues[0].abs().max(f64::MIN_POSITIVE);
    let zero_threshold = 1e-12 * lambda_max;

    // First scan for a "perfect gap" — λ_{j+1} numerically zero.
    for j in 0..d.saturating_sub(1) {
        if eigenvalues[j].abs() > zero_threshold && eigenvalues[j + 1].abs() <= zero_threshold {
            return j + 1;
        }
    }

    // Otherwise pick argmax of finite ratios.
    let mut best_idx = 0_usize;
    let mut best_ratio = 0.0_f64;
    for j in 0..d.saturating_sub(1) {
        let denom = eigenvalues[j + 1].abs().max(zero_threshold);
        let ratio = eigenvalues[j].abs() / denom;
        if ratio.is_finite() && ratio > best_ratio {
            best_ratio = ratio;
            best_idx = j;
        }
    }

    if let Some(t) = gap_threshold {
        if best_ratio < t {
            return d; // No gap meets threshold — keep all directions.
        }
    }
    best_idx + 1
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {
    use super::*;
    use ndarray::array;

    fn unit_aligned_gradient_samples(direction: &[f64], m: usize) -> Array2<f64> {
        let d = direction.len();
        let mut g = Array2::<f64>::zeros((m, d));
        for j in 0..m {
            for i in 0..d {
                g[[j, i]] = direction[i];
            }
        }
        g
    }

    // ── Validation ────────────────────────────────────────────────

    #[test]
    fn empty_gradients_errors() {
        let g = Array2::<f64>::zeros((0, 3));
        let err = compute_active_subspace(g.view(), None).unwrap_err();
        assert_eq!(err, ActiveSubspaceError::EmptyGradients);
    }

    #[test]
    fn zero_d_errors() {
        let g = Array2::<f64>::zeros((10, 0));
        let err = compute_active_subspace(g.view(), None).unwrap_err();
        assert_eq!(err, ActiveSubspaceError::ZeroD);
    }

    #[test]
    fn invalid_gap_threshold_errors() {
        let g = Array2::<f64>::zeros((10, 3));
        let err = compute_active_subspace(g.view(), Some(0.5)).unwrap_err();
        assert!(matches!(
            err,
            ActiveSubspaceError::InvalidGapThreshold { .. }
        ));
        let err = compute_active_subspace(g.view(), Some(f64::NAN)).unwrap_err();
        assert!(matches!(
            err,
            ActiveSubspaceError::InvalidGapThreshold { .. }
        ));
    }

    // ── Constantine § 2.1 ridge-function special case ────────────

    #[test]
    fn ridge_function_yields_rank_one_c_with_leading_eigenvector_aligned_to_a() {
        // f(x) = aᵀ·x with a = (3, 0, 4) (chosen non-axis-aligned to
        // make alignment a meaningful check). Gradient is constant
        // everywhere = a, so every sample contributes a·aᵀ to C̃.
        // C̃ = a·aᵀ — rank 1. Leading eigenvector should be a/||a||,
        // λ_1 = ||a||² = 25, λ_2 = λ_3 = 0.
        let a = [3.0_f64, 0.0, 4.0];
        let norm_a = (a.iter().map(|v| v * v).sum::<f64>()).sqrt();
        let g = unit_aligned_gradient_samples(&a, 50);
        let result = compute_active_subspace(g.view(), None).unwrap();
        // λ_1 ≈ ||a||² = 25.
        assert!(
            (result.eigenvalues[0] - 25.0).abs() < 1e-9,
            "λ_1 = {}, expected 25",
            result.eigenvalues[0]
        );
        // λ_2, λ_3 ≈ 0.
        assert!(result.eigenvalues[1].abs() < 1e-9);
        assert!(result.eigenvalues[2].abs() < 1e-9);
        // Leading eigenvector = ±a/||a||. Sign is arbitrary; check
        // up to sign.
        let v: Vec<f64> = (0..3).map(|i| result.eigenvectors[[i, 0]]).collect();
        let dot: f64 = v.iter().zip(a.iter()).map(|(&w, &ai)| w * ai).sum();
        let alignment = dot.abs() / norm_a;
        assert!(
            (alignment - 1.0).abs() < 1e-9,
            "alignment = {alignment}, expected ±1"
        );
        // k_active = 1 (perfect gap at j = 0).
        assert_eq!(result.k_active, 1);
    }

    #[test]
    fn ridge_function_k_active_is_one_under_largest_gap_heuristic() {
        let a = [1.0_f64, 1.0, 1.0, 1.0]; // d = 4
        let g = unit_aligned_gradient_samples(&a, 100);
        let result = compute_active_subspace(g.view(), None).unwrap();
        assert_eq!(result.k_active, 1);
    }

    // ── Two-direction ridge ───────────────────────────────────────

    #[test]
    fn two_direction_ridge_recovers_two_active_dimensions() {
        // f(x) = (aᵀx)·sample_var_1 + (bᵀx)·sample_var_2 — the
        // gradient is a (when sample_var_1 = 1, sample_var_2 = 0)
        // or b (vice versa). With orthogonal a, b and balanced
        // samples we get C̃ = ½(a aᵀ + b bᵀ) — rank 2 with
        // λ_1, λ_2 > 0 and λ_3 = ... = 0.
        let a = [2.0_f64, 0.0, 0.0, 0.0];
        let b = [0.0_f64, 0.0, 3.0, 0.0];
        let m = 100;
        let mut g = Array2::<f64>::zeros((m, 4));
        for j in 0..m {
            let row = if j % 2 == 0 { &a } else { &b };
            for i in 0..4 {
                g[[j, i]] = row[i];
            }
        }
        let result = compute_active_subspace(g.view(), None).unwrap();
        // λ_1 = max(||a||², ||b||²)/2 = 9/2 = 4.5; λ_2 = 4/2 = 2.0.
        assert!((result.eigenvalues[0] - 4.5).abs() < 1e-9);
        assert!((result.eigenvalues[1] - 2.0).abs() < 1e-9);
        assert!(result.eigenvalues[2].abs() < 1e-9);
        assert!(result.eigenvalues[3].abs() < 1e-9);
        // Perfect gap detector: λ_3 numerically zero → k_active = 2.
        assert_eq!(result.k_active, 2);
    }

    // ── Eigenvalue ordering / shape contract ─────────────────────

    #[test]
    fn eigenvalues_are_descending() {
        // Random-ish gradient sample.
        let g = array![
            [1.0, 0.5, 0.2],
            [0.8, 0.3, -0.1],
            [-0.6, 0.4, 0.2],
            [0.7, -0.2, 0.5],
            [0.1, 0.4, -0.3],
        ];
        let result = compute_active_subspace(g.view(), None).unwrap();
        for j in 0..result.eigenvalues.len().saturating_sub(1) {
            assert!(
                result.eigenvalues[j] >= result.eigenvalues[j + 1] - 1e-12,
                "eigenvalues not descending at j={j}: {} < {}",
                result.eigenvalues[j],
                result.eigenvalues[j + 1]
            );
        }
        assert_eq!(result.eigenvectors.shape(), &[3, 3]);
    }

    #[test]
    fn eigenvectors_are_unit_norm_and_orthogonal() {
        let g = array![
            [1.0, 0.5, 0.2],
            [0.8, 0.3, -0.1],
            [-0.6, 0.4, 0.2],
            [0.7, -0.2, 0.5],
            [0.1, 0.4, -0.3],
        ];
        let result = compute_active_subspace(g.view(), None).unwrap();
        let d = result.eigenvalues.len();
        for col in 0..d {
            let v: Vec<f64> = (0..d).map(|r| result.eigenvectors[[r, col]]).collect();
            let norm_sq: f64 = v.iter().map(|&x| x * x).sum();
            assert!(
                (norm_sq - 1.0).abs() < 1e-9,
                "eigenvector col {col} has norm² = {norm_sq}"
            );
        }
        // Orthogonality.
        for i in 0..d {
            for j in (i + 1)..d {
                let dot: f64 = (0..d)
                    .map(|r| result.eigenvectors[[r, i]] * result.eigenvectors[[r, j]])
                    .sum();
                assert!(
                    dot.abs() < 1e-9,
                    "eigenvectors {i} ⊥ {j} violated: dot = {dot}"
                );
            }
        }
    }

    // ── Gap-threshold handling ───────────────────────────────────

    #[test]
    fn gap_threshold_below_max_ratio_returns_largest_gap() {
        let a = [1.0_f64, 1.0, 1.0];
        let g = unit_aligned_gradient_samples(&a, 50);
        // Ridge → λ_1 = 3, λ_2 = λ_3 = 0 (perfect gap at j=0).
        // gap_threshold = 100 still resolves to k=1 because the
        // perfect-gap path fires before the threshold check.
        let result = compute_active_subspace(g.view(), Some(100.0)).unwrap();
        assert_eq!(result.k_active, 1);
    }

    #[test]
    fn gap_threshold_high_keeps_all_directions_when_no_clear_gap() {
        // Three orthogonal unit-magnitude gradient samples ⇒
        // C̃ = (1/3) I_3, all three eigenvalues equal. No gap at all
        // — argmax ratio is 1.0, threshold of 1000 fails to qualify
        // any gap, all directions retained.
        let g = array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],];
        let result = compute_active_subspace(g.view(), Some(1000.0)).unwrap();
        assert_eq!(result.k_active, 3);
    }

    // ── d == 1 boundary ──────────────────────────────────────────

    #[test]
    fn d_one_returns_single_eigenvalue() {
        let g = array![[2.0_f64], [3.0], [4.0]];
        let result = compute_active_subspace(g.view(), None).unwrap();
        assert_eq!(result.eigenvalues.len(), 1);
        // λ_1 = mean(g²) = (4 + 9 + 16)/3 = 29/3.
        assert!((result.eigenvalues[0] - 29.0 / 3.0).abs() < 1e-9);
        assert_eq!(result.k_active, 1);
        // Eigenvector is ±1.
        assert!((result.eigenvectors[[0, 0]].abs() - 1.0).abs() < 1e-12);
    }
}
