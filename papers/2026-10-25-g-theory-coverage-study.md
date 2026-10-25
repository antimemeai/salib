# G-theory coverage study (salib #5)

Run 2026-10-23 to 2026-10-25 on neuroses with `crates/salib-gtheory-coverage`
(plan: `journal/2026-10-23-g-theory-bootstrap-plan.md`). The run was split
across machine outages:
- cells 0–42 at 610a9c7;
- cells 43–47 and 48–89 at 0000970.

0000970 changes only the harness's χ² underflow guard, which matters only
where 610a9c7 panicked. Seeds are keyed by cell, so every cell's datasets are
fixed. Raw per-cell rows are in `coverage-2026-10-25/`; the tables below come
from `scripts/summarize-coverage.py` over those files.

**Design.**
- 90 cells, n_sim = 1900, B = 999, nominal 0.95.
- 81 normal cells: n_p ∈ {20, 50, 200} × n_i ∈ {5, 8, 20} × n_r ∈ {2, 3, 5}
  × three patterns (all-one; σ²(r) = σ²(pr) = σ²(ir) = 0.01; high-G with
  σ²(p) = 4 and the rest 0.5).
- 9 binary cells: a normal latent thresholded at 0, with exact truth from
  Sheppard plus Möbius inversion; n_p = 50, all-one pattern.

**Pass criterion.** Coverage in [.94, .96] and each tail miss in
[.015, .035]. A missing interval counts as a miss. At n_sim = 1900 the band
is about ±2 MCSE, so a truly calibrated method still fails about 5% of cells.

## Findings

| Quantity | Method | Result | Claim |
|---|---|---|---|
| G | Closed-form F interval | 86/90 pass, including 9/9 binary; coverage .938–.964; balanced tails | Calibrated over the studied cells: normal and binary, n_r = 2–5, all three patterns |
| σ²(pir) | Satterthwaite | 77/81 normal pass; binary 0/9 (as low as .69) | Calibrated for normal data |
| σ²(pi), σ²(pr), σ²(ir) | Satterthwaite | Mostly pass for normal data with n_p ≥ 50 when the component is not near zero; near-zero components often have σ̂ ≤ 0, leaving no interval | Normal data, n_p ≥ 50, non-negligible component |
| σ²(p) | Satterthwaite | 37/90; misses mostly on the upper side (median coverage .954) | Not calibrated |
| σ²(i), σ²(r) | Satterthwaite | 11/90 and 0/90 | Unsupported |
| All components | Parametric percentile | 109/630; intervals sit too low because a percentile interval does not correct a variance estimate's skew | Superseded by Satterthwaite |
| Component SEs | Tong & Brennan facet plans with CASMA 21 correction | SE ratio (√mean SE²/SD), median 0.96–1.00 for every component on normal data; binary interaction SEs too small (0.57–0.84) | Supported for normal data |
| Phi | Parametric percentile | 14/90; coverage .694–.971, worst in high-G and at n_r = 2 | Not calibrated |
| All | 0.3.0 boot-p,i,r | 0/36 baseline rows; σ²(pir) as low as .134, σ²(ir) .64 | Replaced |

Attaching Phi only when the signed Phi is available left it missing in about
24% of datasets (median availability .76).

## Summary tables

90 cells, 1566 rows.

## By estimand and method

| estimand | method | pass | coverage min / median / max | mean lower miss | mean upper miss | median avail |
|---|---|---|---|---|---|---|
| p | 0.3.0 boot-p,i,r | 0/4 | 0.883 / 0.974 / 0.991 | 0.000 | 0.044 | 1.000 |
| p | Satterthwaite | 37/90 | 0.899 / 0.954 / 0.972 | 0.012 | 0.034 | 1.000 |
| p | parametric | 4/90 | 0.904 / 0.937 / 0.955 | 0.056 | 0.010 | 1.000 |
| i | 0.3.0 boot-p,i,r | 0/4 | 0.964 / 0.982 / 0.995 | 0.015 | 0.004 | 1.000 |
| i | Satterthwaite | 11/90 | 0.738 / 0.947 / 0.977 | 0.003 | 0.040 | 0.988 |
| i | parametric | 0/90 | 0.827 / 0.908 / 0.953 | 0.101 | 0.001 | 1.000 |
| r | 0.3.0 boot-p,i,r | 0/4 | 0.661 / 0.717 / 0.911 | 0.248 | 0.000 | 1.000 |
| r | Satterthwaite | 0/90 | 0.396 / 0.799 / 0.971 | 0.000 | 0.043 | 0.837 |
| r | parametric | 0/90 | 0.627 / 0.816 / 1.000 | 0.192 | 0.000 | 1.000 |
| pi | 0.3.0 boot-p,i,r | 0/4 | 0.999 / 1.000 / 1.000 | 0.000 | 0.000 | 1.000 |
| pi | Satterthwaite | 52/90 | 0.726 / 0.952 / 0.968 | 0.020 | 0.037 | 1.000 |
| pi | parametric | 40/90 | 0.721 / 0.946 / 0.958 | 0.043 | 0.023 | 1.000 |
| pr | 0.3.0 boot-p,i,r | 0/4 | 0.913 / 0.964 / 1.000 | 0.040 | 0.000 | 1.000 |
| pr | Satterthwaite | 43/90 | 0.235 / 0.948 / 0.968 | 0.014 | 0.074 | 1.000 |
| pr | parametric | 11/90 | 0.842 / 0.946 / 0.997 | 0.038 | 0.015 | 1.000 |
| ir | 0.3.0 boot-p,i,r | 0/4 | 0.636 / 0.816 / 0.877 | 0.214 | 0.000 | 1.000 |
| ir | Satterthwaite | 45/90 | 0.323 / 0.951 / 0.975 | 0.013 | 0.049 | 1.000 |
| ir | parametric | 0/90 | 0.837 / 0.920 / 1.000 | 0.073 | 0.003 | 1.000 |
| pir | 0.3.0 boot-p,i,r | 0/4 | 0.134 / 0.404 / 0.813 | 0.561 | 0.000 | 1.000 |
| pir | Satterthwaite | 77/90 | 0.693 / 0.950 / 0.960 | 0.034 | 0.032 | 1.000 |
| pir | parametric | 54/90 | 0.689 / 0.948 / 0.960 | 0.042 | 0.026 | 1.000 |
| G | 0.3.0 boot-p,i,r | 0/4 | 0.922 / 0.948 / 0.984 | 0.000 | 0.050 | 1.000 |
| G | F interval | 86/90 | 0.938 / 0.951 / 0.964 | 0.024 | 0.025 | 1.000 |
| Phi | 0.3.0 boot-p,i,r | 0/4 | 0.745 / 0.883 / 0.981 | 0.000 | 0.127 | 1.000 |
| Phi | parametric | 14/90 | 0.694 / 0.946 / 0.971 | 0.016 | 0.054 | 1.000 |
| Phi | parametric, attached | 0/90 | 0.095 / 0.727 / 0.954 | 0.010 | 0.029 | 0.760 |

## Facet standard-error ratios (√mean SE² / empirical SD)

| component | min / median / max |
|---|---|
| p | 0.853 / 0.986 / 1.032 |
| i | 0.913 / 0.956 / 1.031 |
| r | 0.914 / 0.976 / 1.223 |
| pi | 0.600 / 0.993 / 1.035 |
| pr | 0.741 / 0.990 / 1.031 |
| ir | 0.916 / 0.964 / 1.043 |
| pir | 0.572 / 0.997 / 1.104 |

## G and Phi by pattern, data and rater count

| estimand | method | slice | pass | coverage min / median / max |
|---|---|---|---|---|
| G | F interval | pattern=all-one | 34/36 | 0.938 / 0.950 / 0.964 |
| G | F interval | pattern=high-g | 25/27 | 0.938 / 0.950 / 0.961 |
| G | F interval | pattern=near-zero-rater | 27/27 | 0.944 / 0.952 / 0.959 |
| G | F interval | data=binary | 9/9 | 0.942 / 0.954 / 0.956 |
| G | F interval | data=normal | 77/81 | 0.938 / 0.951 / 0.964 |
| G | F interval | n_r=2 | 28/30 | 0.938 / 0.948 / 0.955 |
| G | F interval | n_r=3 | 29/30 | 0.944 / 0.952 / 0.964 |
| G | F interval | n_r=5 | 29/30 | 0.944 / 0.952 / 0.961 |
| Phi | parametric | pattern=all-one | 8/36 | 0.782 / 0.946 / 0.971 |
| Phi | parametric | pattern=high-g | 1/27 | 0.694 / 0.937 / 0.963 |
| Phi | parametric | pattern=near-zero-rater | 5/27 | 0.880 / 0.945 / 0.957 |
| Phi | parametric | data=binary | 5/9 | 0.927 / 0.946 / 0.955 |
| Phi | parametric | data=normal | 9/81 | 0.694 / 0.944 / 0.971 |
| Phi | parametric | n_r=2 | 3/30 | 0.694 / 0.939 / 0.971 |
| Phi | parametric | n_r=3 | 5/30 | 0.800 / 0.946 / 0.966 |
| Phi | parametric | n_r=5 | 6/30 | 0.870 / 0.947 / 0.963 |

## Phi experiment (2026-10-25)

Raw rows are in `coverage-2026-10-25/phi-experiment.md`: n_sim = 1900,
B = 999, at c5ee8b4. Design: journal "Phi experiment".

| Cell | Zeroed estimate / percentile (salib) | Bias-corrected | Basic | Width: estimate vs true spread |
|---|---|---|---|---|
| Jiang et al. 100×20×2 (their normal condition) | **.960** | .944 | .916 | .245 vs .237 |
| Control 20×20×3 all-one | .953 | .946 | .801 | .579 vs .591 |
| Control 50×8×5 all-one | .951 | .943 | .873 | .345 vs .349 |
| 200×20×2 high-G | .696 | .706 | .558 | .192 vs .231 |
| 200×20×2 all-one | .798 | .794 | .631 | .356 vs .443 |
| 200×20×3 high-G | .798 | .816 | .674 | .122 vs .133 |
| 200×8×2 high-G | .814 | .746 | .595 | .193 vs .239 |
| 50×20×2 high-G | .846 | .817 | .653 | .223 vs .258 |

- **Fidelity.** salib's method reproduces Jiang et al.'s coverage in their own
  condition (.960 against their reported .952), and passes the control cells.
  The departures from their procedure (zeroed ANOVA instead of REML
  generating values, and a percentile interval, which they do not specify)
  are not what fails.
- **Limit.** In cells with many persons and items but two or three raters,
  the bootstrap spread is 14–20% narrower than the true spread, and every
  miss lies above the truth. This fits the plug-in hypothesis: with one or
  two rater degrees of freedom the estimated rater variance is often zeroed
  or too small, so the bootstrap simulates too little rater variation and too
  high a Φ. Bias-corrected percentile does not fix it, and the basic
  interval is worse.
- **Flawed oracle arm.** Generating from the true components makes the
  percentile interval essentially fixed across replications, so it always
  covers the truth (1.000 by construction), and the basic and bias-corrected
  rows built on it are not meaningful. Only its widths, the true sampling
  spread of Φ̂, are used above.

**Decision (operator).** Keep the parametric percentile Phi interval,
documented as reproducing Jiang et al. (median .946 over the 90 study cells)
and unreliable when n_r ≤ 3 with large n_p and n_i. A follow-up should
propagate rater-component uncertainty, either with a nested bootstrap or a
closed form analogous to G's F interval.
