# salib

Global sensitivity analysis for Rust, implemented from the primary literature.
20+ methods cover variance attribution, screening, distributional changes,
gradients, regression, and surrogates.

## Why salib?

For a model already written in Rust, salib keeps sampling and estimation in
compiled code without a Python boundary. Compared with using Python SALib:

- **Native execution:** model closures and array operations run in Rust;
  [benchmarks](docs/benchmarks.md) describe measured costs and their scope.
- **Typed designs:** separate Saltelli, Owen, Morris, and FAST types make the
  expected sampling layout explicit; `ProblemBuilder` validates input parameters.
- **Bit-reproducibility:** identical inputs and `RngState` produce identical bits
  across thread counts for the same binary and platform, with a reproducible model.
- **Verification:** analytic references, metamorphic oracles, bounded Kani proofs,
  and Stateright state exploration check different classes of implementation errors.

Choose Python SALib when its Python ecosystem suits your workflow. salib offers
similar method families with Rust APIs; it does not promise identical Python
samples or numerical results.

## Quickstart

Estimate which of three independent inputs explains the Ishigami model’s output
variance. The third input matters only through an interaction, making it a useful
example of why both first-order (`S1`) and total-effect (`ST`) indices matter.

```toml
[dependencies]
salib = "0.2"
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

## Testing

There is no formal specification to conform to — salib implements methods
from published papers, and its correctness is measured against those papers.
The test suite reflects that reality.

Estimators are validated against closed-form analytic results. The
Ishigami function, Sobol' G-function, and other canonical test problems
have exact Sobol' indices known in closed form; we compute estimates from
samples and check they land within Monte Carlo tolerance of the published
values. This is the primary correctness gate — `salib-validation` provides
the reference values.

But exact expected values only catch gross errors. The more insidious
failures are silent: a normalization that drifts under scaling, an index
that swaps under factor reordering, a variance formula that collapses on
large-offset inputs. To catch those, the suite also exercises metamorphic
oracles — mathematical identities that must hold between runs with
different inputs but related structure. Scale the output by a constant and
every Sobol' index should be unchanged. Permute the design columns and the
indices should permute with them. Feed in a purely additive model and the
interaction terms should vanish. These relations require no ground truth:
they are properties of the math itself, verified against the code's own
outputs.

The suite includes 28 metamorphic oracles. Bounded Kani verification found
four bugs that now have fixes and regression tests; four Stateright models
check problem construction, RNG use, Saltelli assembly, and estimator
completeness. These checks cover specific invariants, not a proof of every
estimator’s statistical correctness. The former TCK has been retired.

On top of that, structural tests pin down the sampling machinery — LHS
stratification, Sobol' canonical sequences, Saltelli matrix construction —
and bit-reproducibility tests confirm that the same seed produces the same
bits regardless of thread count.

Run the full suite with `cargo test --workspace`. During development,
`cargo test -p salib-estimators <pattern>` scopes the run to the relevant estimator.

The testing strategy lives in `docs/test-modernization-plan.md`. Deeper
analyses — every metamorphic relation, float hazard, formal verification
target, and type-level constraint opportunity — are catalogued in
`docs/analysis/`.

## Requirements

Rust 1.87 or later; edition 2021.

## License

MIT OR Apache-2.0, at your option.
