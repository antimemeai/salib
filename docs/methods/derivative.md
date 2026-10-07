# Derivative-based methods

DGSM averages squared model derivatives and uses them to bound total-effect
Sobol' indices. It is useful for screening when gradients are available from
analytic formulas, an adjoint solver, or finite differences.

## DGSM

Sobol' & Kucherenko (2009) *Math. Comp. Sim.* 79(10), 3009--3017. [paper](https://doi.org/10.1016/j.matcom.2009.01.023) · [reference](../bibliography.md#sobol-kucherenko2009)

### Theory

For a square-integrable model $f(\mathbf{x})$ with independent inputs and square-integrable weak partial derivatives, the **Derivative-based Global Sensitivity Measure** for factor $X_i$ is the expected squared partial derivative:

$$\nu_i = \mathbb{E}\!\left[\left(\frac{\partial f}{\partial x_i}\right)^2\right]$$

The Poincare inequality links $\nu_i$ to the total-effect Sobol' index. For an input $X_i$ with distribution $\mu_i$ and Poincare constant $C_P(\mu_i)$:

$$S_{Ti} \leq \frac{C_P(\mu_i) \cdot \nu_i}{\operatorname{Var}(Y)}$$

The inequality bounds the population quantities under its assumptions. The
estimator substitutes sampled gradients and an estimated output variance, so
its result also has sampling and numerical error. A small bound can support
screening a factor out; a large bound may simply be loose.

### Poincare constants

Per Roustant, Barthe & Iooss (2017), the closed-form constants for common distributions are:

| Distribution | $C_P$ |
|---|---|
| $\operatorname{Uniform}[a, b]$ | $(b - a)^2 / \pi^2$ |
| $\mathcal{N}(\mu, \sigma^2)$ | $\sigma^2$ |

Use `poincare_constant` for uniform and normal distributions. For other
distributions it returns `PoincareError::Unsupported`; supply a suitable
constant directly to `estimate_dgsm`.

### Gradient computation

`estimate_dgsm` takes a pre-computed gradient matrix. The caller chooses the gradient source:

- **Analytical** — evaluate a closed-form derivative.
- **Adjoint** — obtain gradients from your model solver.
- **Finite difference** — call `finite_difference_gradients`:
  - `FdKind::Forward`: $(f(x + \varepsilon e_i) - f(x)) / \varepsilon$, $O(\varepsilon)$ error, $N(d+1)$ model evaluations including the base values.
  - `FdKind::Central`: $(f(x + \varepsilon e_i) - f(x - \varepsilon e_i)) / (2\varepsilon)$, $O(\varepsilon^2)$ error, $2Nd$ model evaluations.

Choose step sizes for the input scale and check several values: smaller steps reduce truncation error but amplify rounding and model noise. The stated error orders require sufficient smoothness.

### Code

```rust
use salib::estimators::{
    estimate_dgsm, finite_difference_gradients,
    poincare_constant, FdKind,
};
use salib::{Distribution, tree_var};
use salib::samplers::{LhsSampler, Sampler};
use ndarray::Array2;

// Sample inputs (N = 4096, d = 3, Uniform[-pi, pi])
let dist = Distribution::Uniform { lo: -std::f64::consts::PI, hi: std::f64::consts::PI };
let cp = poincare_constant(&dist).unwrap();   // (2*pi)^2 / pi^2 = 4.0

// Compute gradients via central finite-difference
let gradients = finite_difference_gradients(
    x.view(), 1e-5, FdKind::Central, |xi| model(xi),
);

let var_y = tree_var(&y);
let indices = estimate_dgsm(
    gradients.view(),
    &[cp, cp, cp],   // one Poincare constant per factor
    var_y,
).unwrap();

println!("{indices}");
// Factor   nu     ST_upper
//      0  7.7721    2.1820
//      1 24.5001    6.8770
//      2 10.9184    3.0650
```

Example results against Ishigami closed-form ($N = 4096$, seed `[0u8; 32]`, central FD with $\varepsilon = 10^{-5}$):

| Factor | $\hat{\nu}_i$ | Analytic $\nu_i$ | $\hat{S}_{Ti}^{\text{upper}}$ | Analytic $S_{Ti}$ |
|--------|--------------|------------------|-------------------------------|-------------------|
| $x_1$  | 7.7721       | 7.72             | 2.1820                        | 0.5576            |
| $x_2$  | 24.5001      | 24.50            | 6.8770                        | 0.4424            |
| $x_3$  | 10.9184      | 10.99            | 3.0650                        | 0.2437            |

All estimated bounds exceed the analytic total effects in this example, but
are too loose to identify a negligible factor.

## Choosing DGSM vs Sobol'

| Criterion | DGSM | Sobol' (Saltelli) |
|---|---|---|
| Output | Upper bound on $S_{Ti}$ | Direct estimate of $S_i$, $S_{Ti}$ |
| Cost | $N(d+1)$ to $2Nd$ (FD) | $N(d+2)$ |
| Gradient needed | Yes | No |
| Interactions | Bound only | Full decomposition |
| Input assumption here | Independent inputs with suitable Poincaré constants | Independent inputs |

Use DGSM to screen factors whose estimated bounds are small relative to your
tolerance, accounting for gradient and sampling error. A bound above the
tolerance does not establish importance; use a direct variance-based estimate
if you need a more precise answer.
