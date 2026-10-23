# G-theory semantics fix: plan

The operator authorized fixing #3, #4 and #6 and documenting #5, with the
recommended #3 policy: never compute a coefficient from a negative component,
keep the raw signed estimates, and make truncation an explicit, named
operation. These are breaking changes to `salib-estimators`' G-theory API, so
they belong in the next minor release (0.4.0). This branch does not bump
versions or publish.

## Purpose

`g_theory.rs` is salib's small crossed p×i×r G-study: balanced moment (EMS)
estimates, their D-study projection, a plain bootstrap, and a brute-force
design search. Its point arithmetic is correct: it reproduces Brennan (2001)
Synthetic Data Set No. 3. The defects are in the *meaning* of what it emits:
impossible coefficients, unit-dependent answers, a fabricated 0, and an
unqualified interval. The fix makes every emitted number either a meaningful
value or a typed reason it is unavailable. crolib owns the deeper theory; this
fix does not add estimators or interval methods.

## Fault classes and fixes

1. **Coefficient from negative components (#3).** A coefficient becomes
   `Result<f64, GTheoryCoefficientError>`. Computing G uses σ²(p), σ²(pi),
   σ²(pr), σ²(pir); Phi uses all seven. If any component it uses is negative,
   the result is `Err(NegativeComponent { component })`, naming the first
   negative component in p, i, r, pi, pr, ir, pir order. With all components
   nonnegative and positive counts, 0 ≤ Phi ≤ G ≤ 1 holds by construction.
   Components keep their signed values. `GTheoryResult::with_negative_components_zeroed()`
   is the named truncation: it returns a copy with negative components set to
   0, recomputed coefficients, `negative_components_zeroed = true`, and no
   bootstrap fields (those describe the signed procedure).
2. **Fabricated 0/0 (#4).** A zero denominator is `Err(ZeroDenominator)`,
   never 0. A nonfinite numerator or denominator is `Err(NonFinite)`.
3. **Absolute thresholds (#4).** `ZeroVariance` means every observation is
   identical, tested exactly. The `<= 1e-15` checks are deleted.
4. **Rounding residue mistaken for a negative estimate.** This becomes a new
   hazard once rule 1 rejects negatives: additive data with offset 0.1 or
   123456.789 leaves interaction estimates of 1e-28 to 1e-19 that can have
   either sign. Compute on centered data `z = y - grand`, using a two-pass grand
   mean. Snap an estimate to exactly 0 when
   `|d_k σ̂_k| <= K N ε Σ_j |c_j| MS_j + (K ε max|y|)^2`, where
   `σ̂_k = Σ_j c_j MS_j / d_k` is its EMS combination, N is the cell count, ε is
   f64 epsilon and K = 8. The first term bounds cancellation among the mean
   squares. The second is the data's own resolution: input values are only
   known to within about `ε max|y|`. Both terms scale with y², so snapping is
   unit-invariant. A snapped value is below any statistical meaning, so this is
   numerical resolution, not truncation. Oracles: additive grids over offsets
   up to 1e9 × spread give exact zeros, and estimates from random integer
   grids are unchanged unless inside the bound.
5. **Nonfinite data and overflow.** Nonfinite observations give
   `Err(NonFiniteObservation)`. Mean squares that overflow give
   `Err(NumericalOverflow)`. The documented scale-invariance range is data
   whose centered squares are normal floats (|z| from about 1e-150 to 1e150).
6. **Projection.** `DStudyPoint` coefficients become `Result`s, using the same
   rules. `project_g_theory_d_study` errors only for zero counts.
   `find_minimum_design` returns `Err(GTheoryError::Coefficient(e))` when Phi
   is unavailable. With admissible components, availability does not depend
   on the counts, so the first failure decides. `GTheoryResult::from_components`
   takes the seven components plus the item/rater counts its coefficients
   refer to, and computes the coefficients, so callers can no longer pass
   inconsistent ones. The result records `n_items` and `n_raters`.
7. **Bootstrap (#5, document only).** Replicate fits fail only on constant
   resamples or overflow. Coefficient replicates that are unavailable are
   counted (`g_coefficient_failed`, `phi_coefficient_failed`) and excluded.
   A coefficient interval is `None` when its point coefficient is unavailable,
   or when every replicate failed. The docs state the plan exactly:
   independent with-replacement resampling of persons, items and raters; no
   bias correction; percentiles over successful replicates, which conditions
   on success. They also cite the measured replicate bias from #5. The choice
   between correcting and deprecating stays open on #5.
8. **Oracles (#6).** Add Brennan Synthetic Data Set No. 3 (as transcribed by
   GeneralizIT 8af4fae; the book is not acquired, so that transcription is
   secondary) for the components, G and Phi. Rename the "textbook" gate1
   test: its data stays because it exercises negative σ²(r) and σ²(ir). Its
   Phi becomes `Err(NegativeComponent(R))`. The zeroed copy has Phi = 243/301,
   derived by hand from exact MS fractions. Move the optimizer tests to
   Synthetic No. 3 components, keeping their independent enumeration oracle.

`GTheoryError::UndefinedReliability` is removed. New public items:
`GTheoryComponent`, `GTheoryCoefficientError`, and the error variants above.

## Tests that must fail on 0.3.0 first

Written against the 0.3.0 API, run, and recorded red before implementation:

- Property: over random integer grids, every returned coefficient is in
  [0, 1] and Phi ≤ G. Fails with G = 2.25.
- Scale: the Synthetic No. 3 and gate1 G values are unchanged for
  s ∈ {1e-150, 1e-9, 1e-8, 1, 1e150}. Fails at 1e-8 (0) and 1e-9 (error).
- 0/0: the `50 + 4i` grid must not report G = 0. Fails.

They are then ported to the new types with the same predicates.
Rounding-snap and bootstrap-availability tests cover brand-new behavior.

## Review and delivery

An adversarial reviewer attacks this plan before code, then the diff before
the PR. Run the workspace tests, strict Clippy on all targets with full+serde,
rustdoc and doctests. Update `docs/methods/experimental-design.md`, the module
docs and the serde fixtures. Open a PR that closes #3, #4 and #6 and
references #5. Merging and publishing 0.4.0 need operator confirmation.

## Plan review outcome

An independent reviewer attacked this plan before the code was finished. It
confirmed the coefficient algebra, 0 ≤ Phi ≤ G ≤ 1 in floating point (Phi's
sum extends G's in the same order), and the zeroed gate1 Phi = 243/301. Every
finding was valid and is adopted:

- **Snap bound unsound (blocker).** On additive grids such as 30×20×10,
  2×2×2000 and 200×200×5, residue exceeded the item-4 bound by up to 8e6×:
  the resolution term lacked the divisor, and a zero component's own
  `Σ|c_j| MS_j` is residue-sized. Adopted the reviewer's validated rule,
  `|σ̂_k| ≤ K N ε SS_total/(N−1) + (K ε max|y|)² + K N ε Σ|c_j| MS_j/d_k`,
  whose worst residue/bound ratio was 2.5e-15. A component that small has
  no statistical meaning. The input-resolution term cannot tell exact
  integers from rounded decimals, so at offset/spread ≳ 1e13 real components
  snap; this limit is documented.
- **Circular snap oracle (blocker).** Replaced by exact rational arithmetic
  (i128 T terms) on integer grids, including large and skinny shapes. Exact
  zeros must snap; other estimates must lie within the bound of the exact value.
- **Survivor-only bootstrap percentiles bias G upward (blocker).** Under
  rule 1, replicates fail mostly on the low side: the lower bound moved from
  0.005 to 0.166 at a 2.6% failure rate. Intervals now report the bounds that
  hold for any values the failed replicates could take. A bound is
  unavailable when failures reach α/2 of the replicates. Constant resamples are not failures: all
  their components are exactly 0.
- **Range.** Rescale observations by an exact power of two, so max|y| lies
  in [1, 2). Squares cannot overflow and the two-pass mean cannot overflow.
  Coefficients are then invariant wherever the components themselves are
  representable. Components outside the normal f64 range return
  `ScaleOutOfRange`, which replaces `NumericalOverflow`. #4's 1e±200
  acceptance amends to: invariant for 1e±150, `ScaleOutOfRange` at 1e±200,
  where σ² ≈ 1e±400 cannot be stored.
- **`from_components`** returns `Result`. It rejects zero counts, nonfinite
  components (`NonFiniteComponent`) and component sums that overflow. Because
  of that, Phi being unavailable at one searched count means it is
  unavailable at all of them, unless public fields were set by hand.
  Public fields and serde can still bypass validation; the docs say so.
- **Zeroing convention.** Negatives are clipped after solving and not fed
  back into other estimates. This is not the Cronbach et al. (1972) procedure.
  The flag is set even when nothing was negative, because it records which
  procedure produced the result. Gate1 zeroed Phi(6,4) = 3888/4345.
- Snapped and normalized zeros are +0.0. `Display` prints the reason for an
  unavailable coefficient.

## Implementation

Red on 0.3.0, before any implementation, with the same predicates on its `f64`
coefficients:

- a random 3×3×2 grid reported G = −69 and Phi = 2.875;
- scale 1e-150 returned `ZeroVariance`;
- the 0/0 grid reported G = 0.

#3's own grid gave G = 2.25 on 0.3.0 and now gives `Err(NegativeComponent(Pi))`.

**Symbolic check (SymPy, scratch).** Derived each `E[MS_e] = tr(A_e Σ)/df_e`
from the random-effects model for four shapes. The seven estimator
combinations invert them exactly. The D-study δ and Δ derived from their
definitions match `coefficients()`, and `G − Phi = σ²p(Δ − δ)/((σ²p + δ)(σ²p + Δ))`.
Every exact test constant was confirmed. A further check showed that the
eigenvalues of the implied `Cov(y)` are the EMS at the design's counts, so the
covariance matrix is PSD at every count exactly when every component is ≥ 0.
This is why infinite-universe G-theory admits no negative component, under
any interpretation of the components.

**Deviation from the plan: bootstrap coefficients.** Under the signed rule,
about 40% of Synthetic No. 3 resamples have a negative σ²(i) or σ²(r), so
signed replicate coefficients would almost always leave the interval
unavailable. Replicate G/Phi therefore use the clip-after-solving estimator.
When the point coefficient is available it equals that estimator, and the
result attaches the interval only in that case; `GTheoryBootstrap` exposes it
unconditionally. Component intervals use the signed replicates. Constant
resamples contribute exact zeros. Each bound holds for any value a failed
replicate (0/0 or a failed fit) could have, and becomes `None` once failures
reach α/2.

**Code review, first round** (fresh reviewer, diff only). Blocker: the
plan-review snap term `K N ε SS_total/(N−1)` scales with the total variance,
so a dominant main effect zeroed accurate components (σ²(pir) = 0.668 with
about 9,000 df → 0; G flipped to 1). A red test (`dominant_effects_do_not_snap_real_components`)
failed on that rule. The replacement bound, `δ² + δ√M_k + K N ε M_k + (2ε max|y|)²`
with `δ = K N ε max|z|`, ties the error to the contrast rounding and to the
component's own scale. The resolution term is one squared ulp of the largest
observation, no longer 64. 108 additive float grids, up to 200×200×5 and
2×2×2000 with offsets up to 1e9×spread, give exactly zero interactions.
All the other findings were valid and are fixed:
- NaN interval bounds broke serde; they are now typed `Option` bounds.
- The clipped Phi interval was hidden; it is now exposed.
- `from_components` accepted subnormal components; it now rejects them.
- A hand-set −0.0 reached the coefficients; it is now normalized.
- A trivially satisfied bootstrap oracle now has a person effect, so its
  bounds lie strictly inside (0, 1).
- Two doc figures were off and are corrected.
- #3's red-test grid was missing and is added.

The dead error variants copied from `anova.rs` (`FactorCountMismatch`,
`UnbalancedDesign`, `DuplicateCell`, `MissingCell`) were removed.
`DegenerateAxis` now carries its minimum, so a zero D-study count no longer
reports "must be >= 2".

**Code review, second round** (same reviewer). Confirmed findings 3–7 are
resolved. No real component was zeroed without an outlier, across 210
integer grids checked against the exact oracle: shapes up to 200×50×10 and
2×2×5000, dominant effects up to ×1e7, offsets up to 1e12. No exact-zero
component survived snapping, across 90 rounded-decimal grids and 400k
multi-step-rounded grids. Minor items, all taken:
- The resolution term sat at its exact limit for ±1-ulp inputs, so it is
  widened to `(4ε max|y|)²`. This replaces the `(2ε max|y|)²` above.
- The docs now say the bounds are worst-case. Next to a single huge outlier
  they can zero components around 1e-10 of the largest one, which is far
  below the components' own sampling error.
- `find_minimum_design` notes that error terms can underflow at counts near
  1e16 when σ²(p) = 0.

Final: 1068 workspace tests pass with full+serde, 0 failed, 0 ignored.
Strict Clippy on all targets, rustdoc with `-D warnings`, the docs checker
(26 pages, 36 citations), fmt and whitespace checks all pass. No version bump
and no publication.
