# Frequency correctness fixes, 2026-10-07

The operator requested implementation of all audited defects and another independent
review before the push/release assessment. The written lane plan was reviewed by
the root owner before implementation.

The frequency invariant is discrete Parseval arithmetic: an ordinary positive
frequency carries both members of its conjugate pair, but even-length Nyquist is
self-conjugate and contributes once. FAST and RBD now share one weighted-spectrum
helper; each total, first-order and complementary band consumes those weights.
RBD additionally uses Plischke's EASI odd-ranks-ascending/even-ranks-descending
traversal. Tied inputs are explicitly rejected instead of creating arbitrary
within-tie ranks. Nonfinite observations get explicit errors; FAST also rejects
nonfinite model output.

Red evidence: the pre-fix cosine API fixture returned .9093929122 at N=4096
instead of 1. The old spectrum helper did not satisfy the new per-bin variance
contract. The original numerical reproduction in the literature report records
FAST .2 and RBD .174193548387 on the Nyquist fixture.

Green evidence (rustup 1.95.0, cached dependencies):

- `cargo test --offline --locked -p salib-estimators --lib fast::`: 27 passed.
  Oracles cover odd/even direct population-divisor variance, pure/mixed Nyquist,
  actual FAST first/complement bands, inverse-EASI RBD traces, cosine indices,
  joint row permutations, tied inputs, and nonfinite values.
- `python3 scripts/check-frequency-literature.py`: all 3 passed; cosine=1,
  corrected RBD Nyquist=.311827956989, FAST Nyquist=1/3.
- `cargo test --offline --locked -p salib-estimators --test fast_e2e
  --test rbd_fast_e2e --test given_data_sobol_e2e`: 13 passed, retaining analytic
  Ishigami targets and existing tolerances.

Removed three comparison tests with unverified frozen SALib arrays (two FAST,
one RBD). Analytic model tests and stronger exact spectral checks cover the
real claims; no tolerance was widened. Source/method documentation and the
script now describe corrected behavior. Remaining finite-M truncation and
frequency aliasing are approximation limits; these fixes do not establish
population consistency for every model. The absolute 1e-15 variance floor is
unchanged and documented.

RBD's newly explicit rejection of tied inputs is a stricter API behavior to
mention in release notes. Both error enums are non-exhaustive. No dependency,
commit, or push was made.
