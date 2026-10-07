//! The `Sampler` trait — produces unit-cube `[0, 1)^d` sample
//! matrices that downstream code maps through factor distributions
//! via `Distribution::quantile`.
//!
//! The trait is `Send + Sync` so samplers can be passed across rayon
//! parallel boundaries.
//!
//! # The `config_hash` contract
//!
//! Identifies the *configuration* of the sampler — dim, kind, any
//! tunable parameters — not its *output*. Two samplers with the same
//! `config_hash` produce the same output sequence given the same
//! `RngState` input. SHA-256 over canonical-JSON of the sampler's
//! config struct. Mirrors `Problem::content_hash`.
//!
//! # Bit-reproducibility
//!
//! `unit_sample` takes `&mut RngState` and *advances* it through the
//! draws. Same `RngState` in → bit-identical `Array2<f64>` out, with
//! `RngState::word_pos` advanced to reflect the consumed bytes.

use ndarray::Array2;
use salib_core::RngState;

/// A sampler that produces unit-cube samples in `[0, 1)^d`.
///
/// Map coordinates through input distributions before evaluating a physical model.
/// Unscrambled Sobol' ignores `RngState` and restarts on each call; LHS advances it.
///
/// # Examples
///
/// ```rust
/// use salib_core::RngState;
/// use salib_samplers::{LhsSampler, Sampler};
/// let sampler = LhsSampler::classic(2);
/// let mut rng = RngState::from_seed([42; 32]);
/// let points = sampler.unit_sample(64, &mut rng);
/// assert_eq!(points.dim(), (64, 2));
/// assert!(points.iter().all(|&u| (0.0..1.0).contains(&u)));
/// ```
pub trait Sampler: Send + Sync {
    /// Number of factors. The output of [`unit_sample`](Self::unit_sample)
    /// has shape `(n, dim())`.
    fn dim(&self) -> usize;

    /// Generate an `n × dim` matrix of unit-cube samples in
    /// `[0, 1)^dim`. Pure: same `(self_config, rng_state)` → same
    /// output. Advances `rng` to reflect bytes consumed; the caller
    /// can snapshot before and after to record the draw range.
    fn unit_sample(&self, n: usize, rng: &mut RngState) -> Array2<f64>;

    /// SHA-256 over canonical-JSON of the sampler's configuration.
    /// Identifies the *configuration* (dim, kind, tunable
    /// parameters), not the output. Two samplers with the same
    /// `config_hash` produce the same output given the same RNG
    /// state.
    fn config_hash(&self) -> [u8; 32];
}
