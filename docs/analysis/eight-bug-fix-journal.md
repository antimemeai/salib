# Eight numerical and API bug fixes — 2026-10-02

The requested scope was eight fixes, with a failing regression before each behavior change. Iteration used targeted crate tests (unit tests also used `--lib`, and the privacy tests used `--doc`), followed by crate-level tests and Clippy. No new dependencies were introduced.

| Bug | Regression test file and name | Observed failure before fix | Fix location and behavior | Targeted result |
| --- | --- | --- | --- | --- |
| PAWN nonfinite outputs | `crates/salib-estimators/src/pawn.rs`: `pawn_rejects_nonfinite_outputs_promptly` | Two-second receive timeout on NaN model outputs | `estimate_pawn`: reject NaN and both infinities before sorting, with `NonfiniteOutput` identifying the index | Pass |
| FAST harmonic limit | `crates/salib-estimators/src/fast.rs` and `crates/salib-samplers/src/fast.rs`: `fast_rejects_excessive_harmonic` | Estimator panicked; sampler accepted harmonic 40 | `estimate_fast` validates 1–32 even for mutated designs; `build_fast_design` rejects harmonics over 32 before arithmetic | Both pass |
| Distribution finiteness | `crates/salib-core/src/problem.rs`: `problem_rejects_all_nonfinite_parameters` | Builder accepted Normal with NaN mean | `validate_distribution`: explicit finiteness checks for every floating-point parameter; integer bounds are inherently finite | Pass |
| LHS upper endpoint | `crates/salib-samplers/src/lhs.rs`: `lhs_large_sample_stays_below_one` | Last-stratum arithmetic rounded to exactly 1.0 | `lhs_cell_value`: clamp to the predecessor of 1.0 for classic and centered sampling | Pass, including 2,097,153 rows |
| Jacobi degree-zero norm | `crates/salib-surrogate/src/polynomial.rs`: `jacobi_degree_zero_norm_is_one_for_valid_negative_parameters` | Nonfinite norm for alpha = beta = -0.75 | `jacobi_norm_squared`: return exactly 1.0 for degree zero before log-gamma evaluation | Pass |
| Saltelli constant outputs | `crates/salib-estimators/src/saltelli2010.rs`: strengthened `constant_model_yields_zero_variance` and new `saltelli_near_zero_variance_matches_cached_with_second_order` | Nonfinite first/total indices, then nonfinite optional second-order indices | `estimate_saltelli2010`: apply cached-path absolute-variance threshold of 1e-30 to all three index orders | Both pass, with exact cached-path agreement |
| Problem mutation | `crates/salib-core/src/problem.rs`: two compile-fail doctests on `Problem` | Both forbidden mutations compiled successfully, failing the doctest oracle | `Problem`: private factors/groups, existing immutable `factors()` slice, new immutable `groups()` slice accessor | Both pass; workspace checks passed before and after the change |
| Bootstrap alpha | `crates/salib-estimators/src/bootstrap.rs`: `bootstrap_model_rejects_invalid_alpha` and `bootstrap_cached_rejects_invalid_alpha` | Both APIs accepted alpha 0.0 and returned successful values | Both APIs return `Result<_, BootstrapError>` and reject nonfinite alpha or alpha outside the exclusive interval (0, 1) before work | Both pass for 0, 1, -0.1, 2, NaN and both infinities |

For LHS, the existing arithmetic was extracted unchanged into a small private helper after writing the test, before the red run. This allowed forcing the rare maximum-u32 jitter rather than relying on random sampling to hit it. The clamp was added only after observing the failure.

For bootstrap, the red tests checked for an error through the debug outcome because the old infallible type had no error accessor. After observing that invalid alpha returned a successful value, the APIs became fallible and those same checks became typed `unwrap_err()` assertions. Existing successful callers were updated to unwrap their results.

The privacy tests use Rustdoc's compile-fail oracle so public-field mutation is checked from an external consumer's perspective. Making standalone Factor and Group fields private was unnecessary: Problem exposes only immutable slices of its owned values.

The Saltelli review additionally checked the optional second-order path and tiny nonconstant variance; it shared the same mismatch with the cached API and now uses the same guard.

## Final validation

- Final `cargo test -p salib-estimators`: 332 passed, 0 failed.
- Core crate after privacy fix: 196 passed, 0 failed (including both compile-fail doctests).
- Sampler crate after LHS fix: 189 passed, 0 failed.
- Surrogate crate after Jacobi fix: 91 passed, 0 failed.
- `cargo clippy -p salib-core`, `-p salib-samplers`, `-p salib-surrogate`, and final `-p salib-estimators`: successful; final Clippy output is clean.
- `cargo check --workspace`: successful before and after the privacy change.
- Final `cargo test --workspace`: 909 passed, 0 failed, 0 ignored.
- Formatting checks on all changed Rust files and `git diff --check`: successful.

Other tests and README changes were being added concurrently. A statistical test initially failed with RBD-FAST harmonic truncation at four terms; its subsequently updated 32-term version passed both unchanged HEAD in a temporary checkout and the final working-tree runs. No unrelated production change was made for that test.

Git metadata is read-only under this session's filesystem permissions, so changes remain local and uncommitted. The repository has no Beads database; no issue database was initialized as part of these focused fixes.
