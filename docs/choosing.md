# Choosing a method

Choose the quantity you want to measure before choosing an algorithm. A factor
can have a small main effect and a large interaction effect; a change in a tail
quantile can be important even when it barely changes variance.

## Start with the data and the question

1. **Do you have only existing $(X,Y)$ observations?** Use given-data Sobol',
   PAWN, Borgonovo δ, QOSA, RBD-FAST, or regression. These consume aligned
   input/output rows. You cannot apply a designed-sample Saltelli estimator to
   arbitrary observations. Observational sensitivity is an association under the
   observed input distribution, not causal attribution.
2. **Can you run the model, but many inputs compete for a small budget?** Start
   with Morris screening. With gradients already available, DGSM can be a useful
   alternative. Follow screening with quantitative attribution for retained
   factors; fixing discarded factors changes the experiment being analyzed.
3. **Do you want shares of output variance for independent inputs?** Use Saltelli
   2010 with a Saltelli design, optionally comparing Jansen main effects. Include total effects to detect inputs
   whose importance is mostly through interactions. Enable second-order sampling
   only if you need specific pairwise interactions.
4. **Do you care about the whole output distribution?** Use PAWN for CDF changes
   or Borgonovo δ for density changes. These can detect changes that variance
   alone misses. They do not partition variance into additive shares.
5. **Do you care about a particular output quantile?** Use QOSA at the chosen
   quantile level. It measures quantile-oriented sensitivity, not exceedance
   probability directly; sparse tails need substantially more observations.
6. **Is the simulator too expensive for a large direct campaign?** Consider PCE
   if a polynomial surrogate can predict held-out outputs well. Derive indices
   from that fit only after validating approximation error under your input law.

For **dependent inputs**, ordinary independent-input Sobol' decompositions lose
their usual interpretation, and column swaps can create implausible combinations.
Shapley theory can handle dependence, but **salib's current Shapley implementation
samples independent marginals only**. It is not a correlated-input solution.
Iman–Conover can build correlated samples; it does not supply a dependent-input
attribution estimator. PAWN or regression can summarize associations in dependent
data, with interpretation tied to the joint distribution.

## Match the measure to the assumptions

| Measure | Appropriate question | Main limitation |
|---|---|---|
| [Variance-based](methods/variance-based.md) | How much variance comes from each input alone or in interactions? | Standard decomposition assumes independent inputs and finite, nonzero output variance |
| [Morris](methods/elementary-effects.md) | Which inputs should receive further analysis? | Elementary effects are screening statistics, not variance fractions |
| [PAWN / Borgonovo](methods/distribution.md) | Does conditioning on an input change the output distribution? | Conditioning/binning and density estimation require enough data per partition |
| [QOSA](methods/distribution.md#qosa) | Which input affects the chosen quantile? | Extreme quantiles have few informative observations |
| [DGSM](methods/derivative.md) | Can a small gradient measure rule out an input? | Requires derivatives and a valid Poincaré constant; a loose upper bound does not establish importance |
| [Regression](methods/regression.md) | Is response approximately linear, or monotone after rank transformation? | Coefficients can miss nonlinear and nonmonotone effects; inspect fit diagnostics |
| [PCE](methods/surrogate.md) | Can a validated approximation provide analytic variance indices? | Basis size, conditioning, input law, and surrogate error matter |
| [Shapley](methods/game-theoretic.md) | How should interaction variance be allocated across independent inputs? | Nested sampling is expensive; dependent inputs are not implemented |

ANOVA, fractional factorial, and G-theory answer questions about structured
experimental grids and measurement designs. See
[experimental design](methods/experimental-design.md); do not interpret their
components as continuous-input Sobol' indices without matching the design assumptions.

## Choosing within the Sobol' family

- **Saltelli 2010:** a starting point for both first-order and total-effect
  indices. `second_order = true` adds pairwise estimates.
- **Jansen:** squared-difference first-order estimates; its public result has no
  total-effect vector. Reuse the Saltelli design to compare main effects and use
  Saltelli’s squared-difference formula for total effects.
- **Janon:** first-order estimates from symmetrized pick-freeze pairs. The public
  result has no total-effect vector; pair it with a total-effect estimator.
- **Owen:** a separate three-vector design for estimating small first-order
  indices. The current implementation returns first-order indices only and
  leaves `second_order` as `None`.
- **FAST/eFAST:** frequency designs offer first-order indices and, for eFAST,
  total effects. Respect the harmonic/sample-size constraints of the constructor.
- **RBD-FAST:** first-order estimates from a single input/output dataset; useful
  when you cannot afford the extra Saltelli blocks. It does not recover total effects.

## Evaluation budget and analysis cost

Let $d$ be factor count, $N$ rows per base matrix (or dataset), $r$ Morris
trajectories, $m$ Shapley permutations, and $N_O,N_I,N_V$ its nested-sampling
counts. Costs below count model calls; analysis work comes afterward.

| Method | Model calls | Outputs / analysis cost |
|---|---:|---|
| Saltelli / Jansen / Janon, basic design | $N(d+2)$ | $O(Nd)$ aggregation; Jansen/Janon have no total-effect vector |
| Saltelli design with second-order blocks | $N(2d+2)$ | Pairwise aggregation adds $O(Nd^2)$ work |
| Owen | $N(2+2d)$ currently | First-order only; design stores $C$, but estimator does not evaluate it |
| Morris | $r(d+1)$ | $\mu$, $\mu^*$, $\sigma$; grouped design uses group count in place of $d$ |
| FAST/eFAST | $dN$ | FFT work per factor; no explicit pairwise attribution |
| RBD-FAST | $N$ | Sorting plus FFT per factor |
| PAWN / Borgonovo / QOSA / given-data Sobol' | $N$ (zero new calls for saved data) | Partitioning and sorting; Borgonovo additionally evaluates KDEs |
| DGSM with forward differences | $N(d+1)$ | $E[(\partial_i f)^2]$ and total-effect upper bounds |
| DGSM with central differences | $2Nd$ | Separate outputs may be needed to estimate output variance |
| DGSM with supplied gradients | No new calls in estimator | $O(Nd)$; gradient acquisition cost is model-dependent |
| Regression | $N$ | Matrix solve and rank transforms; singular/poor fits can invalidate interpretation |
| PCE | $N$ training calls | Basis size $P=\binom{d+p}{p}$ for degree $p$; dense least-squares cost grows with $P$ |
| Shapley | $N_V+mN_ON_I(d-1)$ | Independent-input random-permutation algorithm, not exhaustive $2^d$ subsets |

`OwenMatrix::total_evaluations()` reports the conservative design budget
$N(3+2d)$ including $C$. The table records calls made by `estimate_owen` today.
For expensive simulators, model calls dominate; for saved data, sorting, KDE,
matrix fitting, and bootstrap repeats can dominate. See
[benchmarks](benchmarks.md) for what the timing harness actually measures.

## Sample size: start, measure, increase

These are pilot budgets, not accuracy guarantees. Smoothness, effective dimension,
interactions, noise, and the required error tolerance determine the final size.

| Campaign | Starting point | What to check before accepting it |
|---|---|---|
| Morris screening | 10–20 trajectories; even grid levels such as 4 or 6 | Rankings and $\mu^*,\sigma$ stability across more trajectories / seeds |
| Sobol' with QMC | $N=1024$ or 4096, then double | Main and total effects stabilize at the precision you report |
| Final Sobol' study | Often $N=8192$ or 16384 as a next step | Convergence and uncertainty, especially near-zero indices; no universal publication threshold |
| Given-data distribution measures | Pilot around 1000 rows if feasible | Adequate observations per bin/slice; change partition settings and bootstrap |
| Tail-oriented QOSA | Budget against the chosen quantile | At level 0.99, 1000 rows leave only about 10 above-quantile observations |
| PCE | More training rows than retained basis terms | Held-out prediction error and stability of indices as degree/data increase |

For example, with $d=20$ and a budget of 5000 calls, Saltelli permits only
$\lfloor5000/22\rfloor=227$ base rows, while 20 Morris trajectories cost 420
calls. Screening may be more useful than reporting unstable quantitative indices.
A sparse PCE fit has no universal “2–5 samples per factor” guarantee: active basis
size and model complexity determine the training requirement.

Bootstrap intervals add no model calls when outputs are cached. Try 200 resamples
while developing and 1000 when assessing interval stability; increase $N$ if
intervals are too wide rather than merely increasing resamples. For given-data
bootstrap, inspect skipped resamples. For unscrambled QMC, row bootstrap is a
resampling diagnostic rather than a calibrated QMC error estimate. The
[quickstart](quickstart.md#6-add-bootstrap-confidence-intervals) shows the API.

Record input distributions, sampler settings, sample size, RNG state, model
revision, and uncertainty alongside reported indices. [Bit-reproducibility](internals.md)
makes reruns comparable; it does not establish statistical convergence.
