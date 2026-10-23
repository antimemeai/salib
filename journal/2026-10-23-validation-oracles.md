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

## Review

A fresh reviewer confirmed the values independently: brute-force Sobol' G by
exact quadrature matched bit for bit, as did Ishigami's textbook forms.
Injected bugs miss by 706 ulp to 2e15 ulp. All findings are adopted:

- **The log-sum D was worse at large D.** The `expm1(Σ ln1p V)` form loses
  accuracy as D grows: 65 ulp at d = 50 and 6907 ulp at d = 500 with all
  a = 0 (the Kucherenko type-C setup), against 12 and 121 for the direct
  product. It was also not exact for one factor, so S could exceed 1. D is
  now accumulated as `D ← D + V_k(1 + D)`, a sum of nonnegative terms that is
  exact for d = 1. S_Ti uses the direct product.
  - The oracle adds d = 1, d = 50 and d = 500 cases.
  - The Sobol' G tolerance is now `8 + 3d` ulp, about three rounded
    operations per factor at ε/2 each. A flat 8 ulp is wrong for a d-term
    product (the recurrence is 11.9 ulp at d = 50).
  - The d = 500 case fails the intermediate log-sum version; checked.
- **The Sobol' G oracle shared the product form with the code.** Cases with
  d ≤ 3 are now also derived by integrating the full product
  (D = E g² − 1, S_Ti = (D − Var E[g | x_~i])/D), and the two routes agree
  exactly. Larger cases rely on the product form, and the header says so.
- **Exact inputs.** Ishigami's a and b, and every Sobol' G a, are the exact
  f64 values Rust receives.
- **Smaller fixes.**
  - Module docs no longer say total-order indices are deferred.
  - V_i divides twice, which postpones overflow to a ≈ 1e162; the remaining
    limit is documented.
  - The test checks the vector lengths.
  - `sage-check.sh` requires crolib's library only when a script loads it.
- `docs/tck-retirement-plan.md` is a historical plan and keeps its wording.
