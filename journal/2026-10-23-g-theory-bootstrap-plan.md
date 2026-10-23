# G-theory bootstrap correction (#5): plan

The operator chose the literature-supported procedure for the components, a
parametric bootstrap for G/Phi, and a coverage study before any calibration
claim. Correction comes first; salib may sunset these intervals in favour of
crolib later. Research: `papers/2026-10-23-gtheory-bootstrap-performance.md`
and `papers/2026-10-23-pigeonhole-bootstrap-theory.md`.

## Why the current plan cannot simply be corrected

`bootstrap_g_theory_pir` resamples persons, items and raters together
(Tong & Brennan's boot-p,i,h). The exact correction removes its bias. That
correction is Brennan 2006 (CASMA 21) Eq. 73/74, derived again independently
and checked by exact enumeration, and it predicts #5's measured biases to
within about one MCSE. The spread stays inflated, though:

- corrected boot-p,i,h SEs were 2–30× too large (CASMA 5, Table 5), and 144.6
  against a true 4.70 for σ²(pih) with two raters;
- the corrected σ²(pir) replicate SD is 4× too wide at 20×8×4 and 50×20×10;
- G percentile intervals covered 98–99% at nominal 95%.

No source recommends boot-p,i,h for any component.

## Components: Tong & Brennan single-facet plans with exact centring

- **Plan per component** (CASMA 5 Rules 1–2, p. 2; CASMA 15 §3.1.2):
  - a main effect uses its own facet;
  - an interaction uses its facet with the largest count, ties broken in
    p, i, r order;
  - σ²(pir) uses the largest of the three.
  - Rule 3 ("a very large facet may do as well") is discretionary and is not
    implemented.
  - The result reports the plan used for each component.
- **Streams.** For each facet used by any component, draw `n_resamples`
  resamples of that facet alone, with replacement, keeping the other facets
  fixed. Refit the signed EMS estimates and correct them with CASMA 21 Eq. 73
  for λ = {j}:
  - σ̃²(α) = σ̂²(α|j) / t_j when j ∈ α, with t_j = (n_j − 1)/n_j;
  - σ̃²(α) = σ̂²(α|j) − σ̂²(α∪j|j)/(n_j − 1) when j ∉ α.
  - Sage enumeration of every resample of 3×2×2, 2×3×2, 2×2×3, 4×2×2 and 3×3×2
    rational grids shows the corrected replicates centre exactly on the
    observed estimates, for all three facets.
- **Interval.** The percentile interval of each component's corrected
  replicates from its assigned stream, with the existing placement bounds for
  failed fits.
  - Corrected replicates are signed, so a lower bound may be negative. It is
    an interval for the signed estimator.
  - A constant resample contributes exact zeros.
- **Stream order.** RNG use is fixed: p stream, then i, then r (only those
  used), then the parametric stream. This keeps results bit-reproducible.

## G and Phi: parametric bootstrap

- **Generation.** Use the point components with negatives zeroed. Draw seven
  independent normal effect arrays at the grid's counts and sum them; the
  ANOVA is invariant to the mean.
- **Refit.** Refit the signed EMS estimates. Evaluate the replicate's clipped
  G/Phi at the grid's counts, the same estimator as now. Take percentile
  intervals with placement bounds for 0/0.
- **Point unavailable.** The result attaches an interval only where the
  signed point coefficient is available, as it does now.
- **Normal draws.** Use owned machinery: a Box–Muller transform on the ChaCha
  stream, or salib-core's normal quantile if it already exposes one.
- **Departures from Jiang et al. (2022).** Jiang fits lme4 (REML, nonnegative)
  and evaluates Φ at 100×20×2 only. salib generates from zeroed ANOVA
  components and refits signed ANOVA. The coverage study has to earn the
  claim.
- **Out of scope.** Projections to other counts are unsupported for intervals
  and are documented as such.

## Verification

1. **Exact centring in Rust.** Enumerate every single-facet resample of
   3×2×2 and 2×3×2 grids. The mean of the corrected replicates equals the
   observed estimates to rounding. This is the Sage result, ported as a test
   oracle against salib's own code.
2. **Plan rule.** Assignment table tests, including ties and the 20×8×4
   example: p, i, r, p, p, i, p.
3. **Generator.** Over many parametric draws, ANOVA estimates average to the
   generating components within 4 MCSE (unbiasedness), and every interval is
   bit-reproducible for a fixed seed.
4. **Coverage study, run on neuroses.** A harness in the unpublished
   `salib-models` crate, following Morris, White & Crowther (2019):
   - facet sizes: n_p ∈ {20, 50, 200}, n_i ∈ {5, 8, 20}, n_r ∈ {2, 3, 5};
   - component patterns: all 1; σ²(r), σ²(pr), σ²(ir) near zero; G ≈ 0.9;
   - distributions: normal, plus dichotomous (thresholded) for robustness;
   - n_sim = 1900 per cell, B = 1000, α = 0.05.

   Report coverage with its MCSE, lower and upper misses separately, width,
   and failure counts. Calibration is claimed only for cells within ±2 MCSE of
   nominal in both tails. The report goes in `papers/`, and the docs scope
   their claims to it.
5. **Gate.** The workspace gate, adversarial plan review, code review and a
   PR. No release.

Documented as unsupported until the study shows otherwise: relative-G
intervals beyond the studied cells, non-normal data, near-zero components,
two- or three-level facets, σ²(Δ), finite universes, D-study counts, and
simultaneous coverage.

## Plan review outcome and revision

An adversarial reviewer simulated the plan with salib's own estimator. It
confirmed that the single-facet formulas are exactly CASMA 21 Eq. 73 with
Algorithm 3, and that they reduce to CASMA 5/15 Eq. 4–7. It confirmed Rule 2's
wording and the 20×8×4 assignment. Two blockers changed the method. The
operator approved the evidence-matched revision.

- **Tong & Brennan validated SEs, never intervals.** Both reports compute
  only the mean and SD of 100 replicates. Percentile intervals from the rule
  plans undercover at nominal .95:
  - .86–.87 for σ²(p), σ²(i) and σ²(ir) at 20×8×4, with misses on the upper
    side;
  - .70 for σ²(r) at 20×8×4, and .47 at 20×8×2.

  With two raters, boot-r can only produce the interval [0, 2σ̂²(r)].
- **The parametric bootstrap undercovers G.** Coverage was .916–.939, with
  lopsided tails. Percentile, signed, bias-corrected and basic variants do not
  fix it. Phi covers .939–.957.

**Revised design.**
- **Component SEs.** The SD of the corrected replicates from each
  component's rule plan: a single-facet stream with the exact CASMA 21
  centring. This is what the literature validated.
- **Component intervals.** Parametric-bootstrap percentiles (the reviewer
  measured .807–.958 at 20×8×4), with Satterthwaite as a study comparator.
  The study picks the method. σ²(r) and facets with two or three levels are
  documented as unsupported.
- **G interval.** `[1 − (1−Ĝ)/F_{α/2}, 1 − (1−Ĝ)/F_{1−α/2}]` with
  `F ~ F(ν̂, n_p − 1)` and `ν̂ = L²/Σ MS_j²/df_j`, where
  `L = MS_pi + MS_pr − MS_pir`. It holds at the G-study counts only.
  - The identity `1 − G = (EMS_pi + EMS_pr − EMS_pir)/EMS_p` and the ν form
    are verified symbolically in `verify/sage/g_theory_f_interval.sage`.
  - The F law is a Satterthwaite approximation. It generalizes Feldt's p×i
    interval and is not from the G-theory literature.
  - The reviewer measured .9485–.9516 with balanced tails (5 cells, 20,000
    reps each); the study must reproduce it.
  - Unavailable when L = 0.
  - F quantiles come from statrs `FisherSnedecor`, an existing salib
    dependency.
- **Phi interval.** Parametric-bootstrap percentile with placement bounds.
- **Which result carries the interval.** The parametric generator uses the
  zeroed components, so its intervals belong to the clipped estimator. In
  the near-zero pattern the signed point was available in only 48% (G) and
  6–7% (Phi) of datasets.
  - `GTheoryBootstrap` reports the intervals unconditionally.
  - `with_negative_components_zeroed()` keeps the coefficient intervals and
    the parametric component intervals, but not the signed-stream SEs.
  - The signed result attaches the G interval (F, a function of the signed
    mean squares) only when the signed point G exists, and attaches Phi
    likewise.
  - The study reports availability, plus coverage both overall (a `None`
    counts as a miss) and conditional on availability.
- **Streams.** Child streams from `RngState::fork(salt)`, which is already
  model-checked in salib-models: `b"g-theory/boot-p"`, `/boot-i`, `/boot-r`
  and `/parametric`. `n_resamples` applies per stream, and `n_skipped` is
  reported per stream. Every rule plan uses all three facets, because every
  main effect needs its own.
- **Generator.** Draw independent scaled χ² mean squares directly, since
  they are equivalent in distribution to normal effect arrays for balanced
  designs (O(1) per replicate). If the χ² sampler would be new machinery,
  use normal arrays instead.
- **Normal draws.** Use salib-core's `Distribution::Normal` quantile (statrs
  `inverse_cdf`) on the ChaCha stream; this choice fixes the
  bit-reproducibility contract.
  - Generator tests: the means of the mean squares equal the EMS, and
    `Var(MS) = 2·EMS²/df`, both within 4 MCSE.

**Coverage study (revised).**
- **Estimands:** the seven components, G and Phi at the grid's counts.
- **Cells:**
  - sizes n_p ∈ {20, 50, 200}, n_i ∈ {5, 8, 20}, n_r ∈ {2, 3, 5};
  - three patterns: all components 1; σ²(r), σ²(pr) and σ²(ir) = 0.01;
    and a high-G pattern with fixed components. G varies with the counts and
    is reported per cell, not rescaled.
- **Dichotomous robustness arm:** threshold a normal latent at 0, with exact
  truth from bivariate-normal orthant covariances and Möbius inversion. Run
  it only on a subset of cells.
- **Measures:**
  - coverage with its MCSE;
  - lower and upper misses;
  - bias-eliminated coverage;
  - width;
  - availability;
  - per-stream failures.
- **Comparators:** Satterthwaite for components, and the old boot-p,i,r plan
  as a baseline on 4 cells.
- **Criterion:** a fixed band of [.94, .96] at nominal .95, with MCSE
  reported. This is not a ±2-MCSE significance test.
- **Scale:**
  - n_sim = 1900 and B = 999 on neuroses;
  - per-dataset seeds derived from (cell, rep), so shards reproduce;
  - smoke run on 2 cells, n_sim = 200, B = 199, which checks the harness
    only and is not evidence.
- **Harness:** a standalone bin crate, not salib-models' Stateright main.
- **Sage oracle** (crolib-hmz pattern): exact enumeration of the
  single-facet replicate mean and variance on tiny grids, emitted as
  committed Rust in `tests/oracles/`.

## Second plan review

The revision resolves the earlier findings. The F interval's orientation is
correct: W = (1−Ĝ)/(1−G) ≥ F_{α/2} gives the lower bound. All findings are
adopted:

- **Generator.** statrs `ChiSquared::inverse_cdf` returns NaN at df = 1
  (u ≲ 1.5e-4), which is MS_r with two raters, and garbage at df ≈ 1e6. The
  generator stays normal effect arrays, using salib-core's `Distribution::Normal`
  quantile on open-interval uniforms. The generator tests check the
  mean-square moments, E MS = EMS and Var MS = 2·EMS²/df, without a χ² sampler.
- **F interval.**
  - It is unavailable when L ≤ 0. A negative L reverses the bounds and puts
    them above 1.
  - It is unavailable when ν̂ < 0.5, or when a returned quantile's cdf misses
    its probability by more than 1e-9. statrs `FisherSnedecor::inverse_cdf`
    fails at tiny ν.
  - The lower bound is clipped at 0.
  - The reviewer measured coverage of .947–.967 over 20,000 reps in four
    patterns.
- **Attachment.** The F interval depends only on the signed mean squares, so
  it attaches whenever it exists, on both the signed result and the zeroed
  copy. On the zeroed copy the clipped Ĝ can fall outside it (1.85% of
  datasets at G ≈ 0.06); this is documented. The Phi interval (parametric,
  for the clipped estimator) attaches to the signed result only when the
  signed Phi exists, and always to the zeroed copy. The parametric component
  intervals are intervals for the parameters and attach to both. The
  signed-stream SEs attach to the signed result only.
- **Study.**
  - Add the SE ratio, mean SE over the empirical SD. The σ²(r) SE is
    unsupported: the SD ratio was 0.80 at 20×8×4 and 0.62 at 20×8×2.
  - Require each tail's miss rate within [.015, .035] in addition to the
    [.94, .96] band. At n_sim = 1900 that band is about ±2 MCSE, so a true
    .95 method fails about 5% of cells; the report says so.
  - Pin the high-G pattern at σ²(p) = 4 with every other component 0.5.
  - Bias-eliminated coverage targets the mean of the point estimator: signed
    for components, clipped for coefficients.
  - Dichotomous truth uses Sheppard's Cov_M = arcsin(ρ_M)/(2π) at threshold 0.
- **API.** `GTheoryResult` carries one `bootstrap: Option<GTheoryBootstrap>`
  in place of the flat interval fields. Methods apply the attachment rules.
  Bit-reproducibility now also depends on the statrs version, and the docs
  state it.

## Code review

A fresh reviewer read the diff and found no blockers. It verified:
- Eq. 73 equals the T-term boot-λ solve replicate by replicate, with zero
  difference on five shapes and every facet.
- The enumeration tolerance catches plausible bugs: dropping n/(n−1),
  dividing by n, or subtracting the corrected term each shift the means by
  0.5–8, against a tolerance of at most 7e-11.
- The generator indexing is correct.
- The dichotomous truth (Sheppard plus Möbius inversion) matches 200,000
  simulated datasets, |z| ≤ 2.07.
- The 0.3.0 baseline is faithful to the old code.

Fixed:
- **F interval.** It could return (0, negative) near Ĝ = 0; both bounds are
  now clipped to [0, 1].
- **Quantiles.** statrs `inverse_cdf` is unreliable at small and huge degrees
  of freedom, so quantiles now come from bisection on statrs' cdf, which is
  reliable there. This also fixes the harness's Satterthwaite comparator,
  which was NaN for every ν ≤ 1.
- **Harness robustness.**
  - A replicate error is recorded and counted as a miss; it no longer aborts
    the run.
  - Rows are written as each cell finishes.
  - A cell-range argument allows sharding.
  - Seeds are keyed by the cell's description, not its position.
  - The SE ratio is now √(mean SE²)/SD (Morris et al. 2019).
  - The generator uses Box–Muller, independent of salib's inverse-CDF
    machinery.
- **Pinned F oracle.** Four cases computed in Sage with 50-digit mpmath F
  quantiles; this catches a swapped degrees-of-freedom pair.
- **Sage drift check.** It never matched any file: a plain pathspec `*` does
  not cross directories as written, so the check passed on any drift. It now
  uses `:(glob)` and includes untracked files, and failing on uncommitted
  oracles is confirmed. The oracle grids are fixed integer formulas, not
  Sage's random generator, and the 4×2×2 case is added.
- **RNG.** It advances before any error return.
- **Docs.** The method docs now describe the actual procedures and
  `n_resamples` per stream.

The first full study run was started on neuroses with the pre-review harness.
It is discarded, because its Satterthwaite rows would have been invalid and
one replicate error could abort the whole run.
