# Push and release assessment, 2026-10-07

**Release complete: PR #1 merged, v0.3.0 tagged, all eight crates published,
and the GitHub release published.**
The operator authorized shipment with "send it". Commit `96124e8` was pushed and
merged as `e0876d65181580edea653edf7eb90180c5065572`, which is the source tag's
commit. No unresolved blocker remains in the audited implementation or release
checks.

The initial attempt with Cargo's saved token failed before any upload. The
operator then identified `~/projects/.env`; its `crates` entry was passed as
`CARGO_REGISTRY_TOKEN` to the retry process only, without printing or persisting
its value. Publication ran from the clean tagged source and succeeded for
salib-core, salib-samplers, salib-surrogate, salib-estimators, salib-shapley,
salib-validation, salib and salib-cli. The official registry API confirms every
0.3.0 version exists and is not yanked. All eight registry SHA-256 checksums match
the locally uploaded archives, whose VCS metadata identifies the clean tagged
commit.

The [GitHub release](https://github.com/antimemeai/salib/releases/tag/v0.3.0)
is published, with compatibility notes and validation results. The table below
records the pre-shipment validation and availability checks.

## Validation of the final working tree

| Check | Result |
|---|---|
| Workspace tests, full analysis and serde features | **1053 passed, 0 failed, 0 ignored**, including 27 doctests |
| Fresh independent correctness review | Six initial findings corrected and public API witnesses replayed; final narrow test/manifest/helper recheck also passed |
| Strict Clippy | Workspace, all targets, `salib/full,salib/serde`, `-D warnings`: pass |
| Formatting and whitespace | `cargo fmt --all -- --check` and `git diff --check`: pass |
| Public documentation | 26 pages; release examples, fences, local links/anchors, and 36 registered direct citation destinations: pass |
| Rustdoc | Workspace with full analysis/serde features, no dependency docs: pass |
| Mathematical audit scripts | HDMR3 checks, spectral3 checks, QOSA contrast/affine/partition checks, and tied-rank regression example: all pass |
| Declared minimum compiler | Rust 1.87 default workspace check and **final all-features locked workspace check**: pass |
| Publication dry run | **All eight 0.3.0 crates packaged and verified** with full analysis/serde features; every upload aborted by `--dry-run` |
| Archive contents | All eight archives' Rust and README bytes match the final checkout |
| Release versions | All eight publishable crates and runtime sibling requirements use 0.3.0; unpublished salib-models remains 0.2.0 |
| Registry availability | Crates.io API reports latest 0.2.0 for all eight crates, and no existing 0.3.0 release |
| GitHub readiness | PR #1 is open/mergeable against master; authenticated repository permissions include push/maintain/admin; remote PR head remains 805248e |
| Tag availability | Neither v0.3.0 nor 0.3.0 exists on the remote |

The independent review is recorded in
[correctness-review.md](2026-10-07-correctness-review.md). The
[release notes](../docs/release-0.3.0.md) describe behavior changes and errors.
No scientific correctness claim is inferred merely from an HTTP link check or
from agreement with an untraceable external-package reference array.

## Behavior and compatibility

RBD now rejects tied columns; its intended EASI rank construction requires
untied input observations. HDMR accepts automatic Uniform/Normal mappings and
rejects other distributions. QOSA now uses empirical pinball loss, includes
`global_loss` in serialized results, and reports atom-aware expected shortfall.
Older serialized QOSA results need the new field before deserializing to the
new type. Sparse fitting can select different terms after correct centering,
normalization and simultaneous LARS knot handling, and invalid fits return errors.
These are deliberate changes in the 0.3 minor release of a pre-1.0 library.

The capped conditioning partitions, fixed Borgonovo quadrature grid, fixed sparse
candidate basis, uncorrected PRESS diagnostic and HDMR reporting-only max_order
remain documented approximation/API choices. They do not establish universal
consistency or model-level surrogate accuracy. The CLI package remains a stub;
Shapley supports independent marginals only. None is presented as implemented
functionality beyond its current scope.

Default/full analysis features support Rust 1.87. The checked repository lockfile
also supports optional Arrow/Polars on 1.87 after selecting compatible versions
of existing home/psm transitives. **A fresh downstream Polars dependency resolution
currently requires Rust 1.88**, because consumers do not inherit this repository's
lockfile. Public requirements state that distinction. No library was adopted.
The downgrade is a checkout reproducibility choice, not a claim to constrain a
consumer's resolver.

## Packaging and publication procedure

Versioned cross-crate dev dependencies originally created an unpublished 0.3.0
cycle during packaging. They are now path-only workspace test dependencies.
[Cargo's documented publication behavior](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies)
omits dev dependencies without a version from published manifests. Runtime
requirements are unchanged. The full cross-crate test suite therefore needs a
repository checkout; this assessment does not promise that unpacked crate
archives contain a standalone runnable copy of that suite. The benchmark lives
outside the facade crate and is likewise a checkout artifact; Cargo's expected
warning about excluding it is not a missing production source file.

`scripts/publish.sh` uses Cargo's multi-package publisher, verified with Cargo 1.95.
It does not bump versions or require cargo-workspaces. Its default is a real
dry run. `--publish` requires a clean Git tree and explicitly uploads the existing
versions. The dry run proves packaging/build readiness. Registry authorization was
subsequently exercised successfully during actual publication with the workspace
credential identified by the operator.

Reproduce the material checks with an installed toolchain:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
cargo test --workspace --features salib/full,salib/serde --locked --offline --no-fail-fast
cargo clippy --workspace --all-targets --features salib/full,salib/serde --locked --offline -- -D warnings
cargo fmt --all -- --check
cargo doc --workspace --no-deps --features salib/full,salib/serde --locked --offline
cargo +1.87.0 check --workspace --all-features --locked --offline -j 2
python3 scripts/check_docs.py
python3 scripts/check-hdmr-literature.py
python3 scripts/check-frequency-literature.py
python3 scripts/audit_qosa.py --toolchain 1.95.0
cargo run --offline --locked -p salib-estimators --example audit_regression_ties
CARGO_BUILD_JOBS=2 ./scripts/publish.sh --dry-run
```

The explicit publication command is `./scripts/publish.sh --publish`.
Push, merge, source tagging, registry uploads and the GitHub release are
complete. All eight immutable 0.3.0 versions are available; do not republish them.
