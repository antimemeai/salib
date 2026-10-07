# salib

Facade for global sensitivity analysis in Rust: validated input descriptions,
sampling designs, and 20+ analysis methods implemented from primary papers.
Use this crate for a complete Rust analysis. Core types are re-exported at the
root; sampling and estimation live under `salib::samplers` and
`salib::estimators`. Depend on individual crates for a narrower API surface.

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
zero main effect. See the [step-by-step tutorial](https://github.com/antimeme-ai/salib/blob/main/docs/quickstart.md) for
interpretation, sampler alternatives, and bootstrap intervals.

## Choose dependencies and features

| Need | API / dependency |
|---|---|
| Standard sampling and estimation | `salib` defaults: `samplers`, `estimators`, `parallel` |
| Input definitions and reproducible RNG only | `salib-core` (`ProblemBuilder`, `Distribution`, `RngState`) |
| Generate designs for an external evaluator | `salib-samplers` (`Sampler`, `SaltelliMatrix`, Morris/FAST designs) |
| Analyze cached outputs | `salib-estimators` (Sobol', given-data measures, bootstrap) |
| PCE or active subspaces | `salib` with `surrogate`, or `salib-surrogate` directly |
| Independent-input Shapley attribution | `salib` with `shapley`, or `salib-shapley` |
| Reference models | `validation`, or `salib-validation` as a dev dependency |

`full` enables samplers, estimators, surrogate, Shapley, and validation. `serde`
adds result serialization; `arrow` adds RecordBatch conversions; `polars` adds
DataFrame conversions and implies `arrow`. These interop features are separate
from `full`. HDMR requires both estimator and surrogate support. Core definitions
already support serde.

```toml
# Core, samplers, and estimators with serial reduction fallback.
[dependencies]
salib = { version = "0.2", default-features = false, features = ["samplers", "estimators"] }
```

Features are additive across dependencies, so another crate can still enable
rayon. Bit-reproducibility means identical inputs and full RNG state yield
identical bits across thread counts for the same binary/platform and reproducible
model. Kani and Stateright check bounded implementation invariants alongside
analytic and metamorphic tests; they do not prove all estimators correct.

[Documentation hub](https://github.com/antimeme-ai/salib/blob/main/docs/index.md) ·
[API reference](https://docs.rs/salib/latest/salib/)

Rust 1.87 or later. MIT OR Apache-2.0.
