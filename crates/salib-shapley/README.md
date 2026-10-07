# salib-shapley

Random-permutation Shapley effects using nested Monte Carlo from Song, Nelson,
and Staum (2016). Depend on this crate directly to allocate interaction variance
among **independent** inputs. Use `salib` with `shapley` for the facade API;
use Sobol' estimators when you need separate main and total effects.

| API | Purpose |
|---|---|
| `estimate_shapley` | Evaluate marginal variance contributions along sampled permutations |
| `ShapleyIndices` | Unnormalized per-factor `sh`, output variance `var_y`, permutation count |
| `ShapleyError` | Reject empty factor sets or insufficient sampling counts |

## Example

```rust
use salib_core::{Distribution, RngState};
use salib_shapley::estimate_shapley;

let inputs = vec![Distribution::Uniform { lo: 0.0, hi: 1.0 }; 2];
let mut rng = RngState::from_seed([42; 32]);
// Counts: permutations, outer samples, inner samples, variance samples.
let result = estimate_shapley(
    &inputs, |x| x[0] + x[1], 100, 1, 3, 1024, &mut rng,
).unwrap();
assert_eq!(result.sh.len(), 2);
let shares: Vec<f64> = result.sh.iter().map(|sh| sh / result.var_y).collect();
println!("variance shares: {shares:?}");
```

```toml
[dependencies]
salib-core = "0.2"
salib-shapley = "0.2"
```

This API samples physical input values from the supplied distributions itself;
its model closure does not receive unit-cube coordinates. Validate distributions
before use. The budget is `n_var + n_perm * n_outer * n_inner * (d-1)` model calls.
More permutations reduce Monte Carlo uncertainty. `sh` is in output-variance
units; divide by `var_y` for dimensionless shares. Marginal contributions
telescope to the estimated total variance, up to rounding and tiny-negative
clamping; individual contributions remain uncertain. Keep the model reproducible even though
the closure accepts `FnMut`. Dependent-input conditional sampling is not
implemented. `serde` serializes result types.

[API reference](https://docs.rs/salib-shapley/latest/salib_shapley/).
MIT OR Apache-2.0.
