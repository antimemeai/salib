# Distribution-based methods

These methods measure how the output distribution changes when an input is
conditioned on. Borgonovo compares densities, PAWN compares CDFs, and QOSA
examines a chosen quantile. All three accept aligned input/output observations,
including saved model runs.

<a id="borgonovo-delta"></a>

## Borgonovo $\delta$

Borgonovo (2007) *Rel. Eng. Sys. Safety* 92(6), 771--784. [paper](https://doi.org/10.1016/j.ress.2006.04.015) · [reference](../bibliography.md#borgonovo2007)

Estimated with class-conditional KDEs based on Plischke, Borgonovo and Smith (2013). [paper](https://doi.org/10.1016/j.ejor.2012.11.047) · [reference](../bibliography.md#plischke2013)

### Definition

The moment-independent importance measure $\delta_i$ is the expected $L^1$ distance between the unconditional output density $f_Y$ and the conditional density $f_{Y|X_i}$:

$$\delta_i = \frac{1}{2}\,\mathbb{E}_{X_i}\!\left[\int \big|f_Y(y) - f_{Y|X_i}(y)\big|\,dy\right]$$

When the required densities exist, $\delta_i \in [0,1]$ and equals zero exactly
when $Y$ and $X_i$ are independent. Unlike Sobol' indices, $\delta$ responds to *any* distributional change — location, scale, shape, modality — not just variance.

### Algorithm

The implementation uses the following class count and quadrature settings:

1. Normalize the output to $[0,1]$ and build the unconditional density $\hat{f}_Y$ via Gaussian KDE with Silverman's bandwidth. A common affine output transformation leaves the integrated density distance unchanged and avoids overflowing bandwidth arithmetic.
2. Partition $X_i$ into approximately equal-frequency classes, keeping tied values together. $M = \min\!\big(\lceil N^{\text{exp}}\rceil,\, 48\big)$ where $\text{exp} = 2 / (7 + \tanh((1500 - N)/500))$.
3. For each class $j$: build conditional $\hat{f}_{Y|\text{class}_j}$ via KDE on $Y$ values in the class; integrate $|f_Y - f_{Y|\text{class}_j}|$ by trapezoidal rule over a 100-point grid.
4. $\hat{\delta}_i = \sum_j \frac{n_j}{2N} \int |\hat{f}_Y - \hat{f}_{Y|\text{class}_j}|\,dy$.

### Code

```rust
use salib::estimators::estimate_borgonovo_delta;
use ndarray::Array2;

// x: (N, d) input matrix, y: N-element output vector
let indices = estimate_borgonovo_delta(x.view(), &y).unwrap();

// indices.delta[i] is δ for factor i
println!("{indices}");
```

This implementation uses the uncorrected class-conditional density estimate.
It does not implement the bias-reducing bootstrap procedure described by
[Plischke et al. (2013)](https://doi.org/10.1016/j.ejor.2012.11.047).
Its 48-class cap and fixed 100-point integration grid limit resolution even
with more observations; no general consistency claim is made for these settings.
Ties are assigned by the midpoint of their sorted rank block, and empty classes
are omitted. A constant input uses one unconditional class and gives zero.

## PAWN

Pianosi & Wagener (2015) *Env. Mod. Soft.* 67, 1--11. [paper](https://doi.org/10.1016/j.envsoft.2015.01.004) · [reference](../bibliography.md#pianosi2015)

Pianosi and Wagener (2018) introduced estimation from a generic input-output
sample. [paper](https://research-information.bris.ac.uk/ws/portalfiles/portal/175128594/Pianosi_PAWN_givendata_final.pdf) · [reference](../bibliography.md#pianosi2018)

### Definition

This implementation conditions on $X_i$ using up to $S$ approximately
equal-frequency slices that keep tied values together, then measures the Kolmogorov–Smirnov distance between the unconditional
CDF $F_Y$ and the conditional CDF $F_{Y|X_i \in \text{slice}_k}$.
The 2018 paper uses equal-width intervals in §2.3, Eq. (5); rank slicing is
salib's partition choice.

$$\text{KS}_k = \sup_y \big|F_Y(y) - F_{Y|\text{slice}_k}(y)\big|$$

The per-factor PAWN index summarizes the nonempty slice-wise KS distances.
[Pianosi and Wagener (2015), §3.4](https://doi.org/10.1016/j.envsoft.2015.01.004),
recommend the median, with the maximum as complementary information:

$$T_i^{\text{(median)}} = \text{median}_k\,\text{KS}_k, \qquad T_i^{\text{(max)}} = \max_k\,\text{KS}_k$$

The estimator also returns `mean`, `minimum`, and `cv` (coefficient of variation) across slices.

### Comparing CDFs

PAWN computes empirical CDFs from sorted samples. It needs a conditioning
slice count, but no density kernel, bandwidth, or integration grid. Its KS
statistic measures the largest CDF difference, so it responds differently from
Borgonovo's integrated density difference.

### Code

```rust
use salib::estimators::estimate_pawn;

// n_slices: number of conditioning slices; check sensitivity to this choice.
let indices = estimate_pawn(x.view(), &y, 10).unwrap();

// indices.median[i], indices.maximum[i], etc.
println!("{indices}");
```

PAWN index values change with the slice count, even when rankings remain
stable. Tied values are assigned by the midpoint of their sorted rank block.
Empty slices are omitted, so discrete inputs can yield fewer slices than
requested. A constant input gives zero. The estimator requires $N \geq 2S$, but that check does not establish accuracy.
[Pianosi and Wagener (2018), §4](https://research-information.bris.ac.uk/ws/portalfiles/portal/175128594/Pianosi_PAWN_givendata_final.pdf),
suggest starting with ten intervals and checking nearby counts, along with
smaller subsamples of the data. Their example sample sizes are not general
accuracy thresholds.

## QOSA

Fort, Klein & Rachdi (2016) *Comm. Stat. Theory Methods* 45(15), 4349--4364. [paper](https://arxiv.org/abs/1305.2329) · [reference](../bibliography.md#fort2016)

The implementation uses empirical quantile-loss reduction within input classes,
based on the population contrast in
[Maume-Deschamps and Niang (2018), Eq. (2.3)](https://arxiv.org/pdf/1702.00925).
Their §4 estimator uses kernel conditional quantiles and independent fitting
and evaluation samples; salib does not implement that estimator.

### Definition

Quantile-oriented sensitivity analysis measures input importance at a chosen
output quantile. For example, use $\alpha=0.95$ to study the factors affecting
a latency distribution's 95th percentile. This is different from measuring
the probability of exceeding a fixed threshold.

Define the quantile loss as
$\psi_\alpha(y,\theta)=(y-\theta)(\alpha-\mathbf{1}_{y\leq\theta})$.
The population index is the relative reduction in optimal expected loss after
conditioning on $X_i$:

$$S_i^\alpha=1-\frac{E[\min_\theta E[\psi_\alpha(Y,\theta)\mid X_i]]}
{\min_\theta E[\psi_\alpha(Y,\theta)]}.$$

This requires a finite first moment and a positive denominator. The following
tail-expectation form also requires the relevant strict tails to have
probability $1-\alpha$; atoms at conditional quantiles can invalidate it.

$$S_i^\alpha = 1 - \frac{\mathbb{E}\!\big[Y \mid Y > F_{Y|X_i}^{-1}(\alpha)\big] - \mathbb{E}[Y]}{\text{CTE}_\alpha(Y) - \mathbb{E}[Y]}$$

where $\text{CTE}_\alpha(Y) = \mathbb{E}[Y \mid Y > F_Y^{-1}(\alpha)]$ is the tail mean above the $\alpha$-quantile.

$S_i^\alpha \in [0, 1]$. It equals zero if $Y \perp X_i$ (the factor has no influence on the $\alpha$-quantile) and one if $Y$ is $X_i$-measurable (the factor fully determines the output).

### Partition-based estimator

salib minimizes empirical pinball loss globally and within input classes,
using the same observations for fitting and evaluation:

1. Sort $Y$ and take the $\lceil\alpha N\rceil$-th value as the global quantile $\hat q$.
2. Compute the mean global loss $\hat L=N^{-1}\sum_j\psi_\alpha(Y_j,\hat q)$.
3. For each factor, form approximately equal-frequency classes without splitting ties. Compute each class's empirical quantile $\hat q_k$.
4. Return $\hat S_i^\alpha=1-\sum_k\sum_{j\in k}\psi_\alpha(Y_j,\hat q_k)/(N\hat L)$.

This formulation supports atoms and preserves output translation and positive
scaling, subject to floating-point rounding. The diagnostic `global_cte` is
empirical expected shortfall,
$\hat q+N^{-1}\sum_j(Y_j-\hat q)_+/(1-\alpha)$; it includes fractional mass
at an atomic quantile. `global_loss` reports $\hat L$.

### Code

```rust
use salib::estimators::estimate_qosa;

// alpha: quantile level in (0, 1)
// 0.5 = median, 0.95 = tail, 0.99 = extreme tail
let indices = estimate_qosa(x.view(), &y, 0.95).unwrap();

// indices.s[i] is S^alpha for factor i
// indices.alpha, indices.global_quantile, indices.global_cte, indices.global_loss
// record the quantile level and fitted loss
println!("{indices}");
```

A synthetic test in `qosa_e2e.rs` uses independent unit-uniform inputs and
$Y=X_1+8X_2\mathbf{1}_{X_3>0.95}$. It checks that $X_1$ leads at the median
and that $X_3$ exceeds $X_1$ at the 0.95 quantile. This is a qualitative test,
not an analytic benchmark for QOSA values.

Changing $\alpha$ lets you compare input importance at the median and in
the tail.

The requested class count is capped at 48; tied blocks use their sorted
rank midpoint, and empty classes are omitted. This leaves conditioning error
even as the sample grows. Fitting and evaluating each quantile on the same
observations can also bias the loss reduction upward. A constant output has
zero loss and returns `QosaError::ZeroVariance`; a constant input gives zero.
The reported indices are clamped to $[0,1]$ for rounding, which does not establish
accuracy. Inspect per-class sample sizes, especially for extreme quantiles,
and check sample-size sensitivity. The paper's consistency result does not
cover this implementation.

## Choosing among distribution-based methods

| Method       | Measures                     | Internals             | Best for                                          |
|--------------|------------------------------|-----------------------|---------------------------------------------------|
| Borgonovo $\delta$ | Full density shift     | KDE + trapezoidal integration | Detecting any distributional change, including shape and modality. |
| PAWN         | CDF shift (KS distance)     | Empirical CDFs and conditioning slices | Comparing distributions without density estimation. |
| QOSA         | Quantile-specific sensitivity | Partition + pinball loss       | Tail-focused analysis: which factor drives the 95th percentile?    |

Use PAWN for CDF differences without fitting a density, Borgonovo $\delta$
for density differences, and QOSA for a particular quantile. Choose the measure
that matches the question, then check sample-size and partition sensitivity.

These measures can be used alongside Sobol' indices when you need to study
distributional changes as well as variance contributions.
