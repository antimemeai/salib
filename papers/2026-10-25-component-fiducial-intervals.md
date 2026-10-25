# Component intervals: fiducial generalized pivotal quantity (salib #12)

## Question

The Satterthwaite component intervals shipped with #5 are calibrated only for
σ²(pir), and for σ²(pi), σ²(pr) and σ²(ir) with many persons and a
non-negligible component (`2026-10-25-g-theory-coverage-study.md`). They give
no interval when the estimate is zero or negative, and σ²(r) passes no cell.
The fiducial experiment for #10 measured fiducial intervals for every
component as a side finding (`2026-10-25-phi-fiducial-interval.md`).

## Method chosen

The same fiducial generalized pivotal quantity as the Phi interval (Hannig,
Iyer & Patterson 2006, JASA 101:254, Theorem 2):

- Each draw replaces every mean square `MS_j` by `MS_j df_j / U_j`, with
  `U_j ~ χ²(df_j)` independent.
- It solves the expected-mean-square equations and clips the components at
  zero.
- A component's interval is the percentile range of its clipped draws.

`bootstrap_g_theory_pir` already made these draws for Phi, so the component
intervals cost nothing extra. Clipping is monotone, so each interval is the
signed fiducial interval with its bounds clipped at zero. An interval exists
whatever the sign of the estimate.

The bounds carry Monte Carlo error from `n_resamples`, where the Satterthwaite
bounds were closed form.

## Evidence

The rows come from `salib-gtheory-coverage gpq` on neuroses: 90 cells,
n_sim = 1900, 9999 draws, at 84c61ac (`coverage-2026-10-25/fiducial-clipped-signed.md`).
The harness computes its own ANOVA and samples χ² with its own generator, so
it does not share salib's code. The pass criterion is unchanged: coverage in
[.94, .96] and each tail miss in [.015, .035].

The Satterthwaite rows are from the 2026-10-25 study, which used other
datasets for the same cells. A missing interval counts as a miss.

Normal data, 81 cells:

| Component | Fiducial pass | Fiducial coverage min / median / max | Fiducial tail misses, per cell | Satterthwaite pass | Satterthwaite coverage min / median / max |
|---|---|---|---|---|---|
| σ²(p) | 77 | .933 / .949 / .964 | .016–.037 | 37 | .910 / .954 / .972 |
| σ²(i) | 73 | .941 / .951 / .963 | .008–.038 | 11 | .740 / .948 / .977 |
| σ²(r) | 72 | .937 / .952 / .965 | .012–.036 | 0 | .396 / .792 / .968 |
| σ²(pi) | 73 | .935 / .950 / .968 | .016–.036 | 52 | .941 / .953 / .968 |
| σ²(pr) | 79 | .940 / .950 / .961 | .016–.033 | 43 | .235 / .950 / .968 |
| σ²(ir) | 77 | .936 / .950 / .963 | .017–.037 | 41 | .323 / .951 / .975 |
| σ²(pir) | 80 | .940 / .951 / .958 | .016–.034 | 77 | .937 / .951 / .960 |

The fiducial failures on normal data are near misses in coverage (.933 at
worst). A few have one tail as small as .008 (σ²(i)) or .012 (σ²(r)).

The near-zero-rater cells set σ²(r), σ²(pr) and σ²(ir) to 0.01 beside
components of 1. There the fiducial intervals cover .936–.965. Satterthwaite
fell to .235 there, because nearly half the estimates were nonpositive and got
no interval.

Binary data, 9 cells: σ²(pi), σ²(pr) and σ²(pir) undercover, at .69–.91
(Satterthwaite .69–.94), a normal-theory failure both methods share. σ²(p)
covers .916–.950. σ²(ir) passes 8 of 9 cells. σ²(i) and σ²(r) cover .950–.973
and pass 4 and 3 of 9.

## Verification

- `phi_interval_replays_the_fiducial_stream` replays the documented stream,
  solves each draw's equations by Gaussian elimination on the EMS columns, and
  checks every component bound and the Phi bound. Four of that grid's seven
  estimates are negative.
- `fiducial_draws_match_exact_quantiles` checks the draws of σ²(pir) and
  σ²(pi) against exact quantiles from `verify/sage/g_theory_fiducial.sage`.
  That script also derives the EMS table from crolib's expected T terms.
- `fiducial_intervals_do_not_depend_on_units` checks that scaling the data by
  2^±480 scales every bound by exactly 2^±960.

Not established: tail balance at fewer than 9999 draws; components below
0.01; D-study counts; finite universes; non-normal data beyond the binary
cells.
