# Regression methods

Regression coefficients and partial correlations describe input/output
associations on the original data or on ranks. Use them as an initial analysis
when the response is approximately linear or monotonic. Check the fit and
residuals before interpreting the coefficients.

## Theory

### SRC — Standardized Regression Coefficients

Saltelli & Marivoet (1990) *Reliability Engineering & System Safety* 28(2), 229–253. [paper](https://doi.org/10.1016/0951-8320%2890%2990065-U) · [reference](../bibliography.md#saltelli-marivoet1990)

Fit an OLS regression $Y \approx \beta_0 + \boldsymbol{\beta} \cdot \mathbf{X}$ and standardize each coefficient by the input/output standard deviations:

$$\text{SRC}_i = \beta_i \cdot \frac{\sigma_{X_i}}{\sigma_Y}$$

For a linear model with independent inputs, squared standardized coefficients
correspond to first-order Sobol' indices. With sampled data,
$\text{SRC}_i^2 \approx S_i$. Check $R^2$ and residuals to judge how well the
linear model represents the response; an $R^2$ cutoff alone cannot establish
that its sensitivity estimates are adequate.

### SRRC — Standardized Rank Regression Coefficients

Replace both $\mathbf{X}$ and $Y$ with their ranks, averaging occupied ranks for tied values, then compute SRC
on the transformed data. This can capture monotonic nonlinear associations.
Inspect the rank regression's $R^2$ and residuals as well.

Tied values receive average ranks, as described in
[Marino et al. (2008), footnote 3](https://pmc.ncbi.nlm.nih.gov/articles/PMC2570191/).
The balanced binary fixture in
`cargo run -p salib-estimators --example audit_regression_ties` checks that
independent inputs and outputs have zero association after joint row permutations.

### PCC — Partial Correlation Coefficients

For each factor $X_i$, regress $X_i$ on all other factors and $Y$ on all other factors. The Pearson correlation of the residuals is the partial correlation:

$$\text{PCC}_i = \operatorname{corr}\!\big(X_i - \hat{X}_i^{(\sim i)},\; Y - \hat{Y}^{(\sim i)}\big)$$

PCC measures the remaining linear association after residualizing both
variables against the other inputs. Strong collinearity can make that
calculation unstable or singular. It does not turn association into a causal
effect or a variance share.

### PRCC — Partial Rank Correlation Coefficients

PRCC applies partial correlation to rank-transformed data to measure monotonic
association after accounting for the other factors.

## Code

`estimate_regression_indices` returns all four indices plus both $R^2$ diagnostics in a single call. It takes an aligned $(X,Y)$ dataset.

```rust
use salib::estimators::estimate_regression_indices;
use ndarray::Array2;

// x: (N, d) input matrix, y: N-element output vector
let indices = estimate_regression_indices(x.view(), &y).unwrap();

println!("{indices}");
```

For an exact linear model $Y=2X_0+X_1$ with equally variable independent inputs,
SRC has a coefficient ratio of two, and an absent factor has coefficient zero.
These are model identities; finite-sample input variances determine the precise
standardized coefficients. Nonmonotonic responses, such as the Ishigami model's
$\sin^2(x_2)$ term, can have substantial variance contributions despite weak
linear and rank associations.

## When to use each variant

| Index | Captures | Diagnostic | Use for |
|---|---|---|---|
| SRC | Linear effects | $R^2_{\text{linear}}$ and residuals | Additive linear models |
| SRRC | Monotonic effects | $R^2_{\text{rank}}$ and rank residuals | Monotonic nonlinearities |
| PCC | Linear partial contribution | $R^2_{\text{linear}}$ and residuals | Correlated inputs — isolates individual factors |
| PRCC | Monotonic partial contribution | $R^2_{\text{rank}}$ and rank residuals | Monotonic + correlated inputs |

All four use existing input/output data. This implementation refits two
regressions per factor for each partial-correlation calculation, costing
$O(Nd^3+d^4)$ overall, plus rank sorting. Rank-based indices describe
associations on transformed data; they are not generally variance shares of
the original output.

## Workflow

1. Run `estimate_regression_indices` on the existing $(X,Y)$ data.
2. Inspect $R^2$ and residuals for both fits.
3. If a fit is adequate for your purpose, compare the corresponding coefficient
   magnitudes and signs.
4. If both fits are poor, use variance-based or distribution-based methods to
   study effects the regressions miss.
