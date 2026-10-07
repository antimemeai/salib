# Independent correctness review, 2026-10-07

This review examines the combined uncommitted implementation and documentation
changes for 0.3.0. It is separate from the agents that implemented the fixes.
The reviewer read the original literature audit, lane journals, production diffs,
new analytic and metamorphic tests, and release notes. No production code was
edited by this reviewer. Final recheck status is recorded below after integration.

## Initial findings

| Priority | Finding | Direct evidence and required correction |
|---|---|---|
| P1 | HDMR returns successful nonfinite indices. | With one Normal(0,1) input, `x=0..15`, degree/order 1, all-NaN outputs return `Ok` with NaN coefficients and indices. Finite outputs `y_i=i*1e200` return `Ok` with infinite variance and NaN indices. Reject nonfinite observations before fitting and nonfinite coefficient-derived variance before division. A successful result must carry finite meaningful values. |
| P1 | LARS advances before admitting all tied maximal correlations. | The mathematical active set includes every maximal absolute correlation. The implementation selected one and discarded zero-length join steps; a tied-predictor permutation changed the next selected variable. Admit the tied knot set before a positive equiangular step and cover the independently calculated fixture below. |
| P2 | Borgonovo silently becomes zero after finite output scaling. | For 64 observations, `X_i=floor(i/32)` and `Y_i=2X_i+(i mod32)/32`, the API reports .5113350636034162. Scaling Y by `1e200`, with every observation and its range still finite, reports zero. Squared residuals overflow in the KDE bandwidth and infinite bandwidth turns both densities into zero. Normalize by an affine output scale or report numerical failure; never report a finite false zero. |
| P2 | QOSA's successful expected-shortfall diagnostic can be infinite. | Eight zero outputs followed by eight `1e308` outputs at alpha=`1e-308` yield finite empirical loss near .5 and successful indices, but `global_cte=inf`. The excess sum overflows before its representable mean is formed. Form a scaled mean or return an explicit numerical error; retain the proper expected-shortfall definition for atoms. |
| P2 | Surrogate README still describes the removed HDMR behavior. | The opening paragraph said unsupported bounded distributions could be accepted and yield wrong indices. Change it to the new `UnsupportedDistribution` rejection behavior, matching the method guide and release notes. |
| P3 | Spectral error wording mislabels numerical overflow as a constant model. | FAST and RBD reject nonfinite total spectral variance as well as below-floor variance, but their existing error text only described a constant output. Clarify the accepted error meaning without changing the enum. |

The public API reproductions were run from an isolated temporary Cargo project
using the actual workspace crates and installed Rust 1.95.0, with no new
dependency adopted by the library. The extreme-scale cases are finite-input
numerical failure tests, rather than population Monte Carlo comparisons.

### Independent LARS knot calculation

Let a,b,c,d be four orthogonal, mean-zero Walsh columns of length 64, each with
entries +/-1. Use predictors `(a,b,c,z/2)`, where

```
z = .36a - .04b - .65c - sqrt(.4463)d
y = a + b - .43c - .2d
```

All predictors lie in Legendre's canonical range. Normalizing them gives initial
correlations proportional to `(1,1,-.43,r)`, with
`r=.32+.2795+.2*sqrt(.4463)`, approximately .73311. Both a and b enter the
first knot. Along their joint direction, the next knot occurs at
`t=(1-r)/(1-.32)`, giving common correlation `1-t`, approximately .6075.
This exceeds .43, so z joins before c. This is a direct specialization of
[Efron et al. (2004), Eqs. 2.9 and 2.13](https://arxiv.org/pdf/math/0406456),
read from the original paper during this review.

With degree 1 and a four-term budget including the intercept, the pre-recheck
API selected z in one column order and c after swapping a,b. Its reported PRESS
scores were .0313366525395057 and .045511111111111155, respectively. Mapping the
coefficients back to the original inputs made the changed support observable.
The full active-set knot must not depend on which tied input is listed first.

## Reviewed corrections and oracle quality

- Average ranks correctly replace ordinal tie breaking in regression. Balanced
  independent binary data provides a zero-association oracle, and joint row
  permutations attack the former arbitrary association.
- Conditioning classes preserve equal-value blocks across PAWN, given-data
  Sobol', QOSA and Borgonovo. Constant-input, independent-input and exact atomic
  information fixtures exercise the mathematical behavior rather than only
  checking ranges.
- QOSA globally and conditionally minimizes empirical pinball loss. Quantile
  atoms and positive affine transformations have direct exact/mathematical
  oracles. In-sample optimism and the fixed class cap are documented limitations.
- Weighted one-sided Fourier power satisfies the discrete Parseval identity,
  with even-length Nyquist counted once. FAST band fixtures and inverse-EASI
  RBD fixtures check actual estimator behavior. Odd/even cosine checks cover the
  triangular traversal independently of the original increasing-sort code.
- HDMR's distribution guard avoids pretending that an affine support mapping
  changes a nonuniform probability measure. Uniform/Normal linear fits retain
  their independently calculated population coefficient variances.
- Sparse selection centers/scales columns while OLS uses the original basis and
  intercept. PRESS is checked against explicit deletion fits, and a leverage-one
  observation invalidates the whole diagnostic instead of being dropped.
- The new Borgonovo Gaussian-mixture oracle follows directly from the index
  definition: balanced conditional N(-mu,1)/N(mu,1) gives
  `delta=Phi(mu)-1/2`. Its finite KDE tolerance is described as configuration
  accuracy, without asserting a universal convergence or bias bound.

## Release assessment considerations

No blocker follows from the documented class-count cap, fixed KDE grid, limited
HDMR order reporting, fixed sparse candidate basis or uncorrected PRESS alone.
They remain approximation/API choices; population theorems must not be presented
as finite-sample correctness guarantees for those choices.

The move to path-only workspace dev-dependencies is Cargo's documented solution
to unpublished cyclic dev-dependencies during workspace packaging. Package
verification ordinarily builds the library, not the sibling-dependent test
suite. A published archive can consequently require a repository checkout for
its full tests; the release assessment must state that limitation honestly.

Rust 1.87 verification with the repository lockfile and fresh library-consumer
resolution are distinct claims. A lockfile downgrade of transitive packages does
not force a consumer's resolver to retain those versions. Assess and document
optional-feature resolution using the actual declared compiler version.

## Final recheck

All six findings are resolved. The reviewer inspected the corrective diffs and
reran the original actual-API witnesses after implementation:

- QOSA now reports finite `global_cte=5e307` for the extreme finite-tail fixture;
  `global_loss` remains .49999999999999994 and the index remains .75.
- Borgonovo returns .5113350636034161 for both the baseline observations and
  their `1e200` scaling. The change normalizes the common output coordinate
  before bandwidth and density arithmetic; it does not widen a test tolerance.
- HDMR returns `PceFitFailed(NonFiniteInput)` for NaN responses and
  `PceFitFailed(NonFiniteFit)` for finite responses with overflowing coefficient
  variance. Shared PCE fitting validation also catches invalid observations
  before debug domain assertions and invalid intermediate matrix arithmetic.
- LARS selects a,b,z in both tied column orders, with identical mapped
  coefficients and PRESS=.0313366525395057. All maximal correlations join before
  a positive step. If the entire tied group cannot fit the term budget, the
  previous best model is retained; a singular joined group returns an explicit
  design error. These policies are documented rather than silently splitting
  the group by input order.
- The surrogate README describes unsupported-measure rejection. Both spectral
  errors now describe too-small or nonfinite variance.

The implementation agent additionally reported 81 passing surrogate unit tests
and eight passing HDMR tests, including the mathematical knot and error cases.
The reviewer independently confirmed the numerical failures are repaired through
the public APIs; this statement does not substitute for the root's broader suite.

No unresolved correctness blocker remains in the reviewed changes. Push readiness
and publication readiness still depend on the root's complete workspace, feature,
MSRV, formatting and package checks, recorded in the
[release assessment](2026-10-07-release-assessment.md). In particular, the reviewed
MSRV wording now distinguishes Rust 1.87 default/full support and audited locked
optional builds from fresh Polars resolution, which currently requires 1.88.
The approximation and package-test limitations above remain explicit scope
qualifications rather than claims of general estimator correctness.

### Narrow recheck after full-suite integration

The full suite exposed a historical QOSA test that deliberately expected the
old shift-dependent strict-tail results. Its replacement now checks empirical
median pinball loss and positive affine invariance. The reviewer independently
calculated the target using exact rational arithmetic: Y=1..64 has global loss
sum 512; its 21,21,22-observation classes have loss sums 55,55,60.5. Therefore
the index is `1-170.5/512=683/1024`, the mean loss is 8, and the upper-half
expected shortfall is 48.5. These match the replacement's assertions. The test
does not bless the previous broken result or relax its tolerance.

The final release helper defaults to an actual Cargo publish dry run, preserves
the existing versions, verifies full/serde features, and uploads only with the
explicit `--publish` option and a clean Git tree. Shell syntax validation passed.
The manifest changes remove versions only from workspace dev-dependencies, so
Cargo omits those edges from published manifests; runtime version requirements
remain unchanged. This resolves the unpublished development-cycle issue without
changing library dependencies. The repository-checkout requirement for sibling
test fixtures remains the stated packaging limitation. No new blocker emerged
from this narrow recheck; final dry-run completion belongs to the root's release
assessment.
