# salib-surrogate

Fit polynomial chaos expansions (PCE), compute their Sobol' indices, or find
active subspaces from model gradients. These APIs are also available through
`salib` with the `surrogate` feature. RS-HDMR is in `salib-estimators`, under
its own `surrogate` feature. Its automatic distribution mapping is currently
supported only for independent uniform and normal inputs; other distributions
return `HdmrError::UnsupportedDistribution`.

| API | Purpose |
|---|---|
| `fit_full_pce` → `PolynomialChaos` | Dense least-squares fit over a total-degree polynomial basis |
| `fit_sparse_pce`, `SparseSolver`, `TruncationScheme` | LARS/OMP term selection, with fit diagnostics |
| `PolynomialFamily`, `MultiIndex` | Canonical polynomial families and basis exponents |
| `sobol_indices_from_pce` → `SobolFromPce` | Analytic variance indices from orthogonal coefficients |
| `compute_active_subspace` → `ActiveSubspace` | Eigendecomposition of the uncentered gradient covariance |

## Example

```rust
use ndarray::array;
use salib_surrogate::{fit_full_pce, sobol_indices_from_pce, PolynomialFamily};

// Legendre inputs are canonical Uniform(-1,1), not unit-cube coordinates.
let x = array![[-1.0], [-0.5], [0.0], [0.5], [1.0]];
let y = [-1.0, 0.0, 1.0, 2.0, 3.0]; // y = 1 + 2x.
let pce = fit_full_pce(x.view(), &y, &[PolynomialFamily::Legendre], 1).unwrap();
assert!((pce.evaluate(&[0.25]) - 1.5).abs() < 1e-10);
let indices = sobol_indices_from_pce(&pce).unwrap();
assert!((indices.first_order[0] - 1.0).abs() < 1e-10);
```

```toml
[dependencies]
salib-surrogate = "0.3"
ndarray = "0.16"
```

Map inputs into the canonical domain of each family: Legendre uses `[-1,1]`,
Hermite uses standard-normal coordinates, and other families have their own
weight/domain conventions. The indices describe the fitted surrogate under
those distributions. Check predictions on held-out data and the stability of
the indices before using them to describe the original model. `serde`
serializes fitted models and results.

[API reference](https://docs.rs/salib-surrogate/latest/salib_surrogate/).
MIT OR Apache-2.0.
