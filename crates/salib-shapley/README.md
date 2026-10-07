# salib-shapley

Shapley effects allocate output variance, including interactions, among
independent inputs. This crate implements the random-permutation, nested
Monte Carlo algorithm of Song, Nelson, and Staum (2016). It is also available
through `salib` with the `shapley` feature. Use Sobol' estimators if you need
separate main and total effects.

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
salib-core = "0.3"
salib-shapley = "0.3"
```

The estimator samples physical values from the supplied distributions and
passes them to your model. Validate those distributions before use. It requires
`n_var + n_perm * n_outer * n_inner * (d-1)` model calls; increase the permutation
count to reduce Monte Carlo uncertainty.

`sh` is in output-variance units. Divide by `var_y` for dimensionless shares.
The contributions sum to the estimated variance up to rounding and clamping of
tiny negative values, though each contribution still has sampling error. For
reproducible results, the model must return the same output for the same inputs,
even though its closure accepts `FnMut`.

Dependent inputs are not supported. `serde` serializes result types.

[API reference](https://docs.rs/salib-shapley/latest/salib_shapley/).
MIT OR Apache-2.0.
