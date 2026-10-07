# Surrogate corrections, 2026-10-07

This implementation follow-up supersedes the outstanding-code status in the
[surrogate audit](2026-10-07-surrogate-audit.md). It does not claim a full
adaptive sparse-PCE algorithm or support for every input distribution.

## Design and mathematical oracles

1. **HDMR supports only Uniform and Normal factors.**
   [Xiu and Karniadakis (2002), Table 4.1](https://doi.org/10.1137/S1064827501387826)
   pairs Legendre with Uniform and Hermite with Normal. The unsupported-measure
   guard rejects other distributions instead of computing incorrect Sobol'
   weights. Shapes are validated before physical-to-canonical indexing.
   The red tests exposed accepted triangular inputs and an out-of-bounds panic
   for an input matrix narrower than the problem. A linear Uniform[2,8] plus
   Normal(10,2) model checks variance 7 and indices (3/7,4/7) exactly; its
   regression has no truncation or Monte Carlo error.

2. **LARS centers and unit-normalizes predictors and centers the response.**
   [Efron et al. (2004), Eq. 1.1](https://arxiv.org/pdf/math/0406456) specifies
   those transformations. OLS refits retain original polynomial columns and
   their intercept, so returned coefficients retain their units. Constant
   predictors are excluded from the selection pool. The first red fixture
   selected the second predictor when a mean offset hid the analytically
   strongest predictor. Orthogonal a,b,c and y=3a+2b+0.5c give the first
   knot gamma=sqrt(N) and the first active predictors a,b.
   A separate correlated fixture uses z=0.8a+0.6b and y=3a+0.5b+c:
   standardized correlations sqrt(N)*(3,2.7,1) yield the first knot
   gamma=1.5sqrt(N) from Eq. 2.13, so LARS adds z before c.
   OMP adds c instead. The test checks these analytically derived active sets
   and OLS coefficients, not internal knot instrumentation.

3. **OMP uses centered unit-length predictor scores.**
   Its always-active intercept makes centered predictors the relevant
   directions; normalized scores remove dependence on arbitrary column units.
   A red metamorphic test showed that multiplying the weaker orthogonal
   predictor by 100 changed the selected column. It now preserves selection.

4. **PRESS invalidates the entire candidate when any deletion is singular.**
   For a fixed OLS basis, the usual leave-one-out residual is
   (y_i-yhat_i)/(1-h_ii), as in
   [Blatman and Sudret (2011), §4.2.2](https://doi.org/10.1016/j.jcp.2010.12.021).
   A denominator <=1e-10 or nonfinite denominator gives that candidate an
   infinite score. It cannot displace a valid best fit. If no candidate has a
   finite score, sparse fitting returns the additive non-exhaustive error
   `PceError::UndefinedLooError`.
   The red singular-deletion fixture returned 1.5 by hiding its leverage-one
   row. It now returns infinity. A distinct finite-design test compares PRESS
   directly against actual leave-one-out OLS refits. A public fitting test
   checks that invalid diagnostics never become successful fits.

## Scope retained

`max_order` remains the length limit for HDMR order reporting. It does not
truncate fitted interactions. Sparse PCE remains fixed-basis hybrid selection
with uncorrected PRESS and local stopping, rather than the full adaptive
Blatman–Sudret procedure. Both remain documented explicitly.

## Validation

Initial surrogate oracles: 72 passed, 3 failed; HDMR: 5 passed, 2 failed.
After implementation: the 76-test surrogate unit suite passed before adding
one further correlated-path oracle; the final 14-test sparse-PCE unit subset,
including that oracle, passed. All 7 HDMR integration tests and all 11 existing
sparse-PCE integration tests passed. `python3 scripts/check-hdmr-literature.py`
passes Uniform/Uniform and Uniform/Normal analytic controls and explicit
triangular rejection. Scope-local `git diff --check` passed.

Strict all-target serde-enabled surrogate Clippy compiled this lane without
remaining warnings, then failed on concurrent estimator edits (`qosa.rs`
doc-comment whitespace and `fast.rs` parity expression). Those belong to other
owners and were reported for the parent's final workspace lint run; this is
not recorded as a successful overall Clippy run.

Working toolchain: Rust 1.95.0 via rustup; cached dependencies, locked/offline
Cargo runs. No new dependencies, commits, or publication operations.

## Findings from the fresh review

The independent reviewer found two further release blockers; both were treated
as correctness findings rather than documented-away limitations.

- **HDMR nonfinite success.** Normal(0,1), x_i=i for i=0..15 and all-NaN
  outputs returned an `Ok` result containing NaN. Finite y_i=i*1e200 returned
  infinite variance and NaN indices, since the true variance 1e400 exceeds
  f64 range. Red tests confirmed the first failure. Full and sparse PCE now
  share finite-observation validation before canonical assertions, reject
  nonfinite basis/normal-equation/normalization/fit arithmetic, and HDMR rejects
  a nonfinite aggregate variance even when all fitted coefficients are finite.
  The errors are `PceError::NonFiniteInput` and `PceError::NonFiniteFit`;
  HDMR delegates them through `HdmrError::PceFitFailed`.

- **LARS zero-length joining knots.** The previous loop admitted one tied
  predictor, ignored the zero-length knot for the other, and walked in the
  wrong direction. It now admits every inactive predictor whose absolute
  correlation is tied for maximal correlation before moving, as in
  [Efron et al. (2004), Eq. 2.9](https://arxiv.org/pdf/math/0406456).
  Equality tolerance is 64 machine epsilons times the maximal correlation.
  This applies at the initial knot and at subsequent knots. A group that
  cannot fit the remaining term budget stops selection before that group;
  a collinear group deliberately reports the existing singular-design error.
  The budget policy is documented in source and guide.

  The reviewer fixture uses orthogonal Walsh a,b,c,d, a predictor
  z=0.36a-0.04b-0.65c-sqrt(0.4463)d (scaled by 1/2 in the raw matrix), and
  y=a+b-0.43c-0.2d. Both a,b have first correlation 1. Along their bisector,
  z joins at coefficient increment (1-c_z)/0.68 approximately 0.3925;
  the common joining correlation approximately 0.6075 exceeds 0.43.
  The third predictor must therefore be z, not c. The red public-API test
  returned support [a,b,c] instead of [a,b,z]. The corrected test checks
  [a,b,z], mapped support, identical predictions and PRESS after swapping
  a,b, and a separate whole-group budget-stop fixture.

Finite-data correction validation: all 79 surrogate unit tests and 8 HDMR
integration tests passed. After the tied-knot corrections, all 81 surrogate
unit tests passed, including the reviewer fixture and budget-stop oracle.
The parent's final workspace checks and the independent reviewer's public
replay establish the remaining release assessment.
