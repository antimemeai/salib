# Game-theoretic methods

Shapley effects allocate output variance, including interactions, among inputs.
salib supports independent inputs. Its `sh` vector is in output-variance units;
divide by `var_y` for dimensionless shares.

The theory also covers dependent inputs with appropriate conditional sampling,
which this implementation does not provide. The population identities below
do not guarantee the same ordering for finite-sample estimates.

## Shapley Effects

Song, Nelson & Staum (2016) *SIAM/ASA J. Unc. Quant.* 4(1), 1060–1083. [paper](https://users.iems.northwestern.edu/~nelsonb/Publications/SongNelsonStaum.pdf) · [reference](../bibliography.md#song2016)

### Theory

Shapley values originate in cooperative game theory (Shapley 1953). Applied to sensitivity analysis, the "game" is the variance of $Y = f(\mathbf{X})$ and each input factor $X_i$ is a player. The Shapley effect of factor $i$ averages its marginal variance contribution over all possible coalitions:

$$\text{Sh}_i = \frac{1}{d} \sum_{S \subseteq \{1,\ldots,d\} \setminus \{i\}} \binom{d-1}{|S|}^{-1} \big[c(S \cup \{i\}) - c(S)\big]$$

where $c(S) = \mathbb{E}_{X_{-S}}\!\big[\operatorname{Var}_{X_S}(Y \mid X_{-S})\big]$ is the expected conditional variance when the factors in $S$ are left free. The boundary conditions are $c(\varnothing) = 0$ and $c(\{1,\ldots,d\}) = \operatorname{Var}(Y)$.

The defining property (Song 2016 Eq 10):

$$\sum_{i=1}^{d} \text{Sh}_i = \operatorname{Var}(Y)$$

This is an identity for unnormalized population Shapley effects, including
dependent-input games with the correct conditional law. For independent
inputs, splitting nonnegative interaction components equally implies:

$$V_i \leq \text{Sh}_i \leq V_{T_i}$$

where $V_i = \operatorname{Var}\!\big[\mathbb{E}(Y \mid X_i)\big]$ is the first-order variance contribution and $V_{T_i} = \mathbb{E}\!\big[\operatorname{Var}(Y \mid X_{-i})\big]$ is the total-effect variance contribution. For independent inputs, Shapley divides each interaction contribution equally among the factors involved.

### Algorithm

Enumerating all coalitions and permutations becomes expensive as $d$ grows.
The implementation uses Song 2016 Algorithm 1:

1. **Random-permutation sampling** (Castro-Gomez-Cazorla 2009): sample $m$ random permutations $\pi_1, \ldots, \pi_m$ of $\{1,\ldots,d\}$ and accumulate marginal contributions $\hat{\Delta}_{\pi(j)} = \hat{c}(\text{prefix}_j) - \hat{c}(\text{prefix}_{j-1})$ along each permutation.

2. **Sequential cost reuse** (Song 2016 Section 4.1): walk $j = 1 \ldots d$ along each permutation, caching $\hat{c}(\text{prefix}_{j-1})$ from the previous step. Halves the cost-evaluation budget compared to independent coalition estimates.

3. **Double-loop Monte Carlo** for each $\hat{c}(J)$: $N_O$ outer samples of $X_{-J}$ and $N_I$ inner samples of $X_J \mid X_{-J}$. The boundary $\hat{c}(\{1,\ldots,d\}) = \widehat{\operatorname{Var}}(Y)$ uses a dedicated variance block of $N_V$ samples.

Total budget: $N_V + m \cdot N_I \cdot N_O \cdot (d - 1)$ model evaluations.

### Code

```rust
use salib::shapley::estimate_shapley;
use salib::{Distribution, RngState};

let distributions = vec![
    Distribution::Uniform { lo: -3.14159, hi: 3.14159 },
    Distribution::Uniform { lo: -3.14159, hi: 3.14159 },
    Distribution::Uniform { lo: -3.14159, hi: 3.14159 },
];

let mut rng = RngState::from_seed([0u8; 32]);

let result = estimate_shapley(
    &distributions,
    |x| x[0].sin() + 7.0 * x[1].sin().powi(2) + 0.1 * x[2].powi(4) * x[0].sin(),
    500,   // n_perm (m)
    1,     // n_outer (N_O)
    3,     // n_inner (N_I)
    4000,  // n_var (N_V)
    &mut rng,
).unwrap();

println!("{result}");
```

For randomly sampled permutations, [Song et al., Appendix B, Claim 2](https://users.iems.northwestern.edu/~nelsonb/Publications/SongNelsonStaum.pdf),
recommend $N_I=3$, $N_O=1$, and spending the remaining budget on $m$.
Choose $N_V$ separately by checking stability of the variance estimate.
Theorem 3's variance bound assumes exact coalition costs; it does not bound
the error of this nested Monte Carlo implementation.

Each sampled permutation telescopes to the same estimated total variance.
Thus the sum of salib's contributions differs from `var_y` through rounding
and tiny-negative clamping, not Monte Carlo sampling error. Individual
contributions still have sampling error.

### Additive-model example

Linear-additive model: $Y = X_1 + 2X_2 + 3X_3$, $X_i \sim \mathcal{N}(0, 1)$ independent. Analytic: $\operatorname{Var}(Y) = 14$, $\text{Sh}_i = a_i^2$ (no interactions, so $\text{Sh}_i = V_i = V_{T_i}$).

| Factor | Analytic $\text{Sh}_i$ | Estimated ($m = 500$, seed `[42; 32]`) |
|--------|------------------------|----------------------------------------|
| $X_1$  | 1.0                    | within 5% MC tolerance                |
| $X_2$  | 4.0                    | within 5% MC tolerance                |
| $X_3$  | 9.0                    | within 5% MC tolerance                |
| $\sum$ | 14.0                   | $\approx \widehat{\operatorname{Var}}(Y)$ |

Dependent-input Shapley effects would require conditional sampling, which is
not implemented.
