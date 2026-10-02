# Phase 3 float hazard hardening — 2026-10-02

Read `float-hazards-type-constraints.md` before implementation. Changes are limited to the requested formulas, guards, validation coverage, and regression tests; no dependencies were added. All numerical comparison assertions in the new tests first check finiteness. The discrepancy failure oracle deliberately checks NaN because preserving numerical failure is the requested behavior.

## Fixes and regression evidence

All file paths below are relative to the repository root. Test names are fully qualified unit-test names.

| Fix | Location | Test | Before | After |
| --- | --- | --- | --- | --- |
| Centered population variance in all three Saltelli APIs | `crates/salib-estimators/src/saltelli2010.rs` | `saltelli2010::tests::phase3_centered_variance_large_offset` | Variance differed from 1.25 | Pass for model, cached first/total, and cached second-order APIs |
| Centered population variance in Jansen | `crates/salib-estimators/src/jansen.rs` | `jansen::tests::phase3_centered_variance_large_offset` | Variance differed from 1.25 | Pass |
| Centered population variance in Janon; centered joint covariance and denominator preserve Eq 6 | `crates/salib-estimators/src/janon.rs` | `janon::tests::phase3_centered_variance_large_offset` | Variance differed from 1.25 | Pass; also verifies S1 = 1 for identical paired series |
| Finite variance/numerator guards in Saltelli, including total and second order | `crates/salib-estimators/src/saltelli2010.rs` | `saltelli2010::tests::phase3_nonfinite_variance_yields_zero_indices`; `saltelli2010::tests::phase3_cached_nonfinite_outputs_yield_zero_indices` | Nonfinite indices | Pass; checks NaN/infinity and cached hybrid failures with finite base variance |
| Finite joint-denominator/numerator and second-order variance guards in Janon | `crates/salib-estimators/src/janon.rs` | `janon::tests::phase3_nonfinite_variance_yields_zero_indices` | Nonfinite second-order indices | Pass |
| Finite variance/numerator guards in Jansen, including second order | `crates/salib-estimators/src/jansen.rs` | `jansen::tests::phase3_nonfinite_variance_yields_zero_indices` | Nonfinite second-order indices | Pass |
| Finite variance/numerator guards in Owen | `crates/salib-estimators/src/owen.rs` | `owen::tests::phase3_nonfinite_variance_yields_zero_indices` | Nonfinite indices for finite outputs with overflowing variance | Pass; also checks NaN/infinity model outputs |
| Reject nonfinite output/factor variances in regression | `crates/salib-estimators/src/regression.rs` | `regression::tests::phase3_regression_rejects_nonfinite_variance` | Expected variance error was absent | Pass; existing `ZeroVariance` / `ZeroFactorVariance` errors |
| Reject nonfinite reliability numerator/denominator in G-theory | `crates/salib-estimators/src/g_theory.rs`, `reliability_ratio` | `g_theory::tests::phase3_reliability_rejects_nonfinite_denominator` | NaN denominator accepted | Pass for both reliability coefficients; public estimator also rejects nonfinite grids through `UndefinedReliability` |
| Use `-ln_1p(-u)` in Weibull and Exponential | `crates/salib-core/src/distribution.rs:199`, `:207` | `distribution::tests::phase3_small_probability_quantiles_preserve_precision` | Relative accuracy check failed at 1e-16 | Pass at 1e-16 and 1e-17 for both quantiles |
| Validate exclusive alpha range in given-data bootstrap | `crates/salib-estimators/src/bootstrap_given_data.rs:222` | `bootstrap_given_data::tests::phase3_bootstrap_rejects_all_invalid_alpha_before_resampling` | Already passed: production validation existed | Pass for 0, 1, negative, >1, NaN, and both infinities; estimator is never called and RNG stays unchanged |
| Preserve NaN before discrepancy clamping | `crates/salib-estimators/src/discrepancy.rs`, all four discrepancy kernels | `discrepancy::tests::phase3_overflow_discrepancy_preserves_nan` | Overflow returned centered discrepancy zero | Pass for overflowing centered, wrap-around, and modified computations; all four kernels explicitly preserve NaN |
| Reject nonfinite correlation entries before epsilon comparisons | `crates/salib-samplers/src/iman_conover.rs:149`, `:155` | `iman_conover::tests::phase3_correlation_rejects_nonfinite_entries` | NaN diagonal bypassed expected validation error | Pass for NaN and both infinities on diagonals and either/both off-diagonal entries |

For `[1e9, 1e9+1, 1e9+2, 1e9+3]`, centered deviations are `[-1.5, -0.5, 0.5, 1.5]`, yielding population variance `5/4 = 1.25`. The denominator remains population variance, without Bessel correction.

The original quantile expression at `u=1e-16` is nonzero but inaccurate, since `1-u` rounds to the preceding representable value. The added `u=1e-17` case covers the actual rounding-to-one/zero-quantile failure.

Infallible Sobol APIs retain the existing zero-index convention for unusable denominators and now also guard nonfinite numerators. Nonfinite reported total variance becomes zero. Regression and G-theory retain their existing error types. Discrepancy propagates NaN, as permitted by the request, so overflow cannot masquerade as perfect space filling.

## Validation

- Before production edits: `cargo test -p salib-estimators phase3_ --lib`: 10 failing regressions, 1 already-passing alpha regression. An additional cached-output regression also failed before guard edits.
- Before production edits: core quantile regression and sampler correlation regression both failed.
- After centered variance: `cargo test -p salib-estimators phase3_centered --lib`: 3 passed; `cargo clippy -p salib-estimators`: passed.
- After estimator guards: targeted estimator regressions passed; the still-unfixed discrepancy regression failed as expected. `cargo clippy -p salib-estimators`: passed.
- Quantiles: targeted core regression and `cargo clippy -p salib-core`: passed.
- Bootstrap validation coverage: targeted alpha regression and `cargo clippy -p salib-estimators`: passed.
- Discrepancy: targeted overflow regression and `cargo clippy -p salib-estimators`: passed.
- Correlation validation: targeted sampler regression and `cargo clippy -p salib-samplers`: passed.
- Final targeted estimator regressions: 12 passed, 0 failed.
- Final `cargo test --workspace`: 931 passed, 0 failed, 0 ignored.
- Final `cargo clippy --workspace`: passed, without warnings.
- `rustfmt --check` on all changed Rust files and `git diff --check`: passed.

The existing change to `docs/test-modernization-plan.md` was preserved. Beads has no database in this repository. Git metadata is read-only in this session, so committing and pushing are unavailable.
