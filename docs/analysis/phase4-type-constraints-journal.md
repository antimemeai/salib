# Phase 4: checked numeric types

Implement private-field `FiniteF64` and `Probability` in core and
`HarmonicBudget` in samplers, exporting each with descriptive checked
constructors. Route distribution validation through finite values before range
checks. Bernoulli uses the requested open probability interval; Beta shapes
retain their mathematically valid positive finite domain (including values
greater than one). A probability constraint cannot represent Beta shapes.

Keep the existing integer FAST constructor as a checked compatibility boundary
and add a constructor accepting `HarmonicBudget`. Revalidate the public FAST
metadata with that type in the estimator. Convert sample/resample counts at
existing fallible boundaries to `NonZeroUsize`; return errors instead of
panicking for zero Saltelli bootstrap counts or empty input. Preserve the
infallible sampler trait's documented empty-matrix behavior.

Verify private `Problem` fields and absence of mutable accessors. Exercise
numeric endpoints, nonfinite inputs, typed FAST construction, and rejection
before callbacks/RNG consumption. Run targeted crate tests and clippy after
each type, then workspace tests and clippy.

The inherited beads workflow is unavailable: `bd ready --json` reports no
beads database in this repository.

## Implementation and targeted checks

- `FiniteF64` and `Probability` live in `salib-core/src/types.rs` with
  checked constructors, `TryFrom`, accessors, and checked serde conversions.
  Every floating distribution parameter constructs `FiniteF64` before range
  checks. Bernoulli now rejects the endpoints as requested; Saltelli bootstrap
  alpha also uses `Probability`.
- `HarmonicBudget` lives in `salib-samplers/src/harmonic_budget.rs`, exported
  at the crate root and through `fast`. `build_fast_design_with_budget` accepts
  the checked type; the original integer constructor delegates after checking.
  The FAST estimator reconstructs the type from mutable design metadata before
  FFT planning or model calls. Serde construction is checked when enabled.
- Saltelli (including grouped), Owen, Morris (including grouped), Saltelli
  bootstrap, and given-data bootstrap validate counts through `NonZeroUsize`.
  Saltelli bootstrap returns `ZeroResamples` and `EmptySample` errors before
  callbacks/RNG consumption. ANOVA, G-theory, and Shapley already have positive
  count/minimum-count checks. LHS/Sobol retain their existing low-level
  zero-row matrix convention to preserve the infallible sampler API.
- `Problem.factors` and `Problem.groups` are private; accessors return shared
  slices and there are no mutable accessors. Existing compile-fail doctests
  cover attempted mutation.

Passed targeted checks: core finite-value/probability/Beta-shape tests,
sampler harmonic-budget tests (including the serde feature), typed FAST
equivalence at orders 1, 4, and 32, zero-count sampler tests, estimator
bootstrap tests (34), and FAST invalid-harmonic rejection. Crate-level clippy
passed for core, samplers, and estimators.

## Final validation

`cargo test --workspace` passed: 948 tests across 67 suite summaries,
zero failures and zero ignored tests, including all compile-fail doctests.
`cargo clippy --workspace` passed without warnings. `git diff --check` passed.
The separate sampler serde-feature run passed all three harmonic-budget tests.
