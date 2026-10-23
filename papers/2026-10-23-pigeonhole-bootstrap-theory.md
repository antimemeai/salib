# Pigeonhole bootstrap theory for crossed arrays: grand mean, variance components, G

Date: 2026-10-23. Scope: reading plus derivation, for salib #5
(`bootstrap_g_theory_pir`, boot-p,i,r on a balanced crossed p×i×r grid).
Brennan's CASMA/ACT reports are being read separately and are not covered here.

## Sources and status

- **Owen (2007)**, "The pigeonhole bootstrap", Ann. Appl. Stat. 1:386–411.
  Read in full: `crolib/papers/downloads/uncertainty/owen-2007-pigeonhole.txt`.
- **Owen & Eckles (2012)**, "Bootstrapping data arrays of arbitrary order",
  Ann. Appl. Stat. 6:895–927. Read in full:
  `crolib/papers/downloads/uncertainty/owen-eckles-2012-data-arrays.txt`.
- **McCullagh (2000)**, "Resampling and exchangeable arrays", Bernoulli
  6:285–301. **Not acquired.** It has no entry in `crolib/papers/catalogue.json`.
  `lanes/uncertainty/search-log.json` shows a search for it and lists it as a
  citation target of the two Owen papers, but no local copy exists. Everything
  below about McCullagh is what the Owen papers say he showed. I have not
  reconstructed his paper.

Section A reports what the sources say. Section B reports what I derived and
checked. Throughout, *n_j* is the number of levels of factor *j*, and
*N = n_p n_i n_r*.

---

## A. What the sources state

### A1. Expected bootstrap variance of the grand mean

**Two factors, resampling (Owen 2007).** The random-effects model is
X_ij = µ + a_i + b_j + ε_ij. It may be unbalanced and heteroscedastic, and it
conditions on the observation pattern Z_ij.

- True variance (Lemma 1, eq. 3, homogeneous case):
  V_RE(µ̂) = (ν_A σ²_A + ν_B σ²_B + σ²_E)/N, where ν_A = N⁻¹ Σ_i n²_i•. For a
  complete R×C table, ν_A = C and ν_B = R.
- Naive IID bootstrap (Lemma 2, eq. 5): σ²_A(1−ν_A/N)/N + σ²_B(1−ν_B/N)/N +
  σ²_E/N. That understates the row and column terms by a factor of about ν.
  Owen & Eckles (§3.1) correct the coefficient of the interaction term to
  1/(N−1).
- Pigeonhole bootstrap, with rows and columns resampled independently with
  replacement (§3.2). The exact bootstrap variance is in Theorem 1 (totals)
  and Theorem 2 (ratio/delta-method mean). Its exact expectation under the
  model is in **Theorem 3**: (1/N²)[Σ_i σ²_A(i) λ^A_i + Σ_j σ²_B(j) λ^B_j +
  ΣΣ Z_ij σ²_E(ij) λ^E_ij], with λ's given explicitly. **Corollary 1** gives
  the approximation when 1/R, 1/C, n_i•/N, … are negligible:

  E[V̂_PB] ≈ [(ν_A + 2)σ²_A + (ν_B + 2)σ²_B + 3σ²_E]/N.

  The interaction term is inflated by about 3, and each main effect gains
  about +2σ²/N. **Theorem 4** ("mean consistency"): the relative error is
  O(ε_N) under bounded heteroscedasticity, where ε_N collects 1/R, 1/C,
  ν/N, 1/ν and max n/N.

**r factors (Owen & Eckles 2012).** The model is X_i = µ + Σ_{u≠∅} ε_{i,u}
(eq. 1). The true variance is Var(X̄) = N⁻¹ Σ_u ν_u σ²_u (**Theorem 1**). The
authors analyse only **product reweighting**: W_i = Π_j W_{j,i_j} with IID
weights of mean 1 and variance τ². They use the delta-method variance (eq. 13).
They state that the earlier results for multinomial resampling "only apply to
two factors" (abstract, §1).

- **Theorem 5** (exact): E[Var~_PW] = N⁻¹ Σ_u γ_u σ²_u, with
  γ_u = Σ_k (1+τ²)^k (ν_{k,u} − 2ν̃_{k,u} + ρ_k ν_u).
- **Theorem 6**: γ_u = ν_u[(1+τ²)^{|u|} − 1 + θ_u ε] +
  Σ_{v⊋u}(1+τ²)^{|v|}(τ²)^{|v−u|} ν_v, with |θ_u| ≤ 2^{r+1} − 2 at τ² = 1. For
  r = 3: γ_{1} ≈ ν_1 + 4ν_12 + 4ν_13 + 8, γ_{12} ≈ 3ν_12 + 8, and
  γ_{123} ≈ 7.
- **Theorem 7**: 2^{|u|} − 1 − (2^{r+1} − 2)ε < γ_u/ν_u ≤
  2^{|u|}(1+2η)^{|v−u|} − 1 + (2^{r+1} − 2)ε (exponent as printed). A k-factor component is therefore
  magnified by about 2^k − 1, main effects come out nearly right, and the
  ratio to the true variance is 1 + O(η + ε).
- Heteroscedastic analogues: Theorems 8–11.
- Caveat for salib, stated in §5: for a complete N₁×…×N_r array,
  ε = max_j 1/N_j. In 20×8×4, ε = 1/4, so the O(ε) bounds (|θ| ≤ 14) tell
  us nothing at G-study sizes. The approximations are designed for large
  sparse data.

### A2. Variance components and nonlinear functionals

- **Neither paper proves a consistency, bias or correction result for
  variance components, or for ratios of components such as G.** Both treat
  only (ratio) means. Owen 2007 §2.2 argues that a bootstrap that fails for a
  mean "cannot be trusted for more complicated usages". One that works for
  means "can be expected to be reliable" for smooth functions of means and
  estimating equations. Owen & Eckles §9 extend to smooth g(X̄) via Taylor
  expansion, citing Hall (1992) and Mammen (1992). They say a correctly
  calibrated variance plus a CLT gives asymptotically correct percentile
  intervals, and an overestimated one gives conservative intervals. A
  variance component is a quadratic, not a mean, so none of this covers it.
- Owen & Eckles §1 note that Brennan, Harris & Hanson (1987) and Wiley (2001)
  used product-multinomial resampling for variance components in testing
  data. They give no result of their own on this.
- Bias (Owen 2007 §6.3, §5.1): pigeonhole replicate *means* are biased,
  because N* is random and the mean is a ratio. In the Netflix example the
  bias is small in absolute terms but not small relative to the bootstrap SD,
  and it nearly cancels in differences.
- **McCullagh's no-exact result, as cited.** Owen 2007 §1: no bootstrap
  "can exist, even for the basic problem of finding the variance of the grand
  mean" in a balanced, complete, homoscedastic two-way setting. Owen & Eckles
  §1: "no resampling method, from a very broad class, will provide an unbiased
  estimate of Var(X̄)". Both state it for the bootstrap *variance of the grand
  mean*. Neither applies it to the bootstrap *mean* of a component. Owen 2007
  §1 adds that McCullagh "leaves open the possibility that a linear
  combination of several bootstrap methods will be suitable". It also reports
  that McCullagh found the pigeonhole bootstrap mildly conservative in the
  balanced homoscedastic case. I cannot check his exact class of resampling
  schemes because the paper is not here.

### A3. Alternatives discussed, and the properties claimed

| Scheme | Source | Claimed property |
|---|---|---|
| Naive IID resampling of cells | Owen 2007 Lemma 2; O&E Thm 2 | Captures σ²_u(1−ν_u/N)/N; badly anticonservative when ν_u ≫ 1 |
| Naive IID reweighting (Bayesian bootstrap, Poisson, U{0,2}) | O&E §3.2, Thm 3 | Same mean as naive, times τ²; can be made conservative by setting τ² = max ν_u |
| Choice of weight distribution | O&E §3.3, Thm 4 | Same mean for any weights with this τ²; stability improves with low weight kurtosis: U{0,2} (κ = −2) is best, Poi(1) κ = 1, Exp(1) κ = 6 |
| Pigeonhole (independent multinomial on each factor) | Owen 2007 Thm 3–4, Cor 1 | r = 2: mildly conservative, about (ν+2)σ²_A, (ν+2)σ²_B, 3σ²_E; mean-consistent |
| Product reweighting ("Bayesian pigeonhole" with Exp(1) weights; any τ²) | O&E Thm 5–7, 9–11 | Any r: gain ≈ (2^{\|u\|}−1)ν_u, conservative; parallel/online friendly; matches pigeonhole for r = 2 |
| Pigeonhole minus 2 × naive | Owen 2007 §1 | Removes the excess for r = 2; can give negative variances |
| Fit a random-effects model and resample from it | McCullagh, via Owen 2007 §1 | "nearly correct" for balanced data; Owen finds it unattractive for unbalanced data |
| Reweight only some factors | O&E §8, Figs 1–2 | In the Facebook example, 1- and 2-factor reweighting "may substantially overstate confidence" |
| Nested factors: reweight outer factors only | O&E §7 | Common practice; nested replicates become a heteroscedastic r-factor model |

---

## B. Derivations (mine) for a balanced crossed p×i×r design

### B1. The pigeonhole world is a random-effects model with components v_u = SS_u/N

Write the observed grid as its orthogonal ANOVA decomposition,
x_{abc} = µ̂ + Σ_{u≠∅} e_u(index_u). Each e_u sums to zero over every index in
u, and SS_u = (Π_{j∉u} n_j) Σ e_u². One replicate draws P_a, I_b, R_c IID
uniform, and x*_{abc} = x_{P_a I_b R_c}. Consider two cells of the replicate
that agree in exactly the positions M. Then:

- For u ⊄ M, some index j ∈ u is drawn independently in the two cells, so
  E*[e_u(·) e_u(·)] = E*[e_u(…)]·(…) = 0, because e_u has zero mean over that
  index.
- For u ⊆ M, the expectation is v_u ≡ (Π_{j∈u} n_j)⁻¹ Σ e_u² = SS_u/N.
- For u ≠ w, cross terms vanish by the same zero-margin argument.

So, conditional on the data, the replicate has exactly the covariance
structure of a crossed random-effects model with **uncorrelated** (not
independent) effects and components **v_u = SS_u/N = (df_u/N)·MS_u**. Ties
from sampling with replacement are absorbed automatically, because the draws
are IID. The finite-population "divide by n_j" components replace the
"divide by n_j − 1" ones.

**T-terms.** Let T_t = (Π_{j∉t} n_j) Σ x̄_t², and SS_u = Σ_{t⊆u}
(−1)^{|u−t|} T_t. Then

E*[T*_t] = N µ̂² + N Σ_{w≠∅} v_w / Π_{j∈w∖t} n_j.

This is exactly the random-effects expected T-term, with σ²_w replaced by v_w.

### B2. Bootstrap mean of each EMS component (exact, conditional on the data)

Expected mean squares need only second moments, so E*[MS*_u] =
Σ_{w⊇u} (Π_{j∉w} n_j) v_w, and the signed EMS solve returns

**E*[σ̂*²_u] = v_u = SS_u/N = Σ_{w⊇u} A_{uw} σ̂²_w,**
A_{uw} = Π_{j∈u}(1 − 1/n_j) · Π_{j∈w∖u}(1/n_j).

A is a Kronecker product over the factors of A_j = [[1, 1/n_j], [0, (n_j−1)/n_j]]
(rows and columns: factor absent or present), restricted to nonempty subsets.
Written out:

- E*σ̂*_p = (1−1/n_p)[σ̂_p + σ̂_pi/n_i + σ̂_pr/n_r + σ̂_pir/(n_i n_r)]
- E*σ̂*_pi = (1−1/n_p)(1−1/n_i)[σ̂_pi + σ̂_pir/n_r]
- E*σ̂*_pir = (1−1/n_p)(1−1/n_i)(1−1/n_r) σ̂_pir

The i, r, pr and ir rows follow by symmetry. Note that **E*[σ̂*_u] = SS_u/N ≥ 0
always**. A negative point estimate therefore always has a nonnegative
uncorrected bootstrap mean.

Check against the #5 simulation (20×8×4, all σ̂ = 1; bias = row sum of A − 1):

| | p | i | r | pi | pr | ir | pir |
|---|---|---|---|---|---|---|---|
| derived | +0.336 | +0.148 | −0.114 | +0.039 | −0.198 | −0.311 | −0.377 |
| #5 simulation | +0.328 | +0.144 | −0.119 | +0.042 | −0.189 | −0.313 | −0.377 |

### B3. The exact linear correction

Since A_j⁻¹ = [[1, −1/(n_j−1)], [0, n_j/(n_j−1)]], the corrected replicate is

**σ̃*_u = Σ_{w⊇u} B_{uw} σ̂*_w,  B_{uw} = Π_{j∈u} n_j/(n_j−1) ·
Π_{j∈w∖u} (−1/(n_j−1)),**

and it satisfies E*[σ̃*_u] = σ̂_u exactly. Written out:

- σ̃*_pir = N/[(n_p−1)(n_i−1)(n_r−1)] · σ̂*_pir
- σ̃*_pi = n_p n_i/[(n_p−1)(n_i−1)] · [σ̂*_pi − σ̂*_pir/(n_r−1)]
- σ̃*_p = n_p/(n_p−1) · [σ̂*_p − σ̂*_pi/(n_i−1) − σ̂*_pr/(n_r−1) +
  σ̂*_pir/((n_i−1)(n_r−1))]

An equivalent implementation: in each replicate, replace each mean square
MS*_u by (N/df_u)·σ̂*_u, then run the usual EMS solve (A⁻¹ = E⁻¹·diag(N/df)).
In words, the replicate's uncorrected components estimate the pigeonhole
world's v, and B maps v back to σ̂. The correction is purely multiplicative only for σ²(pir). Every
lower-order component needs the subtraction of higher-order terms, so a
per-component scale factor cannot be unbiased. With a two-level facet
(n_j = 2), the factors become 2 and −1. The colleague reading CASMA 21
should check that its T-term correction for boot-p,i,r reduces to this B.

### B4. Grand mean: the exact r-way expression for balanced arrays

N* = N is fixed for complete grids, so no ratio approximation is needed.
From B1, Var*(X̄*) = Σ_u v_u / Π_{j∈u} n_j exactly. Taking E_RE of v_u (by
EMS) and summing over u ⊆ w with Σ_{∅≠u⊆w} Π_{j∈u}(1−1/n_j) = Π_{j∈w}(2−1/n_j) − 1
gives

**E_RE[Var_PB(X̄*)] = Σ_{w≠∅} [Π_{j∈w}(2 − 1/n_j) − 1] · σ²_w / Π_{j∈w} n_j,**
compared with **Var(X̄) = Σ_{w≠∅} σ²_w / Π_{j∈w} n_j**.

The inflation factor for a k-factor component is Π(2 − 1/n_j) − 1, which tends
to 2^k − 1. For main effects it is 1 − 1/n_j, so they are slightly
*understated*. For pi it is ≈ 3 and for pir ≈ 7. For r = 2 this agrees
exactly with Owen 2007 Theorem 3 specialised to a complete R×C table: I
computed λ^A_i = (1−1/R)C² and λ^E = 3 − 2/R − 2/C + 1/(RC). It also matches
O&E Theorem 7's 2^{|u|} − 1. The **delta-method product reweighting** of
O&E's Theorem 5 behaves differently for balanced arrays. Its closed form is

γ_w = Π_{j∉w} n_j · [(1+τ²)^{|w|} Π_{j∉w}(1+τ²/n_j) − Π_{all j}(1+τ²/n_j)].

This matches Theorem 5 exactly when evaluated on 3×2×2 and 20×8×4 grids. In
20×8×4 with τ² = 1 it inflates σ²_p by 1.336, against 0.95 for resampling.
The random N* matters when the n_j are small.

**Partial resampling and linear combinations.** Resampling only the factors
in S gives Var*_S(X̄*) = Σ_{u⊆S} v_u/Π_{j∈u} n_j. By Möbius inversion over S,
a linear combination of the 2^r − 1 partial bootstraps recovers each v_u, and
hence Σ_w σ̂_w/Π n_w, which is unbiased for Var(X̄). This is one concrete form
of the "linear combination" McCullagh reportedly leaves open. In the balanced
case it is just the ANOVA plug-in estimate, and it can be negative.

### B5. Bearing on McCullagh and on "can the pigeonhole bootstrap be unbiased?"

The obstruction is that the pigeonhole world has components v = Aσ̂, not σ̂.
That is why its grand-mean variance is off (B4). It is also why replicate
components are off-centre (B2). In balanced designs **the centring of the
components can be corrected exactly by a fixed linear map** (B3), and
nothing in the cited form of McCullagh's result rules this out. What the map
cannot correct is the replicate *spread*. The second moments of quadratic
statistics in the pigeonhole world are not those of the target model, and B
amplifies them further (see B7).

### B6. Verification (exact)

Scratch: `/private/tmp/claude-501/-Users-patrickbeam-projects-crolib/ce5e0064-4369-4e4e-8c2e-7b16587e3634/scratchpad/pigeonhole/`

- `exact.py` uses SymPy and enumerates every resample. On a **symbolic 2×2×2**
  grid (8 free data symbols, 64 resamples), and on exact-rational 3×2×2,
  2×3×2, 2×2×3 and 3×3×2 grids (up to 2,916 resamples), it confirms
  E*σ̂*_u − (Aσ̂)_u = 0, (B·E*σ̂*)_u − σ̂_u = 0, E*σ̂*_u − SS_u/N = 0,
  E*X̄* = X̄, and Var*X̄* = Σ v_u/Πn_u. Every difference is identically zero.
- `partial.py` checks the partial-resampling Var*_S formula exactly for
  S = p, i, pi, pr, pir.
- `owen_eckles_t5.py` checks that O&E Theorem 5 equals the balanced closed form
  in B4, with exact rationals, at τ² = 1 and 1/2.
- `salibcheck/` is a scratch crate using salib's `estimate_g_theory_pir`. It
  enumerates all resamples of 3×2×2, 2×3×2, 3×3×2 and 4×2×2 grids, treating an
  all-identical resample as zeros, as `bootstrap_g_theory_pir` does. The
  largest relative error of E* − Aσ̂ is 4.0e-15, and of B·E* − σ̂ is 4.1e-15.
  In one grid σ̂_r = −5.45 while E*σ̂*_r = +0.004 (see B2).

### B7. Monte Carlo: centring fixed, spread not (exploratory, mine)

`sim.py` uses normal random-effects data with all σ² = 1, 400 bootstrap
replicates per dataset and the clipped G. SD* is the mean within-dataset
bootstrap SD, and "true SD" is the across-dataset SD of σ̂. Results for 20×8×4
(400 datasets):

| comp | bias uncorr | bias corr (MCSE) | true SD | SD* uncorr | SD* corr | P(σ̃* < 0) |
|---|---|---|---|---|---|---|
| p | +0.336 | −0.002 (0.002) | 0.445 | 0.654 | 0.718 | 0.085 |
| i | +0.147 | −0.001 (0.003) | 0.724 | 0.794 | 0.946 | 0.178 |
| r | −0.125 | −0.001 (0.003) | 1.063 | 0.781 | 1.041 | 0.263 |
| pi | +0.040 | −0.000 (0.001) | 0.145 | 0.302 | 0.397 | 0.001 |
| pr | −0.198 | +0.002 (0.001) | 0.219 | 0.329 | 0.448 | 0.002 |
| ir | −0.313 | +0.003 (0.002) | 0.305 | 0.383 | 0.578 | 0.010 |
| pir | −0.378 | +0.000 (0.001) | 0.070 | 0.187 | 0.300 | 0.000 |

For G (true value 0.711): mean Ĝ = 0.680, with SD 0.124. The uncorrected G* is
off-centre by +0.062 (SD* 0.129). The corrected G* is off-centre by −0.076
(SD* 0.229). Percentile coverage at nominal 95% is 0.985 uncorrected and 0.993
corrected; for σ²_p it is 0.995 and 0.990.

At 50×20×10 (200 datasets): the correction leaves |bias| ≤ 0.0012. SD* for
pir is 4× the true SD even after correction. The main-effect SD* is 1.0–1.2×
the true SD. G* is off-centre by +0.015 uncorrected and −0.011 corrected, and
G coverage is 0.975 uncorrected and 0.980 corrected. At 10×4×2 the corrected
SD* is 2–4× too large. The grand-mean check agrees with B4 within MCSE at all
three sizes.

**Implication for #5.** B3 is a cheap, exact fix for the off-centre component
replicates that #5 documents. It does **not** yield calibrated intervals:

- Component spreads are inflated, most for interaction components, in line
  with the 2^k − 1 pattern for the mean.
- Corrected replicates are negative far more often.
- Corrected G* is still not centred on Ĝ.
- G percentile intervals came out conservative in every setting I ran.

A corrected method should therefore count as a centring fix pending a
coverage study. The ACT/CASMA corrections and model-based (parametric)
resampling are the alternatives the sources point to.
