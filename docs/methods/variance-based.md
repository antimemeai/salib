# Variance-based methods

Sobol' sensitivity indices partition output variance for independent inputs with
finite, nonzero variance. Report main and total effects together when interactions
matter; a main effect near zero does not establish that an input is irrelevant.

## Theory

For independent inputs, the functional decomposition gives

$$\operatorname{Var}(Y)=\sum_i V_i+\sum_{i<j}V_{ij}+\cdots.$$

The main-effect and total-effect indices are

$$S_i=\frac{\operatorname{Var}(E[Y\mid X_i])}{\operatorname{Var}(Y)},\qquad
S_{Ti}=\frac{E[\operatorname{Var}(Y\mid X_{-i})]}{\operatorname{Var}(Y)}.$$

$S_{Ti}-S_i$ is the sum of interaction shares involving factor $i$, not a
specific pairwise index. First-order indices sum to one only for an additive
model. Total effects count interactions more than once.

## Sampling design

Saltelli, Jansen, and Janon use `SaltelliMatrix`: $A$, $B$, and $A_B^{(i)}$,
where $A_B^{(i)}$ replaces column $i$ of $A$ with column $i$ of $B$. The base
sampler needs $2d$ columns. The matrices are in the **unit cube**; defining a
`Problem` alone does not map distributions. Owen requires its own $3d$ design.

The following setup is shared by the estimator examples below:

```rust
use std::f64::consts::PI;
use salib::RngState;
use salib::samplers::{SobolSampler, build_saltelli_matrix};

let mut rng = RngState::from_seed([0; 32]);
let sampler = SobolSampler::minimal(6); // Two three-factor base matrices.
let saltelli = build_saltelli_matrix(&sampler, 8192, false, &mut rng).unwrap();
let ishigami_unit = |u: &[f64]| {
    let x = [2.0 * PI * u[0] - PI, 2.0 * PI * u[1] - PI, 2.0 * PI * u[2] - PI];
    x[0].sin() + 7.0 * x[1].sin().powi(2) + 0.1 * x[2].powi(4) * x[0].sin()
};
```

The basic design costs $N(d+2)$ calls. `false` disables **second-order blocks**;
it is unrelated to skipping Sobol' points. Enabling them costs $N(2d+2)$.
Choose sample size by convergence at $N$ and $2N$, not by a universal minimum.
Plain Monte Carlo standard error typically decreases as $N^{-1/2}$ under its
usual assumptions; QMC behavior depends on the integrand and point design.
The current Sobol' sampler is unscrambled and defaults to skipping the origin.
Dropping points can impair net balance; see
[Owen (2020)](https://arxiv.org/abs/2008.08051). There is no independent scrambled
replicate API in this release.

## Saltelli 2010

[Saltelli et al. (2010)](https://doi.org/10.1016/j.cpc.2009.09.018).

With $D$ the estimated output variance, the implementation uses

$$\hat S_i=\frac{N^{-1}\sum_j f(B)_j[f(A_B^{(i)})_j-f(A)_j]}{D},\qquad
\hat S_{Ti}=\frac{(2N)^{-1}\sum_j[f(A)_j-f(A_B^{(i)})_j]^2}{D}.$$

```rust
use salib::estimators::estimate_saltelli2010;
let indices = estimate_saltelli2010(&saltelli, ishigami_unit);
println!("{indices}");
```

For canonical Ishigami, analytic first-order targets are
`[0.3139, 0.4424, 0.0000]`; total-effect targets are
`[0.5576, 0.4424, 0.2437]`. Estimates fluctuate with design and size. The
[quickstart](../quickstart.md) explains interpretation and bootstrap intervals.
Use `_from_outputs` for saved `fa`, `fb`, and `fab[i]`, preserving row alignment.

With second-order blocks, `second_order[i][k]` denotes $(i,i+k+1)$ in the
upper triangle. Computing these pairwise estimates adds $O(Nd^2)$ analysis work.

## Jansen 1999

[Jansen (1999)](https://doi.org/10.1016/S0010-4655%2898%2900154-4).

The implemented first-order complement is

$$\hat S_i=1-\frac{(2N)^{-1}\sum_j[f(B)_j-f(A_B^{(i)})_j]^2}{D}.$$

`B` and `A_B^{(i)}` share input $i$, so their squared difference measures the
remaining contribution. The public `JansenIndices` has **no total-effect vector**.
Use Saltelli’s squared-difference total effect above alongside it.

```rust
use salib::estimators::estimate_jansen;
let indices = estimate_jansen(&saltelli, ishigami_unit);
println!("S1: {:?}", indices.first_order);
```

## Janon 2014

[Janon et al. (2014)](https://doi.org/10.1051/ps/2013040).

The symmetrized pick-freeze estimator pairs $Y=f(B)$ with
$Y^{(i)}=f(A_B^{(i)})$, which share input $i$. Let
$\bar Y_2=(\bar Y+\bar Y^{(i)})/2$. Then

$$\hat S_i=\frac{N^{-1}\sum_jY_jY_j^{(i)}-\bar Y_2^2}
{(2N)^{-1}\sum_j(Y_j^2+(Y_j^{(i)})^2)-\bar Y_2^2}.$$

Janon et al., Propositions 3.2–3.5, establish asymptotic results for i.i.d.
exchangeable pick-freeze pairs with a finite fourth output moment. They do not
establish finite-sample superiority or apply automatically to deterministic QMC. The result returns
first-order indices and optional pairwise indices, without a total-effect vector.

```rust
use salib::estimators::estimate_janon;
let indices = estimate_janon(&saltelli, ishigami_unit);
```

## Owen 2013

[Owen (2013)](https://doi.org/10.1145/2457459.2457460).

Owen’s Correlation 2 estimator targets small **first-order** indices with a
three-vector design. `OwenMatrix` stores $A,B,C$, $A_C^{(i)}$, and $B_A^{(i)}$.
The current estimator returns first-order indices; `second_order` remains `None`.

```rust
use salib::samplers::build_owen_matrix;
use salib::estimators::estimate_owen;
let sampler = SobolSampler::minimal(9); // 3d.
let owen = build_owen_matrix(&sampler, 8192, &mut rng).unwrap();
let indices = estimate_owen(&owen, ishigami_unit);
```

The estimator currently evaluates $A,B$ and both hybrid families, costing
$N(2+2d)$ calls. The design’s `total_evaluations()` conservatively includes $C$
as well, reporting $N(3+2d)$. Do not pass a Saltelli design to this API.

## Given-data Sobol'

[Plischke et al. (2013)](https://doi.org/10.1016/j.ejor.2012.11.047).

When rerunning the model is impossible, partition the existing data by each input
and estimate conditional means. This provides approximate first-order indices,
without total effects or the paired design assumptions of the methods above.

```rust
use salib::estimators::estimate_given_data_sobol;
// x: aligned (N,d) input matrix; y: N outputs, in physical units.
let indices = estimate_given_data_sobol(x.view(), &y).unwrap();
```

The implementation caps the partition at 48 classes. More rows improve
estimates within those classes but do not remove the conditioning error.
For example, a factor whose effect oscillates within each class may appear
unimportant even when it determines the output. Tied inputs are split by row
order, so discrete-input results may also depend on row ordering. Under input
dependence, the measure combines model response with input association. Shapley
theory permits dependent-input attribution, but salib’s current Shapley API only
supports independent inputs.

## Choosing among estimators

| Estimator | First-order | Total-effect vector | Sampling layout |
|---|---|---|---|
| Saltelli 2010 | yes | yes | Saltelli, $2d$ sampler |
| Jansen 1999 | yes | no | Same Saltelli layout |
| Janon 2014 | yes | no | Same Saltelli layout |
| Owen 2013 | yes | no | Owen, $3d$ sampler |
| Given-data | approximate | no | Arbitrary aligned $(X,Y)$ observations |

Start with Saltelli when main and total effects are needed. Comparing first-order
formulas on the same design can expose finite-sample instability. See
[method selection](../choosing.md) for budgets, assumptions, and alternatives.
