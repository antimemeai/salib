# salib

Global sensitivity analysis for Rust. salib estimates how uncertain inputs
affect a model's output, using Sobol' indices, Morris screening, FAST, PAWN,
regression, polynomial surrogates, and other methods from the published literature.

Pass a Rust function to an estimator, or generate a sampling design and analyze
the outputs from an external simulator. Each design type records the layout its
estimator needs. `ProblemBuilder` checks input distributions and factor groups.

With the same binary, platform, inputs, RNG state, and reproducible model,
results are bit-for-bit identical across thread counts. See
[internals](docs/internals.md) for the reduction order and RNG behavior, and
[benchmarks](docs/benchmarks.md) for measured runtimes.

salib covers many of the same methods as Python SALib, but its samples and
numerical results can differ.

## Quickstart

This example estimates Sobol' indices for the three-input Ishigami function.
Its third input affects the output only through an interaction, which appears
in the total-effect index (`ST`) but not the first-order index (`S1`).

```toml
[dependencies]
salib = "0.3"
```

```rust
use std::f64::consts::PI;
use salib::{Distribution, ProblemBuilder, RngState};
use salib::samplers::{build_saltelli_matrix, SobolSampler};
use salib::estimators::estimate_saltelli2010;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem = ProblemBuilder::new()
        .factor("x1", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x2", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x3", Distribution::Uniform { lo: -PI, hi: PI })
        .build()?;

    let mut rng = RngState::from_seed([0; 32]);
    let sampler = SobolSampler::minimal(2 * problem.dim());
    let design = build_saltelli_matrix(&sampler, 8192, false, &mut rng)?;

    // Designs contain unit-cube coordinates; map them to physical inputs.
    let model = |u: &[f64]| {
        let x: Vec<f64> = problem.factors().iter().zip(u)
            .map(|(factor, &ui)| factor.distribution.quantile(ui))
            .collect();
        x[0].sin() + 7.0 * x[1].sin().powi(2)
            + 0.1 * x[2].powi(4) * x[0].sin()
    };
    let indices = estimate_saltelli2010(&design, model);
    for (i, factor) in problem.factors().iter().enumerate() {
        println!("{}: S1 = {:.4}, ST = {:.4}",
            factor.name, indices.first_order[i], indices.total_order[i]);
    }
    Ok(())
}
```

The sampler needs six columns to construct two three-factor base matrices.
`8192` is the base sample size: this design requires 40,960 model evaluations.
`false` skips the additional matrices for pairwise indices. The estimator calls
the closure with unit-cube rows; the closure applies each factor’s inverse CDF.

Expect `S1` near `[0.314, 0.442, 0.000]` and `ST` near
`[0.558, 0.442, 0.244]`. Thus `x3` contributes through interactions despite its
zero main effect. See the [step-by-step tutorial](docs/quickstart.md) for
interpretation, sampler alternatives, and bootstrap intervals.

## Crates and features

Most applications should depend on the `salib` facade. Its defaults enable
`samplers`, `estimators`, and `parallel`. Depend on individual crates when you
need their API directly:

| Crate | Use it for |
|---|---|
| [salib-core](crates/salib-core/README.md) | Problems, distributions, RNG state, bit-reproducible reductions |
| [salib-samplers](crates/salib-samplers/README.md) | Sampling designs to evaluate in your own pipeline |
| [salib-estimators](crates/salib-estimators/README.md) | Sensitivity estimates from designs or cached data |
| [salib-surrogate](crates/salib-surrogate/README.md) | Full/sparse PCE and active subspaces (`surrogate` feature) |
| [salib-shapley](crates/salib-shapley/README.md) | Shapley variance attribution for independent inputs (`shapley`) |
| [salib-validation](crates/salib-validation/README.md) | Analytic reference models (`validation`) |
| [salib-cli](crates/salib-cli/README.md) | Reserved CLI package; commands are not implemented yet |

These seven packages plus the facade make eight release crates.
`salib-models` is an additional, unpublished verification crate.
`full` enables all analysis families; `serde`, `arrow`, and `polars` are separate
interop features. See the [crate map](docs/crates.md) for dependency details.

[Documentation hub](docs/index.md) · [Method selection](docs/choosing.md) ·
[API reference](https://docs.rs/salib/latest/salib/) ·
[Bit-reproducibility contract](docs/internals.md)

The [literature audit](papers/2026-10-07-literature-audit.md) records corrected
numerical defects, reproducible checks, and remaining approximation limits.
See the [0.3.0 release notes](docs/release-0.3.0.md) for behavior changes and
supported inputs.

## Testing

The tests compare estimates with analytic results for Ishigami, the Sobol'
G-function, and other models in `salib-validation`, allowing for sampling error.
The formulas and assumptions come from the papers listed in the
[bibliography](docs/bibliography.md).

Another 28 tests check mathematical relations between runs: scaling the output
should preserve Sobol' indices, reordering factors should reorder their indices,
and additive models should have zero interaction terms within sampling error.
Sampler tests check LHS strata, Sobol' sequences, and Saltelli matrix layout.
Reproducibility tests compare result bits across thread counts.

Kani's bounded checks found four bugs, now covered by fixes and regression
tests. Two post-fix proofs passed; the uniform-quantile and percentile
monotonicity proofs timed out and remain inconclusive. Four Stateright models
check problem construction, RNG use, Saltelli assembly, and estimator
completeness within finite domains. These checks cover the stated properties;
they do not prove every estimator correct for every model.

Run the full suite with `cargo test --workspace`. During development,
`cargo test -p salib-estimators <pattern>` scopes the run to the relevant estimator.

See the [testing plan](docs/test-modernization-plan.md) and
[analysis reports](docs/analysis/) for the test rationale and verification results.

## Requirements

Rust 1.87 for the default and `full` analysis features; edition 2021.
Fresh dependency resolution for optional `polars` currently requires Rust 1.88.

## License

MIT OR Apache-2.0, at your option.
