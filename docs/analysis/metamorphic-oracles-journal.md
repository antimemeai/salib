# Metamorphic oracle implementation — 2026-10-02

Implemented the 28 relations in `metamorphic-oracles.md` in three integration
test files. Exact identities reuse designs/data with finite-value checks and
1e-10 tolerance; deterministic array relations use exact equality. Population
tests use four independent reproducible LHS streams with N=16,384 and 0.05
tolerance. Relations 11, 12, and 15 also have balanced-grid counterparts.
Relation 22 is included in the sampler suite to cover the entire analysis.

Validation before the final workspace run:

- Exact suite: 23 passing default tests, 24 passing with `surrogate` (HDMR).
- Statistical suite: 3 passing tests.
- Sampler suite: 8 passing tests.

Corrected test issues during iteration: ndarray column selection required
standard-layout conversion for contiguous model evaluation; RBD-FAST needed
32 harmonics instead of four to keep the sorted linear response's spectral
truncation below the population tolerance. No estimator implementation defect
was demonstrated. Adapted to concurrent bootstrap API changes returning Result;
concurrent source changes were left intact.

Final review expanded Owen and cached-S2 permutation checks, asserted bit-exact
retained evaluations with second-order sampling, and checked HDMR canonical
mapping under affine input-unit changes. No new dependencies were needed.

Final `cargo test --workspace` passed: 915 unit/integration tests and two
doctests (917 total), zero failures or ignored tests. The workspace's default
features run 23 exact tests; the separate `surrogate` run passed all 24.
