# Literature and implementation audit, 2026-10-07

This audit covers the public method guides, READMEs, bibliography, and corresponding
implementation claims at PR #1 head `805248ee4949afe57ee309d84015203a61348cd6`.
The PR itself changes release versions; the defects below already existed in its
base. Documentation and the confirmed implementation defects are corrected in this
working tree following the operator's request to implement the fixes. No
dependency was added. The finding table below records the original red evidence;
[release notes](../docs/release-0.3.0.md) describe the changed behavior and
[release assessment](2026-10-07-release-assessment.md) records final validation.

## Original confirmed code problems and fix plan

| Priority | Finding and evidence | Proposed fix and acceptance check |
|---|---|---|
| High | **HDMR uses the wrong probability measure for bounded nonuniform inputs.** For independent Uniform[-1,1] and symmetric Triangular[-1,1], the exactly linear model Y=X0+X1 has variance 1/2 and indices (2/3,1/3). The API reports 2/3 and (1/2,1/2). | As an immediate guard, reject unsupported distributions and test the expected error while preserving the Uniform/Uniform control. Once matching bases or probability transformations support triangular inputs, require the analytic triangular case to pass. The current `check-hdmr-literature.py` assumes successful output and exposes the existing wrong result; a rejection guard requires an expected-error fixture. |
| High | **SRRC/PRCC mishandle tied ranks.** Independent balanced binary X/Y produce rank coefficients .8; jointly permuting rows gives 0. Raw SRC/PCC are 0. | Use average ranks for ties. Require zero association on this balanced fixture and invariance to joint row permutations. See `audit_regression_ties.rs`; [Marino et al., footnote 3](https://pmc.ncbi.nlm.nih.gov/articles/PMC2570191/) specifies average ranks. |
| High | **QOSA violates output-translation invariance.** For 64 continuous midpoint observations Y=X, alpha=.9, the result is .732009925558; adding 100 to Y gives 0. A valid atomic-output problem can return `DegenerateTail`. | Estimate quantile pinball loss consistently rather than using nominal strict-tail probabilities with empirical quantiles. Specify fitting/evaluation reuse or cross-fitting. Require shift invariance, correct treatment of atoms, and analytic loss fixtures; see `audit_qosa.py`. Merely dividing by observed tail counts does not solve the general contrast problem. |
| High | **FAST and RBD double the even-N Nyquist bin.** A mixed cosine/Nyquist trace has variance 3/2, but spectral arithmetic gives 5/2. FAST returns .2 instead of 1/3; the corrected RBD share is .174193548387 instead of .311827956989. | Count the self-conjugate Nyquist frequency once, consistently in all spectral bands. Check Parseval against direct variance for odd/even lengths, pure Nyquist, and mixed modes. See `check-frequency-literature.py`. |
| Medium | **RBD omits EASI's rank traversal.** For Y=cos(pi X), M=4, N=4097, API=.909393063947 while the triangular-trace control=1. | If EASI conformance is intended, reorder odd ranks ascending then even ranks descending before FFT, as [Plischke 2010 §3](https://www-10b015.pages.gwdg.de/papers/easi_ress.pdf) requires. Test odd/even lengths and ties separately. Tarantola 2006 describes an older increasing-sort approach; this is a discrepancy with EASI specifically. |
| Medium | **LARS centers y but not predictor columns.** This differs from [Efron et al., Eq. 1.1](https://arxiv.org/pdf/math/0406456); the downstream OLS intercept does not correct the selection path. | Center both before normalization and compare a small, independently calculated reference path. Code discrepancy confirmed; effect size has not yet been established by a numerical counterexample. |
| Medium | **PRESS drops rows whose leverage is near one.** The resulting mean is not full leave-one-out error and can guide selection incorrectly. | Report an undefined/infinite diagnostic or a deliberate error for saturated fits, with an explicit policy. Add a fixture containing h_ii=1. Code branch confirmed; no failing numerical fixture yet. |

The regression, HDMR and spectral checks call the actual Rust APIs, using analytic
identities or finite-vector arithmetic. They do not infer correctness from a
second package's agreement. The QOSA script prints counterexamples and asserts
translation invariance and valid atomic-output handling.

## Approximation choices that the old documentation overstated

- Given-data Sobol' and QOSA cap conditioning classes at 48. For Y=X and alpha=.5,
  QOSA remains at 47/48=.979166666667 for N=122880 and 245760, versus population
  index 1. For Y=sin(96*pi*X) on the aligned uniform midpoint grid, 48 class means
  vanish despite population Sobol' index 1. These show conditioning error, not a
  contradiction of the binned formulas. Borgonovo also fixes quadrature at 100
  points. General consistency assertions were removed. A future estimator needs
  configurable/refining partitions, sample-size conditions, and tie handling.
- PAWN's ordinal equal-frequency slices are this implementation's choice; the
  inspected 2018 paper uses equal-width intervals. Ties split by row order can
  change conditioning groups. Prefer value-based groups for discrete inputs and
  expose the conditioning policy; test row-order invariance.
- HDMR `max_order` limits the reported order decomposition, not fitted interaction
  degree. Documented as such; rename or split the parameters if true truncation
  is intended.
- Sparse PCE uses a fixed candidate set, uncorrected PRESS, and local stopping
  rules. It does not implement the complete adaptive Blatman–Sudret procedure.
  OMP's raw inner products are scale dependent. These choices are now explicit;
  normalization and adaptive fitting require separate design and validation.

## Documentation errors corrected

| Area | Correction |
|---|---|
| Variance estimators | Removed the false accusation that Janon's published Eq. 2.8 is wrong. Restricted efficiency claims to the paper's asymptotic i.i.d. exchangeable-pair setting. Corrected Jansen first-order/total-effect descriptions and Owen's singleton first-order scope. |
| Surrogates | PCE identities are exact for the fitted expansion with independent, measure-matched bases, not automatically the original model. Corrected Sudret equation references and the reversed Jacobi/Beta parameter mapping. Corrected fitting complexity. |
| Active subspaces | The approximation bound uses discarded eigenvalue sums and distribution assumptions; a large eigenvalue ratio alone is insufficient. Explained sample-rank limits when gradient count is below dimension. |
| Distribution methods | Removed unsubstantiated “analytic” Borgonovo benchmark values from the guide; corrected PAWN's median/max recommendation and unsupported sample-size thresholds. Distinguished the QOSA population contrast from this binned implementation and the paper's kernel/two-sample estimator. |
| Shapley | Corrected theorem attributions. The permutation variance bound assumes exact coalition costs; nested Monte Carlo introduces additional error. The permutation sum telescopes; floating-point arithmetic/clamping can break the exact sum. |
| DGSM | Population inequalities require regularity and independent inputs. Substituting finite-sample derivative estimates does not produce a certified upper confidence bound. Removed unsupported distribution-solver prescriptions. |
| Regression | Removed the unsupported R²>.7 reliability rule. Corrected Saltelli–Marivoet venue and actual regression complexity. Documented the tied-rank defect. |
| Design/discrepancy | L2-star discrepancy cannot be substituted for star discrepancy in the classical Koksma–Hlawka bound. Unreplicated ANOVA cannot separately estimate every interaction and residual error; generic mean-square ratios do not cover every random/fixed/mixed design. |
| General prose | Updated release examples to 0.3, shortened promotional and repetitive prose, preserved method assumptions, and linked the papers directly. Benchmark timings are preserved. |

The untraceable Borgonovo constants (.214,.371,.157) and spectral external-SALib
comparison arrays were removed. Borgonovo now checks the analytic balanced binary
Gaussian-mixture model: conditional densities N(-mu,1)/N(mu,1) yield
`delta = Phi(mu)-1/2`. It checks independent-factor, row-permutation and output-affine
behavior. Spectral tests check discrete Parseval and single-frequency EASI models.
Existing analytic Ishigami tolerances were not enlarged.

## Evidence and acquisition limits

Detailed source locations, formulas, and recovery commands are in:

- [Variance audit](2026-10-07-variance-audit.md)
- [Frequency audit](2026-10-07-frequency-audit.md)
- [Distribution audit](2026-10-07-distribution-audit.md)
- [Surrogate audit](2026-10-07-surrogate-audit.md)

Stacks on `neuroses` supplied Jansen 1999, Tarantola 2006, Campolongo 2007,
Maume-Deschamps–Niang, earlier Sudret and Blatman–Sudret conference versions, and
Blatman's thesis. Catalog searches were read-only; downloaded PDFs remain ignored.
Several catalog titles/DOIs identify a different version or work, so the reports
record inspected title pages, hashes, and version-specific equation numbers.
Plischke 2013 full journal text and Li 2001 full method text remain acquisition
gaps. The reports distinguish abstract/preview evidence from full text.

Additional primary sources inspected for the remaining methods:

| Source | Relevant evidence |
|---|---|
| [Marino et al. 2008](https://pmc.ncbi.nlm.nih.gov/articles/PMC2570191/) | §2.1 and footnote 3: regression/rank methods and average tied ranks. |
| [Sobol'–Kucherenko 2009](https://doi.org/10.1016/j.matcom.2009.01.023) | Publisher abstract: derivative/variance bounds for independent uniform or normal inputs; not finite-sample coverage. |
| [Roustant, Barthe and Iooss 2017](https://arxiv.org/pdf/1612.03689) | §1: product measures and Poincaré inequalities behind derivative bounds. |
| [Song, Nelson and Staum 2016](https://users.iems.northwestern.edu/~nelsonb/Publications/SongNelsonStaum.pdf) | Algorithm 1, Eq. 10, Theorems 2–3, Appendix B Claim 2: permutation estimation, sum identity, exact-cost variance bound, and NI=3/NO=1 allocation. |
| [NIST two-way ANOVA](https://www.itl.nist.gov/div898/handbook/prc/section4/prc437.htm) | Replication and error degrees of freedom. |
| [NIST Plackett–Burman](https://www.itl.nist.gov/div898/handbook/pri/section3/pri335.htm) | Screening design and confounding. |
| [Brennan generalizability overview](https://www.na-mic.org/w/img_auth.php/1/15/Generalizability_theory_and_example.pdf) | Relative/absolute error components for crossed random-facet designs. |
| [Saltelli–Marivoet institutional record](https://researchportal.sckcen.be/en/publications/non-parametric-statistics-in-sensitivity-analysis-for-model-outpu/) | Correct bibliographic metadata; not a substitute for full-text method validation. |

Hickernell's publication metadata was checked; its full text was not acquired.
The discrepancy correction follows the distinction between the documented norms,
not an asserted rederivation of every theorem in that paper. The source registry
records citation destinations, not a claim that every cited work was read fully.

## Reproducing the checks

Use an installed Rust toolchain and cached workspace dependencies. On this machine,
Homebrew rustc has an LLVM loading failure; rustup 1.95.0 works:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
python3 scripts/check_docs.py
python3 scripts/check-hdmr-literature.py
python3 scripts/check-frequency-literature.py
python3 scripts/audit_qosa.py --toolchain 1.95.0
cargo run --offline --locked -p salib-estimators --example audit_regression_ties
cargo test --workspace --doc --offline --locked
cargo doc --workspace --no-deps --offline --locked
```

The four numerical checks now require the corrected behavior and exit zero on
success. HDMR checks unsupported-measure rejection rather than accepting a
triangular measure with wrong coefficient norms. `check_docs.py` checks links, anchors, code fences, release
examples and registered direct citations. `--online` additionally checks HTTP
access; HTTP success cannot establish whether a source supports a claim.

## Documentation-pass validation and first independent review

- Initial PR review: the existing workspace suite passed 980 tests, including
  27 doctests. This does not cover the new counterexamples.
- After documentation edits: all 27 doctests passed; workspace rustdoc built
  successfully. All modified existing Rust non-comment lines are unchanged.
- Local documentation checker: 25 public pages and 36 citation destinations pass.
  Automated HTTP access: 29 returned 200; seven returned 403 (Janon 2014, Owen
  2013, Cukier 1973, Saltelli 1999, Li 2001, Constantine 2015, Saltelli 2008).
  Alternate full texts inspected are recorded in the lane reports. A blocked
  publisher endpoint is not evidence that a citation is invalid.
- Benchmark generator rerun from retained Criterion data: timing tables are
  identical to HEAD. Whitespace checks pass.
- A [separate reviewer](2026-10-07-independent-review.md) inspected the combined diff and challenged the code
  classifications and fix proposals. The reviewer confirmed the main defects
  and found residual DGSM proof wording, an incorrect eFAST bibliography
  description, a missing weak-derivative assumption, unverified discrepancy
  equation references, and an HDMR acceptance-plan ambiguity. All five were
  corrected. No production behavior change was introduced.

## Implemented resolutions

Average ranks correct the regression witness. Tie-preserving classes correct
conditioning on constant/discrete inputs. QOSA uses empirical pinball-loss minima,
with atom-aware expected shortfall diagnostics. FAST/RBD use one shared weighted
spectrum and RBD uses EASI's rank traversal. HDMR rejects unsupported measures and
validates shapes. LARS/OMP select on centered, normalized predictors. PRESS
invalidates a whole candidate when any leave-one-out denominator is invalid.
The fresh review additionally caught overflowing/nonfinite polynomial fits,
LARS simultaneous knots, and extreme-scale KDE/expected-shortfall arithmetic.
Those are corrected and independently replayed through the public APIs. The
old QOSA metamorphic test encoded translation dependence; its oracle now derives
exact empirical pinball loss and requires translation invariance.

[Surrogate fix evidence](2026-10-07-surrogate-fixes.md) derives the independent
selection/deletion oracles. The [fresh correctness review](2026-10-07-correctness-review.md)
challenges the final implementation. Final suite, packaging, feature, MSRV and
publication checks are in the [release assessment](2026-10-07-release-assessment.md).
The fixed partition caps and other documented approximation choices remain;
no claim of universal consistency or full adaptive-paper conformance is made.
