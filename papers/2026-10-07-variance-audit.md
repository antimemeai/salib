# Variance-based documentation audit — 2026-10-07

Scope: public method page, variance bibliography entries, and comments in
`saltelli2010.rs`, `jansen.rs`, `janon.rs`, `owen.rs`, and `given_data_sobol.rs`.
Executable estimator code was not changed. Source comments now cite primary
papers and distinguish estimator guarantees from implementation choices.

## Sources read

| Source | Primary URL | Location checked | Local copy |
|---|---|---|---|
| Saltelli et al. (2010) | [author PDF](https://www.andreasaltelli.eu/file/repository/PUBLISHED_PAPER.pdf), [DOI](https://doi.org/10.1016/j.cpc.2009.09.018) | Eqs. (18)–(19), Table 2, printed pp. 261–262 | `saltelli-2010.pdf` |
| Janon et al. (2014), published version | [Numdam PDF](https://www.numdam.org/article/PS_2014__18__342_0.pdf), [DOI](https://doi.org/10.1051/ps/2013040) | Eqs. (2.5)–(2.8), Propositions 3.2–3.5, printed pp. 344–346 | `janon-2014.pdf` |
| Owen, April 2012 preprint of 2013 article | [arXiv PDF](https://arxiv.org/pdf/1204.4763), [published DOI](https://doi.org/10.1145/2457459.2457460) | Section 3, Correlation 2 formula, pp. 3–4; Section 5, Theorem 2, pp. 7–9 | `owen-small-indices-2012-preprint.pdf` |
| Plischke, Borgonovo, Smith, conference account of the given-data approach | [author-hosted PDF](https://www-10b015.pages.gwdg.de/papers/_03-Mo2-2_-_Plischke.pdf) | Section 4, Theorems 2 and 4, pp. 4–6 | `plischke-borgonovo-smith-given-data-conference.pdf` |
| Plischke et al. (2013), journal abstract and introduction | [publisher](https://www.sciencedirect.com/science/article/abs/pii/S0377221712008995), [DOI](https://doi.org/10.1016/j.ejor.2012.11.047) | Publisher abstract/introduction only | Full journal PDF not acquired |

PDFs are local research material and must remain ignored by Git. Restore a copy
with `curl -L --fail 'URL_FROM_TABLE' -o papers/LOCAL_FILENAME`.

## Confirmed corrections

**Saltelli/Jansen attribution.** Saltelli Table 2(b) gives the implemented
first-order product-difference formula; Table 2(f) gives the squared-difference
total effect and credits Jansen. Previous comments called the former “Eq c”
and called an optional pairwise formula “Eq d”; Table 2(c) is instead Jansen's
first-order complement and Table 2(d) is a total-effect formula. Corrected
these references, removed universal-best language, and corrected the model-call
count when optional second-order blocks are present.

The bibliography's Jansen entry reverses the distinction: comparing `B` and
`A_B^i`, which share input `i`, estimates the first-order complement; comparing
`A` and `A_B^i` estimates total effect. Jansen's original full paper was subsequently acquired through Stacks.
Its Eqs. (3)–(10), p. 36, define main/all-effects variances and their
complementarity. Section 2.3, pp. 37–38, gives the half-squared-difference
estimate of all-effects variance and recovers the main effect by subtraction.
This confirms the attribution independently of Saltelli's notation.

**Janon.** The published Eq. (2.8), p. 345, is algebraically equivalent to
Eq. (2.6); the PDF was inspected visually as well as as extracted text. The old
comments incorrectly accused the paper of using the square of an averaged pair
in the denominator. Removed that allegation. Proposition 3.3 compares the
paper's two estimators asymptotically; it does not prove finite-N superiority
over this crate's Saltelli estimator. Proposition 3.5 concerns regular estimators
in the exchangeable-pair model. Proposition 3.2 assumes independent identically
distributed replications and a finite fourth output moment. Bibliography wording
should name that setting rather than all estimators using an entire Saltelli
design. The pairwise extension is not covered by this result.

**Owen.** The bibliography misdescribes this as a second-order interaction
estimator. The paper addresses closed Sobol' indices; this implementation uses
singletons and returns first-order indices. The preprint Section 3 formula
matches the code. Its numerator is unbiased under independent sampling; an
estimated denominator does not preserve that claim for the ratio. Theorem 2
optimizes a proxy under product-function assumptions, not all small-index
estimation problems. Removed general optimality claims and unsupported rates.
Actual implementation cost is `N(2+2d)` because `C` is stored but never evaluated;
`OwenMatrix::total_evaluations()` separately reports `N(3+2d)`.

**Given-data estimator.** The author conference account distinguishes a fixed
partition approximation from convergence as partitions are refined. Its
Theorems 2 and 4 concern density-based indices, so they cannot simply be cited
as a theorem for every implementation of binned Sobol' estimation. Source
inspection shows this implementation caps classes at 48. Remove claims of
unconditional asymptotic unbiasedness/convergence to the true point-conditional
index. The cap is an algorithm choice limiting resolution, not a discrepancy
between code and its explicit binned formula. The source now documents it.

Source inspection also establishes `O(d(N log N + NM))` work, including sorting
and scanning all rows for each class; the prior `O(Nd)` description omitted
sorting. Ordinal ranks break ties by row order, so discrete-input results can
change under row permutations. Removed the unverified “matches SALib exactly”
claim. The public method page should mention the 48-bin cap and ties.

## Direct witness for the fixed-bin limitation

For `X ~ Uniform(0,1)`, `Y=sin(96*pi*X)` depends only on `X`, so its population
first-order index is 1. With 48 equal-width/equal-probability bins, every bin
contains one full sine period, so every conditional bin mean is zero: the
population binned index is zero. This is a direct analytic counterexample to
the former general convergence claim, not a claim made by the cited paper.

The following standard-library calculation reproduces the implementation's
class rule and variance formula on a deterministic midpoint grid. It prints
`M=48`, `Var(Y)=0.5000000000000003`, and index `2.220446049250313e-16`.

```python
import math
n = 196608
m = min(48, max(2, math.ceil(n ** (2 / (7 + math.tanh((1500-n)/500))))))
y = [math.sin(96 * math.pi * (k + .5) / n) for k in range(n)]
mean = sum(y) / n
variance = sum((v - mean) ** 2 for v in y) / n
within = 0
for j in range(m):
    block = y[n*j//m:n*(j+1)//m]
    center = sum(block) / len(block)
    within += sum((v - center) ** 2 for v in block) / n
print(m, variance, 1 - within / variance)
```

## Remaining limits

The full Plischke 2013 journal PDF remains an acquisition gap; Jansen 1999
was recovered from Stacks as recorded below. No exact equation claims from
unread full texts were used. Owen equation numbers
in the inspected preprint differ from the old source comments, so corrected
comments cite the named Correlation 2 formula and Section 3 rather than an
unverified published equation number. This audit does not establish statistical
coverage for bootstrap intervals or accuracy of all estimators on arbitrary
models. Parent task owns documentation builds, citation/link checks, and the
consolidated journal and fix plan.

## Stacks follow-up

Read-only access on `neuroses`, 2026-10-07: inspected
`/home/patrick/stacks/README.md` and the library schema, opened
`file:/srv/stacks/db/library.db?mode=ro`, then resolved catalog-retained paths
under `/home/patrick/neurotic_library/lib`. Stacks remains the system of record;
the retained source tree was only read. Broad catalog title scans were cancelled;
bounded FTS and SHA-index lookups were used alongside filename discovery.

- Local: `papers/jansen-1999-stacks.pdf`.
- Catalog SHA-256: `9a780ad6f3e7013a63a913fd99f6d4c2d703b331f40139037e705910828e00ff`.
- Retained source: `/home/patrick/neurotic_library/lib/statistics_and_probability/Analysis of Variance Designs for Model Output - Jansen 1999.pdf`.
- Primary link: [Jansen (1999)](https://doi.org/10.1016/S0010-4655(98)00154-4).

The catalog title is only the journal header, and its year/author/DOI fields are
empty. The PDF title page identifies Jansen and the 1999 CPC publication.
Section 1 assumes deterministic output, independent input groups, and finite
output mean/variance; Section 4's efficiency calculations additionally impose
normality on ANOVA components. These do not support generic best-estimator
claims. No source-code change followed this additional reading.

Restore with:

```sh
scp 'neuroses:/home/patrick/neurotic_library/lib/statistics_and_probability/Analysis of Variance Designs for Model Output - Jansen 1999.pdf' papers/jansen-1999-stacks.pdf
```
