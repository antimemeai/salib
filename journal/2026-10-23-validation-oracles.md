# Sage oracles for salib-validation (crolib-hmz target 3)

The validation functions are the analytic ground truth for salib's Sobol'
estimators. They were checked against four-decimal published values with a
5e-4 tolerance. `verify/sage/validation_analytic.sage` now derives each
function's Sobol' decomposition by symbolic integration of its definition:
- Ishigami over U(−π, π)³, for six (a, b) pairs: D, S_i, S_Ti and S2;
- Sobol' G over U(0, 1)^d, for five a vectors, as a product of mean-one
  factors.

The script emits `tests/oracles/analytic_indices.rs`, and
`tests/analytic_oracle_tests.rs` requires every value to within 8 ulp.

- **Ishigami:** passes on the existing code. Its closed forms are right; Sage
  derives D = π⁸b²/18 + π⁴b/5 + a²/8 + 1/2 from the definition.
- **Sobol' G, failed on the existing code with two defects (salib standing
  rule: fix as found):**
  - Total-order indices were `NaN` placeholders ("not derived in this PR"),
    and two tests enshrined them. They are now `V_i Π_{j≠i}(1 + V_j)/D`, and
    the placeholder tests are removed.
  - `D = Π(1 + V_i) − 1` cancels when the V_i are small. At a = [99, 99, 99, 99]
    the old value was 1594 ulp from exact, measured against the oracle. It is
    now computed as `expm1(Σ ln_1p V_i)`, at 0.9 ulp; the total-order indices
    use the same log-sum.

The oracle is pulled in with `include!`, so rustfmt leaves the generated
source byte-identical and `scripts/sage-check.sh` can detect drift. The
runner and environment files are copied unchanged from the
`fix/g-theory-bootstrap` branch, which is blocked on neuroses.

Dropped crolib-hmz target 2 (committing the EMS and eigenstructure
derivations): it would only test the Sage library, which is a test of a test.
The eigenstructure result belongs to crolib-3bk's finite-universe derivation.
