# Kani production bug fixes

Address the four findings in `docs/analysis/phase5-kani-report.md` without
changing the proof harnesses. Add regular regression tests for the half-width
integer range, rounded upper endpoint, opposite extreme finite uniform bounds,
and equal smallest subnormals (including the seven-element witness pattern).

Use `i128` for discrete range sizes and index arithmetic, explicitly return
quantile endpoints, bound rounded discrete indices, and interpolate finite
uniform bounds without an overflowing difference. Replace percentile weighted
interpolation with an endpoint-preserving difference formulation.

Run each affected Kani harness and the corresponding crate tests after its fix,
then run workspace tests and Clippy. Dependencies are cached locally; use Cargo
offline. The inherited beads workflow is unavailable (`bd ready --json` reports
no database); do not initialize unrelated issue-tracking infrastructure.

All five new regression tests failed on the original production code with the
reported overflow, incorrect endpoint, NaN, and subnormal-zero failures. After
the fixes, `cargo test -p salib-core` passes 210 tests and
`cargo test -p salib-estimators` passes 348 tests (including documentation tests).
`cargo clippy --workspace`, changed-file rustfmt checks, and `git diff --check`
pass.

Final workspace validation passes 958 tests across 70 suite summaries, with
zero failures and zero ignored tests. The overflow harness also passes when
rerun against the final discrete implementation after the endpoint change.

The discrete overflow and endpoint proofs both pass with Kani's default solver.
The intermediate core test runs correctly retained the regressions for bugs
not yet fixed: two failures after the range-size fix, then only the finite
uniform-bounds failure after the endpoint fix. The final core run is green.
No harness assertions or assumptions were changed.

| Finding | Production change | Kani result | Regular tests |
| --- | --- | --- | --- |
| Discrete inclusive size overflow | Compute size, offset, and integer sum in `i128`; support sizes through 2^64 without overflow. | `discrete_uniform_quantile_no_overflow`: PASS, including a rerun of the final code. | Half-width reproducer and full-i64-range quarter/median tests pass; core suite: 210 passes. |
| Rounded discrete upper endpoint | Return endpoints directly; clamp rounded offsets and integer results to the support. Sizes above 2^53 retain approximate f64 indexing for interior probabilities. | `discrete_uniform_quantile_endpoints`: PASS. | Report's `-18_014_398_509_481_985..=0` reproducer passes; core suite: 210 passes. |
| Finite uniform NaN/Inf | Return endpoints before subtracting; use weighted interpolation when width overflows, and clamp the result to the bounds. | `uniform_quantile_monotone`: INCONCLUSIVE; solver did not complete. | `[-f64::MAX, f64::MAX]` endpoints, median, and monotonic grid pass; core suite: 210 passes. |
| Subnormal percentile decrease | Return exact interpolation endpoints; otherwise use `a + frac * (b - a)` for finite widths and cap rounding overshoot at `b`. Retain weighted interpolation for overflowing widths. | `percentile_monotone`: INCONCLUSIVE; solver did not complete. | Two-element and seven-element subnormal regressions, including `percentile_ci`, pass; estimators suite: 348 passes. |

Both monotonicity proofs were attempted with the default CaDiCaL solver and
bundled Kissat. Uniform was additionally attempted with Z3's native
floating-point backend, arithmetic refinement, and CBMC path exploration.
These are unresolved verification runs, not passing proofs. The long-running
solvers were interrupted after approximately 30 minutes for the longest runs,
without an assertion-failure verdict or a completed proof. The optional Bitwuzla
installation could not be completed because sandbox DNS could not resolve
PyPI or GitHub. The user's requirement that all four harnesses pass remains
unmet; conclusive verification of the two monotonicity harnesses is still
required.

Full local output is retained under `target/kani-bug-fixes/`, including the
per-fix crate runs, successful workspace validation, successful discrete
proofs, and unresolved monotonicity solver logs.
