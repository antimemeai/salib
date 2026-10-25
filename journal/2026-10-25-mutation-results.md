# Mutation characterization of the G-theory and validation oracles

Plan: crolib's `journal/2026-10-25-mutation-plan.md` (crolib-hmz). The tool is
`cargo-mutants` v27.1.0, run on neuroses only, with `-j 4` and
`CARGO_BUILD_JOBS=4`. A missed mutant is either a test gap, which gets a new
test, or equivalent, which is recorded here.

## salib-validation (`sobol_g.rs`, `ishigami.rs`)

| run | commit | caught | missed | unviable |
|---|---|---|---|---|
| 1 | 98c1683 | 106 | 23 | 4 |
| 2 | 610433d | 129 | 0 | 4 |

All 23 survivors were in the Ishigami value and gradient, which no test in
the crate evaluated. `verify/sage/validation_analytic.sage` now derives both
symbolically and pins exact values (`ISHIGAMI_POINTS`).

## salib-estimators `g_theory.rs`

| run | commit | caught | missed | timeout | unviable |
|---|---|---|---|---|---|
| 1 | 98c1683 | 710 | 91 | 1 | 11 |
| 2 | 610433d | 769 | 27 | 2 | 11 |
| 3 (targeted) | 878dd60 | 24 of 25 | 1 | 0 | 0 |

Run 1 left 69 gaps and 22 equivalents. The tests for those gaps found two bugs,
both fixed in 610433d:

- **`open_unit` returned 1.0.** For the top word, k + ½ rounds up to 2⁵³,
  and the normal quantile of 1.0 is infinite. It now uses 52 bits, so every
  value is exact in [2⁻⁵³, 1 − 2⁻⁵³]. The coverage harness had a copy of
  the same code; it is fixed too.
- **The closed-form intervals depended on units.** Satterthwaite and the
  G F interval squared absolute magnitudes. Both returned `None` for
  components above about 1e154 or below about 1e-154, although the module
  promises unit invariance. They now work at unit scale with ν formed from
  ratios. `closed_form_intervals_do_not_depend_on_units` covers scales from
  2⁻⁹⁰⁰ to 2¹⁰²².

The new tests are:
- exact integer ANOVA at a 1e14 offset;
- the snap bound and the observation-resolution floor;
- bootstrap failure accounting (0/0 Phi, fits that fail at extreme scale,
  a single resample);
- a replay of the documented streams, checking each standard error against a
  Welford SD;
- α validation, generator placement and `open_unit` endpoints;
- D-study exact rationals, tie order and `Display` text;
- the F-interval domain edges, including an exact ν̂ = 0.5;
- quantile search over its documented range;
- Sage oracles for the χ² cdf below 1e-10 and for a finite log-space
  Satterthwaite bound.

Run 2's 27 survivors were 20 of the known equivalents, 6 gaps in the new
`unit_exponent` and the ν̂ = 0.5 edge, and one new equivalent. The tests in
878dd60 kill the 6.

**Timeouts.** In `quantile_by_bisection`, `high *= 2` → `/= 2` loops forever.
`*= 2` → `+= 2` brackets linearly and does not finish on the 1e200-scale
quantile. Both are detected as hangs.

### Equivalent mutants (21)

Line numbers are at 878dd60.

- **Bit-identical by argument:**
  - 288 `<`→`<=`: zeroing 0 gives +0 either way, through the `+ 0.0` in `new`.
  - 428 `>`→`>=` and 432 `<`→`<=` (`scale_by_pow2`): at the boundary it is
    the same single multiply, followed by ×1.
  - 418 `>`→`>=` (`unit_exponent`): differs only when every component is
    zero, and then every interval is `None` either way.
  - 783 `|`→`^`: this branch runs only when the bit is clear.
  - 894:12 `>`→`>=`: L = 0 gives ν = 0 or NaN, which the final check
    rejects.
  - 904:21 `&&`→`||` and 904:27 `>`→`>=`: `FisherSnedecor::new` rejects
    ν = ∞, and `nu < 0.5` rejects ν = 0.
  - 923 `>`→`>=`: `high` is a power of two and never equals 1e300.
  - 929 `||`→`&&`: adjacent-float endpoints are fixed points of the loop.
  - 942 `>`→`>=`: `high` is never 0.
- **Rounding level only:**
  - 519:44 `/`→`%`, 520:24 `+`→`-` and 520:72 `/`→`%`: these move only the
    centring point, and the result does not depend on it beyond rounding.
  - 936 `<`→`<=`: finds the other edge of a flat cdf step, which changes
    the result by about 1e-14 relative.
- **Below one ulp:** 960:38, 960:45, 960:49 (×2) and 960:54. These touch only
  the y² term of the χ² series, about 1e-21 relative. None of 300k random
  (ν, x) samples differed.
- **No counterexample found:** 1412:18 `||`→`&&`. The lower bound is negative
  exactly when the upper bound passes the top; no split appeared in 2.4M
  random and grid cases.

The scratch harness behind these verdicts was not kept. The verdicts that rest
on rounding or sampling are judgement calls, not proofs.
