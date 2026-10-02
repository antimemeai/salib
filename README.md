# salib

Global sensitivity analysis for Rust, implemented from the primary
literature.

**Bit-reproducible**: identical `RngState` produces identical results
regardless of thread count. Parallel reductions use a tree-structured
accumulation strategy to eliminate float-associativity nondeterminism under
[rayon](https://docs.rs/rayon).

## Quickstart

```toml
# Cargo.toml
[dependencies]
salib = "0.2"
```

```rust
use std::f64::consts::PI;
use salib::*;
use salib::samplers::{SobolSampler, build_saltelli_matrix};
use salib::estimators::estimate_saltelli2010;

fn main() {
    // 1. Define the problem: 3 factors, Uniform(-pi, pi)
    let problem = ProblemBuilder::new()
        .factor("x1", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x2", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x3", Distribution::Uniform { lo: -PI, hi: PI })
        .build()
        .unwrap();

    // 2. Build a Saltelli sample matrix (N=8192 base samples, 3 factors → 6-dim sampler)
    let mut rng = RngState::from_seed([0u8; 32]);
    let sampler = SobolSampler::minimal(2 * problem.dim());
    let saltelli = build_saltelli_matrix(&sampler, 8192, false, &mut rng).unwrap();

    // 3. Estimate Sobol' indices — the estimator calls the model internally
    //    Ishigami: y = sin(x1) + 7*sin(x2)^2 + 0.1*x3^4*sin(x1)
    let indices = estimate_saltelli2010(&saltelli, |x| {
        x[0].sin() + 7.0 * x[1].sin().powi(2) + 0.1 * x[2].powi(4) * x[0].sin()
    });

    // 4. Print results
    for (i, f) in problem.factors().iter().enumerate() {
        println!("{}: S1 = {:.4}, ST = {:.4}", f.name, indices.first_order[i], indices.total_order[i]);
    }
}
```

## Crate structure

`salib` is a facade that re-exports subcrates. Use it for convenience, or
depend on individual crates for finer control.

| Crate | Contents |
|---|---|
| `salib-core` | `Problem`, `Factor`, `Distribution`, `RngState`, bit-reproducible reductions |
| `salib-samplers` | LHS, Sobol' QMC, Halton, Saltelli (A/B/A\_Bi), Morris trajectories, FAST/eFAST/RBD-FAST designs |
| `salib-estimators` | Variance-based Sobol' (Saltelli2010, Jansen, Janon, Owen), Morris EE, FAST/eFAST, RBD-FAST, Borgonovo delta, PAWN, DGSM, regression (SRC/SRRC/PCC/PRCC), given-data Sobol', ANOVA, HDMR, G-theory, fractional factorial, discrepancy |
| `salib-surrogate` | PCE (full + sparse LARS), active subspaces |
| `salib-shapley` | Shapley effects (Song-Nelson-Staum 2016) |
| `salib-validation` | Analytic test functions (Ishigami, Sobol' G, etc.) with closed-form indices |
| `salib-cli` | CLI binary: `sample`, `run`, `analyze` subcommands |

## Feature flags

```toml
[features]
default = ["samplers", "estimators"]
samplers   = ["dep:salib-samplers"]
estimators = ["dep:salib-estimators"]
surrogate  = ["dep:salib-surrogate"]
shapley    = ["dep:salib-shapley"]
validation = ["dep:salib-validation"]
full       = ["samplers", "estimators", "surrogate", "shapley", "validation"]
```

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

On top of that, structural tests pin down the sampling machinery — LHS
stratification, Sobol' canonical sequences, Saltelli matrix construction —
and bit-reproducibility tests confirm that the same seed produces the same
bits regardless of thread count.

Run the full suite with `cargo test --workspace` (about a minute for 917
tests). During development, scope your runs: `cargo test -p salib-estimators
<pattern>` touches only what you're working on.

The testing strategy lives in `docs/test-modernization-plan.md`. Deeper
analyses — every metamorphic relation, float hazard, formal verification
target, and type-level constraint opportunity — are catalogued in
`docs/analysis/`.

## Requirements

- Edition 2021
- MSRV 1.87

## License

Licensed under MIT OR Apache-2.0, at your option.
