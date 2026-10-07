# Distribution estimator corrections, 2026-10-07

This follows the [distribution literature audit](2026-10-07-distribution-audit.md).
The user authorized implementing the confirmed defects before release. No new
library or configurable-estimator API was introduced.

## Quantile contrast design

The inspected Maume-Deschamps–Niang manuscript defines the pinball contrast in
Eq. (2.2) and its relative loss reduction in Eq. (2.3):
[primary manuscript](https://arxiv.org/pdf/1702.00925). Fort–Klein–Rachdi's
Definition 4.2, Eq. (6), gives the general contrast construction:
[primary manuscript](https://arxiv.org/pdf/1305.2329).

The implementation now computes the minimizing empirical quantile and its
pinball loss globally and within input classes. It evaluates both on the same
observations used for fitting. This deliberately defines an empirical partition
estimate, rather than implementing the paper's kernel/two-independent-sample
estimator or claiming that paper's convergence result.

Directly computing loss resolves output-shift dependence and valid atomic-output
failures. The denominator is `global_loss`, also returned as a diagnostic. Exact
zero or nonfinite loss returns `ZeroVariance`; no arbitrary variance floor excludes
otherwise representable small-scale outputs. `DegenerateTail` remains a legacy
error variant for compatibility, but is not emitted by the corrected estimator.

`global_cte` now reports empirical expected shortfall
`q + mean((Y-q)_+) / (1-alpha)`. When empirical mass is at the quantile, this is
not the mean of only observations strictly above it. Its semantics and QOSA's
numeric results have changed deliberately and must appear in release notes.

## Conditioning design

QOSA, given-data Sobol', PAWN and Borgonovo now share a private value-based
conditioning routine. It sorts each input and assigns an entire equal-value
block using its midpoint rank. For untied inputs, the historical nearly
equal-frequency boundaries are preserved. Empty classes are omitted. PAWN
summarizes only the nonempty slices, so a discrete input can have fewer slices
than requested. A constant input produces one unconditional class and zero
index rather than artificial associations with row order.

All four estimators reject nonfinite input observations with the row/column
location. Nonfinite outputs receive a separate error (already present in PAWN).
The existing sample minimum and default class-count heuristic are unchanged.
PAWN checks its required sample count with saturating arithmetic to avoid
`2*n_slices` overflow for an impossible request.

The 48-class cap and Borgonovo's 100-point quadrature remain documented resolution
choices. They do not guarantee conditioning on exact input values as sample size
grows. Configurable/refining partitions and KDE quadrature would require an
explicit API and additional estimator analysis; this correction does not invent
that guarantee. Borgonovo still assumes suitable continuous output densities.

## Direct evidence

The first three tests in
`crates/salib-estimators/tests/distribution_invariants.rs` were run before the
implementation change and all failed:

- A constant input gave given-data Sobol' `.672619047619` instead of zero.
- Joint row permutation changed that result to `.013392857143`.
- QOSA returned `DegenerateTail` for a positive-loss atomic problem.

After the change, all five tests pass. They use balanced binary independent
conditioning, a fully determining input, a constant input, a joint row
permutation, positive affine output changes (including scale `1e-8`), and
nonfinite-input validation. Separate exact scalar oracles check atomic median
loss `1/4`, expected shortfall `3/2`, and a finite 64-point uniform-grid median
partition estimate `683/1024`. No agreement with another package is the oracle.

`python3 scripts/audit_qosa.py --toolchain 1.95.0` passes against the actual API:

- Atomic problem: index `1`, global loss `.25`, expected shortfall `1.5`.
- Alpha `.9`, 64 midpoint observations: base and output shifted by `100` both
  return `.6672095548317047`.
- Capped median fixture at `N=122880` and `245760`: both correctly return the
  partition target `47/48`, which remains below the exact-conditioning target `1`.

Existing QOSA unit tests (10), QOSA e2e tests (6), PAWN e2e tests (6) and given-data
Sobol' e2e tests (5) pass without loosening assertions. Misleading QOSA e2e source
commentary was corrected to describe fixture results without universal ranking
claims. Root owns replacement of the unrelated Borgonovo benchmark-provenance
fixtures. The local documentation checker passes 25 public pages and 36 citation
destinations; diff whitespace checks pass.

## Independent-review corrections

The fresh reviewer found two additional numerical faults. Two new direct tests
were run before corrections and failed: QOSA returned infinite `global_cte` for
8 zeros and 8 outputs of `1e308` at alpha `1e-308`; Borgonovo changed from
`.5113350636034162` to zero when finite outputs were multiplied by `1e200`.

QOSA now computes expected shortfall by a weighted average of the largest
empirical tail mass, including a fractional final observation. Applying weights
before summation avoids overflowing the unscaled sum. This is algebraically the
same diagnostic formula documented above. Pinball differences likewise apply
the loss weight before subtraction if opposite-sign finite outputs have an
unrepresentable raw difference. The new oracle verifies global loss `.5` and
expected shortfall `5e307` on the original witness, plus global loss `1` and
expected shortfall `0` for equally weighted `-1e308` and `1e308`.

Borgonovo now normalizes output coordinates to `[0,1]` before bandwidth and KDE
arithmetic. The density Jacobian cancels in the integrated absolute difference,
so this preserves the statistical target. It eliminates scale overflow in squared
residuals and removes the arbitrary `1e-15` output-range rejection; exactly
constant outputs still reject. Opposite-sign extrema use an initial bounded
scaling if their raw range overflows. The new oracle compares the original data
with output scales `1e200` and `1e-200`, and a finite affine transformation whose
raw range exceeds the largest representable float.

All seven distribution invariants, six QOSA e2e checks and the three new
analytic Borgonovo e2e checks pass. Targeted Clippy on the estimator library and
invariant tests passes with warnings denied. The documentation checker passes
26 public pages; whitespace checks pass. The extra blank line separating QOSA's
doc comment from its function was removed. These changes use standard operations
available at the advertised Rust 1.87 minimum version.

## Historical QOSA test oracle repaired

The full suite exposed a pre-existing test in `tests/metamorphic_exact.rs` that
encoded the strict-tail defect as required behavior: it asserted index
`.708984375` and deliberately expected adding 10 to change it to `.728515625`.
Those expectations contradict quantile-loss translation invariance. The test
block now uses a hand-derived empirical pinball oracle, retaining its strict
comparison tolerance rather than broadening it.

For outputs `1..64` at the median, the minimizing empirical quantile is 32 and
the total loss is `(sum(1..31)+sum(1..32))/2 = 512`, hence mean loss 8. The
existing three untied classes have 21, 21 and 22 observations and median-loss
sums 55, 55 and 60.5. Thus the empirical partition index is
`1 - 170.5/512 = 683/1024 = .6669921875`. The upper-half expected shortfall is
48.5. The replacement checks these independent scalar identities and requires
positive affine output changes to preserve the index, scale loss by the slope,
and transform the quantile and expected shortfall by slope and intercept.
