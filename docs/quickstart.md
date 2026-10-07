# Quickstart

Estimate Sobol' indices for the Ishigami function, then add bootstrap
confidence intervals. The example has three independent inputs and one output.

## 1. Add the dependency

```toml
[dependencies]
salib = "0.3"
```

The default features provide core types, samplers, estimators, and parallel
reduction primitives. Rust 1.87 or later is required. Put the complete program
below in `src/main.rs` and run `cargo run --release`.

## 2. Define the model and its inputs

The Ishigami function uses independent inputs $x_i \sim U(-\pi, \pi)$:

$$f(\mathbf{x}) = \sin(x_1) + 7\sin^2(x_2) + 0.1x_3^4\sin(x_1).$$

`ProblemBuilder::factor` records a name and distribution in column order.
`build()` checks for invalid parameters, duplicate names, and invalid groups.
The `Problem` stores these definitions. In step 4, the model closure uses them
to transform the sampler's unit-cube coordinates into physical input values.

## 3. Construct the sampling design

`SobolSampler::minimal(2 * problem.dim())` provides six unit-cube columns:
three for matrix $A$ and three for $B$. `build_saltelli_matrix` also builds one
hybrid $A_B^{(i)}$ per factor by replacing column $i$ of $A$ with column $i$ of
$B$. These paired evaluations isolate the effect of changing each input.

`8192` is the number of rows **in each matrix**, not the total evaluation budget.
For $d=3$, $N(d+2)=40{,}960$ model evaluations are needed. `false` requests
first-order and total-effect indices without the extra $B_A^{(i)}$ matrices for
pairwise interactions. Use `design.total_evaluations()` to budget before running.

`RngState::from_seed` records the random stream. This Sobol' sampler is
unscrambled and does not consume the RNG: changing its seed will not change its
points. Stochastic samplers and bootstrap resampling do consume the state.

## 4. Evaluate and estimate

`estimate_saltelli2010` evaluates the model on each row of the design and returns
`SobolIndices`. Its closure takes `&[f64]` and returns one `f64`. The closure
below first maps unit coordinates through `Distribution::quantile`, then calls
the physical model. Omitting this step would analyze Ishigami on $[0,1)^3$,
which has different sensitivity indices.

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

To substitute your model, change the factor definitions and the final expression
in the closure together. Keep column order consistent, return finite outputs,
and check that output variance is nonzero before interpreting normalized indices.
For an external simulator, evaluate the design yourself and use
`estimate_saltelli2010_from_outputs(&fa, &fb, &fab)`: each vector must retain the
same row order, and `fab[i]` must correspond to hybrid matrix `a_b[i]`.

## 5. Interpret the results

The exact indices are below. Your estimates will differ because they use a
finite sample:

| Factor | First-order $S_i$ | Total-effect $S_{Ti}$ |
|---|---:|---:|
| x1 | 0.3139 | 0.5576 |
| x2 | 0.4424 | 0.4424 |
| x3 | 0.0000 | 0.2437 |

$S_1=0.31$ means variation in the conditional mean $E[Y\mid X_1]$ accounts for
about 31% of total output variance under these input distributions. It is not a
31% change in output when you change $x_1$, and it does not imply causality.

$S_{T1}\approx0.56$ includes every interaction involving $x_1$.
The gap $S_{T1}-S_1\approx0.24$ is the variance share from those interactions.
For $x_2$, equal first-order and total-effect indices indicate a main effect
with no interaction in this model. $x_3$ has no main effect but participates in
the $x_1,x_3$ interaction: discarding it based only on $S_3$ would be a mistake.

First-order indices sum to about 0.756; the remaining variance belongs to
interactions. Total-effect indices overlap because they count shared
interactions for each participating input, so they need not sum to one.
Finite-sample estimates can be slightly negative or above one. Increase the
sample size and inspect uncertainty before treating a small difference as real;
bit-reproducibility does not remove sampling error.

## 6. Add bootstrap confidence intervals

Replace the estimation call in the program with:

```rust
use salib::estimators::estimate_saltelli2010_with_bootstrap;

let mut bootstrap_rng = rng.fork(b"ishigami-bootstrap");
let result = estimate_saltelli2010_with_bootstrap(
    &design, model, 1000, 0.05, &mut bootstrap_rng,
)?;
println!("x1 S1 = {:.4}, interval = {:?}",
    result.indices.first_order[0], result.first_order_ci[0]);
```

`1000` is the number of resamples; `0.05` is $\alpha$, giving nominal 95%
percentile intervals. Each resample uses matching row indices across all
output blocks. Model evaluations are cached once, so increasing resamples adds
estimation work, not simulator calls. The `_from_outputs_with_bootstrap` variant
accepts outputs you already saved. A separate fork makes the resampling stream
explicit without advancing its parent.

These intervals describe resampling uncertainty, not model error. Row bootstrap
intervals on an unscrambled QMC design are a diagnostic; they are not a calibrated
QMC error guarantee. Compare results at $N$ and $2N$ as well. Large offsets in
outputs can also affect numerical accuracy; inspect the reported variance.

## Variations

- **Latin hypercube:** replace the sampler with
  `salib::samplers::LhsSampler::classic(2 * problem.dim())`. The rest of the
  design and estimation code stays the same. Unlike unscrambled Sobol', LHS
  consumes `RngState`; use distinct seeds to assess stability between designs.
- **More samples:** try powers of two such as 4096, 8192, and 16384 for Sobol'
  sampling. The current default skips the origin; this is not a scrambled net,
  and powers of two alone do not guarantee a particular convergence rate.
- **Pairwise indices:** pass `true` to `build_saltelli_matrix`. Saltelli’s
  `second_order` field uses an upper triangle:
  `second_order[i][k]` represents the pair $(i,i+k+1)$. The budget becomes
  $N(2d+2)$, or 65,536 calls at $N=8192,d=3$.
- **Other estimators:** Jansen and Janon reuse the Saltelli design and return first-order
  estimates; use Saltelli for total-effect indices. Owen needs its
  own three-vector design. See [method selection](choosing.md).

Continue with [variance-based methods](methods/variance-based.md) for formulas,
[the crate map](crates.md) for dependency choices, or
[internals](internals.md) for the exact bit-reproducibility contract.
