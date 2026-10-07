# salib-validation

Analytic reference models and closed-form sensitivity indices for testing and
benchmarking. Depend on this crate directly as a development dependency when
checking an estimator or integration against known answers. Application analyses
usually use their own model; the `salib` facade exposes these references through
its optional `validation` feature.

| API | Purpose |
|---|---|
| `ishigami` module | Three-factor nonlinear function, gradient, analytic Sobol' indices, and `[-π,π]` input problem |
| `sobol_g` module | Tunable-dimensional product function and analytic variance indices on `[0,1]` |
| `morris_test` module | Additive linear/quadratic functions and analytic elementary effects |
| `SobolIndicesAnalytic`, `MorrisEffectsAnalytic` | Reference values to compare with sampled estimates |

## Example

```rust
use salib_validation::ishigami;

let problem = ishigami::input_distribution();
let reference = ishigami::analytic_indices(7.0, 0.1);
assert_eq!(problem.dim(), 3);
assert!(reference.first_order[2].abs() < 1e-12);
assert!(reference.total_order[2] > 0.24); // x3 matters through interaction.
assert_eq!(ishigami::ishigami(&[0.0, 0.0, 0.0]), 0.0);
```

```toml
[dev-dependencies]
salib-validation = "0.2"
```

Use the canonical input distributions when comparing indices. Agreement should
be assessed within justified sampling tolerances; analytic targets are not exact
expected floating-point outputs from a finite sample. The Morris references
here are additive fixtures, not the full original 20-factor Morris function.
`serde` serializes analytic result types.

[API reference](https://docs.rs/salib-validation/latest/salib_validation/).
MIT OR Apache-2.0.
