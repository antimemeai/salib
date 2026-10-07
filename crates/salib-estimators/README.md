# salib-estimators

Sensitivity measures implemented from published formulas: Sobol' variance
indices, Morris elementary effects, FAST/eFAST/RBD-FAST, distribution and
quantile measures, derivative bounds, regression, and experimental-design
statistics. Depend on this crate directly when analyzing cached outputs or
composing estimators in your own workflow. Add `salib-samplers` for designs and
`salib-core` for input/RNG types, or use the `salib` facade for all three.

## Choose an entry point

| API / result | Use it for |
|---|---|
| `estimate_saltelli2010` → `SobolIndices` | First-order, total-effect, and optional pairwise indices from a Saltelli design and model closure |
| `estimate_saltelli2010_from_outputs` | Cached `fa`, `fb`, `fab[i]` output vectors with matching row order |
| `estimate_saltelli2010_with_bootstrap` → `SobolIndicesWithCi` | Percentile intervals without repeating model evaluations per resample |
| `estimate_jansen`, `estimate_janon`, `estimate_owen` | Alternative Sobol' formulas; Jansen/Janon/Owen have no total-effect vector, and Owen requires `OwenMatrix` |
| `estimate_morris_effects`, grouped variant | Screening: signed mean, absolute mean, and spread of elementary effects |
| `estimate_fast`, `estimate_rbd_fast` | Frequency-based indices; RBD-FAST takes given data and returns first-order indices |
| `estimate_pawn`, `estimate_borgonovo_delta`, `estimate_qosa` | Changes in CDF, density, or a chosen quantile from `(X,Y)` data |
| `finite_difference_gradients`, `estimate_dgsm`, `poincare_constant` | Gradient acquisition, mean squared derivatives, and total-effect bounds |
| `estimate_regression_indices`, `estimate_given_data_sobol` | Linear/rank associations or approximate Sobol' main effects from existing data |
| `bootstrap_given_data` → `BootstrapCi` | Row bootstrap for a supplied given-data estimator; inspect skipped resamples |
| ANOVA, G-theory, fractional factorial, discrepancy | Structured grids, measurement reliability, two-level effects, or point-set quality |

## Example

```rust
use salib_core::RngState;
use salib_samplers::{build_saltelli_matrix, LhsSampler};
use salib_estimators::estimate_saltelli2010_with_bootstrap;

let mut rng = RngState::from_seed([42; 32]);
let design = build_saltelli_matrix(&LhsSampler::classic(4), 1024, false, &mut rng)
    .unwrap();
let mut bootstrap_rng = rng.fork(b"bootstrap");
// This model deliberately uses two independent Uniform(0,1) inputs.
let result = estimate_saltelli2010_with_bootstrap(
    &design, |x| x[0] + 2.0 * x[1], 200, 0.05, &mut bootstrap_rng,
).unwrap();
assert_eq!(result.indices.dim, 2);
assert_eq!(result.first_order_ci.len(), 2);
println!("S1: {:?}; intervals: {:?}",
    result.indices.first_order, result.first_order_ci);
```

The analytic first-order targets are `[0.2, 0.8]`; total effects match because
the model is additive. Treat estimates as uncertain, not exact fractions. For
physical inputs, map unit coordinates through each input distribution before
evaluating the model. For cached outputs, preserve block and row alignment.

Ordinary Sobol' attribution assumes independent inputs and finite nonzero output
variance. Slightly negative estimates can reflect sampling noise. Estimator APIs
differ in error handling: some return `Result`, while others return zero indices
for degenerate variance. Inspect the result and method documentation.

## Dependencies and features

```toml
[dependencies]
salib-core = "0.2"
salib-samplers = "0.2"
salib-estimators = "0.2"
```

`serde` serializes result types. `surrogate` enables `estimate_hdmr` and depends
on `salib-surrogate`; PCE fitting and active subspaces live in that crate.
Shapley effects live in `salib-shapley`.

[API reference](https://docs.rs/salib-estimators/latest/salib_estimators/) ·
[Method selection](https://github.com/antimeme-ai/salib/blob/main/docs/choosing.md)

MIT OR Apache-2.0.
