# Distribution sensitivity literature audit — 2026-10-07

Scope: `docs/methods/distribution.md`, distribution entries in the bibliography,
and the Borgonovo, PAWN and QOSA estimator documentation. Executable algorithms
were inspected but not changed. Source doc comments were corrected in
`crates/salib-estimators/src/{borgonovo,pawn,qosa}.rs`.

## QOSA: estimator attribution and guarantees

[Maume-Deschamps and Niang, *Estimation of quantile oriented sensitivity
indices*](https://arxiv.org/pdf/1702.00925), author preprint, pp. 2–6:
Eq. (2.3) defines the quantile-loss index; Prop. 3.1/Eq. (3.2) rewrites it
through tail expectations. Section 4.1 uses kernel conditional quantiles
(Eq. 4.2) and two independent samples (Eq. 4.3). Proposition 4.1 concerns that
estimator. Publication metadata is *Statistics & Probability Letters* 134
(2018), 122–127, [DOI 10.1016/j.spl.2017.10.019](https://doi.org/10.1016/j.spl.2017.10.019).
The preprint is saved as `maume-deschamps-niang-2018-qosa.pdf`.

Corrections required:

- Replace the bibliography's “partition-based estimator” attribution with a
  kernel-based estimator using independent fitting and evaluation samples.
- The library uses the same sample for ordinal classes and tail evaluation.
  Its class count stops growing at 48. This is an implementation-specific
  approximation, not the paper's estimator.
- Delete the claim that these estimators necessarily converge to the same
  index. A fixed partition generally retains conditioning error; it does not
  approach conditioning on the exact input value. Delete the unsupported
  bound by “class-mean variance.” The source contains no derivation, and
  class means do not determine quantile behavior.
- Present the expected quantile-loss reduction as the primary definition.
  The strict-tail identity requires probability `1-alpha` above the relevant
  quantiles. Continuity of the *marginal* output alone does not ensure this
  for conditional distributions. Finite first moment and a positive contrast
  denominator also matter. The contrast definition covers degenerate
  conditional laws for which the tail identity is inappropriate.
- The implementation normalizes strict-tail sums by `N(1-alpha)`, not actual
  empirical tail counts. Ties and small class tails can distort estimates;
  clamping to `[0,1]` does not repair bias. No universal `N >= 1024` accuracy
  statement is justified. Inspect tail observations per class, not just
  `N(1-alpha)` globally.
- Do not promise agreement with Sobol' rankings at the median, even for
  “monotone or near-monotone” models; these indices optimize different losses.

These are conclusions from comparing the implementation with the source,
not new convergence results. In particular, the old comments' claimed
transfer of Prop. 4.1 to this code has no demonstrated proof.

The public synthetic example also disagrees with the checked-in test. The
actual test uses independent unit-uniform inputs and
`Y = X_1 + 8 X_2 1_{X_3 > 0.95}`, not coefficient 5. It is not an
“Ishigami-derived” model. Its assertions establish `X_1` leads at the median
and `X_3 > X_1` at 0.95, not that `X_3` dominates every other factor.

## PAWN: original paper, tuning and interpretation

[Pianosi and Wagener (2015)](https://doi.org/10.1016/j.envsoft.2015.01.004),
*Environmental Modelling & Software* 67, 1–11, §3.4 p. 7, recommends the
median, with the maximum as complementary information. The claim that the
2015 method uses only the maximum is false. Full article text was read from
[the authors' article on ResearchGate](https://www.researchgate.net/publication/271428340_A_simple_and_efficient_method_for_global_sensitivity_analysis_based_on_cumulative_distribution_functions).
Section 3.1 defines an aggregate of KS distances; §4.3 also restricts the
output region for a regional PAWN analysis. Thus do not claim quantile-focused
analysis is the only conceivable way to study tails.

[Pianosi and Wagener (2018)](https://doi.org/10.1016/j.envsoft.2018.07.019),
*Environmental Modelling & Software* 108, 197–207, introduces the generic
input-output sample approximation with conditioning interval count as its
tuning parameter. The [Bristol author record](https://research-information.bris.ac.uk/en/publications/distribution-based-sensitivity-analysis-from-a-generic-input-outp/)
confirms the title, pages, and purpose. The initial manuscript URL returned
HTTP 403; the [alternate Bristol PDF endpoint](https://research-information.bris.ac.uk/ws/portalfiles/portal/175128594/Pianosi_PAWN_givendata_final.pdf)
later supplied the full accepted manuscript, saved as
`pianosi-wagener-2018-pawn.pdf`. Section 2.3, Eq. (5), manuscript p. 8,
uses equal-width intervals; ordinal equal-frequency classes are this library's
choice. Section 4, manuscript p. 15, recommends starting with ten intervals,
varying that count and checking smaller subsamples. Its example sample sizes
are explicitly not universal accuracy guarantees.

Corrections required:

- Remove the unsupported “Pianosi 2020 recommends S in [10,20]” attribution.
  There is no matching bibliography entry. Describe slice count as a tuning
  parameter and ask readers to examine sensitivity to it and to sample size.
- `N >= 2S` is the implementation's acceptance threshold, not evidence of
  accuracy. `N >= 1024`, stable rankings for every `S in [8,16]`, and universal
  sample-size robustness are not justified by the repository fixtures.
- Remove the assertion that validation requires agreement with Sobol' total
  effects. The checked-in Ishigami PAWN test explicitly expects factor
  ordering `2 > 1 > 3`, whereas Sobol' totals order `1 > 2 > 3`.
- The SALib comparison uses separately generated input matrices and loose
  tolerance, not an identical-data implementation equivalence test. The
  reported 0.007 difference concerns median/max only; it is not a verified
  bound across all statistics or models.
- KS distance separates distinct one-dimensional distributions. Avoid saying
  CDF methods categorically miss density shape changes; metric magnitudes and
  finite-sample resolution differ. Likewise avoid a blanket “most robust”
  recommendation without model-specific evidence.
- Ordinal ranks can split ties by row order. With discrete inputs, rank slices
  need not be distinct conditioning intervals. State that limitation.

## Borgonovo delta: benchmarks and convergence

[Plischke, Borgonovo and Smith (2013)](https://doi.org/10.1016/j.ejor.2012.11.047),
*European Journal of Operational Research* 226(3), 536–550, is correctly
identified bibliographically. Its publisher preview calls the correction a
bias-reducing **bootstrap** estimator, not a jackknife. It reports Ishigami
estimates `0.208, 0.391, 0.156, 0.060` including a dummy factor; these are
estimates, not the claimed analytic `0.214, 0.371, 0.157`.

The authors' [conference account](https://www-10b015.pages.gwdg.de/papers/_03-Mo2-2_-_Plischke.pdf)
is saved as `plischke-borgonovo-smith-conference.pdf`. Its §4, Eqs. (11)–(17),
pp. 4–5, defines class-conditional approximation and quadrature; Theorem 2
requires partitions to become increasingly fine, with maximum class
probability tending to zero. It is a related author source, not the full
journal paper, and its equation numbers differ.

Corrections required:

- The “analytic” Ishigami values have no verified primary derivation in the
  accessed material. Remove the analytic label and unsupported attribution,
  or remove the table until a source/derivation is available. Their presence
  as constants in tests does not establish their provenance.
- Both the 48-class cap and fixed 100-point integration grid limit claims of
  consistency for this implementation. The partition theorem does not
  establish convergence with those limits held fixed.
- Remove universal “N >= 4096 for tight estimates” and fixed bias-correction
  differences. These would require an explicit model, sampling method,
  implementation version and experiment.
- The code uses a normal-reference bandwidth with population variance divisor
  `N`. Its previous claim of exact SciPy bandwidth agreement was removed;
  the implementation detail is now stated directly.
- Density formulas require the relevant densities; singular/discrete outputs
  need qualification. Delta is a distributional dependence measure, not an
  additive variance allocation.

The same unsupported analytic constants remain in
`tests/borgonovo_e2e.rs`; this audit changes documentation, not those tests.
Their status should be recorded as an unresolved validation issue.

## Remaining acquisition/evidence gaps

1. Full journal text of Plischke et al. (2013), including the purported
   Ishigami analytic benchmark provenance. The publisher preview and author
   conference account do not substantiate the repository's triple.
2. No proof or external validation located for this library's capped,
   same-sample QOSA variant or its former finite-sample bias bound.

These gaps are reasons to narrow documentation claims, not to invent a
replacement benchmark or promise.

## Executed counterexamples

Run `python3 scripts/audit_qosa.py --toolchain 1.95.0`. The script temporarily
creates a Cargo example, calls the actual estimator using existing workspace
dependencies, and removes its example. It prints observations rather than
asserting that the limitations are desirable behavior. No estimator code or
new dependencies are installed.

1. **Fixed-partition target (implementation choice, disproves old consistency
   claim).** Let `X ~ U(0,1)`, `Y=X`, `alpha=0.5`. Exact QOSA is 1 because
   knowing X removes all quantile loss. Global pinball loss is 1/8. With K
   equal-width classes, expected conditional pinball loss is `1/(8K)`, so the
   partition target is `1 - 1/K`. Uniform midpoint grids with N=122880 and
   N=245760 both return `0.979166666667 = 47/48`; doubling observations does
   not approach 1 after the 48-class cap. This target discrepancy remains
   even if all numerical calculations are exact.
2. **Atoms (outside the continuous-tail representation's domain).** Let X
   take four equally likely values and determine outputs `[0,1,1,2]`.
   At alpha=0.5, global pinball risk is 1/4 and conditional risk is zero,
   giving QOSA 1. The strict-tail formula instead yields global tail estimate
   1 and global mean 1, so the implemented denominator vanishes and the
   actual API returns `Err(DegenerateTail)`. This does not invalidate the
   population contrast definition; it shows that the implementation's input
   acceptance is wider than the representation's mathematical domain.
3. **Output-shift dependence (finite-sample estimator limitation within a
   continuous model).** For the 64-point midpoint grid of `Y=X` on `[0,1]`
   and alpha=0.9, actual output is `0.7320099255583126`. Replacing Y by Y+100
   returns 0. Quantile loss and both the population and empirical
   class-quantile-loss indices are invariant under this translation. Here
   there are 6 upper-tail observations but nominal tail size is 6.4. The
   computed tail-minus-mean denominator shifts by
   `100 * (6/6.4 - 1) = -6.25`; clamping then conceals the negative estimate.
   This is a concrete reason not to equate the implemented tail approximation
   with an exact empirical quantile-loss ratio. It is not evidence that the
   paper's different estimator has or lacks exact finite-sample invariance.

The raw witnesses first ran successfully with Rust 1.99.0. The retained script
also ran successfully with Rust 1.95.0 and reproduced all three results.
The Homebrew rustc in the inherited PATH aborted due to an LLVM dynamic-link
mismatch; selecting a rustup toolchain avoids that unrelated environment issue.


## Original Borgonovo paper checked

The full [Borgonovo (2007) paper](https://www.relialab.org/Upload/files/A%20new%20uncertainty%20importance%20measure.pdf)
was subsequently obtained and saved as `borgonovo-2007-uncertainty.pdf`.
Equations (19) and (20), p. 775, define the density and general measure
versions of delta. Section 4, p. 777, explicitly uses Ishigami with `a=5`,
`b=0.1` and N=1000; Table 2, p. 778, reports numerical estimates
`[0.33, 0.39, 0.28]`. These are not the repository's `a=7` benchmark triple.
Searching and inspecting the paper did not locate `0.214`, `0.371`, or
`0.157`. This additional primary source therefore does not resolve the
benchmark provenance gap. Metadata: *Reliability Engineering & System
Safety* 92 (2007), 771–784; DOI `10.1016/j.ress.2006.04.015`.
The PDF download required a single-request certificate-verification bypass
because that host presented a self-signed certificate; no environment or
persistent network settings were changed.


## Stacks follow-up and resolved source gaps

Stacks on `neuroses` was accessed over Tailscale SSH. The repository README
and library schema were read, and SQLite was opened with
`file:/srv/stacks/db/library.db?mode=ro`. No server data was changed.

The QOSA paper was found through `chunk_fts` using Maume AND Niang, followed
by a primary-key catalog lookup. Catalog SHA-256:
`4c53dea57f9acfae8a8b05592864566e20ee55be9b26e322238eb6b6c279c745`.
Its retained path is
`/home/patrick/neurotic_library/lib/statistics_probability_and_uncertainty/Estimation of Quantile Oriented Sensitivity Indices - Maume-Deschamps and Niang 2018.pdf`.
It was copied to `papers/stacks-maume-deschamps-niang-2018.pdf`. This is the
HAL manuscript `hal-01448360`, submitted 2 February 2017, with the published
2018 DOI on its cover. Reading §4 confirms the kernel/two-independent-sample
estimator and does not change the QOSA findings above.

A filename search throughout the retained `neurotic_library/lib`, a bounded
search of the original catalog records (paper rowids through 25000), and a
search of the `document` table did not locate the PAWN2018, Fort2016, or
Plischke2013 journal PDFs. These are scoped search results, not a claim that
no copy exists anywhere on the server. Broad metadata queries were cancelled
rather than allowing unbounded scans across the much larger arXiv intake.

PAWN2018 was then acquired from the alternate institutional endpoint linked
above. Its full text resolves the prior tuning-guidance acquisition gap.
Fort's author preprint was also saved as
`papers/fort-klein-rachdi-2016-contrast.pdf` from
[arXiv:1305.2329](https://arxiv.org/pdf/1305.2329). Definition 4.2, Eq. (6),
manuscript p. 6, gives the contrast sensitivity index; §5.1 develops quantile
examples and the distinction from Sobol indices.

Fort's existing journal metadata was **correct**, not an error to fix.
The [publisher record](https://www.tandfonline.com/doi/abs/10.1080/03610926.2014.901369)
confirms Fort, Klein and Rachdi (2016), *Communications in Statistics — Theory
and Methods* 45(15), 4349–4364; DOI `10.1080/03610926.2014.901369`.
The DOI suffix reflects the manuscript history; the issue publication year
remains 2016. The full journal Plischke2013 text and provenance of the
repository's alleged analytic delta triple remain unresolved.


The recovered PAWN2018 source is now linked directly from the method page
and bibliography and registered in `papers/literature-sources.json`. Its URL
returned HTTP 200 through `scripts/check_docs.py`'s reachability function.
The method page identifies the equal-width/equal-frequency difference and
records the verified ten-interval recommendation without an accuracy claim.
The local documentation checker found no citation-registry or edited-page
errors; at this point its sole reported failure was the root audit report
link `papers/2026-10-07-literature-audit.md`, whose target was still being
written by the coordinating agent. Diff whitespace checks passed.
