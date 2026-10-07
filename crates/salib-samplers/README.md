# salib-samplers

Sampling designs for sensitivity analysis, stored as `ndarray` matrices with
explicit layout metadata. Depend on this crate directly when generating inputs
for a simulator or service that evaluates samples separately from estimation.
Add `salib-estimators` for the corresponding analyses; use `salib` when both
sampling and estimation live in one application.

## Choose a design

| API | Use it when |
|---|---|
| `Sampler::unit_sample`, `LhsSampler` | You need an `(N,d)` stratified unit-cube sample for given-data methods or model training |
| `SobolSampler`, `SobolDimSet` | You want unscrambled quasi-Monte Carlo points; tables support 100 or 1000 dimensions |
| `build_saltelli_matrix`, `SaltelliMatrix` | Saltelli, Jansen, or Janon needs paired `A`, `B`, and column-swapped hybrids; sampler dimension is `2d` |
| `build_owen_matrix`, `OwenMatrix` | Owen needs a separate three-vector design; sampler dimension is `3d` |
| `build_morris_trajectories`, grouped variant | Screening with one-at-a-time trajectories and recorded step order |
| `build_fast_design`, `HarmonicBudget` | FAST/eFAST requires frequency and phase metadata with a sufficient sample count |
| `build_plackett_burman` | Two-level main-effect screening |
| `iman_conover_transform` | Reorder sample ranks toward a target correlation matrix |

## Example

```rust
use salib_core::{Distribution, RngState};
use salib_samplers::{build_saltelli_matrix, SobolSampler};

let mut rng = RngState::from_seed([0; 32]);
let sampler = SobolSampler::minimal(4); // 2d for two physical factors.
let design = build_saltelli_matrix(&sampler, 1024, false, &mut rng).unwrap();
assert_eq!(design.a.dim(), (1024, 2));
assert_eq!(design.total_evaluations(), 4096);
// The matrices contain unit coordinates. Map before calling a physical model.
let range = Distribution::Uniform { lo: 10.0, hi: 20.0 };
let physical_value = range.quantile(design.a[[0, 0]]);
assert!((10.0..=20.0).contains(&physical_value));
```

Saltelli costs `N(d+2)` evaluations, or `N(2d+2)` with second-order blocks.
Morris and FAST have their own layouts; arbitrary LHS rows cannot replace them.
Unscrambled Sobol' restarts on each call and ignores RNG state. Its default skips
the origin; skipping a point can impair digital-net balance, so do not assume a
scrambled-QMC error guarantee. LHS consumes and advances `RngState`.

## Dependencies and features

```toml
[dependencies]
salib-core = "0.2"
salib-samplers = "0.2"
```

`serde` enables serialization of matrix-carrying designs through ndarray.
Sampler configuration hashing identifies configuration, not the sample size or
model. Iman–Conover produces correlated samples; it does not make an
independent-input Sobol' estimator valid for dependent inputs.

[API reference](https://docs.rs/salib-samplers/latest/salib_samplers/) ·
[Method selection](https://github.com/antimeme-ai/salib/blob/main/docs/choosing.md)

MIT OR Apache-2.0.
