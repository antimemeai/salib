# Crate map

The `salib` crate re-exports the library's types and functions. You can also
depend on its component crates separately; each README shows how. There are
eight published crates and one unpublished crate, `salib-models`, for model
checking.

Feature flags control which APIs `salib` exposes. Disabling a feature does not
necessarily remove its crate from the build: another dependency may still use it.

---

## Dependency graph

```
salib  (facade)
├── salib-core           types, distributions, RngState, bit-reproducible reductions
├── salib-samplers       sampling designs (LHS, Sobol', Saltelli, Morris, FAST)
│   └── salib-core
├── salib-estimators     all sensitivity estimators
│   ├── salib-core
│   └── salib-samplers
├── salib-surrogate      PCE, active subspaces        [optional: "surrogate"]
│   └── salib-core
├── salib-shapley        Shapley effects              [optional: "shapley"]
│   └── salib-core
└── salib-validation     test functions (Ishigami, Sobol' G, Morris)  [optional: "validation"]
│   └── salib-core
│
salib-cli               CLI stub                      [separate package]
    ├── salib-core
    ├── salib-samplers
    └── salib-estimators
```

## Crates

### salib-core

Types that everything else depends on.

| Type | Role |
|------|------|
| `Problem` | Factor names, distributions, dimension |
| `ProblemBuilder` | Construct and validate a `Problem` |
| `Factor` | Name + distribution for one input |
| `Distribution` | `Uniform`, `Normal`, `LogNormal`, `Triangular`, `Beta` |
| `RngState` | Serializable ChaCha20 RNG with `fork(salt)` for named streams |
| `tree_sum`, `tree_dot`, `tree_var` | Binary-tree accumulation for bit-reproducible parallel sums |

### salib-samplers

Sampling designs that produce the input matrices for estimators.

| Design | Function / Type |
|--------|----------------|
| Latin Hypercube | `LhsSampler` |
| Sobol' QMC | `SobolSampler` |
| Saltelli cross-matrix | `build_saltelli_matrix` |
| Morris trajectories | `build_morris_trajectories` |
| FAST/eFAST search curves | `build_fast_design` |
| Iman–Conover correlation | `iman_conover_transform` |

### salib-estimators

Designed-sample estimators take a typed design and a model closure. Given-data
estimators take aligned input/output arrays; DGSM takes gradients and variance.
Results have method-specific fields and optional `serde`. HDMR requires
`salib-estimators/surrogate` and depends on `salib-surrogate`.

| Family | Function |
|--------|----------|
| Sobol' (Saltelli 2010) | `estimate_saltelli2010` |
| Sobol' (Jansen) | `estimate_jansen` |
| Sobol' (Janon) | `estimate_janon` |
| Sobol' (Owen) | `estimate_owen` |
| Given-data Sobol' | `estimate_given_data_sobol` |
| Morris | `estimate_morris_effects` |
| Grouped Morris | `estimate_grouped_morris_effects` |
| FAST / eFAST | `estimate_fast` |
| RBD-FAST | `estimate_rbd_fast` |
| Borgonovo δ | `estimate_borgonovo_delta` |
| PAWN | `estimate_pawn` |
| QOSA | `estimate_qosa` |
| DGSM | `estimate_dgsm` |
| SRC / SRRC / PCC / PRCC | `estimate_regression_indices` |
| HDMR | `estimate_hdmr` |
| ANOVA | `estimate_anova_two_way` |
| G-Theory | `estimate_g_theory_pir` |
| Discrepancy | `compute_discrepancy` |
| Fractional factorial | `estimate_fractional_factorial` |

### salib-surrogate

Surrogate models. Optional — enable with `features = ["surrogate"]` on the facade.

| Component | Function |
|-----------|----------|
| Full PCE (OLS) | `fit_full_pce` |
| Sparse PCE (LARS/OMP) | `fit_sparse_pce` |
| Active subspaces | `compute_active_subspace` |

### salib-shapley

Shapley effects. Optional — enable with `features = ["shapley"]`.

| Component | Function |
|-----------|----------|
| Shapley effects | `estimate_shapley` |

### salib-validation

Analytic test functions with closed-form sensitivity indices. Optional — enable with `features = ["validation"]`.

| Function | Factors | Closed-form |
|----------|---------|-------------|
| Ishigami | 3 | $S_i$, $S_{Ti}$ |
| Sobol' G | $d$ | $S_i$, $S_{Ti}$ |
| Morris | $d$ | $\mu^*$, $\sigma$ classification |

### salib-cli

The `salib` binary currently prints “CLI not yet implemented” and exits with
status 2. There are no working `sample`, `run`, or `analyze` commands. Use the
[library tutorial](quickstart.md) for analyses.

### salib-models (unpublished)

Four Stateright models check problem construction, RNG use, Saltelli assembly,
and result collection within finite domains. Run them from the workspace with
`cargo run -p salib-models --release`.

---

## Feature flags

All flags on the `salib` facade crate:

| Flag | Default | Effect |
|------|---------|--------|
| `samplers` | yes | Sampling designs |
| `estimators` | yes | All sensitivity estimators |
| `parallel` | yes | Rayon-based parallel reductions |
| `surrogate` | no | PCE, active subspaces, and HDMR through estimators |
| `shapley` | no | Shapley effects |
| `validation` | no | Analytic test functions |
| `serde` | no | `Serialize`/`Deserialize` on result types |
| `arrow` | no | Arrow `RecordBatch` conversions |
| `polars` | no | Polars `DataFrame` conversions; fresh dependency resolution currently needs Rust 1.88 |
| `full` | no | Everything except `serde`, `arrow`, `polars` |

```toml
# All analysis methods and result serialization
salib = { version = "0.3", features = ["full", "serde"] }

# Minimal analysis features (other dependencies can still enable rayon)
salib = { version = "0.3", default-features = false, features = ["samplers", "estimators"] }
```
