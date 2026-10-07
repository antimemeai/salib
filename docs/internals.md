# Internals

salib controls random streams and floating-point reduction order so that changing
thread count does not change result bits. Your model must also return the same
output for the same inputs.

## Reproducing result bits

For the same binary and platform, the same ordered inputs, method configuration,
and complete `RngState` yield the same result bits across thread counts when
model evaluations are reproducible. Compare `f64::to_bits()` to check exact
equality.

The seed is only part of the RNG state: stream and word position matter too.
Calling a stochastic sampler twice on one mutable state usually gives different
samples; replaying its saved **pre-call** state reproduces the first draw.
Unscrambled `SobolSampler` is an exception: it restarts its sequence on each call
and consumes no RNG, so two calls repeat the points and changing the seed has no
effect. There is currently no sequence continuation or scrambling API.

```rust
use salib::{RngState, samplers::{LhsSampler, Sampler}};

let sampler = LhsSampler::classic(3);
let mut rng = RngState::from_seed([42; 32]);
let before = rng.clone();
let first = sampler.unit_sample(128, &mut rng);
let mut replay_rng = before;
let replay = sampler.unit_sample(128, &mut replay_rng);
assert!(first.iter().zip(replay.iter())
    .all(|(a, b)| a.to_bits() == b.to_bits()));
assert_eq!(rng, replay_rng); // Same final stream position, too.
```

## Why floating-point order matters

For example, `(1e16 + -1e16) + 1.0` is `1.0`, while
`1e16 + (-1e16 + 1.0)` is `0.0` with ordinary `f64` arithmetic. Both schedules
are mathematically valid reductions, but the rounding differs. A parallel
reduction whose grouping follows worker scheduling can therefore alter a final
sensitivity index even when every model evaluation is identical.

## The fixed tree

`tree_sum` combines adjacent values in rounds. An unpaired trailing value passes
to the next round without being combined with a synthetic zero. For five values:

```text
[a, b, c, d, e]
      ↓
[a+b, c+d, e]
      ↓
[(a+b)+(c+d), e]
      ↓
[((a+b)+(c+d))+e]
```

This order is part of the numerical algorithm. It is independent of thread count
and differs from a left fold `((((a+b)+c)+d)+e)`. It also preserves a singleton’s
bits (including negative zero); empty sums return positive zero.

`par_tree_sum` partitions the ordered slice into `BLOCK=4096` consecutive
chunks. Each chunk uses `tree_sum`, and an indexed collection places each chunk
result at its original position. Those chunk sums then use `tree_sum` again.
Because the block size is a power of two, chunk boundaries coincide with levels
of the sequential tree, including a partial final chunk. Thus the grouping is
the same as `tree_sum` over the full slice. Rayon can compute chunks in any order
without changing the arithmetic tree. Changing `BLOCK` is a numerical change
that requires rechecking this relationship.

`tree_dot` builds the ordered products before summing them. `tree_var` computes
**two-pass unbiased sample variance**: first the tree mean, then the tree sum of
squared deviations divided by $N-1$. It returns zero for fewer than two values.
It is not a Welford accumulator. Individual estimators may use population
normalization ($N$) or other paper-specific formulas; `tree_var` does not define
every estimator’s denominator.

## Parallel execution

- Rayon chooses where chunks run; salib chooses which values combine and in
  which order. The `par_tree_*` functions retain their tree with one worker,
  multiple workers, and with the `parallel` feature disabled.
- `parallel` enables parallel **reduction primitives**. It does not promise
  parallel model evaluations: the main estimator closures currently run in
  sequential row loops. Use an external evaluation pipeline for an expensive
  simulator, then pass cached outputs to a suitable estimator.
- If you evaluate in parallel yourself, store outputs by original row index.
  Completion order must not become input order. Supply reproducible model
  outputs; clocks, shared mutable accumulators, and external random streams are
  outside salib’s control.

With the default `parallel` feature, `RAYON_NUM_THREADS=1` and
`RAYON_NUM_THREADS=8` should leave bits unchanged for the same computation. A
build that omits rayon uses the same reduction grouping:

```toml
[dependencies]
salib = { version = "0.3", default-features = false, features = ["samplers", "estimators"] }
```

Cargo features are additive: this omits rayon only if no other dependency enables
`salib-core/parallel`. See the [crate map](crates.md).

## RNG streams and replay

`RngState` records the ChaCha20 seed, stream, and word position. A sampler that
consumes randomness updates the mutable state to its final position. Forking
uses `rng.fork(salt)`, not `split()`:

```rust
use salib::RngState;
let parent = RngState::from_seed([7; 32]);
let sampling = parent.fork(b"sampling");
let bootstrap = parent.fork(b"bootstrap");
assert_eq!(sampling, parent.fork(b"sampling"));
assert_ne!(sampling.stream, bootstrap.stream);
```

A fork derives a child stream from the parent stream and salt, preserving the
seed and resetting the child word position to zero. It does not advance the
parent and does not incorporate its word position: the same parent stream and
salt select the same child. Use stable distinct salts for separate jobs; do not
assign streams by whichever worker happens to request work first. Stream hashes
are reproducible identifiers, not a mathematical proof of collision freedom.

## Configuration hashes

`Problem::content_hash()` and `Sampler::config_hash()` hash their serialized
configuration using SHA-256. They identify configurations, not complete runs.
They do not include the model implementation, sample count, estimator settings,
bootstrap settings, or RNG position. Save those separately with the hashes.
Current serialization has fixed struct-field order; no cross-version hash
stability is promised if fields or serialization change.

## Verification and limits

Analytic-reference tests check estimates against known answers; 28 metamorphic
oracles check relations such as scaling invariance and factor permutations.
Kani harnesses in core, samplers, and estimators check bounded quantile,
reduction, Sobol' step, and percentile properties. Four discovered bugs were
fixed and covered by regression tests. Stateright explores four finite protocol models
for problem construction, RNG use, Saltelli assembly, and estimator completeness.
Run those models with `cargo run -p salib-models --release`. The result-collection model describes a proposed asynchronous protocol.
Current estimators read arrays synchronously. These checks do not prove statistical accuracy for every possible model.

Bit-reproducibility is distinct from accuracy or convergence. It does not promise
identical bits across CPU architectures, compiler or dependency versions,
optimization settings, or library releases. Math functions and linear algebra
can vary with those choices. Nor does it promise byte-identical results to Python
SALib. Pin the environment and model revision for strict replay; use statistical
validation to assess whether a reproducible estimate answers your question.
