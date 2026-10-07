# Surrogate methods

Fit a polynomial approximation and compute its Sobol' indices from the
coefficients. This avoids further model evaluations for the index calculation.
The result describes the fitted approximation, so check its predictions on
held-out data before using the indices to interpret the original model.

PCE is useful when model runs are expensive, the response is smooth, and a
manageable number of polynomial terms fits it well.

## Polynomial Chaos Expansion

Xiu & Karniadakis (2002) *SIAM J. Sci. Comp.* 24(2), 619--644 (generalized PCE). [paper](https://www.sci.utah.edu/~dxiu/Papers/XiuK_SISC02.pdf) · [reference](../bibliography.md#xiu-karniadakis2002)

Blatman & Sudret (2011) *J. Comp. Phys.* 230(6), 2345--2367 (sparse PCE). [paper](https://doi.org/10.1016/j.jcp.2010.12.021) · [reference](../bibliography.md#blatman-sudret2011)

### Theory

Approximate the model output as a truncated series in orthogonal polynomial basis functions:

$$f(\mathbf{x}) \approx \sum_{|\alpha| \leq p} c_\alpha \, \Psi_\alpha(\boldsymbol{\xi})$$

where $\boldsymbol{\xi}$ is the input mapped to each factor's polynomial-canonical domain, $\alpha$ is a multi-index, and $\Psi_\alpha(\boldsymbol{\xi}) = \prod_k \Psi_{\alpha_k}(\xi_k)$ is the tensor-product basis. The polynomial families are matched to the input distributions per the Wiener-Askey scheme:

| Input distribution | Polynomial family | Canonical domain |
|---|---|---|
| $\operatorname{Uniform}[a, b]$ | Legendre | $[-1, 1]$ |
| $\mathcal{N}(\mu, \sigma^2)$ | Hermite | $\mathbb{R}$ |

Coefficients $\{c_\alpha\}$ are fitted via OLS on the $(N, P)$ basis matrix, where $P = \binom{d + p}{p}$ is the number of basis terms at total degree $p$.

### Sobol' indices from coefficients

For independent inputs and a basis orthogonal under their probability measure,
[Sudret (2008), §5.4, Eqs. (51) and (53)](https://doi.org/10.1016/j.ress.2007.04.002),
computes Sobol' indices by grouping squared coefficients and basis norms:

$$S_i = \frac{\sum_{\alpha \in \mathcal{A}_i} c_\alpha^2 \, \langle \Psi_\alpha, \Psi_\alpha \rangle}{\sum_{\alpha \neq 0} c_\alpha^2 \, \langle \Psi_\alpha, \Psi_\alpha \rangle}$$

where $\mathcal{A}_i = \{\alpha : \alpha_i > 0,\; \alpha_j = 0 \;\forall j \neq i\}$ is the set of main-effect multi-indices for factor $i$. The total-order index sums over all multi-indices where factor $i$ is active:

$$S_{Ti} = \frac{\sum_{\alpha : \alpha_i > 0} c_\alpha^2 \, \langle \Psi_\alpha, \Psi_\alpha \rangle}{\sum_{\alpha \neq 0} c_\alpha^2 \, \langle \Psi_\alpha, \Psi_\alpha \rangle}$$

These formulas give the fitted expansion's indices. Truncation and coefficient
estimation error affect how well they approximate the original model's indices.

### Full OLS: `fit_full_pce`

Use when $N \gg P$. The implementation fits all $P$ terms through Cholesky
factorization of the normal equations, at cost $O(NP^2+P^3)$. Building the basis
requires $O(NPd)$ polynomial evaluations, each with degree-dependent cost.

```rust
use salib::surrogate::{
    fit_full_pce, sobol_indices_from_pce, PolynomialFamily,
};

// samples_canonical: (N, d) in [-1, 1]^d (Legendre domain)
let pce = fit_full_pce(
    samples_canonical.view(),
    &y,
    &[PolynomialFamily::Legendre; 3],
    5,   // max_degree p = 5
).unwrap();

let sobol = sobol_indices_from_pce(&pce).unwrap();
println!("{sobol}");
```

Example results for additive model $Y = \xi_0 + 2\xi_1$ over $[-1, 1]^2$ ($N = 256$, Legendre, $p = 3$):

$\operatorname{Var}(\xi) = 1/3$ for each factor. $\operatorname{Var}(Y) = 1/3 + 4/3 = 5/3$.

| Factor | $\hat{S}_i$ | Analytic $S_i$ | $\hat{S}_{Ti}$ | Analytic $S_{Ti}$ |
|--------|-------------|----------------|-----------------|-------------------|
| $\xi_0$ | 0.2000 | 0.2 | 0.2000 | 0.2 |
| $\xi_1$ | 0.8000 | 0.8 | 0.8000 | 0.8 |

Additive model: $S_{Ti} = S_i$ (no interactions). PCE recovers the exact split because a linear function is in the span of any degree-$p \geq 1$ basis.

### Sparse LARS/OMP: `fit_sparse_pce`

Use when $P$ is large and most coefficients are near zero. The solvers add terms and use leave-one-out cross-validation to choose where to stop:

- **OMP** chooses the centered, unit-length column with the largest absolute
  inner product with the residual, then refits OLS.
- **LARS** uses an equiangular selection path. It centers the response and centers
  and normalizes predictor columns as in
  [Efron et al. (2004), Eq. (1.1)](https://arxiv.org/pdf/math/0406456).

Both exclude constant predictors from selection and refit OLS with the original
polynomial columns and an intercept, preserving coefficient units. LARS admits
all correlations tied at a knot together. It stops before a tied group that
would exceed `max_terms`, rather than choosing a subset by column order.

Both retain the best visited fit and stop after three steps without improvement
in uncorrected PRESS. If any leave-one-out fit is numerically singular
(`1-h_ii <= 1e-10`), the entire candidate receives an infinite score. Fitting
returns an error if no candidate has a finite score. This is a fixed-basis
implementation, not the full adaptive procedure or corrected LOO diagnostic
of Blatman and Sudret (§4.2.3 and §5). Check held-out predictions independently.

Both return a `PolynomialChaos` that you can pass to `sobol_indices_from_pce`.
Full and sparse fitting reject nonfinite samples or outputs with
`PceError::NonFiniteInput`; unrepresentable basis or fitting arithmetic returns
`PceError::NonFiniteFit`. HDMR also rejects unrepresentable variance sums.

```rust
use salib::surrogate::{
    fit_sparse_pce, sobol_indices_from_pce,
    SparseSolver, TruncationScheme, PolynomialFamily,
};

let (pce, diagnostic) = fit_sparse_pce(
    samples_canonical.view(),
    &y,
    &[PolynomialFamily::Legendre; 5],
    4,   // max_degree
    TruncationScheme::Hyperbolic { q: 0.75 },
    SparseSolver::Omp,
    None,   // max_terms: auto
).unwrap();

let sobol = sobol_indices_from_pce(&pce).unwrap();
println!("Active terms: {} / {}", diagnostic.num_active, diagnostic.candidate_basis_size);
println!("{sobol}");
```

### Basis truncation

- **Total-degree** (`TruncationScheme::TotalDegree`): all $\alpha$ with $|\alpha| = \sum \alpha_j \leq p$. Basis size $P = \binom{d+p}{p}$.
- **Hyperbolic $q$-norm** (`TruncationScheme::Hyperbolic { q }`): $\left(\sum \alpha_j^q\right)^{1/q} \leq p$ with $q \in (0, 1]$. At $q < 1$, high-interaction terms are suppressed before the solver sees them. Blatman and Sudret illustrate $q=0.75$; choose it by testing the fit, since truncation can remove important interactions.

## HDMR

Li, Rosenthal & Rabitz (2001) *J. Phys. Chem. A* 105(33), 7765--7777. [paper](https://doi.org/10.1021/jp010450t) · [reference](../bibliography.md#li2001)

### Theory

High-Dimensional Model Representation decomposes $f$ into component functions of increasing interaction order:

$$f(\mathbf{x}) = f_0 + \sum_i f_i(x_i) + \sum_{i < j} f_{ij}(x_i, x_j) + \cdots$$

salib fits a full PCE to $(X,Y)$, then groups coefficients by their active
factor sets, following the PCE/ANOVA construction in Sudret (2008), §5.3–5.4.
This is one regression-based HDMR construction; it is not a claim to implement
every algorithm in Li et al.

The automatic mapping supports independent uniform and normal inputs.
Other distributions return `HdmrError::UnsupportedDistribution`: mapping a
triangular or Beta density affinely to Legendre coordinates leaves it nonuniform
and invalidates the variance weights. Run
`python3 scripts/check-hdmr-literature.py` to check an analytic additive-model
control and unsupported-measure rejection.

For supported input mappings, this produces the same first-order and total-order
indices as `sobol_indices_from_pce`, plus:

- **Second-order pairwise indices** $S_{ij}$ — variance attributed to the $X_i \times X_j$ interaction.
- **Per-order variance fractions** — the variance share at each interaction order.

`max_order` limits the reported `order_variance` entries. It does not remove
higher-order terms from the fit or total-effect indices, and pairwise indices
are computed even when `max_order=1`. Reported order fractions can therefore
sum to less than one.

### Code

```rust
use salib::estimators::estimate_hdmr;

let result = estimate_hdmr(
    x.view(),       // (N, d) physical-domain inputs
    &y,             // N-element model output
    &problem,       // defines factor distributions
    2,              // max_order: report fractions through order 2
    4,              // max_degree: PCE truncation degree
).unwrap();

println!("{result}");
// HDMR decomposition (d=3)
//   Var[Y] = 13.8446
//   Order 1 variance fraction: 0.7563
//   Order 2 variance fraction: 0.2437
//
//   Factor      S1        ST
//        0  0.3139    0.5576
//        1  0.4424    0.4424
//        2  0.0000    0.2437
```

Use HDMR to estimate which pairs interact and how much variance they explain. For first-order and total-order indices alone, `sobol_indices_from_pce` on a full or sparse PCE is simpler.

## Active Subspaces

Constantine (2015) *Active Subspaces: Emerging Ideas for Dimension Reduction in Parameter Studies*, SIAM. [paper](https://doi.org/10.1137/1.9781611973860) · [reference](../bibliography.md#constantine2015)

### Theory

Find the directions in input space along which $f$ varies most. Define the uncentered gradient covariance matrix:

$$C = \mathbb{E}\!\left[\nabla f \, (\nabla f)^T\right]$$

and eigendecompose $C = W \Lambda W^T$ with $\lambda_1 \geq \cdots \geq \lambda_d \geq 0$. Each eigenvalue $\lambda_i = \mathbb{E}\!\left[(({\nabla f})^T w_i)^2\right]$ is the mean-squared directional derivative along eigenvector $w_i$ ([Constantine, Dow and Wang (2014), Lemma 2.1](https://arxiv.org/pdf/1304.2070)).

The **active subspace** is the span of the leading $k$ eigenvectors — directions where the model varies most. A gap in the eigenvalue spectrum ($\lambda_k \gg \lambda_{k+1}$) suggests the approximation $f(\mathbf{x}) \approx g(W_k^T \mathbf{x})$ for some lower-dimensional function $g$. Theorem 3.1 bounds the conditional-mean
approximation error through the sum of discarded eigenvalues and a Poincaré
constant, under its assumptions. A gap ratio alone does not establish an error
bound. Specify the input distribution and coordinate scaling when interpreting
these directions.

### Monte Carlo estimator

Given $M$ gradient samples $\nabla f(x_1), \ldots, \nabla f(x_M)$ stacked into an $(M, d)$ matrix, the estimator is:

$$\tilde{C} = \frac{1}{M} \sum_{j=1}^{M} (\nabla f_j)(\nabla f_j)^T$$

Eigendecomposed via `nalgebra::SymmetricEigen` at cost $O(d^3 + M d^2)$.

### Active-dimension detection

salib's dimension-selection heuristic is $k = \operatorname{argmax}_j (\lambda_j / \lambda_{j+1})$. When $\lambda_{j+1} \leq 10^{-12} \lambda_{\max}$, the implementation selects that $k$ immediately. This covers numerically low-rank cases such as ridge functions.

Optional `gap_threshold` requires the ratio to exceed $t>1$. If no ratio
qualifies, all directions are retained. This selector and the $10^{-12}$
tolerance are implementation choices. With fewer gradient rows than dimensions,
the sample matrix is necessarily rank deficient; a numerical zero eigenvalue
need not mean the original model ignores that direction.

### Code

```rust
use salib::surrogate::compute_active_subspace;

// gradients: (M, d) matrix of sampled gradients
let result = compute_active_subspace(
    gradients.view(),
    None,   // gap_threshold: use default heuristic
).unwrap();

println!("Active dimension: {}", result.k_active);
println!("Eigenvalues: {:?}", result.eigenvalues);
// Leading eigenvectors define the active subspace:
// result.eigenvectors columns 0..k_active
```

Example results for ridge function $f(\mathbf{x}) = \mathbf{a}^T \mathbf{x}$ with $\mathbf{a} = (3, 0, 4)$ ($M = 50$):

| Eigenvalue | Value | Interpretation |
|---|---|---|
| $\lambda_1$ | 25.0 | $= \|\mathbf{a}\|^2$ |
| $\lambda_2$ | 0.0 | Numerically zero |
| $\lambda_3$ | 0.0 | Numerically zero |

$k_{\text{active}} = 1$. Leading eigenvector $= \pm \mathbf{a} / \|\mathbf{a}\|$. Rank-1 gradient covariance recovered exactly.

High-dimensional inputs ($d > 10$) where you suspect the model really depends on a few linear combinations of the inputs. Active subspaces identify those combinations and their relative importance. The same gradient samples can also be used for DGSM.

## Choosing among surrogates

| Method | Produces | Cost | Best for |
|---|---|---|---|
| Full PCE | $S_i$, $S_{Ti}$ for fitted PCE | $O(NP^2+P^3)$ plus basis construction | Moderate $d$ and $p$, smooth models, $N \gg P$ |
| Sparse PCE | $S_i$, $S_{Ti}$ for fitted PCE | Per step: $O(NP)$ scan plus $O(Nk^2+k^3)$ fit/diagnostic at active size $k$ | Large basis ($P \gg N$), sparse coefficient structure |
| HDMR | $S_i$, $S_{ij}$, $S_{Ti}$, order fractions | Full PCE + grouping | Interaction structure analysis |
| Active Subspaces | Dominant directions, eigenvalue spectrum | $O(d^3 + Md^2)$ | Dimension reduction, $d > 10$ |

Start with sparse PCE if $d \cdot p$ is large. Use full PCE when $N$ is generous relative to $P$. Add HDMR when you need pairwise interaction indices. Use active subspaces when you want to reduce the input dimension before fitting any surrogate.
