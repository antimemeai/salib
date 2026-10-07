# Phase 5: Kani bounded model checking

Toolchain: Kani Rust Verifier 0.68.0, CBMC 6.11.0, aarch64 macOS.
Targets come from [the target analysis](stateright-kani-targets.md).

## Implementation and verification scope

Added eleven `#[kani::proof]` harnesses in the four requested files, with
`#[cfg(kani)] pub mod kani;` at each crate root. Normal builds exclude them.
Preconditions use the supported `kani::assume(expression)` function, rather
than the nonexistent `#[kani::assume(expression)]` attribute. Every symbolic
`f64` is constrained to be finite. The workspace recognizes `cfg(kani)` for
Rust's configuration lint.

The requested `kani = "0.68.0"` dev-dependencies were tried first, but cannot
resolve: the upstream `kani` proof library has `publish = false`, and Cargo
reports no matching registry package. Kani's installed verifier automatically
injects this library into target crates; see [the documented build
process](https://model-checking.github.io/kani/usage.html#the-build-process).
The manifests document this instead of retaining dependencies that break all
Cargo commands. `kani-verifier` is the CLI installation package, not a
replacement proof-library dependency.

For reduction sum equality, inputs are independently symbolic exact floats
from `{-1, 0, 1}`. An independent integer sum provides the oracle; arbitrary
floating inputs would invalidate equality with a differently ordered sum.
Lengths cover every value from zero through eight, including odd tails.
Literal-length dispatch preserves that domain while keeping internal Vec
lengths concrete during symbolic execution. Reduction loops use unwind 9;
the two array harnesses select Kani's bundled Kissat solver.

The Sobol harness checks the production resolution constant and an arbitrary
direction table of that shape, so the indexing claim covers every possible
table content. It also checks dimension construction and symbolic column
bounds. It skips direction access for `k = 0`, matching the production origin
branch. It does not verify the direction-number recurrence or sequence values.
Only the resolution constant's crate visibility changed; production
algorithms were not changed.

Percentile samples are symbolic nondecreasing finite arrays, with repetitions
allowed. Length one is covered; empty arrays are outside the requested domain.
Percentile loops use unwind 9. Endpoint verification additionally checks that
an arbitrary bounded alpha produces a finite result.

## Harness results

| Crate | Harness | Property and domain | Result |
| --- | --- | --- | --- |
| core | `discrete_uniform_quantile_endpoints` | All ordered symbolic `i64` bounds: quantiles 0 and 1 equal bounds converted to `f64` | FAIL |
| core | `discrete_uniform_quantile_no_overflow` | Fixed `i64::MIN/2..=i64::MAX/2`, finite symbolic `u` in `[0,1]`: no panic | FAIL |
| core | `bernoulli_quantile` | Finite symbolic `p,u` in `[0,1]`: output is 0 or 1 | PASS |
| core | `uniform_quantile_monotone` | All finite symbolic `lo < hi`, `0 <= u1 <= u2 <= 1`: ordered quantiles | FAIL |
| core | `tree_sum_indexing` | Length 0–8, values in `{-1,0,1}`: no panic and exact independent integer-sum oracle | PASS |
| core | `tree_sum_empty` | Empty input returns zero | PASS |
| core | `tree_sum_single` | Every finite symbolic singleton is returned bit-for-bit, including signed zero | PASS |
| core | `tree_dot_no_panic` | Equal lengths 0–8, independently symbolic finite values in `[-16,16]`: no panic | PASS |
| samplers | `sobol_direction_index_bounds` | `k` 0–1000, dimensions 1–8, any valid column: nonzero-step index in `1..=32` and valid table access | PASS |
| estimators | `percentile_endpoints` | Sorted samples of length 1–8, values `[0,100]`: percentiles 0/1 are min/max; finite output for alpha `[0,1]` | PASS |
| estimators | `percentile_monotone` | Same sample domain, finite `0 <= alpha1 <= alpha2 <= 1`: ordered percentiles | FAIL |

These are bounded claims over the stated domains. Failing properties are
retained as failing harnesses, without `should_panic` or assumptions that
exclude the discovered defects.

## Findings

### 1. Discrete-uniform inclusive range size overflows

`distribution::discrete_uniform_quantile` computes `hi - lo + 1` in `i64`.
Ordered endpoints alone do not prevent either subtraction or addition
overflow. Even the requested half-width bounds fail: their difference is
`i64::MAX`, so the inclusive size is `2^63`.

Kani found both arithmetic overflow checks in the endpoint harness and the
addition overflow in `discrete_uniform_quantile_no_overflow`. Calling the
public API on the half-width range with `u = 0.5` also reproduces a panic in
a normal debug build.

### 2. Discrete-uniform upper endpoint can be wrong without overflow

An inclusive size that fits `i64` can lose precision when converted to `f64`.
At `u = 1`, flooring the rounded size can give an index smaller than `n - 1`.

A small reproducer in magnitude is:

```rust
let d = Distribution::DiscreteUniform {
    lo: -18_014_398_509_481_985,
    hi: 0,
};
assert_eq!(d.quantile(1.0), 0.0); // Actual result: -1.0.
```

Here the exact size `18_014_398_509_481_986` rounds to
`18_014_398_509_481_984`. The public API reproduces this independently of
the model checker. Kani's generated endpoint witness also has a
representable inclusive size and fails the upper-endpoint assertion.

### 3. Finite uniform bounds can produce NaN and infinity

`uniform_quantile` evaluates `lo + u * (hi - lo)`. Finiteness of each bound
does not guarantee finiteness of their difference. With `lo = -f64::MAX`
and `hi = f64::MAX`, the difference is positive infinity. The public API
returns NaN at `u = 0` and positive infinity at `u = 1`; therefore even
the weak monotonic comparison fails. Kani produced a witness with finite
large opposite-sign bounds and `u1 = 0`.

### 4. Percentile interpolation is not strictly monotone in floating point

The weighted expression `a * (1 - frac) + b * frac` can round both positive
terms down to zero for subnormal samples. For the sorted two-element sample
`[f64::from_bits(1); 2]`, percentile 0 has bit pattern 1, while percentile
0.5 has bit pattern 0. Thus increasing alpha decreases the percentile.

Kani's concrete witness uses a length-seven sorted sample beginning with
three copies of that smallest positive subnormal, with alpha 0 and 0.25.
The latter selects a half-way interpolation between two identical smallest
subnormals and returns zero. Compiling the unchanged percentile kernel
extracted from the source confirms the simpler two-element reproducer.
This finding concerns exact monotonicity under IEEE-754 rounding; its absolute
error in this reproducer is one smallest subnormal.

## Commands and validation

Each named harness was run individually with:

```sh
CARGO_NET_OFFLINE=true cargo kani -p <crate> --harness <name>
```

The crates were then run in full:

```sh
CARGO_NET_OFFLINE=true cargo kani -p salib-core
CARGO_NET_OFFLINE=true cargo kani -p salib-samplers
CARGO_NET_OFFLINE=true cargo kani -p salib-estimators
```

All aggregate runs completed: core **5 passes, 3 failures**; samplers
**1 pass, 0 failures**; estimators **1 pass, 1 failure**. Total: **7 passes,
4 failures, 11 harnesses**. All four failures reproduce the findings above;
none is an unwinding, unsupported-feature, or harness compilation failure.
Individual logs and final crate logs are retained locally under
`target/kani-phase5/`. Concrete counterexamples were printed with
`-Z concrete-playback --concrete-playback print`.

Normal validation: `cargo test -p salib-core -p salib-samplers -p
salib-estimators` passed 756 tests across 45 suite summaries with zero failures
and zero ignored tests. `cargo clippy` for those crates with `--all-targets`
passed. A final `cargo check` for the three crates passed. Formatting checks
pass for the added modules, and `git diff --check` passed. Workspace-wide
`cargo fmt --all -- --check` reports pre-existing formatting differences in
`metamorphic_exact.rs` and `metamorphic_statistical.rs`; those unrelated files
were left unchanged.

## Execution notes

The initial registry command failed because registry DNS is unavailable in
the execution sandbox. Subsequent commands use the installed toolchain and
cached dependencies offline. Early reduction runs with symbolic heap lengths
were interrupted because symbolic execution expanded unnecessary halving
rounds. Literal-length dispatch addresses that harness issue without excluding
any length in the specified bound.

The inherited beads workflow cannot run: `bd ready --json` reports no beads
database in this repository. No production bug fixes were made in this phase.

Repository handoff: staging the final report and committing were blocked by
the sandbox: Git could not create `.git/index.lock` (`Operation not
permitted`). The source changes and complete report remain in the workspace;
no commit or push of this phase was completed.

## Post-fix status (after bda959d)

The four bugs were fixed in `bda959d`. Re-running the harnesses:

| Harness | Pre-fix | Post-fix |
| --- | --- | --- |
| `discrete_uniform_quantile_endpoints` | FAIL | **PASS** |
| `discrete_uniform_quantile_no_overflow` | FAIL | **PASS** |
| `uniform_quantile_monotone` | FAIL | **INCONCLUSIVE** — solver timeout after 5.5h on 48-core server |
| `percentile_monotone` | FAIL | **INCONCLUSIVE** — solver timeout after 5.2h on 48-core server |

The two inconclusive proofs are CBMC propositional-reduction timeouts on
f64-heavy formulas, not verification failures. The fixes are corroborated
by the 2 passing proofs, 5 regression tests, and the full 958-test suite.
These proofs will be revisited with better compute.
