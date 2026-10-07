# Frequency-method literature audit — 2026-10-07

Scope: FAST/eFAST/RBD-FAST public prose, sampler and estimator comments, and
relevant test commentary. Algorithms and existing test assertions were not changed.

## Sources

- [Saltelli, Tarantola and Chan (1999), author PDF](https://www.andreasaltelli.eu/file/repository/Saltelli_Technom.pdf),
  [DOI](https://doi.org/10.1080/00401706.1999.10485594). Read Sections 2.2,
  3–4.2, Table 3, and Appendix A. Saved as `saltelli-1999-efast.pdf`.
- [Plischke (2010), author preprint](https://www-10b015.pages.gwdg.de/papers/easi_ress.pdf),
  [DOI](https://doi.org/10.1016/j.ress.2009.11.005). Read Section 3,
  printed pp. 2–4, and Section 7, p. 12. Saved as `plischke-2010-easi.pdf`.
- [Tarantola, Gatelli and Mara (2006), HAL manuscript mirrored by SWAT+](https://celray.github.io/SWATPlusToolbox/3.0.0/papers/RBD-FAST/Tarantola06RESS_HAL.pdf),
  [DOI](https://doi.org/10.1016/j.ress.2005.06.003). Read Section 2,
  manuscript pp. 4–6. The direct HAL download was blocked.
- [Tissot and Prieur (2012), publisher abstract/introduction](https://www.sciencedirect.com/science/article/abs/pii/S0951832012001159),
  [DOI](https://doi.org/10.1016/j.ress.2012.06.010). The introduction distinguishes
  given-data EASI from designed RBD/RBD-FAST. Full paper not acquired; no
  equation claims from its unread text are used.

Restore local PDFs with `curl -L --fail 'URL_ABOVE' -o papers/FILENAME.pdf`.
These remain ignored research artifacts.

## Confirmed implementation discrepancies

### RBD API omits EASI's triangular reordering

Plischke Section 3, p. 3, explicitly takes sorted odd ranks ascending, then
sorted even ranks descending, before computing the Fourier spectrum. Code in
`rbd_fast.rs` applies ascending ranks directly. This changes the trace and its
truncation error. Section 7 Eq. (7), p. 12, does contain the correction
`(N*S_raw-2M)/(N-2M)`, equivalent to the code's `lambda=2M/N` formula.
The correction attribution is supported; the old “Eq 5–6” reference was wrong.

For independent `X~Uniform(0,1)`, `Y=cos(pi*X)` has true first-order index 1.
The triangular trace becomes one cosine period; the direct sorted trace covers
only half a period and has a discontinuity under periodic extension. On an
odd midpoint grid of 4097 points and `M=4`, the Rust API gives
`0.909393063947`, while the triangular trace control gives 1 to printed precision.

Classification: missing operation relative to the cited EASI algorithm;
not merely inaccurate explanatory prose. Tarantola's older Section 2 itself
uses increasing-order language, so do not claim that ascending-order analysis
contradicts every historical variant of RBD. State exactly which algorithm the
API implements and which cited variant it intends to support.

Proposed fix: compose the input-rank permutation with the odd-ascending/even-
descending reshape. Check both odd and even lengths. The `cos(pi*x)` oracle
should recover one with only discretization tolerance, unlike a generic
finite-harmonic approximation. Preserve arbitrary input marginals through ranks;
review the treatment of ties separately.

### Even-N Nyquist bin is doubled in both estimators

Both `power_spectrum_one_sided` helpers include frequency `N/2`, and both
estimators multiply the entire sum by two. For real even-length signals the
Nyquist coefficient is its own negative-frequency partner and is counted once.
This is an algebraic Parseval error, independently derived from the DFT definition;
it is not an interpretation of an empirical paper result. Odd sample counts
avoid it, but both public APIs accept even counts.

For `y_j=cos(2*pi*k*j/N)+(-1)^j`, the ordinary population-divisor variance
is `1/2+1=3/2`. The implementation's spectral variance is `1/2+2=5/2`.
Thus the low-frequency share is reported as `1/5` instead of `1/3`.
The Rust FAST fixture reproduces this exactly. The RBD fixture includes its
stated `lambda=2/N` correction: at `N=64`, it returns `0.174193548387`
instead of `0.311827956989` for its current directly sorted trace.

Proposed fix: give all non-DC Fourier bins their correct multiplicity, with
Nyquist weight one. Apply the same weights to every numerator/denominator band.
Use direct sample variance as the finite-vector oracle for odd/even signals,
including pure Nyquist and mixed ordinary/Nyquist modes. This is a code defect,
not statistical bias. Do not explain it away using Monte Carlo variability.

## Runnable checks

Run the public Rust APIs through the dependency-free Python driver:

```sh
PATH="$HOME/.cargo/bin:$PATH" RUSTUP_TOOLCHAIN=1.95.0 python3 scripts/check-frequency-literature.py
```

The Homebrew Rust compiler currently aborts with an LLVM dynamic-symbol error;
the rustup compiler works. The driver creates and removes a temporary Cargo
project and uses already-cached dependencies (`--offline`). Exit 1 means an
oracle failed; compiler failures produce Cargo's own status.

Observed output:

```text
PASS EASI triangular trace control: got=1.000000000000, expected=1.000000000000
FAIL RBD API vs EASI single-frequency oracle: got=0.909393063947, expected=1.000000000000
FAIL RBD even-N Nyquist weighting: got=0.174193548387, expected=0.311827956989
FAIL FAST even-N Nyquist weighting: got=0.200000000000, expected=0.333333333333
```

The current RBD Nyquist fixture and triangular control isolate the existing
ascending-sort pipeline. When correcting the rank permutation, move the Nyquist
oracle to the spectrum helper or invert the new permutation when preparing its
fixture. Avoid changing the expected mathematical variance. The FAST fixture
uses a deterministic lookup model to provide known Fourier modes at sampled
coordinates; it checks spectral arithmetic, not sensitivity convergence of a
smooth population model.

## Documentation findings

The Saltelli 1999 bibliography says complementary variance uses all frequencies
other than the input's characteristic frequency. That is false: the method
estimates complementary effects from assigned low frequencies. Sections 3–4
and Table 3 support the low-frequency construction. Section 4.2 acknowledges
frequency reuse and space-coverage tradeoffs; collisions do not imply “bins
remain well-separated” or elimination of every interaction alias.

A fixed `M` truncates the spectrum, so increasing `N` alone need not eliminate
error. No inspected source supports universal, fixed Ishigami offsets such as
`S3≈.02`, `ST1` low by `.04`, or `ST2` high by `.05`. Existing test comments
call these permanent and well documented without a source. They also label
fixed arrays “SALib differential” without a saved generation procedure or
version. These are provenance gaps; this audit did not establish that the
numbers were fabricated or reproduce those external reference arrays.

Recommend removing unsupported universal bias statements, describing tolerance
checks as sample/configuration-specific, and generating any published example
numbers directly from the executable example. A minimum sample-count check
establishes available frequency bins, not publication quality. No claim of
“exact SALib parity modulo MC noise” is supported by matching broad tolerances.

Doc comments in `salib-estimators/src/{fast,rbd_fast}.rs` and
`salib-samplers/src/fast.rs` now state the algorithms, cite primary papers, and
expose the two current implementation limitations. Root owns method-page and
bibliography integration, the consolidated code-fix plan, and test-provenance
cleanup. No existing test assertions were changed.

## Stacks follow-up

Read-only Stacks search recovered the original Tarantola 2006 HAL manuscript.
The `paper` catalog's indexed SHA lookup confirms the authors, title, DOI and
retained path. The document matches the already-read manuscript version and
confirms Section 2's increasing-order wording; it does not change the distinction
between this early RBD account and Plischke's explicit triangular EASI reshape.

- Local: `papers/tarantola-2006-stacks.pdf`.
- Catalog SHA-256: `ea1561fc0bc822b09f69e91ee550b9beb9310ca7bd6267ba48fb85f4a2a1665e`.
- Retained path: `/home/patrick/neurotic_library/lib/statistics_probability_and_uncertainty/Random Balance Designs for First Order Global Sensitivity Indices - RBD-FAST - Tarantola Gatelli Mara 2006.pdf`.
- Primary identifier: [doi:10.1016/j.ress.2005.06.003](https://doi.org/10.1016/j.ress.2005.06.003).

Restore with:

```sh
scp 'neuroses:/home/patrick/neurotic_library/lib/statistics_probability_and_uncertainty/Random Balance Designs for First Order Global Sensitivity Indices - RBD-FAST - Tarantola Gatelli Mara 2006.pdf' papers/tarantola-2006-stacks.pdf
```

### Additional screening source acquired

`papers/campolongo-2007-stacks.pdf` was copied from
`/home/patrick/neurotic_library/lib/statistics_probability_and_uncertainty/An Effective Screening Design for Sensitivity Analysis of Large Models - Campolongo et al. 2007.pdf`.
Catalog SHA-256: `65486ab3ce3b7d35113e6e219e5804cfbbe5d9a0cd682f3bbec7c388b3bf566b`.
The PDF identifies Francesca Campolongo, Jessica Cariboni and Andrea Saltelli,
[doi:10.1016/j.envsoft.2006.10.004](https://doi.org/10.1016/j.envsoft.2006.10.004).
The catalog author field incorrectly says Thomas Santner; use the primary PDF.

Read Sections 3.1–3.2, pp. 1510–1512. Trajectory distance sums Euclidean distances
between all pairs of points across two trajectories; the displayed selected-set
objective is the square root of the sum of squared trajectory distances.
The original optimization examines combinations, while implementations often
substitute heuristics. Section 3.2 defines mu-star as the mean absolute elementary
effect, preventing sign cancellation; sigma remains an indicator of variation
in elementary effects. This is screening evidence, not a proof that a factor
below a finite-sample threshold can safely be fixed. The paper's numerical
comparison with total Sobol' effects does not make mu-star a variance share.

No estimator or sampler implementation was changed during the Stacks follow-up.
