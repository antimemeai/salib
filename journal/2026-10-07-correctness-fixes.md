# Correctness fixes and release assessment

The operator requested implementation of all audited defects, a new independent
subagent review, and a push/release readiness assessment. Preserve the existing
documentation work. Fix the confirmed faults in four conceptual units: rank
regression; conditional-distribution partitions/QOSA; spectral estimators;
surrogate fitting and supported input measures. Each lane studies the existing
primary-source audit, proposes a plan, obtains review, adds failing mathematical
or metamorphic oracles, then implements and reruns the checks. No new library.

Approved plans: average ranks for regression; empirical pinball minima for QOSA
and tie-preserving conditioning classes; multiplicity-weighted FFT power plus
EASI traversal, with explicit rejection of unsupported tied spectral inputs;
HDMR measure guard, centered LARS/normalized OMP, and invalid-leverage PRESS
rejection. Fixed partition caps, HDMR reporting max_order, and the sparse basis
selection strategy are explicitly documented approximation/API choices, not
promises to implement whole new estimators in this release.

Replace unverified numeric comparison fixtures with direct analytic oracles or a
saved independent reference generator. Run the full suite, supported feature
checks, docs, formatting/lints, package checks, and declared-MSRV checks as
appropriate. A separate fresh reviewer must challenge the combined final code
and release claims. Fix all valid findings and recheck before reporting readiness.
Assess push and release separately. Do not push or publish during this task.


Implemented the approved plans and corrected all fresh-review findings. The
independent reviewer found nonfinite HDMR success results, simultaneous LARS knot
handling, overflowing QOSA tail diagnostics, extreme-scale Borgonovo bandwidths,
and two stale wording locations. Added red witnesses, implemented deliberate
validation/stable arithmetic and simultaneous tie admission, then replayed the
reviewer's public API fixtures successfully. The final review reports no
unresolved correctness blocker in its scope. The full suite also revealed a
historical QOSA test explicitly requiring translation dependence; replaced it
with the derived global512/conditional170.5 pinball oracle and affine invariance,
without enlarging tolerances. New QOSA serde fixture includes global_loss.

Packaging failed initially on versioned cyclic workspace dev dependencies.
Cargo's documented path-only dev-dependency handling removes those edges from
published manifests without altering runtime requirements. Retained cross-crate
fixture ownership; package tests require a repository checkout. Replaced the
cargo-workspaces publishing helper with Cargo's own multi-package publish:
default dry run, explicit --publish, clean-tree guard, no automatic version bump.
The first complete dry run was interrupted by a shell parse error because I
edited the running script's comment length while Bash was reading it. The stable
script was syntax-checked and rerun successfully for all eight crates with full
analysis/serde verification. No upload occurred. Archive comparisons confirm
all packaged Rust/README bytes match the working tree.

The repository lockfile's optional Polars transitives originally required Rust
1.88. Selected compatible existing home/psm versions in the lockfile and verified
all workspace features on Rust1.87. This does not constrain downstream consumers;
public requirements now distinguish default/full Rust1.87 from fresh optional
Polars resolution requiring1.88. No library was adopted. Corrected repository
metadata and the CLI package's description to say it is a stub.

Latest completed checks: docs26pages/36registered citations, formatting, strict
workspace/all-target full+serdeClippy, rustdoc, mathematical audit scripts, and
8-crate publishdryrun. Final full regression count and release assessment follow
once the current run finishes. Recorded benchmark timings remain unchanged and
are explicitly identified as predating these correctness changes.

Final validation completed:1053 workspace/full+serde tests passed, including27
doctests; zero failures/ignored. Final strict Clippy and all-features Rust1.87
checks pass against the current source. Formatting, docs26pages/36citations,
rustdoc, mathematical scripts and the stable8-crate full+serde publishdryrun
pass. The final independent reviewer rechecked the corrected QOSA oracle,
manifest dev edges and release helper and found no additional blocker.

Read-only GitHub check confirms push/admin access and an open mergeable PR;
registry names/version0.3.0 remain available, and no remote0.3 tag exists.
`papers/2026-10-07-release-assessment.md` records ready-to-commit/push and release
build readiness, behavior changes, fresh-Polars Rust1.88 qualification, and the
checkout-only cross-crate testing limitation. No commit/push/upload was made.

## Authorized shipment

The operator said "send it" after the readiness assessment. Proceed with
committing the reviewed tree, pushing PR #1, merging its exact reviewed head,
publishing all eight 0.3.0 crates with the checked publisher, and tagging and
creating the GitHub release. Record actual outcomes separately below. Existing
validation above applies to the production source being shipped; registry
publication authorization has not yet been exercised.

## Shipment outcome

Committed the reviewed tree as 96124e8 and pushed release/0.3.0. Updated PR #1
to describe the final implementation and validation, then merged its exact
head as e0876d65181580edea653edf7eb90180c5065572. Fast-forwarded local master;
verified its crates, manifests, lockfile and publisher match the reviewed
commit.

The explicit publisher built and verified all eight crates again, then the
first registry upload (salib-core) failed with HTTP403 "authentication failed".
No upload succeeded. Official crates.io version endpoints returned404 for
all eight0.3.0 versions afterward. Cargo has one saved token, no token
environment override, no alternative configured provider and no repository
publishing workflow. Credential values were never printed. Requested a local
Cargo login refresh from the operator; do not retry the rejected token.

Pushed annotated tag v0.3.0 at the merged reviewed source and prepared a GitHub
release draft explaining the publication blocker. Keep the draft unpublished
until all registry uploads succeed. Publication remains authorized when the
credential is refreshed. Updated the release assessment to reflect actual
shipment status; this follow-up changes only documentation, not released code.
