//! First-order and total-effect estimates from FAST search curves.
//!
//! [Saltelli, Tarantola and Chan (1999)](https://doi.org/10.1080/00401706.1999.10485594),
//! Sections 3–4, estimate total effects from the variance of the complementary
//! inputs. This implementation analyzes each factor's block separately:
//!
//! ```text
//! w[k] = 1 at even-N Nyquist, 2 otherwise
//! P[k] = w[k] * |DFT(y)[k]|^2 / N^2
//! V    = sum_{k=1..floor(N/2)} P[k]
//! V1   = sum_{p=1..M} P[p * omega_i]
//! Vc   = sum_{k=1..floor(omega_i/2)} P[k]
//! S_i  = V1/V;  ST_i = 1-Vc/V.
//! ```
//!
//! The weighted one-sided spectrum gives the population-divisor sample
//! variance for both odd and even `N`.
//! Harmonic truncation and frequency interference affect accuracy; increasing
//! the number of samples alone does not control every source of error.
//! Both reported indices are clamped to `[0,1]`.
//!
//! Costs `N*d` model evaluations and `d` Fourier transforms of length `N`.
//! Model inputs are passed through exactly as stored in `FastDesign`.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::many_single_char_names,
    clippy::needless_range_loop
)]

use std::fmt;
use std::sync::Arc;

use rustfft::{num_complex::Complex, Fft, FftPlanner};
use salib_core::tree_sum;
use salib_samplers::{FastDesign, HarmonicBudget};

/// First-order and total-order Sobol' index estimates per factor.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct FastIndices {
    /// First-order Sobol' indices, length `d`. `s[i]` is the
    /// fraction of total variance attributable to factor `i` alone.
    pub s: Vec<f64>,
    /// Total-order Sobol' indices, length `d`. `st[i]` is the
    /// fraction of total variance attributable to factor `i` and
    /// all interactions involving it.
    pub st: Vec<f64>,
}

impl FastIndices {
    /// Constructor with shape validation.
    #[must_use]
    pub fn new(s: Vec<f64>, st: Vec<f64>) -> Self {
        assert_eq!(s.len(), st.len(), "FastIndices: s and st must agree on d");
        Self { s, st }
    }

    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.s.len()
    }
}

impl fmt::Display for FastIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FAST indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}  {:>8.4}", i, self.s[i], self.st[i])?;
        }
        Ok(())
    }
}

/// Errors from [`estimate_fast`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum FastEstimatorError {
    /// The supplied harmonic count is outside the supported range.
    #[error("FAST estimator: harmonic must be in 1..=32, got {harmonic}")]
    InvalidHarmonic {
        /// Maximum harmonic supplied by the caller.
        harmonic: u32,
    },
    /// A model evaluation produced a nonfinite value.
    #[error("FAST estimator: nonfinite model output at sample row {row}")]
    NonFiniteOutput {
        /// Row of the design that produced the invalid output.
        row: usize,
    },
    /// Total variance is zero (or numerical floor) — model is
    /// constant, no sensitivity to recover.
    #[error("FAST estimator: spectral variance is too small or nonfinite")]
    ZeroVariance,
}

/// Estimate `Sᵢ` and `Sᵀᵢ` from a [`FastDesign`] and a model.
///
/// `model` is called `n_per_factor · d` times, once per row of
/// `design.samples`. The closure must be pure (deterministic given
/// input).
///
/// # Errors
///
/// - [`FastEstimatorError::InvalidHarmonic`] if harmonic is outside `1..=32`.
/// - [`FastEstimatorError::NonFiniteOutput`] if a model output is not finite.
/// - [`FastEstimatorError::ZeroVariance`] if spectral variance is below
///   `1e-15` or nonfinite from numerical overflow.
///
/// [`salib_samplers::build_fast_design`] constructs a consistent design.
/// Its public fields can be changed; callers must preserve sample shapes
/// and frequency metadata. Only the harmonic order is revalidated here.
pub fn estimate_fast<F>(
    design: &FastDesign,
    mut model: F,
) -> Result<FastIndices, FastEstimatorError>
where
    F: FnMut(&[f64]) -> f64,
{
    let n = design.n_per_factor;
    let d = design.d;
    // FastDesign metadata is public (and may be deserialized), so validate
    // it again before FFT planning or invoking the model.
    let m = HarmonicBudget::new(design.harmonic)
        .map_err(|_| FastEstimatorError::InvalidHarmonic {
            harmonic: design.harmonic,
        })?
        .get();

    let fft = build_fft_planner(n);

    let mut s = vec![0.0_f64; d];
    let mut st = vec![0.0_f64; d];

    // Reusable buffers across blocks.
    let mut row_buf = vec![0.0_f64; d];

    for i in 0..d {
        // Evaluate model at the N samples for factor-of-interest i.
        let mut y: Vec<f64> = Vec::with_capacity(n);
        for n_idx in 0..n {
            let row = i * n + n_idx;
            for j in 0..d {
                row_buf[j] = design.samples[[row, j]];
            }
            let value = model(&row_buf);
            if !value.is_finite() {
                return Err(FastEstimatorError::NonFiniteOutput { row });
            }
            y.push(value);
        }

        // Weighted one-sided spectral variance for frequencies 1..=N/2.
        let spectrum = power_spectrum_one_sided(&y, fft.as_ref());

        // The helper has already counted each conjugate pair, and Nyquist once.
        // The absolute 1e-15 threshold can reject small but nonzero variance.
        let v_total = tree_sum(&spectrum);
        if !v_total.is_finite() || v_total < 1e-15 {
            return Err(FastEstimatorError::ZeroVariance);
        }

        // First-order: sum weighted power at the characteristic harmonics.
        // (Sp is indexed from 0 corresponding to frequency 1, hence
        // the −1 shift from the math notation.) The bandwidth
        // precondition `n_per_factor ≥ 4·M² + 1` upstream guarantees
        // `M · ωᵢ ≤ ⌊N/2⌋`, so every bin is in range.
        let omega_i = design.omegas[[i, i]] as usize;
        let mut harmonic_bins = [0.0_f64; 32];
        let m_usize = m as usize;
        debug_assert!(m_usize <= harmonic_bins.len(), "harmonic order too large");
        for p in 1..=m_usize {
            let bin = p * omega_i;
            debug_assert!(bin >= 1 && bin - 1 < spectrum.len());
            harmonic_bins[p - 1] = spectrum[bin - 1];
        }
        let v1 = tree_sum(&harmonic_bins[..m_usize]);

        // Approximate complementary variance from the low-frequency band.
        // Interactions can still cause spectral interference.
        let half = (omega_i / 2).max(1).min(spectrum.len());
        let v_comp = tree_sum(&spectrum[..half]);

        s[i] = (v1 / v_total).clamp(0.0, 1.0);
        st[i] = (1.0 - v_comp / v_total).clamp(0.0, 1.0);
    }

    Ok(FastIndices::new(s, st))
}

/// Build a forward FFT plan for length `n`. `rustfft` is
/// deterministic for fixed input.
fn build_fft_planner(n: usize) -> Arc<dyn Fft<f64>> {
    let mut planner: FftPlanner<f64> = FftPlanner::new();
    planner.plan_fft_forward(n)
}

/// Spectral variance at each positive frequency: `w[k] * |Y[k]|² / N²`.
/// Conjugate pairs have weight two, the even-length Nyquist bin weight one.
/// DC is omitted; index `k-1` corresponds to frequency `k`.
pub(super) fn power_spectrum_one_sided(y: &[f64], fft: &dyn Fft<f64>) -> Vec<f64> {
    let n = y.len();
    let mut buffer: Vec<Complex<f64>> = y.iter().map(|&v| Complex::new(v, 0.0)).collect();
    fft.process(&mut buffer);
    let half = n / 2;
    let n_sq = (n as f64).powi(2);
    (1..=half)
        .map(|k| {
            let c = buffer[k];
            let weight = if n.is_multiple_of(2) && k == half {
                1.0
            } else {
                2.0
            };
            weight * (c.re * c.re + c.im * c.im) / n_sq
        })
        .collect()
}

#[cfg(test)]
#[allow(
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::approx_constant
)]
mod tests {
    use super::*;
    use salib_core::RngState;
    use salib_samplers::build_fast_design;

    #[test]
    fn spectral_variance_matches_direct_variance_for_odd_and_even_lengths() {
        for n in [63, 64] {
            let y: Vec<_> = (0..n)
                .map(|j| {
                    7.0 + (2.0 * std::f64::consts::PI * j as f64 / n as f64).cos()
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
            let mean = y.iter().sum::<f64>() / n as f64;
            let direct = y.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
            let fft = build_fft_planner(n);
            let spectral = tree_sum(&power_spectrum_one_sided(&y, fft.as_ref()));
            assert!(
                (spectral - direct).abs() < 1e-12,
                "N={n}: spectral={spectral}, direct={direct}"
            );
        }
        let y: Vec<_> = (0..64)
            .map(|j| if j % 2 == 0 { 1.0 } else { -1.0 })
            .collect();
        let fft = build_fft_planner(y.len());
        assert!((tree_sum(&power_spectrum_one_sided(&y, fft.as_ref())) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn fast_uses_correct_weights_in_total_and_complementary_bands() {
        for n in [73, 74] {
            let design = build(1, n);
            let omega = f64::from(design.omegas[[0, 0]]);
            let coordinates: Vec<_> = (0..n).map(|j| design.samples[[j, 0]]).collect();
            assert!(coordinates.iter().enumerate().all(|(j, x)| coordinates[..j]
                .iter()
                .all(|prior| prior.to_bits() != x.to_bits())));
            let estimate = estimate_fast(&design, |u| {
                let j = coordinates
                    .iter()
                    .position(|x| x.to_bits() == u[0].to_bits())
                    .unwrap();
                let phase = 2.0 * std::f64::consts::PI * j as f64 / n as f64;
                (omega * phase).cos()
                    + 0.5 * phase.cos()
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
            .unwrap();
            // Orthogonal discrete Fourier modes: ordinary modes each have
            // variance amplitude²/2; the even-N Nyquist mode has variance 1.
            let total = 0.5 + 0.125 + if n % 2 == 0 { 1.0 } else { 0.0 };
            assert!((estimate.s[0] - 0.5 / total).abs() < 1e-12);
            assert!((estimate.st[0] - (1.0 - 0.125 / total)).abs() < 1e-12);
        }
    }

    #[test]
    fn fast_rejects_nonfinite_model_output() {
        let design = build(1, 65);
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                estimate_fast(&design, |_| value).unwrap_err(),
                FastEstimatorError::NonFiniteOutput { row: 0 }
            );
        }
    }

    #[test]
    fn fast_rejects_excessive_harmonic() {
        // Public designs can be mutated or deserialized, so the estimator
        // must validate even when the sampler already does.
        let mut design = build(1, 6401);
        for harmonic in [0, 33, 40, u32::MAX] {
            design.harmonic = harmonic;
            assert_eq!(
                estimate_fast(&design, |_| panic!("invalid harmonic reached model")).unwrap_err(),
                FastEstimatorError::InvalidHarmonic { harmonic },
            );
        }
    }

    const SEED: [u8; 32] = [0x42; 32];

    fn build(d: usize, n: usize) -> FastDesign {
        let mut rng = RngState::from_seed(SEED);
        build_fast_design(d, n, 4, &mut rng).expect("valid")
    }

    // ── FastIndices basics ────────────────────────────────────────

    #[test]
    fn fast_indices_d_matches_vec_length() {
        let i = FastIndices::new(vec![0.1, 0.2], vec![0.3, 0.4]);
        assert_eq!(i.d(), 2);
    }

    #[test]
    #[should_panic(expected = "must agree on d")]
    fn fast_indices_mismatch_panics() {
        let _ = FastIndices::new(vec![0.1, 0.2], vec![0.3]);
    }

    // ── Constant model: ZeroVariance ──────────────────────────────

    #[test]
    fn constant_model_returns_zero_variance() {
        let design = build(3, 65);
        let err = estimate_fast(&design, |_| 1.0).unwrap_err();
        assert_eq!(err, FastEstimatorError::ZeroVariance);
    }

    // ── Indices in [0, 1] ─────────────────────────────────────────

    #[test]
    fn indices_clamped_to_unit_interval() {
        let design = build(3, 257);
        // Linear model in factor 0: Y = x_0.
        let est = estimate_fast(&design, |x| x[0]).expect("estimate");
        for &v in &est.s {
            assert!((0.0..=1.0).contains(&v), "S out of range: {v}");
        }
        for &v in &est.st {
            assert!((0.0..=1.0).contains(&v), "ST out of range: {v}");
        }
    }

    // ── Linear single-factor: factor 0 dominant ───────────────────

    #[test]
    fn linear_single_factor_concentrates_variance() {
        // Y = x_0 — all variance attributable to factor 0.
        // S_0 ≈ 1, ST_0 ≈ 1, S_{1,2} ≈ 0, ST_{1,2} ≈ 0.
        let design = build(3, 257);
        let est = estimate_fast(&design, |x| x[0]).expect("estimate");
        assert!(
            est.s[0] > 0.5,
            "S_0 = {} should dominate for Y = x_0",
            est.s[0]
        );
        assert!(est.st[0] > 0.5, "ST_0 = {} should dominate", est.st[0]);
        assert!(
            est.s[1] < 0.2,
            "S_1 = {} should be small (factor 1 not in model)",
            est.s[1]
        );
        assert!(est.s[2] < 0.2, "S_2 = {} should be small", est.s[2]);
    }

    // ── ST ≥ S identity ───────────────────────────────────────────

    #[test]
    fn st_at_least_s_for_additive_model() {
        // Y = x_0 + x_1 + x_2 — additive, no interactions.
        // ST_i ≥ S_i is the universal Sobol' identity.
        let design = build(3, 257);
        let est = estimate_fast(&design, |x| x[0] + x[1] + x[2]).expect("estimate");
        for i in 0..3 {
            assert!(
                est.st[i] + 1e-6 >= est.s[i],
                "factor {i}: ST = {}, S = {} (ST ≥ S violated)",
                est.st[i],
                est.s[i]
            );
        }
    }

    // ── Determinism ───────────────────────────────────────────────

    #[test]
    fn same_design_yields_identical_estimates() {
        let design = build(3, 257);
        let model = |x: &[f64]| x[0] + 0.5 * x[1] * x[2];
        let a = estimate_fast(&design, model).expect("a");
        let b = estimate_fast(&design, model).expect("b");
        assert_eq!(a.s, b.s);
        assert_eq!(a.st, b.st);
    }

    // ── Power spectrum unit ───────────────────────────────────────

    #[test]
    fn power_spectrum_of_zero_signal_is_zero() {
        let n = 16;
        let fft = build_fft_planner(n);
        let zeros = vec![0.0_f64; n];
        let sp = power_spectrum_one_sided(&zeros, fft.as_ref());
        assert_eq!(sp.len(), n / 2);
        for &v in &sp {
            assert_eq!(v, 0.0);
        }
    }

    #[test]
    fn power_spectrum_of_pure_sinusoid_concentrates_at_frequency() {
        // y[n] = sin(2π · k · n / N) — energy concentrates at bin k.
        let n = 64;
        let k = 5_usize;
        let fft = build_fft_planner(n);
        let y: Vec<f64> = (0..n)
            .map(|i| {
                let arg = 2.0 * std::f64::consts::PI * (k as f64) * (i as f64) / (n as f64);
                arg.sin()
            })
            .collect();
        let sp = power_spectrum_one_sided(&y, fft.as_ref());
        // Bin k-1 (0-indexed) corresponds to frequency k. Expect
        // peak there; other bins ≈ 0.
        let peak = sp[k - 1];
        for (idx, &v) in sp.iter().enumerate() {
            if idx != k - 1 {
                assert!(
                    v < peak * 1e-6,
                    "bin {idx}: {v} not negligible vs peak {peak}"
                );
            }
        }
    }
}
