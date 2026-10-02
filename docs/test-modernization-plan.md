# Test Modernization Plan

Replaces the retired TCK layer with a targeted, evidence-driven test strategy.
Goal: close known gaps, add high-value oracles, harden the type system — without
drowning in test suite overhead.

## Constraints

- **Targeted test runs**, not `cargo test --workspace` for every change.
  Use `cargo test -p <crate> <pattern>` during development; full suite only at commit gates.
- **872 existing tests, ~57s full suite.** That's the budget we're protecting.
- **One layer of proof per kind** (BLACKBIRD tenet): don't write tests that certify other tests.

## Phase 1 — Bug fixes (correctness before coverage)

Fix what the analysis found broken. Each fix gets a **failing test first**, then the fix.

| # | Bug | Location | Severity |
|---|-----|----------|----------|
| 1 | PAWN ECDF merge infinite-loops on NaN | `pawn.rs:282-295` | **Hang** |
| 2 | FAST estimator panics when sampler sends harmonic > 32 | `fast.rs:184` (estimator) vs `fast.rs:148` (sampler) | **Panic in release** |
| 3 | Problem builder accepts NaN `mu`, infinite params | `problem.rs:292,287,302` | Validation gap |
| 4 | LHS emits exactly `1.0` (violates `[0,1)` contract) | `lhs.rs:156` | Contract violation |
| 5 | Jacobi norm NaN for valid params (degree 0, α+β ≤ -1) | `polynomial.rs:253` | Math bug |
| 6 | Saltelli model vs cached path: constant output → NaN vs zero | `saltelli2010.rs:108` vs `:204` | API inconsistency |
| 7 | `Problem.factors`/`groups` public mutable — validated-type invariant doesn't hold | `problem.rs:55,76` | Design flaw |
| 8 | Bootstrap doesn't validate `alpha` | `bootstrap.rs:133` | Input validation |

**Test pattern**: one `#[test]` per bug, in the relevant crate's `tests/` directory.
`cargo test -p <crate> <test_name>` — seconds, not minutes.

## Phase 2 — Metamorphic oracle suite (the math crate's real tests)

From `docs/analysis/metamorphic-oracles.md` — **all 28 relations**. Organized into three test files by what they exercise.

### Tier A: Exact algebraic identities (tight tolerance, fast, no sampling)
Test file: `crates/salib-estimators/tests/metamorphic_exact.rs`

1. Output scaling: `g = a·f` → indices unchanged, variance × a²
2. Affine shift: Jansen/Janon/Owen S1 and Saltelli ST preserved
3. Saltelli shift correction: exact finite-sample formula
4. PAWN invariance under strictly increasing h(Y)
5. Borgonovo affine invariance (Y, 2Y+3, -2Y+3)
6. QOSA positive-scaling preservation
7. Unnormalized effects: Morris μ×a, μ*×|a|, σ×|a|
8. Factor permutation equivariance (design permuted)
9. Joint row permutation → estimates unchanged
10. Fully additive: S_i = ST_i = v_i/Σv_j, S_ij = 0, ΣS_i = 1
11. Monotonicity: increasing coefficient → increasing S_i
12. Interaction addition: increases ST, decreases S1
13. Inactive factor → S_i = ST_i = 0 exactly; active-only → Jansen/Janon S1 = 1
14. Grouped additive: S_G = Σ_{i∈G} S_i (exact for Saltelli)
15. Larger groups: S_G ≤ S_H for G ⊆ H (population)
16. HDMR conservation: Σ S_i = order_variance[0], Σ order_variance[k] = 1
17. Estimator-specific finite-sample bounds (not universal [0,1])
18. Janon/Jansen squared-difference identity
19. DGSM: ν_i(af+b) = a²ν_i(f), U_i invariant under scaling
20. Rank-preserving input transforms → rank-based estimates unchanged
21. Affine input unit changes commute with sampling
22. Quantile monotonicity + parameter-order oracles
23. Iman-Conover preserves column marginals exactly
24. Sampler size/dimension prefix properties
25. S2-on preserves S2-off matrices bit-identically
26. Bootstrap CI transformation + confidence-level nesting
27. ANOVA/G-theory affine + permutation oracles
28. Discrepancy permutation, replication, reflection invariance

Split across three files by dependency:
- `metamorphic_exact.rs` (estimators): relations 1-20, 25-28
- `metamorphic_statistical.rs` (estimators): relations 11, 12, 15 (population-level assertions)
- `metamorphic_sampler.rs` (samplers): relations 21, 23, 24

Relations 1-10, 13-14, 16-20, 22, 25-28 are exact identities (tight tolerance).
Relations 11-12, 15 are population relations (statistical tolerance, moderate N).
Relation 21 tests the quantile/distribution layer (in salib-core or samplers).

**Test pattern**: reuse a single design per test, compare two runs.
`cargo test -p salib-estimators metamorphic` — runs only the new file.

## Phase 3 — Float hazard hardening

From `docs/analysis/float-hazards-type-constraints.md`.

### 3a: NaN-safe assertion helper
New shared test utility (in each crate's `tests/` or a small test_support module):

```rust
fn assert_close(got: f64, want: f64, tol: f64, ctx: &str) {
    assert!(got.is_finite(), "{ctx}: got non-finite {got}");
    assert!((got - want).abs() <= tol,
        "{ctx}: got {got}, want {want} (tol {tol})");
}
```

Audit existing tolerance checks that can pass on NaN (the `(a-b).abs() > tol` rejection pattern).

### 3b: Systematic fixes for the worst float hazards

| Fix | Scope |
|-----|-------|
| Replace raw-moment `E[Y²]-E[Y]²` with centered variance in Saltelli, Jansen, Janon | 3 estimators |
| Validate `alpha ∈ (0,1)` in bootstrap public APIs | 2 functions |
| Check `is_finite()` before variance guards (NaN bypasses `< 1e-30`) | ~15 sites |
| Use `ln_1p(-u)` instead of `ln(1-u)` in Weibull/Exponential quantile | 2 functions |

**Test pattern**: regression tests with specific adversarial inputs (large offsets, near-equal values, NaN, infinity). Each is a targeted unit test.

## Phase 4 — Type-level constraints

From the 19 proposals in the float-hazards report. Prioritized by misuse prevention:

### High value (prevent real bugs)
| Type | Prevents | Crate |
|------|----------|-------|
| `FiniteF64` newtype (private field, checked constructor) | NaN/infinity in distribution params | core |
| `Probability` newtype for `p ∈ [0,1]` | Invalid Bernoulli/Beta params | core |
| Private `Problem` fields + checked `Deserialize` | Builder validation bypass via serde | core |
| `HarmonicBudget(1..=32)` | FAST sampler/estimator mismatch | samplers |
| `NonZeroUsize` for sample counts | Zero-size sample matrices | samplers |

### Medium value (structural clarity)
| Type | Prevents | Crate |
|------|----------|-------|
| `SaltelliDesign` with private matrices + row count | Shape mismatch between A/B/AB | samplers |
| Separate `physical_dim` from `effect_count` | Grouped design metadata confusion | samplers |
| Validated group layout (nonempty, no overlap, in-bounds) | Morris/Saltelli group bugs | samplers |

**Test pattern**: compile-time failures (trybuild) for newtypes, unit tests for checked constructors.
`cargo test -p salib-core` — fast.

## Phase 5 — Kani bounded verification

Install Kani, then add harnesses for the highest-value targets from `docs/analysis/stateright-kani-targets.md`:

| Target | Domain | Property |
|--------|--------|----------|
| `discrete_uniform_quantile` | symbolic i64 bounds | overflow detection, endpoint correctness |
| Sobol direction indexing | symbolic k, small dim | index bounds at k=2^32 |
| LHS permutation kernel | small n, symbolic draws | permutation preservation, strata |
| Saltelli split-and-swap | symbolic small matrices | cell provenance |
| `percentile_value` | short arrays, α ∈ [0,1] | bounds, monotonicity |
| `tree_sum`/`tree_dot` | short arrays | indexing, odd-tail |

**Test pattern**: `cargo kani` on individual harness files, not workspace-wide.

## Phase 6 — Stateright models (optional, after 1-5)

Four bounded models from the analysis, in priority order:

1. **Problem builder** — validation transitions, error precedence, deserialization bypass
2. **Estimator completeness** — output arrival, shape validation, degenerate variance handling
3. **RNG checkpoint/resume** — fork purity, snapshot equivalence, stream properties
4. **Saltelli assembly** — hybrid construction, provenance, grouped designs

These are verification artifacts, not tests. They live in a `models/` directory and run independently.

## Execution order

```
Phase 1 (bugs) ──→ Phase 2 (metamorphic) ──→ Phase 3 (float hardening)
                                                      │
                                                      ▼
Phase 4 (types) ──→ Phase 5 (Kani) ──→ Phase 6 (Stateright)
```

Phases 1 and 2 are independent — can be parallelized.
Phases 3 and 4 reinforce each other — types prevent the bugs, float fixes handle what types can't.
Phases 5 and 6 come last because they benefit from stabilized APIs.

## Testing discipline per phase

| Phase | Dev loop | Gate |
|-------|----------|------|
| 1 Bug fixes | `cargo test -p <crate> <name>` | `cargo clippy --workspace` |
| 2 Metamorphic | `cargo test -p <crate> metamorphic` | `cargo test --workspace` |
| 3 Float | `cargo test -p <crate>` | `cargo test --workspace` |
| 4 Types | `cargo test -p <crate>` | `cargo test --workspace` + trybuild |
| 5 Kani | `cargo kani --harness <name>` | individual harness pass |
| 6 Stateright | `cargo run --bin model_<name>` | state space exhausts cleanly |

Full `cargo test --workspace` runs only at phase boundaries and before commits — not per test.
