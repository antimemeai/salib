# salib-core

Foundational input definitions, validated numeric values, RNG state, and
bit-reproducible reductions for global sensitivity analysis. Depend on this
crate directly when defining experiments, writing a custom sampler/estimator,
or replaying random streams without the analysis suite. For a complete analysis,
use the `salib` facade, or add `salib-samplers` and `salib-estimators`.

## Key APIs

| API | Purpose |
|---|---|
| `ProblemBuilder`, `Problem`, `BuildError` | Validate named factor distributions and optional groups; read them in column order |
| `Factor`, `FactorKind`, `Group` | Input metadata and sets of zero-based factor indices |
| `Distribution::quantile` | Map unit-cube coordinates to physical input values |
| `RngState::from_seed`, `fork`, `snapshot` | Start, derive, and save reproducible ChaCha20 streams |
| `FiniteF64`, `Probability` | Checked finite values and probabilities strictly inside `(0,1)` |
| `tree_sum`, `tree_dot`, `tree_var` | Fixed pairwise reduction order; variance uses two passes and divides by `N-1` |
| `par_tree_*` | The same bits with rayon or the serial fallback |

## Example

```rust
use salib_core::{Distribution, ProblemBuilder, RngState, tree_var};

let problem = ProblemBuilder::new()
    .factor("rate", Distribution::Uniform { lo: 2.0, hi: 6.0 })
    .build().unwrap();
let rate = problem.factors()[0].distribution.quantile(0.25);
assert_eq!(rate, 3.0);
let rng = RngState::from_seed([42; 32]);
let bootstrap_rng = rng.fork(b"bootstrap");
assert_eq!(bootstrap_rng, rng.fork(b"bootstrap"));
assert_eq!(tree_var(&[1.0, 2.0, 3.0]), 1.0);
```

`Problem` stores distributions; it does not transform samples automatically.
When loading external serialized problems, rebuild through `ProblemBuilder` to
validate them: derived deserialization does not run its checks.
Record the full RNG state, configuration, and model revision for replay. A fork
does not advance its parent and resets its child's position to zero.

## Dependency and features

```toml
[dependencies]
salib-core = "0.2"
```

`parallel` is enabled by default. Use `default-features = false` for the serial
fallback, subject to Cargo feature unification. Core types support serde without
an opt-in feature. Bit-reproducibility applies to the same binary and platform;
it does not imply cross-platform identical math or statistical accuracy.

[API reference](https://docs.rs/salib-core/latest/salib_core/) ·
[Documentation](https://github.com/antimeme-ai/salib/blob/main/docs/index.md)

MIT OR Apache-2.0.
