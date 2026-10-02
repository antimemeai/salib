# TCK Retirement Plan

TCK (Technology Compatibility Kit) implies conformance to a formal specification.
No such specification exists — salib implements methods from academic papers and
validates against closed-form analytic results. Calling this "TCK" is intellectually
dishonest. The Gherkin infrastructure (`salib-tck` crate + `tck/*.feature` files)
is retired.

## Status

- **8 `*_tck.rs` files** are already native Rust tests → rename only
- **31 `*_tck.rs` files** use Gherkin → reimplement as standard Rust tests
- **`salib-tck` crate** + `tck/` directory + scaffold → delete
- **Non-TCK tests** (e2e, serde roundtrip, display) → keep as-is

## Category A: Already Native (rename `_tck` → `_tests` or drop suffix)

| File | Tests | Action |
|------|-------|--------|
| `salib-estimators/discrepancy_tck.rs` | 6 | Rename |
| `salib-estimators/fractional_factorial_tck.rs` | 3 | Rename |
| `salib-estimators/from_outputs_second_order_tck.rs` | 3 | Rename |
| `salib-estimators/g_theory_d_study_optimizer_tck.rs` | 8 | Rename |
| `salib-estimators/grouped_factors_tck.rs` | 6 | Rename |
| `salib-estimators/hdmr_tck.rs` | 4 | Rename |
| `salib-estimators/saltelli2010_from_outputs_tck.rs` | 3 | Rename |
| `salib-estimators/second_order_tck.rs` | 3 | Rename |

## Category B: Gherkin with e2e counterpart (likely redundant — diff and merge)

These Gherkin tests have parallel `*_e2e.rs` files. Need to verify assertion
coverage overlap; merge any unique assertions into the e2e file, then delete.

| Gherkin test | e2e counterpart | Unique scenarios to check |
|---|---|---|
| `anova_tck.rs` | `anova_e2e.rs` (9 tests) | Compare |
| `borgonovo_tck.rs` | `borgonovo_e2e.rs` (5) | Compare |
| `dgsm_tck.rs` | `dgsm_e2e.rs` (6) | Compare |
| `fast_tck.rs` | `fast_e2e.rs` (6) | Compare |
| `g_theory_tck.rs` | `g_theory_e2e.rs` (2) | Compare |
| `given_data_sobol_tck.rs` | `given_data_sobol_e2e.rs` (5) | Compare |
| `morris_tck.rs` | `morris_e2e.rs` (10) | Compare |
| `pawn_tck.rs` | `pawn_e2e.rs` (6) | Compare |
| `phase_d_efficiency_tck.rs` | `phase_d_efficiency_e2e.rs` (9) | Compare |
| `qosa_tck.rs` | `qosa_e2e.rs` (5) | Compare |
| `rbd_fast_tck.rs` | `rbd_fast_e2e.rs` (5) | Compare |
| `regression_tck.rs` | `regression_e2e.rs` (6) | Compare |
| `iman_conover_tck.rs` | `iman_conover_e2e.rs` (5) | Compare |
| `shapley_tck.rs` | `shapley_ishigami_e2e.rs` (5) | Compare |
| `active_subspace_tck.rs` | `active_subspace_e2e.rs` (3) | Compare |
| `pce_ishigami_tck.rs` | `pce_ishigami_e2e.rs` (5) | Compare |
| `sparse_pce_tck.rs` | `sparse_pce_ishigami_e2e.rs` (8) | Compare |

## Category C: Gherkin without e2e (must reimplement)

### salib-core

**`distribution_tck.rs`** → `distribution_tests.rs`
Scenarios: Uniform quantile linearity, Triangular/Beta boundaries, Normal/Beta/LogNormal medians,
monotonicity of quantile for Uniform/Normal/Beta/Bernoulli, Beta(1,1)≡Uniform,
Weibull(1)≡Exp(1/scale), Gamma(1)≡Exp(1/scale), out-of-range saturation.
→ **Property tests + table-driven analytic checks.** 15 scenarios.

**`problem_tck.rs`** → `problem_tests.rs`
Scenarios: content_hash stability, equality hashing, parametric sensitivity,
factor name/order/kind sensitivity, 32-byte output, serde roundtrip.
→ **Standard unit tests.** 8 scenarios.

**`rng_determinism_tck.rs`** → `rng_determinism_tests.rs`
Scenarios: same seed→identical bytes, deterministic forking, distinct salts→distinct streams,
fork purity, word_pos snapshot/resume.
→ **Property tests + determinism assertions.** 5 scenarios. *Candidate for Stateright model.*

**`tree_fold_tck.rs`** → `tree_fold_tests.rs`
Scenarios: par_tree_sum ≡ tree_sum bit-identical, par_tree_dot ≡ tree_dot,
re-run determinism, thread-count invariance, empty→0, single-element bypass.
→ **Standard tests with `#[cfg(feature = "parallel")]`.** 6 scenarios.

### salib-estimators

**`saltelli2010_tck.rs`** → merge into `ishigami_e2e.rs`
Scenarios: Ishigami recovery within MC tolerance, S_3 canary, S_2≡ST_2, sum ≤ 1.
→ **Merge unique assertions into ishigami_e2e.rs.**

**`morris_quadratic_tck.rs`** → `morris_quadratic_tests.rs`
Scenarios: quadratic-additive μ/σ recovery, μ* ≥ |μ|, σ decay at R=1000.
→ **Standard tests with MC tolerance.** 3 scenarios.

**`g_theory_d_study_tck.rs`** → `g_theory_d_study_tests.rs`
Scenarios: projected reliability improves with more items/raters.
→ **Monotonicity property test.** 1 scenario.

### salib-samplers

**`lhs_tck.rs`** → `lhs_tests.rs`
Scenarios (structural): n×d shape, unit interval, stratification, centered at cell centers,
zero rows, zero dim, n=1 cases.
Scenarios (determinism): same seed→identical, RNG advancement, distinct streams differ,
zero-row no consumption, centered vs classic byte count.
→ **Property tests + structural assertions.** 14 scenarios total.

**`saltelli_matrix_tck.rs`** → `saltelli_matrix_tests.rs`
Scenarios (determinism): same seed→identical matrix, pure under inputs.
Scenarios (structure): A_B^i replaces column i, B_A^i symmetric with second_order,
eval count formulas, A/B are disjoint halves.
Scenarios (validation): dimension mismatch, zero samples, etc.
→ **Standard tests + structural property tests.** ~13 scenarios.

**`sobol_tck.rs`** → `sobol_tests.rs`
Scenarios: canonical dim-1 values, canonical dim-2 Joe-Kuo values, skip_first behavior.
→ **Table-driven tests against known Sobol sequence values.** 4+ scenarios.

**`fast_tck.rs`** (samplers) → `fast_design_tests.rs`
Scenarios: FAST design shape, unit interval, per-factor search curves.
→ **Structural tests.** 5 scenarios.

### salib-validation

**`ishigami_tck.rs`** → `ishigami_analytic_tests.rs`
Scenarios: S_3=0 canary, S_2=ST_2, S_i ≤ ST_i, non-negativity, sum ≤ 1,
canonical values (0.3139, 0.4424, 0.5576, 0.2436), parameter edge cases,
positive variance, input distribution shape.
→ **Table-driven analytic tests.** 11 scenarios. *The primary correctness gate.*

**`sobol_g_tck.rs`** → `sobol_g_analytic_tests.rs`
Scenarios: V_i closed form, total variance product, monotonicity in a_i,
bounds, NaN sentinel, screening ranking, dimensionality.
→ **Analytic formula tests.** 8 scenarios.

### salib-tck

**`scaffold_tck.rs`** → delete (tests the Gherkin runner itself).

## Category D: Orphan feature files (no harness)

These `.feature` files have no executable test harness:
- `tck/salib/g-theory-estimator/features/d_study_card_render.feature` (7 scenarios — API not in workspace)

→ Delete with the rest of `tck/`.

## Execution order

1. **Rename Category A** (8 files) — trivial, zero risk
2. **Audit Category B** (17 pairs) — diff assertions, merge unique ones, delete Gherkin
3. **Reimplement Category C** (14 files) — by crate, starting with core
4. **Delete `salib-tck` crate + `tck/` directory** — last step
5. **Update workspace `Cargo.toml`** — remove `salib-tck` member
6. **Update docs** — remove TCK references from `docs/crates.md`, READMEs

## Assertion quality note

Multiple tolerance checks use `(got - want).abs() < tol` which **passes silently
when `got` is NaN** (NaN < tol is false, but some patterns invert this).
All reimplemented tests must use NaN-safe assertions.
