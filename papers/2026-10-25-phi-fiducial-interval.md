# Phi interval: fiducial generalized pivotal quantity (salib #10)

## Question

The parametric-bootstrap Phi interval shipped with #5 undercovers when raters
are few and persons and items many. Coverage reaches .696 at 200 × 20 × 2, and
the misses lie almost all above the truth (`2026-10-25-g-theory-coverage-study.md`,
"Phi experiment"). The diagnosis was plug-in error. With one or two rater
degrees of freedom, σ̂²(r) and σ̂²(ir) are noisy and often zero, and the
bootstrap treats them as the truth.

## Method chosen

The fiducial generalized pivotal quantity (FGPQ) of Hannig, Iyer & Patterson
(2006), *Fiducial generalized confidence intervals*, JASA 101(473):254–269,
Theorem 2 (the structural method):

- In the balanced normal random model, `SS_j / EMS_j ~ χ²(df_j)`
  independently for the seven mean squares.
- So `R_j = SS_j / U_j`, with `U_j ~ χ²(df_j)`, is an FGPQ for `EMS_j`.
- Each draw solves the expected-mean-square equations for the components,
  clips them at zero (the paper's `max(0, R)`), and evaluates Phi.
- The interval is the percentile range of the draws.

Their Theorem 1 gives asymptotically correct coverage, not exact small-sample
coverage. Patterson's 2006 dissertation (Colorado State) extends the recipe to
general balanced mixed models.

The mean squares are complete sufficient for the variance components here, so
this FGPQ is the natural one. Unlike the bootstrap, it carries the uncertainty
of every mean square, including a 1-df rater mean square. With two raters,
σ²(r)'s own 95% interval spans a factor of about 5000 (the χ²₁ quantiles are
.00098 and 5.02), so Phi's lower bound is far from Φ̂. It also refits no data,
so it is far cheaper than the bootstrap.

Related evidence: Ionan et al. (2014, BMC Med Res Methodol 14:121) find GCI
and MLS intervals within .01 of nominal for an ICC in a two-way crossed model
with three labs, on normal data. Like most of this literature, they report
only two-sided coverage.

## Evidence

`salib-gtheory-coverage gpq`, in `crates/salib-gtheory-coverage/src/gpq.rs`,
was run on neuroses over the study's 90 cells, with n_sim = 1900 and 9999
draws. The harness computes its own ANOVA and samples χ² with its own
Marsaglia–Tsang generator. The pass criterion is unchanged: coverage in
[.94, .96] and each tail miss in [.015, .035]. A "lower miss" means the
interval lies entirely below the truth.

Raw rows:

- `coverage-2026-10-25/fiducial-clipped-signed.md` at 84c61ac: the clipped
  and signed variants, plus components and G.
- `coverage-2026-10-25/fiducial-calibrated.md` at dbf7aa5: the clipped and
  calibrated variants. The clipped rows reproduce the first file exactly.

| Phi interval | pass | coverage min / median / max | lower miss | upper miss |
|---|---|---|---|---|
| parametric bootstrap (#5, shipped) | 14/90 | .694 / .946 / .971 | .016 | .054 |
| **fiducial GPQ, clipped (chosen)** | 6/90 | .918 / .952 / .967 | .043 | .006 |
| fiducial GPQ, signed components | 6/90 | .934 / .953 / .974 | .041 | .007 |
| fiducial GPQ, calibrated levels | 25/90 | .808 / .928 / .953 | .031 | .050 |

The chosen interval's two-sided coverage on normal data is .938–.967 in all
81 cells (median .952), and .918–.947 in the 9 binary cells. It fails the
strict criterion only on tail balance. It falls below Phi in about 4% of
samples and above it in under 1%. The lower bound is therefore conservative,
and the upper bound behaves like a 96% one-sided bound.

In the cells where the bootstrap failed:

| Cell | Bootstrap coverage / width | Fiducial coverage / width |
|---|---|---|
| 200×20×2 high-G | .696 / .192 | .947 / .790 |
| 200×20×2 all-one | .798 / .356 | .953 / .588 |
| 200×20×3 high-G | .798 / .122 | .942 / .469 |
| 200×8×2 high-G | .814 / .193 | .965 / .781 |
| 50×20×2 high-G | .846 / .223 | .956 / .799 |
| 20×20×3 all-one (control) | .953 / .579 | .950 / .664 |

The bootstrap's widths come from the 2026-10-25 Phi experiment, which used
different datasets.

## Variants rejected

- **Signed components** (Phi from unclipped draws, clamped to [0, 1]): the same
  imbalance, so clipping does not cause it.
- **Calibrated levels** (prepivoting): the levels are chosen so that the GPQ
  is calibrated at the clipped estimate. Under the normal model a simulated
  dataset's mean squares are `EMS_j χ²(df_j)/df_j`, so this is cheap. But it
  calibrates at θ̂ and so brings back the plug-in failure: coverage falls to
  .81, and G drops from 80/90 to 75/90.
- **Nested bootstrap**: not tried. Calibration at θ̂ is the same plug-in step
  at greater cost.

## Side findings

- **G.** The fiducial G interval passes 80/90. The shipped F interval (86/90)
  stays.
- **Components.** Fiducial component intervals pass 73–85 of 90 for every
  component. The shipped Satterthwaite intervals pass 0–77 (σ²(r): 75/90
  against 0/90). This is salib #12.
- **Binary data.** All normal-theory intervals undercover on binary data for
  σ²(pi), σ²(pr) and σ²(pir), at .69–.91.

## Verification

- `verify/sage/g_theory_fiducial.sage` derives the expected-mean-square table
  from crolib's expected T terms and checks salib's table against it. It
  also emits exact fiducial quantiles of σ²(pir) (closed form) and σ²(pi)
  (quadrature).
- `fiducial_draws_match_exact_quantiles` checks salib's draws against those
  quantiles within five binomial standard errors.
- `fiducial_quantity_at_its_centre_returns_the_estimates` checks Hannig et
  al.'s defining property, R(s, s) = θ.

Not established: tail balance; D-study counts; finite universes;
non-normal data beyond the binary cells.
