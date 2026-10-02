The best Stateright opportunities are **checkpoint/resume protocols, design assembly, and evaluation completeness**. Most current APIs are synchronous functions; their intermediate states exist as local variables, rather than explicit ADTs. Modeling those states would clarify which transitions should be legal.

I read the core and sampler implementations and the relevant estimator paths. This was read-only analysis; no model-checking harnesses were run.

**1. RNG state**

Source: [rng.rs](/Users/patrickbeam/projects/salib/crates/salib-core/src/rng.rs:68).

The state is `(algorithm, seed, stream, word_pos)`. A useful model also distinguishes a **recorded checkpoint** from a **live generator**, because drawing from `into_chacha()` does not update the original `RngState`.

| Action | Transition |
|---|---|
| `from_seed(seed)` | Creates checkpoint `(ChaCha20, seed, 0, 0)` |
| `from_parts(seed, stream, pos)` | Creates an arbitrary checkpoint without validation |
| `into_chacha()` | Restores a detached live generator |
| Draw | Advances the live generator’s position |
| `snapshot(live, parent)` | Records live stream/position with the supplied parent’s seed/algorithm |
| `fork(salt)` | Creates a child checkpoint; parent stays unchanged |
| Resume | Reconstructs a live generator from a checkpoint |

Forking implements:

```text
child.stream = parent.stream XOR SHA256(parent.stream || salt)[0..8]
child.seed = parent.seed
child.word_pos = 0
```

The stream derivation does **not** depend on seed or parent position; seed is copied into the child.

Properties to check:

- **Fork determinism:** equal parent identity and salt produce equal children.
- **Position independence:** changing only parent position does not change its child.
- **Parent preservation:** forking does not advance or mutate the parent.
- **Child reset:** every child starts at position zero.
- **Resume equivalence:** uninterrupted draws equal draws after checkpoint/resume.
- **Provenance consistency:** the seed supplied to `snapshot` matches the live generator’s actual seed.
- **Schedule independence:** fixed logical tasks get the same RNG identities regardless of execution order.

Two qualifications matter:

1. **Stream uniqueness is not guaranteed.** Arbitrarily many salts map to 64-bit stream IDs. XOR does not make the hash derivation injective, including across different parents. Existing distinct-stream tests establish particular examples, not universal uniqueness. A strict uniqueness requirement needs allocation or collision detection over the campaign’s actual streams.
2. **Position is effectively 68 bits, not 128.** The installed `rand_chacha` implementation ignores the upper 60 bits of `set_word_pos`. Therefore arbitrary `from_parts` positions do not round-trip exactly through restoration/snapshot. The model should either canonicalize positions modulo `2^68` or reject noncanonical positions.

**ADT opportunity:** a `LiveRng` wrapper owning both the generator and its provenance could expose `checkpoint(&self)` without accepting an unrelated parent. A validated `WordPosition` could enforce canonical positions.

**Stateright approach:** bound seeds, salts, fork depth, live handles, and draws. Explore `Fork`, `Restore`, `Draw`, `Checkpoint`, and `Resume` in different orders. Compare a resumed generator with an uninterrupted reference. Include wrong-parent snapshots and duplicate salt reuse as adversarial actions.

For cryptography, either call the real implementation over a small concrete domain or abstract output as a function of identity and position. An abstract model can verify protocol behavior, but cannot establish SHA-256 collision resistance or ChaCha statistical independence.

**2. Sampler pipelines**

Sources: [sobol.rs](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/sobol.rs:177), [lhs.rs](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/lhs.rs:111), [saltelli_matrix.rs](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/saltelli_matrix.rs:138).

**Sobol**

The logical states are:

```text
Configured → DirectionsReady → Generating(row, accumulators) → Complete
```

Construction asserts a dimension within the selected table’s capacity. Generation initializes integer accumulators to zero. For each nonzero generation index `k`, it updates each accumulator using:

```text
x[j] ^= directions[j][trailing_zeros(k) + 1]
```

Then it emits `x[j] / 2^32`.

Invariants:

- Direction-number accesses stay within indices `1..=32`.
- Each emitted row matches the Gray-code recurrence.
- Output has shape `(n, dim)` and values in `[0,1)`.
- `skip_first` determines whether the origin is included.
- No RNG words are consumed; canonical RNG state is preserved.

**Crucial current behavior:** every call restarts the Sobol sequence. There is no persistent cursor, and RNG snapshot/resume does not resume Sobol generation. Two calls requesting `n` rows repeat the same rows. Concatenating calls is not equivalent to requesting their combined length.

The 32-bit resolution also implies a boundary: generation index `2^32` would access direction number 33. That boundary is not explicitly validated.

**ADT opportunity:** retain the current configuration type, but add a separate `SobolCursor { next_index, accumulators }` for incremental generation. Represent exhaustion explicitly.

**Stateright:** for small dimensions and row counts, compare incremental transitions against a direct Gray-code reference. For a proposed cursor, explore pause/resume and batch partitioning and require identical combined output. Check the production index boundary separately with Kani.

**LHS**

Logical states:

```text
Configured → IdentityPermutation(column)
           → Shuffling(column, i)
           → Filling(column, row)
           → NextColumn / Complete
```

Valid transitions follow the descending Fisher–Yates loop, then fill that column. The RNG checkpoint is committed after all columns finish.

Invariants:

- Every shuffle preserves a permutation of `0..n`.
- Each column assigns exactly one row to each stratum.
- Centered samples use `(stratum + 0.5)/n`.
- Classic samples use `(stratum + u)/n`, with `u` from a 32-bit draw.
- For `n > 0`, RNG consumption in 32-bit words is:
  - Classic: `dim × (2n − 1)`.
  - Centered: `dim × (n − 1)`.
- Zero rows or dimensions consume no words.

The modulo-based shuffle preserves permutation validity but is not exactly uniform for all bounds. Also, concatenating smaller LHS batches does not preserve the design of one larger batch.

**Stateright:** use `n ≤ 4`, a few columns, and nondeterministically choose every legal shuffle destination. Track stratum IDs rather than floating-point values. Verify permutation preservation, unique stratum assignment, completion, and draw counts. Check actual floating-point cell boundaries with Kani.

**Saltelli construction**

Logical states:

```text
Requested → Validated → BaseSampled → Split(A,B)
          → HybridsBuilt(subset) → Complete
```

Validation failures transition to an error before sampling, so returned validation errors preserve the RNG.

For physical dimension `d`, the base sample has `2d` columns. The essential invariant is:

```text
AB[i][row,col] = B[row,col] if col == i, else A[row,col]
BA[i][row,col] = A[row,col] if col == i, else B[row,col]
```

Additional invariants:

- `A` and `B` are the corresponding base-sample halves.
- Every hybrid has the same shape as its bases.
- First/total mode contains `d` AB hybrids.
- Second-order mode additionally contains `d` BA hybrids.
- Evaluation counts are `n(d+2)` or `n(2d+2)`.

Grouped construction replaces a set of columns. Here, **`SaltelliMatrix.dim` means group count**, while matrix column count remains physical dimension. A universal invariant `a.ncols() == matrix.dim` would therefore be incorrect.

Other boundary gaps:

- Zero base dimension is accepted.
- Grouped construction checks index bounds, but accepts empty groups, overlapping groups, and repeated indices.
- The trait’s promised sample shape is trusted without validation.

**ADT opportunity:** distinguish `physical_dim` from `effect_count`, use an explicit factor/group layout, and represent first/total versus second-order designs with an enum or distinct types. Keep validated design internals private.

**Stateright:** use symbolic cell values such as `(base_half, row, column)`. Explore hybrid construction order and assert provenance of every cell. Completion requires all requested hybrids. Include small grouped layouts and invalid requests; no numerical model is needed.

The remaining sampler files offer similar models:

| Pipeline | Useful invariant |
|---|---|
| Morris: validate → base/permutation → trajectory steps | Each factor changes once by Δ; grouped steps change the specified group |
| Owen: validate → sample `3d` columns → split → hybrids | Correct A/B/C provenance and replacement columns |
| FAST: validate → frequencies/phases → curve blocks | Block layout and harmonic-bin bounds |
| Iman–Conover: validate → Cholesky → normal scores → ranks → reorder | Each output column preserves the input multiset |
| Plackett–Burman: validate → generating vector → cyclic rows → final row | Two-level entries, balance, and orthogonality |

Grouped Morris particularly needs a validated group-layout boundary: its constructor currently does not check group indices, overlap, or emptiness.

**3. Problem construction**

Source: [problem.rs](/Users/patrickbeam/projects/salib/crates/salib-core/src/problem.rs:205).

The public transition is:

```text
DraftBuilder --add factor/group--> DraftBuilder
DraftBuilder --build--> Problem | BuildError
```

The builder is consumed by `build`. Validation stages are internal rather than explicit states:

```text
Nonempty → UniqueNames → ValidDistributionsAndKinds → ValidGroups → Built
```

Actual checks, in order:

- At least one factor.
- Unique factor names.
- Distribution parameter inequalities.
- Categorical level count greater than zero.
- Nonempty groups.
- Group indices within the final factor count.
- No repeated ownership of a factor, including duplicate indices within one group.

Distribution checks cover ordered bounds, positive scale/shape/rate parameters, triangular mode bounds, Bernoulli probability range, and discrete bound ordering.

Important limits:

- There is no general finiteness check. For example, Normal `mu = NaN` is not examined, and several positive-infinity parameters pass.
- Factor kind and distribution are not checked for compatibility.
- Group names need not be unique; groups need not cover all factors.
- Groups may reference factors added later: indices are checked only during `build`.

**The claimed validated-type invariant does not hold.** `Problem.factors` and `Problem.groups` are public and mutable, and derived deserialization bypasses the builder.

**ADT opportunity:** `ProblemDraft → ValidatedProblem`, with private fields and validation on deserialization. Distribution/kind compatibility could be represented structurally, such as Boolean factors carrying a probability and categorical factors carrying validated levels.

**Stateright:** bound factor names, distribution parameter classes, and group indices. Explore different addition orders, build attempts, deserialization, and post-build mutations. Check:

- Every successful build satisfies the implemented validation predicate.
- Insertion order is preserved.
- Error precedence matches the code.
- Claimed validity survives every permitted operation.

The last property should currently produce counterexamples through mutation or deserialization. Stronger finiteness and kind-compatibility properties would also fail with small concrete inputs.

**4. Estimator pipelines**

Sources: [saltelli2010.rs](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/saltelli2010.rs:69), [bootstrap.rs](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/bootstrap.rs:73), [jansen.rs](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/jansen.rs:97), [janon.rs](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/janon.rs:127).

A useful pipeline model is:

```text
DesignReady → Evaluating(completed slots) → OutputsComplete
            → MomentsComputed → VarianceClassified
            → FirstTotalComputed → OptionalSecondOrder → Published
```

Bootstrap adds:

```text
OutputsComplete → Resampling(k, row_indices)
                → ReplicateIndices → Percentiles → WithCI
```

Distribution mapping is a caller-level step; the Saltelli builder itself produces unit-cube samples.

Core invariants:

- Every output has an identity: campaign, matrix role, hybrid index, and row.
- Output lengths match `n`; hybrid counts match effect count.
- Estimation starts only when all required outputs exist.
- Second-order computation requires complete BA outputs.
- Bootstrap applies the **same row-index vector** to A, B, and every AB series.
- Results contain the expected vector lengths.
- Second-order row `i` has length `effect_count − i − 1`.
- Reduction structure remains fixed if bitwise reproducibility is required.
- Publication handles zero variance, nonfinite outputs, and invalid shapes explicitly.

Do **not** assert that all estimated indices lie in `[0,1]`, sum to one, or satisfy `S1 ≤ ST`: finite-sample estimates need not obey population identities.

Current counterexample opportunities:

- Matrix-based Saltelli divides by variance without a zero-variance guard.
- Cached-output Saltelli returns zeros near zero variance, but accepts zero-length output series.
- Jansen/Janon guard first-order denominators, while second-order computation still divides by total variance without that guard.
- Bootstrap replicates can have zero variance even when the original data does not.
- Bootstrap does not validate `alpha`.
- The model-based bootstrap wrapper evaluates A/B/AB for its cache, then evaluates them again through the point estimator.
- `Fn` does not enforce model purity; interior mutation can violate replay determinism.
- Public mutable matrix/result fields allow inconsistent metadata and shapes.

**ADT opportunity:** `PendingOutputs`, `CompleteOutputs`, and `ValidatedOutputs`, with explicit outcomes such as `Indices`, `DegenerateVariance`, and `InvalidOutputs`. A sum type for first/total versus second-order outputs would replace several implicit `Option` dependencies.

**Stateright:** model a small evaluation campaign and explore arbitrary result arrival order, duplicate results, stale campaign IDs, retries, and checkpoints. Require that incomplete or mismatched outputs cannot reach estimation or publication. These asynchronous actions model a prospective orchestration layer; current estimators evaluate synchronously.

For bootstrap, enumerate small row-index vectors and verify alignment. Represent numerical outcomes as finite categories—valid variance, degenerate variance, nonfinite—while using Kani for the arithmetic branches.

**How to structure the Stateright models**

Use separate bounded models for these four concerns, implementing `init_states`, `actions`, and `next_state`. State representations should use integers, enums, bitsets, and symbolic cells; actual float values can be stored as bits where necessary. Stateright’s [`Model` API](https://docs.rs/stateright/latest/stateright/trait.Model.html) supports this directly.

Use `always` for safety and `sometimes` to demonstrate reachable success/error cases. Use `eventually` only with a finite, progressing model: Stateright documents a limitation for cyclic paths, relevant to retry and repeated-resume protocols. [Property documentation](https://docs.rs/stateright/latest/stateright/struct.Property.html).

**Pure functions suitable for Kani**

These are the strongest bounded targets. Some useful kernels are currently embedded in larger functions and would benefit from extraction.

| Target | Domain and property |
|---|---|
| `validate_distribution` / `ProblemBuilder::build` | A few factors/groups; symbolic parameter values. Verify validation, bounds safety, and exact error precedence. Include NaN/infinity separately. |
| `discrete_uniform_quantile` | Symbolic `i64` bounds: detect overflow in `hi − lo + 1`. Small safe ranges: verify endpoints, monotonicity, and support membership. |
| `bernoulli_quantile` | Valid probability and bounded unit inputs: verify binary output and inclusive threshold behavior. |
| Sobol direction recurrence and step indexing | Finite table rows and symbolic generation index: verify shifts, indices, and recurrence. Target `k = 2^32` explicitly. |
| LHS permutation/fill kernel | Small `n` with symbolic draw words: verify permutation preservation, strata, value bounds, and draw counts. |
| Saltelli/Owen split-and-swap kernels | Small symbolic matrices: verify exact cell provenance, shapes, and hybrid counts. |
| `complementary_frequencies` and FAST sizing arithmetic | Verify frequency range, distinctness where promised, and harmonic-bin bounds. Check overflow and narrowing separately. |
| `ordinal_ranks` | Short arrays from a small finite value set: verify ranks form `1..=n` and ties follow input order. |
| Plackett–Burman construction | All supported dimensions `1..=23`: verify entries, column balance, and pairwise orthogonality. |
| `percentile_value` / `percentile_ci` | Short finite arrays and `0 ≤ low ≤ high ≤ 1`: verify bounds, endpoint behavior, and ordered intervals. |
| `tree_sum`, `tree_dot`, `tree_var` | Short arrays: verify indexing, odd-tail preservation, fixed reduction semantics, and edge cases. |
| Cached-output estimator functions | Small exact-valued arrays: verify shape handling, degenerate branches, result lengths, and equivalence with model-based paths under shared preconditions. |

Two particularly valuable Kani counterexample targets are the accepted full-width discrete-uniform bounds and FAST configurations with harmonic order above 32: the FAST builder permits them, but the estimator uses a 32-element harmonic buffer.

Use symbolic inputs, explicit domain assumptions, and sufficient loop-unwinding bounds. A successful harness establishes the property only over its stated domain; small bounds do not establish unbounded correctness. [Kani harness and unwinding documentation](https://model-checking.github.io/kani/reference/attributes.html).

My priority order would be **validated problem/design boundaries**, **estimator completeness and degenerate outcomes**, then **RNG provenance/resume**. Those address concrete gaps already visible in the implementation and give Stateright useful protocols to explore.