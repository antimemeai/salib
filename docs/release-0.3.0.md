# 0.3.0 release notes

This release corrects numerical defects found while checking implementation
claims against the primary literature. Existing results from the affected
methods may change; rerun analyses that depend on them.

- Regression SRRC/PRCC use average ranks for equal observations. Tied-data
  associations no longer depend on an arbitrary ordering of the ties.
- QOSA estimates empirical quantile pinball loss globally and within conditioning
  classes. It handles point masses and preserves output translation and positive
  scaling within rounding. `global_loss` exposes the denominator. `global_cte`
  now reports empirical expected shortfall with fractional mass at the quantile and overflow-safe weighted averaging,
  rather than an incorrectly normalized strict-tail sum. Serialized QOSA results
  now include `global_loss`; older stored results need that field before
  deserializing as the new type.
- PAWN, Borgonovo, QOSA and given-data Sobol' keep equal input values in the same
  conditioning class. Empty requested classes are omitted; constant inputs give
  one class. Requested slices can therefore exceed the number of actual classes. Borgonovo
  normalizes its output coordinate before KDE to preserve very small and large
  finite affine scales.
- FAST/eFAST and RBD count even-sample Nyquist power once. RBD applies EASI's
  triangular rank traversal before its FFT. RBD rejects tied input columns and
  nonfinite observations; FAST rejects nonfinite model outputs.
- HDMR's automatic input mapping accepts Uniform and Normal distributions only.
  Other measures return `HdmrError::UnsupportedDistribution` instead of producing
  variance indices in the wrong measure. Input/output shape checks return errors. Nonfinite data and overflowing
  polynomial fits or variance calculations return explicit errors.
- Sparse PCE centers and normalizes predictors for LARS and OMP selection.
  LARS admits equal-correlation predictors together at each knot; if the whole
  group exceeds the term budget, selection stops before that group.
  Near-unit or invalid leverage invalidates the entire PRESS score; saturated
  candidates are not scored by dropping observations. If no candidate has a
  finite score, fitting returns `PceError::UndefinedLooError`.

The public documentation now cites the literature directly and distinguishes
population results from finite-sample estimators. Untraceable Borgonovo numbers
and spectral external-package comparisons were replaced with analytic and
finite-vector oracles. New error variants are added to non-exhaustive enums.

Partition estimators still cap conditioning classes at 48, and Borgonovo uses a
fixed density quadrature grid. More data alone need not remove this approximation
error. Sparse PCE uses a fixed candidate basis and uncorrected in-sample PRESS;
it is not the complete adaptive Blatman–Sudret algorithm. HDMR `max_order` limits
the reported decomposition, not fitted interactions. Validate surrogate prediction
error and check sampling/partition choices for the intended analysis.

The CLI package remains a stub. Shapley estimation supports independent marginals
only. The recorded benchmark table describes its stated earlier revision and has
not been rerun for the corrected estimators.

See the [implementation audit](../papers/2026-10-07-literature-audit.md) and
[release assessment](../papers/2026-10-07-release-assessment.md) for the checks,
review findings, and publication status.
