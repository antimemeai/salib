# Bootstrap and interval procedures for crossed G-studies: what the literature establishes

Date: 2026-10-23. Reading-only literature note for salib issue #5 (`bootstrap_g_theory_pir`, plain persons/items/raters resampling, signed ANOVA refits, uncorrected percentile intervals). Sources are the extracted texts and PDFs in `crolib/papers/downloads/uncertainty/`. Page numbers are each report's printed numbers, or journal pages where a journal is cited. Brennan (2006, CASMA 21) and Owen's pigeonhole papers are being read separately. They are cited here only where my sources depend on them. Text marked **[Inference]** is mine. Everything else is stated by the cited source.

**Naming hazard.** In ACT RR 87-7 (Brennan, Harris & Hanson 1987, p. 10) and in Li & Zhang (2009, p. 891), the "r" in "boot-p,i,r" means **residuals** of a p×i design. salib's plan resamples persons, items and **raters**, which is Tong & Brennan's **boot-p,i,h** for p×i×h. Below I write rater-facet plans with *r* and say so whenever a source means residuals.

## 1. Bootstrap plans and component rules

**Plans.** A p×i×h design has seven plans: boot-p, boot-i, boot-h, boot-p,i, boot-p,h, boot-i,h and boot-p,i,h. Each resamples the named facets independently with replacement and keeps the crossed responses of the drawn levels (Tong & Brennan 2004 [CASMA 5], §2.2, p. 3; Tong & Brennan 2006 [CASMA 15], §2.2, pp. 3–4). For p×i, Li & Zhang (2009, p. 891) list six single strategies: boot-p, boot-i, boot-pi, boot-pr, boot-ir and boot-pir, where r is the residual. Li & Zhang (2013, p. 115) add Wiley's composite strategies boot-oneway, boot-individual and boot-main.

**Tong & Brennan rules (CASMA 5, p. 2).** These rules apply to both component estimates and their SEs, together with the bias corrections in §2:
- Rule 1: for a main effect, use the matching plan, for example boot-p for σ²(p).
- Rule 2: for an interaction, use the one-facet plan for the facet in the interaction with the largest sample size.
- Rule 3: when one facet's sample size is very large, that facet's plan may work as well as, or better than, Rules 1–2.

The authors say these rules "are not formally proved … but … supported by normal-data simulations" (p. 2).

**Rule table for p×i×h.** The CASMA 5 rows come from Table 6 (p. 12), whose values match the cited plan's rows in Tables 4–5 (pp. 11). The design was 100×20×2, normal data. The CASMA 15 rows come from §3.1.2 (pp. 6–7), whose evidence covers normal, dichotomous and polytomous data. The last column is **[Inference]**: the rules applied to salib's 20×8×4 simulation (n_p > n_i > n_r).

| Component | CASMA 5 (Rules 1–2, Table 6) | CASMA 15 (best for SE) | salib 20×8×4 under the rules |
|---|---|---|---|
| σ²(p) | boot-p | boot-p | boot-p |
| σ²(i) | boot-i | boot-i | boot-i |
| σ²(h)=σ²(r) | boot-h | boot-h | boot-r |
| σ²(pi) | boot-p | boot-p | boot-p |
| σ²(ph) | boot-p | boot-p | boot-p |
| σ²(ih) | boot-i | boot-i | boot-i |
| σ²(pih) | boot-p | boot-p | boot-p |
| σ²(δ) | — | boot-p ("consistently … most accurate", p. 8) | boot-p |
| σ²(Δ) | — | **none** ("No bootstrap procedure performed well", p. 8) | none |

No source recommends boot-p,i,h for any component.

**p×i sources (two-component designs only).**

| Source | σ²(p) | σ²(i) | σ²(pi) |
|---|---|---|---|
| ACT 87-7, normal data (pp. 19, 25) | boot-p | boot-i | boot-p if n>k, boot-i if k>n |
| ACT 87-7, binary data (p. 25) | boot-p | boot-i | boot-p,i |
| Li & Zhang 2009, SE (p. 899) | boot-padj or boot-iradj | boot-piadj | boot-iradj, boot-piradj, boot-pradj |
| Li & Zhang 2009, CI (p. 899) | boot-iradj | boot-piadj | boot-iradj, boot-pradj, boot-padj |
| Li & Zhang 2013 (pp. 120–122); Li 2023 (Table 6, p. 13; §5) | boot-p | boot-pi | boot-i |

The p×i sources disagree on σ²(i) (boot-i versus boot-pi) and on σ²(pi) (boot-p or boot-p,i versus boot-i). The authors themselves note that the best plan depends on the data distribution (ACT 87-7, pp. 20–21, 25).

## 2. Bias corrections

**Wiley (2000) multipliers for p×i** (reproduced in CASMA 5, Appendix A, p. 16). For boot-p:

- σ̂²(p) = n_p/(n_p−1)·σ̂²(p|boot-p)
- σ̂²(i) = σ̂²(i|boot-p) − σ̂²(pi|boot-p)/(n_p−1)
- σ̂²(pi) = n_p/(n_p−1)·σ̂²(pi|boot-p)

boot-i is symmetric. For boot-p,i:

- σ̂²(p) = n_p/(n_p−1)·σ̂²(p|·) − n_p/[(n_p−1)(n_i−1)]·σ̂²(pi|·)
- σ̂²(pi) = n_p n_i/[(n_p−1)(n_i−1)]·σ̂²(pi|·)

ACT 87-7 found the multipliers empirically. Under boot-p, "n/(n−1)σ̂²_B(p) and n/(n−1)σ̂²_B(pi) are nearly unbiased", and under boot-i, k/(k−1) gives the same (p. 16). It applies n/(n−1), k/(k−1) and [n/(n−1)][k/(k−1)] to the **estimates, their SEs** (p. 25) **and the percentile limits** (Table 2 note b, p. 48).

**General Tong & Brennan correction** (CASMA 15, Eq. 6–7, pp. 4–5; CASMA 5, Eq. 4–5, p. 4). Let α be the effect, K the set of resampled facets k_j, and c_i the facets in both α and K. When α and K share facets:

σ̂²(α) = [Π n_ci/Π df(c_i)]·σ̂²(α|K) − Σ_{k_j∉α} [Π n_ci/(df(k_j) Π df(c_i))]·σ̂²(αk_j|K) + Σ_{k_j≠k_m ∉α} [Π n_ci/(df(k_m)df(k_j)Π df(c_i))]·σ̂²(αk_jk_m|K)

When they share none, the factors Π n_ci/Π df(c_i) are dropped (Eq. 7). Here df(x) = n_x − 1. The authors state "no rigorous derivations are provided" (CASMA 15 abstract, p. iv; p. 8).

**Exact corrections for salib's plan.** These are CASMA 5, Appendix B, "boot-p,i,h", pp. 19–20, with h written as r and b(·) = σ̂²(·|boot-p,i,r):

- σ̂²(p) = a·b(p) − a/(n_i−1)·b(pi) − a/(n_r−1)·b(pr) + a/[(n_i−1)(n_r−1)]·b(pir), with a = n_p/(n_p−1). σ̂²(i) and σ̂²(r) follow by symmetry.
- σ̂²(pi) = n_pn_i/[(n_p−1)(n_i−1)]·[b(pi) − b(pir)/(n_r−1)]. σ̂²(pr) and σ̂²(ir) follow by symmetry.
- σ̂²(pir) = n_pn_in_r/[(n_p−1)(n_i−1)(n_r−1)]·b(pir).

**[Inference, checked numerically.]** Inverting these equations predicts the replicate-minus-estimate bias at the mean point estimates in issue #5 (20×8×4, all components 1):

| | σ²(p) | σ²(i) | σ²(r) | σ²(pi) | σ²(pr) | σ²(ir) | σ²(pir) |
|---|---|---|---|---|---|---|---|
| Predicted | +0.329 | +0.144 | −0.118 | +0.040 | −0.192 | −0.310 | −0.376 |
| Observed (MCSE) | +0.328 (0.006) | +0.144 (0.009) | −0.119 (0.016) | +0.042 (0.003) | −0.189 (0.005) | −0.313 (0.008) | −0.377 (0.002) |

Every prediction falls within about one MCSE. The bias salib observed is therefore exactly the known boot-p,i,h bias.

**What the corrections fix.** They fix the **mean** of the replicates, not their **spread**.
- CASMA 5 applies the corrections "for each replication" (p. 6). Corrected means are close to the parameters under every plan (Table 4, p. 11). For SEs the bias correction "produced mixed results" (p. 6).
- Corrected boot-p,i,h gives SE(p) = 7.77 against a parameter of 3.45, and SE(pih) = 144.6 against 4.70 (Table 5, p. 11; CASMA 15 Table 5, p. 14). Accurate SEs came only from the component-specific rules.
- Li & Zhang (2009, p. 899, conclusion 2) found no evidence that adjusted bootstrap improves SEs over unadjusted.
- Li & Zhang (2013) report a marginal SE improvement (paired t = 2.021, p = 0.051; §4.2, p. 119) and a significant reduction in coverage deviation (Table 4, p. 121: mean |coverage − 0.80| was 0.081 adjusted against 0.264 unadjusted).
- Brennan (2006, CASMA 21) generalizes the unbiased-point corrections. Li & Zhang 2009 cite it (p. 899). It is not covered here.

## 3. Interval methods evaluated

| Source and conditions | Method → coverage (nominal) |
|---|---|
| ACT 87-7, p×i 200×20, normal, σ²(p)=4, σ²(i)=16, σ²(pi)=64, 1000 trials (p. 18) | Satterthwaite for p, i, pi: 81.7, 81.5, 79.7% (80%). Bootstrap and jackknife intervals were compared with simulated percentiles in a single trial only (Table 2, p. 48). |
| ACT 87-7, binary data, finite population N=2000, K=200, samples 200×20, 1000 trials (Table 7, p. 53) | Satterthwaite: 72.4, 83.9, **36.8%** (80%). Jackknife: 78.9, 76.4, 77.2%. Bootstrap coverage "not obtained", judged too costly (p. 23). |
| Li & Zhang 2009, p×i 100×20, normal, 4/16/64, 1000 replications (Table 2, p. 895) | Satterthwaite 0.811, 0.832, 0.815. TBGJL 0.792, 0.826, 0.808. Percentile, unadjusted boot-pi (residual-free analogue of salib's plan) **0.209**, 0.790, **0.617**. Percentile boot-padj (p), boot-piadj (i), boot-iadj (pi): 0.769, 0.799, 0.847. Percentile boot-piadj for p and pi: **0.988, 0.970**. BCa ≈ percentile ("no higher accuracy", p. 899). MCMC with informative priors 0.769, 0.795, 0.790. Jackknife 0.058, 0.177, 0.000 (all at 80%). |
| Li & Zhang 2013, p×i, normal/dichotomous/polytomous/skewed (Table 3, p. 118) | Adjusted rule plans: boot-p for p 0.769, 0.768, 0.783, 0.737; boot-pi for i 0.799, 0.817, 0.671, 0.672; boot-i for pi 0.847, 0.799, 0.838, 0.812 (80%). |
| Jiang et al. 2022, p×i×r 100×20×2, Tong & Brennan parameters, 1000 replications, 500 draws each (Method; Table 2) | Phi interval: parametric bootstrap (PB) **0.9516** normal, **0.9522** dichotomous, **0.9164** polytomous. Semiparametric 0.615, 0.824, 0.190. Nonparametric observation-level resampling 0.257, 0.609, 0.065. Posterior simulation about 0.05 (95%). Nested p×(i:r): PB 0.958, 0.962, **0.810**. |
| Jiang et al. 2022, crossed, n_p∈{50,200,500}, n_i∈{5,15,30}, n_r∈{3,5}, 18 cells (Table 4; Appendix) | Components only, PB ≈ Satterthwaite (difference < 0.01). Mean coverage: σ²(p) .945, σ²(i) .903, **σ²(r) .819**, σ²(pi) .946, σ²(ir) .916, σ²(pr) .929, σ²(pir) .948 (95%). Worst σ²(r): **0.572 / 0.585** in condition 9. |
| Betebenner 1998, modified large-sample (MLS) intervals on σ²(Δ) and σ²(δ), normal pivot simulation, 10,000 replications (Table 1, p. 7) | p×i×h σ²(Δ): lower [0,U] .946–.955 and two-sided .896–.905 (90%). Upper [L,∞) .935–.959, "sometimes … liberal" (p. 8). |

**Formulas.**
- **Satterthwaite** (ACT 87-7, Appendix A, pp. 30–31): σ̂²ν/χ²_{1−α/2}(ν) ≤ σ² ≤ σ̂²ν/χ²_{α/2}(ν), with ν = 2r² and r = σ̂²/σ̂[σ̂²]. Here σ̂[σ̂²] = [Σ_j 2(f_jMS_j)²/df_j]^{1/2}, where f_j is the coefficient of mean square MS_j.
- **Jackknife for p×i** (ACT 87-7, Appendix B, pp. 32–34):
  - Pseudovalues: θ̂_{*pi} = nkθ̂_{−00} − (n−1)kθ̂_{−p0} − n(k−1)θ̂_{−0i} + (n−1)(k−1)θ̂_{−pi}. The subscript names the deleted person, item or both.
  - SE: [c_nσ̂²(rows)/n + c_kσ̂²(cols)/k + c_nc_kσ̂²(res)/nk]^{1/2}, with c_n = 1−n/N and c_k = 1−k/K.
  - Interval: θ̂_J ± t·SE. None of my sources extends this to p×i×r.
- **MLS bounds for signed combinations** σ̂² = Σc_qS_q² − Σc_rS_r² (Betebenner, Eq. 4–5, pp. 5–6): U = σ̂² + √V_U and L = σ̂² − √V_L. V_U and V_L are built from G = 1 − 1/F_{α:n,∞}, H = 1/F_{1−α:n,∞} − 1 and the cross terms H_qr, H*_rt (G_qr, G*_qu for V_L). Printed Eq. 6 shows √V_L on both sides; the upper side should read √V_U. Betebenner notes that Satterthwaite and Welch assume positive coefficients, an assumption the p×i σ²(p) estimator already violates (p. 3).

**Where methods fail, according to the sources.**
- **Absolute error variance.** No facet bootstrap estimates SE(Δ). Normal data: boot-p gives 0.770 against a parameter of 1.143, while boot-p,i,h gives 1.77 raw and 3.83 adjusted (CASMA 15, Tables 4–5, pp. 13–14; p. 8).
- **Few facet levels.** With n_h = 2, every plan that resamples h gives corrected SE(pi) and SE(pih) of about 144 against 5.0 and 4.7 (CASMA 5, Table 5, p. 11). In Jiang et al., σ²(r) coverage stays well below 95% with 3–5 raters (Table 4).
- **Nonnormal data.**
  - Satterthwaite covers binary σ²(pi) only 36.8% of the time at nominal 80% (ACT 87-7, Table 7).
  - The parametric bootstrap drops to 0.92 on polytomous crossed data and 0.81 on nested (Jiang et al., Table 2).
  - Traditional and MCMC SEs are about 33% low under high skew, and MCMC fails for polytomous σ²(i) (Li 2023, p×i 100×20, Tables 3–4, pp. 10–11).
- **Near-zero components.** No source reports interval coverage with a true component at or near zero in p×i×r. The closest case is polytomous σ²(ih) = 0.0025 (SE 0.0081), for which only corrected point estimates and SEs are reported (CASMA 15, Tables 1, 3 and 5).
- **Caveats on Li & Zhang 2009.** The informative MCMC priors use the generating values (p~τ(2,4), i~τ(2,16), pi~τ(2,64); Li 2023 §2.2.4, p. 6). The 2009 jackknife coverage (0.000–0.177) contradicts ACT 87-7 and Li 2023's later "synthesized" jackknife (0.761–0.775, Table 1, p. 8). **[Inference]** The 2009 jackknife result is probably an implementation artefact.

## 4. Intervals for G (Eρ²) and Φ

- **Φ.** Only Jiang et al. (2022) evaluate a coefficient interval. Despite the title, their target is Φ: "the generalizability coefficient Eρ²_Δ corresponds to absolute error, instead of relative error" (Method, Eq. 5–7, footnote 1).
  - Recommended method: a parametric bootstrap from a fitted linear mixed model (lme4). Fit the model, draw b* ~ N(0, Ĝ) and ε* ~ N(0, R̂), refit, recompute Φ.
  - Evidence: one crossed cell (100×20×2), where the parametric bootstrap reached nominal coverage for normal and dichotomous data but 0.916 for polytomous (Table 2).
  - The 18-cell second study reports component coverage only.
  - The interval construction (percentile or otherwise) is not stated in the extracted text.
- **G (relative).** No source evaluates an interval for G in any design. Li (2023, §4.3, p. 16) lists D-study coefficients as future work.
- **Combining component intervals.** ACT 87-7 says it is "not clear how to extend [the bootstrap] to … functions of variance components", such as σ²(Δ) (p. 26). Jiang et al. state that component CIs "cannot be converted" to coefficient CIs in closed form (Discussion).
- **Answer.** No established, reasonably calibrated method exists for G in crossed p×i×r. For Φ, the only evidence is the single-condition parametric bootstrap result. The component-specific bootstrap rules (CASMA 5 and 15; Li 2023) have never been evaluated for coefficient intervals.

## 5. Explicitly unresolved

- **CASMA 5.** The rules and corrections are "tentative" and "not formally proved". Extending them to dichotomous and polytomous data and to more complex designs is still needed (pp. iv, 2, 8).
- **CASMA 15.** No rigorous derivations are given, and no plan works for SE(Δ) (pp. iv, 8).
- **ACT 87-7.**
  - The bootstrap needs "several ad hoc procedures" (p. iii).
  - It is unclear how to bootstrap functions of components (p. 26).
  - Bootstrap coverage was not simulated (pp. 18, 23).
  - "Future research may reveal … a different way to conceive of bootstrap samples" (p. 27).
  - Familywise coverage would need Bell/Khuri-type adjustments (footnote 2, p. 46).
- **Li & Zhang 2009.** The conclusions hold only for normal data in a fully random p×i design (p. 899). Why BCa is no better than percentile needs "further analysis" (p. 898).
- **Li 2023, §4.3.** Results come from a fixed 100×20 p×i design with a relative-percentage-bias criterion. D-study coefficients are untested. **[Inference]** The |relative bias| < 25% criterion accepts coverage anywhere in 0.60–1.00 at nominal 0.80, so its "accurate" label is not a calibration claim.
- **Jiang et al.** Coverage degrades for polytomous data and for facets with few levels. Bayesian methods are not addressed (Discussion).
- **Betebenner.** Upper one-sided bounds are sometimes liberal, and "more analysis using larger data sets is needed" (pp. 8–9).

## 6. Recommendation for salib (all [Inference])

**Do not keep boot-p,i,r, even with the CASMA 5 Appendix B corrections.** The corrections remove the centring bias in issue #5; the table in §2 confirms that. They leave the spread inflated, though. Corrected boot-p,i,h SEs were 2–30× too large (CASMA 5, Table 5). In p×i, adjusted boot-pi percentile intervals over-covered: 0.988 and 0.970 at nominal 0.80 (Li & Zhang 2009, Table 2). Raters with few levels make this worse.

**Best supported small implementation: a parametric bootstrap.**
1. Take the fitted components, clipping negative ones to zero for generation only.
2. Simulate Y* = μ̂ + seven independent normal effect arrays at the grid's (n_p, n_i, n_r).
3. Refit with salib's signed ANOVA estimator.
4. Report percentile intervals for the components, G and Φ, keeping the existing failure-accounting rule for 0/0 and failed fits.

It is the only procedure with coefficient-level coverage evidence in crossed p×i×r, it matched Satterthwaite for components in 18 crossed cells (Jiang et al.), and it needs one resampling plan rather than a per-component patchwork. It departs from that evidence in two ways: Jiang et al. refit a nonnegative lme4 model where salib refits signed ANOVA, and clipping the generating components is my choice. So salib must earn its own coverage evidence. Satterthwaite and MLS bounds (ACT 87-7, Appendix A; Betebenner, Eq. 4–5) are cheap normal-theory cross-checks for the component intervals.

If SEs for individual components are wanted, the CASMA 15 rule table with corrections is the documented nonparametric option. It is supported for 100×20×2 SEs only: no intervals, no G or Φ, and never for SE(Δ).

**Evidence needed before claiming calibration** (Morris, White & Crowther 2019, §§3, 5.1–5.3):
- **Design.** Pre-specify aims, data-generating mechanisms, estimands, methods and performance measures.
- **Factors to vary independently.**
  - Facet sizes: n_p ∈ {20, 50, 200}, n_i ∈ {5, 8, 20}, n_r ∈ {2, 3, 5}.
  - Component patterns: include σ²(r), σ²(pr) and σ²(ir) equal to zero or near zero, and G and Φ near 0.5 and near 0.9.
  - Distributions: normal, dichotomous, polytomous and skewed.
- **Replications.** Use n_sim ≥ 1900 per cell, which gives MCSE ≈ 0.5% at 95% coverage (Eq. 1, §5.3); 1000 replications give 0.69%.
- **Measures to report.**
  - Coverage with its MCSE.
  - Lower-tail and upper-tail misses separately, since Betebenner found one-sided asymmetry.
  - Bias-eliminated coverage, to separate bias from width (§5.2).
  - Interval width.
- **Failures.** Count failed replicates as a performance measure rather than silently conditioning on success (§5.1).
- **Claim scope.** Claim calibration only for cells where coverage is within about ±2 MCSE of nominal in both tails.

**Document as unsupported until that study exists:**
- Intervals for relative G.
- Nonnormal or discrete scores.
- True zero or near-zero components.
- Facets with two or three levels.
- σ²(Δ) and SE(Δ) from any facet bootstrap.
- Finite universes (ACT 87-7 uses 1 − n/N corrections).
- D-study projections at counts other than the grid's.
- Simultaneous coverage across components or coefficients.
