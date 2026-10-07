//! First-order spectral estimates from paired input/output data.
//!
//! Related methods are [Tarantola et al. (2006)](https://doi.org/10.1016/j.ress.2005.06.003)
//! and [Plischke's EASI (2010)](https://doi.org/10.1016/j.ress.2009.11.005).
//! This implementation sorts rows by each input and traverses odd ranks
//! ascending, then even ranks descending, as EASI Section 3 specifies.
//! It divides power in frequencies `1..=M` by total spectral power.
//! It applies Plischke's Eq. (7): `(S_raw - 2M/N)/(1 - 2M/N)`.
//!
//! Conjugate frequency pairs are counted twice; even-length Nyquist is counted
//! once. Tied input values are rejected: the rank trace would otherwise impose
//! an arbitrary order on their outputs. Results can be negative after correction.
//! `harmonic` must be supplied explicitly; its value controls truncation.
//! No total-effect indices are returned. Inputs must be independent for
//! the usual Sobol' interpretation; finite, aligned observations are required.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::many_single_char_names
)]

use std::fmt;
use std::sync::Arc;

use crate::fast::power_spectrum_one_sided;
#[cfg(test)]
use ndarray::Array2;
use ndarray::ArrayView2;
use rustfft::{Fft, FftPlanner};
use salib_core::tree_sum;

/// First-order Sobol' index estimates per factor with Plischke 2010
/// bias correction.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct RbdFastIndices {
    /// Approximate first-order indices, length `d`. Can be negative.
    /// See the module documentation for spectral truncation assumptions.
    pub s: Vec<f64>,
}

impl RbdFastIndices {
    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.s.len()
    }
}

impl fmt::Display for RbdFastIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RBD-FAST indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.s[i])?;
        }
        Ok(())
    }
}

/// Errors from [`estimate_rbd_fast`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum RbdFastError {
    /// `X.nrows() != y.len()`.
    #[error("RBD-FAST: shape mismatch — X has {x_rows} rows, y has {y_len} elements")]
    ShapeMismatch {
        /// Number of rows in the input matrix.
        x_rows: usize,
        /// Number of output observations.
        y_len: usize,
    },
    /// `X.ncols() == 0` — no factors.
    #[error("RBD-FAST: d must be ≥ 1, got 0")]
    ZeroD,
    /// `harmonic == 0`.
    #[error("RBD-FAST: harmonic must be ≥ 1, got 0")]
    ZeroHarmonic,
    /// `N < 2·M + 1`. Plischke 2010 correction `λ = 2·M / N`
    /// requires `N > 2·M` for `(1 − λ)` to be positive.
    #[error(
        "RBD-FAST: N must be ≥ 2·harmonic + 1 (got N={n}, harmonic={harmonic}, \
         minimum={minimum}); else Plischke correction denominator collapses"
    )]
    InsufficientSamples {
        /// Sample count supplied by the caller.
        n: usize,
        /// Maximum harmonic supplied by the caller.
        harmonic: u32,
        /// Minimum sample count required by this configuration.
        minimum: usize,
    },
    /// A factor contains equal values and therefore has no unique rank trace.
    #[error("RBD-FAST: tied input values in factor {factor}; rank traces require distinct values")]
    TiedInput {
        /// Zero-based input column containing a tie.
        factor: usize,
    },
    /// An input value is nonfinite.
    #[error("RBD-FAST: nonfinite input at row {row}, factor {factor}")]
    NonFiniteInput {
        /// Input row.
        row: usize,
        /// Input column.
        factor: usize,
    },
    /// An output value is nonfinite.
    #[error("RBD-FAST: nonfinite output at row {row}")]
    NonFiniteOutput {
        /// Output row.
        row: usize,
    },
    /// Spectral variance is below `1e-15` or nonfinite from numerical overflow.
    #[error("RBD-FAST: spectral variance is too small or nonfinite")]
    ZeroVariance,
}

/// Estimate first-order Sobol' indices from generic `(X, Y)` data
/// via RBD-FAST with Plischke 2010 bias correction.
///
/// `x` is the `(N, d)` input matrix (each row a sample, each column
/// a factor). `y` is the corresponding model output vector of
/// length `N`. `harmonic` is the spectral truncation order `M`.
///
/// Observations must represent the distribution of interest. Strictly
/// increasing input transformations preserve the ordering. Ties are rejected
/// because no unique rank trace exists; use a conditioning estimator for
/// discrete inputs instead.
///
/// # Errors
///
/// - [`RbdFastError::ShapeMismatch`] if `x.nrows() != y.len()`.
/// - [`RbdFastError::ZeroD`] if `x.ncols() == 0`.
/// - [`RbdFastError::ZeroHarmonic`] if `harmonic == 0`.
/// - [`RbdFastError::InsufficientSamples`] if `N < 2·harmonic + 1`.
/// - [`RbdFastError::TiedInput`] if an input column contains equal values.
/// - [`RbdFastError::NonFiniteInput`] or [`RbdFastError::NonFiniteOutput`]
///   if an observation is not finite.
/// - [`RbdFastError::ZeroVariance`] if spectral variance is below `1e-15`
///   or nonfinite from numerical overflow.
pub fn estimate_rbd_fast(
    x: ArrayView2<'_, f64>,
    y: &[f64],
    harmonic: u32,
) -> Result<RbdFastIndices, RbdFastError> {
    let n = x.nrows();
    let d = x.ncols();
    if d == 0 {
        return Err(RbdFastError::ZeroD);
    }
    if y.len() != n {
        return Err(RbdFastError::ShapeMismatch {
            x_rows: n,
            y_len: y.len(),
        });
    }
    if harmonic == 0 {
        return Err(RbdFastError::ZeroHarmonic);
    }
    let minimum = 2 * (harmonic as usize) + 1;
    if n < minimum {
        return Err(RbdFastError::InsufficientSamples {
            n,
            harmonic,
            minimum,
        });
    }

    for ((row, factor), value) in x.indexed_iter() {
        if !value.is_finite() {
            return Err(RbdFastError::NonFiniteInput { row, factor });
        }
    }
    for (row, value) in y.iter().enumerate() {
        if !value.is_finite() {
            return Err(RbdFastError::NonFiniteOutput { row });
        }
    }
    let fft = build_fft_planner(n);

    let m_usize = harmonic as usize;
    let lambda = 2.0 * f64::from(harmonic) / (n as f64);
    let one_minus_lambda = 1.0 - lambda;

    let mut s = vec![0.0_f64; d];

    let mut permutation: Vec<usize> = (0..n).collect();
    let mut y_perm: Vec<f64> = vec![0.0; n];

    for i in 0..d {
        // argsort(X[:, i]) with stable order.
        permutation
            .iter_mut()
            .enumerate()
            .for_each(|(k, slot)| *slot = k);
        permutation.sort_by(|&a, &b| x[[a, i]].total_cmp(&x[[b, i]]));
        if permutation
            .windows(2)
            .any(|pair| x[[pair[0], i]] == x[[pair[1], i]])
        {
            return Err(RbdFastError::TiedInput { factor: i });
        }

        // EASI: one-based odd sorted ranks ascend, then even ranks descend.
        // This triangular traversal joins both endpoints of the periodic trace.
        for (k, rank) in (0..n).step_by(2).chain((1..n).step_by(2).rev()).enumerate() {
            y_perm[k] = y[permutation[rank]];
        }

        // One-sided power spectrum.
        let spectrum = power_spectrum_one_sided(&y_perm, fft.as_ref());

        // Each spectrum bin already includes its conjugate multiplicity.
        let v_total = tree_sum(&spectrum);
        if !v_total.is_finite() || v_total < 1e-15 {
            return Err(RbdFastError::ZeroVariance);
        }

        // First-order: sum weighted bins through M. Sp is 0-indexed
        // with Sp[0] corresponding to frequency 1, so the first M
        // bins are Sp[0..M].
        let take = m_usize.min(spectrum.len());
        let v1 = tree_sum(&spectrum[..take]);

        let s_naive = v1 / v_total;
        // Plischke 2010: S = (S_naive − λ) / (1 − λ).
        s[i] = (s_naive - lambda) / one_minus_lambda;
    }

    Ok(RbdFastIndices { s })
}

fn build_fft_planner(n: usize) -> Arc<dyn Fft<f64>> {
    let mut planner: FftPlanner<f64> = FftPlanner::new();
    planner.plan_fft_forward(n)
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {
    use super::*;

    #[test]
    fn single_factor_cosine_recovers_unit_index_for_odd_and_even_lengths() {
        for n in [4096, 4097] {
            let x = Array2::from_shape_fn((n, 1), |(j, _)| (j as f64 + 0.5) / n as f64);
            let y: Vec<_> = (0..n)
                .map(|j| (std::f64::consts::PI * x[[j, 0]]).cos())
                .collect();
            let actual = estimate_rbd_fast(x.view(), &y, 4).unwrap().s[0];
            assert!((actual - 1.0).abs() < 1e-5, "N={n}: S={actual}");
        }
    }

    #[test]
    fn rbd_spectrum_matches_inverse_triangular_parseval_fixture() {
        for n in [63, 64] {
            let x = Array2::from_shape_fn((n, 1), |(j, _)| j as f64 / n as f64);
            let trace: Vec<_> = (0..n)
                .map(|j| {
                    (2.0 * std::f64::consts::PI * j as f64 / n as f64).cos()
                        + if n % 2 == 0 {
                            if j % 2 == 0 {
                                1.0
                            } else {
                                -1.0
                            }
                        } else {
                            0.0
                        }
                })
                .collect();
            let mut y = vec![0.0; n];
            for (j, rank) in (0..n).step_by(2).chain((1..n).step_by(2).rev()).enumerate() {
                y[rank] = trace[j];
            }
            let raw_share = if n % 2 == 0 { 1.0 / 3.0 } else { 1.0 };
            let lambda = 2.0 / n as f64;
            let expected = (raw_share - lambda) / (1.0 - lambda);
            let actual = estimate_rbd_fast(x.view(), &y, 1).unwrap().s[0];
            assert!(
                (actual - expected).abs() < 1e-12,
                "N={n}: {actual} vs {expected}"
            );
        }
    }

    #[test]
    fn ties_are_rejected_independently_of_row_order() {
        let x = Array2::from_shape_fn((16, 1), |(j, _)| (j / 2) as f64);
        let y: Vec<_> = (0..16).map(|j| j as f64).collect();
        assert_eq!(
            estimate_rbd_fast(x.view(), &y, 4).unwrap_err(),
            RbdFastError::TiedInput { factor: 0 }
        );
        let reverse = Array2::from_shape_fn((16, 1), |(j, _)| x[[15 - j, 0]]);
        let reverse_y: Vec<_> = y.iter().copied().rev().collect();
        assert_eq!(
            estimate_rbd_fast(reverse.view(), &reverse_y, 4).unwrap_err(),
            RbdFastError::TiedInput { factor: 0 }
        );
    }

    #[test]
    fn nonfinite_observations_are_rejected() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut x = lhs_x(16, 1);
            let mut y: Vec<_> = (0..16).map(|j| j as f64).collect();
            x[[3, 0]] = value;
            assert_eq!(
                estimate_rbd_fast(x.view(), &y, 4).unwrap_err(),
                RbdFastError::NonFiniteInput { row: 3, factor: 0 }
            );
            x = lhs_x(16, 1);
            y[5] = value;
            assert_eq!(
                estimate_rbd_fast(x.view(), &y, 4).unwrap_err(),
                RbdFastError::NonFiniteOutput { row: 5 }
            );
        }
    }

    fn lhs_x(n: usize, d: usize) -> Array2<f64> {
        // Deterministic per-column independent permutation of the
        // grid `(k + 0.5)/n` — one stratum per cell, factor columns
        // uncorrelated. Permutation seed varies by `j` so columns
        // are independent in the rank-correlation sense.
        let mut x = Array2::<f64>::zeros((n, d));
        for j in 0..d {
            // Generate a permutation deterministically via a small
            // linear-congruential walk.
            let mut perm: Vec<usize> = (0..n).collect();
            // Fisher-Yates with a per-column LCG seed.
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
                let v = (perm[i] as f64 + 0.5) / (n as f64);
                x[[i, j]] = v;
            }
        }
        x
    }

    // ── Validation ────────────────────────────────────────────────

    #[test]
    fn zero_d_errors() {
        let x = Array2::<f64>::zeros((10, 0));
        let y = vec![0.0; 10];
        assert_eq!(
            estimate_rbd_fast(x.view(), &y, 4).unwrap_err(),
            RbdFastError::ZeroD
        );
    }

    #[test]
    fn shape_mismatch_errors() {
        let x = Array2::<f64>::zeros((10, 3));
        let y = vec![0.0; 9];
        let err = estimate_rbd_fast(x.view(), &y, 4).unwrap_err();
        assert!(matches!(err, RbdFastError::ShapeMismatch { .. }));
    }

    #[test]
    fn zero_harmonic_errors() {
        let x = lhs_x(20, 3);
        let y = vec![0.0; 20];
        assert_eq!(
            estimate_rbd_fast(x.view(), &y, 0).unwrap_err(),
            RbdFastError::ZeroHarmonic
        );
    }

    #[test]
    fn insufficient_samples_errors() {
        // N=8, M=4 → need ≥ 9.
        let x = lhs_x(8, 3);
        let y = vec![0.0; 8];
        let err = estimate_rbd_fast(x.view(), &y, 4).unwrap_err();
        assert!(matches!(err, RbdFastError::InsufficientSamples { .. }));
    }

    #[test]
    fn constant_model_errors() {
        let x = lhs_x(64, 3);
        let y = vec![1.0; 64];
        let err = estimate_rbd_fast(x.view(), &y, 4).unwrap_err();
        assert_eq!(err, RbdFastError::ZeroVariance);
    }

    // ── Output shape ──────────────────────────────────────────────

    #[test]
    fn output_length_matches_d() {
        let x = lhs_x(64, 5);
        let y: Vec<f64> = (0..64_u32).map(f64::from).collect();
        let est = estimate_rbd_fast(x.view(), &y, 4).unwrap();
        assert_eq!(est.d(), 5);
        assert_eq!(est.s.len(), 5);
    }

    // ── Linear single-factor: factor 0 dominant ───────────────────

    #[test]
    fn linear_single_factor_concentrates_variance() {
        // Y = X[:, 0] — all variance from factor 0.
        let n = 256;
        let d = 3;
        let x = lhs_x(n, d);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let est = estimate_rbd_fast(x.view(), &y, 10).unwrap();
        assert!(
            est.s[0] > 0.5,
            "S_0 should dominate for Y = X_0, got {}",
            est.s[0]
        );
        assert!(
            est.s[1].abs() < 0.2,
            "S_1 should be small (factor 1 absent), got {}",
            est.s[1]
        );
        assert!(
            est.s[2].abs() < 0.2,
            "S_2 should be small, got {}",
            est.s[2]
        );
    }

    // ── Plischke correction can produce small negatives ───────────

    #[test]
    fn plischke_correction_allows_small_negatives() {
        // For a model where factor 2 has zero true effect, the
        // bias-corrected estimate can be slightly negative due to
        // MC noise around 0. We don't clamp.
        let n = 256;
        let x = lhs_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let est = estimate_rbd_fast(x.view(), &y, 10).unwrap();
        // Just assert finite — negatives are allowed by design.
        for &v in &est.s {
            assert!(v.is_finite(), "estimate non-finite: {v}");
        }
    }

    // ── Determinism ───────────────────────────────────────────────

    #[test]
    fn same_input_yields_identical_output() {
        let n = 64;
        let x = lhs_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + x[[k, 1]] * x[[k, 2]]).collect();
        let a = estimate_rbd_fast(x.view(), &y, 4).unwrap();
        let b = estimate_rbd_fast(x.view(), &y, 4).unwrap();
        assert_eq!(a.s, b.s);
    }

    // ── Permutation invariance ────────────────────────────────────

    #[test]
    fn rbd_fast_invariant_to_input_row_order() {
        // RBD-FAST sorts X internally; permuting rows of (X, Y)
        // together must yield the same estimate.
        let n = 256;
        let x = lhs_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + 2.0 * x[[k, 1]]).collect();
        let est_a = estimate_rbd_fast(x.view(), &y, 10).unwrap();

        // Reverse the input row order (equivalent permutation).
        let mut x_rev = Array2::<f64>::zeros((n, 3));
        let mut y_rev = vec![0.0; n];
        for k in 0..n {
            for j in 0..3 {
                x_rev[[k, j]] = x[[n - 1 - k, j]];
            }
            y_rev[k] = y[n - 1 - k];
        }
        let est_b = estimate_rbd_fast(x_rev.view(), &y_rev, 10).unwrap();

        // Permutation invariance is exact: argsort gives the same
        // post-sort sample order regardless of input row order, so
        // Y_perm and the resulting spectrum are bit-identical.
        for i in 0..3 {
            assert!(
                (est_a.s[i] - est_b.s[i]).abs() < 1e-10,
                "factor {i}: a={} b={}",
                est_a.s[i],
                est_b.s[i]
            );
        }
    }
}
