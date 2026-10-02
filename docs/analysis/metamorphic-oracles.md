The most useful oracles here are **finite-sample algebraic identities**, followed by population relations tested with statistical tolerances. Three tempting assertions do **not** hold generally for the current implementations:

- Saltelli first-order estimates are not exactly shift invariant.
- QOSA estimates are not exactly shift invariant.
- Pick-freeze second-order estimates are not exactly equivariant under arbitrary factor permutations.

Below, **exact** means exact in real arithmetic, with floating-point tolerance in tests. Use finite, nonconstant outputs and avoid crossing numerical variance/bandwidth thresholds. Population Sobol relations assume **independent factors** and finite positive output variance.

For reference, the pick-freeze functions are:

| Name used below | Function and location |
|---|---|
| Saltelli | [`estimate_saltelli2010`, saltelli2010.rs:69](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/saltelli2010.rs:69) |
| Saltelli cached | [`estimate_saltelli2010_from_outputs`, saltelli2010.rs:179](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/saltelli2010.rs:179) |
| Saltelli cached S2 | [`estimate_saltelli2010_from_outputs_with_second_order`, saltelli2010.rs:245](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/saltelli2010.rs:245) |
| Jansen | [`estimate_jansen`, jansen.rs:96](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/jansen.rs:96) |
| Janon | [`estimate_janon`, janon.rs:126](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/janon.rs:126) |
| Owen | [`estimate_owen`, owen.rs:117](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/owen.rs:117) |

1. **Output scaling preserves Sobol indices.**

   For \(g(x)=a f(x)\), \(a\ne0\),
   \[
   \widehat S_i(g)=\widehat S_i(f),\quad
   \widehat{ST}_i(g)=\widehat{ST}_i(f),\quad
   \widehat S_{ij}(g)=\widehat S_{ij}(f),
   \]
   for whichever fields exist, while \(\widehat V(g)=a^2\widehat V(f)\).

   **Applies:** All pick-freeze functions above; [`estimate_fast`, fast.rs:132](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/fast.rs:132), [`estimate_rbd_fast`, rbd_fast.rs:155](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/rbd_fast.rs:155), [`estimate_given_data_sobol`, given_data_sobol.rs:137](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/given_data_sobol.rs:137), and [`estimate_hdmr`, hdmr.rs:135](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/hdmr.rs:135).

   **Status:** Exact algebraically; HDMR additionally has fitting error.

   **Test:** Reuse one design/data set with multipliers \(2,-1,-4\). Compare every index and the variance scaling. Keep both variances comfortably above implementation floors.

2. **Output shifts preserve some, but not all, finite-sample Sobol estimates.**

   For \(g=af+b\), \(a\ne0\), affine invariance is exact for **Jansen S1, Janon S1, Owen S1, Saltelli ST**, FAST, RBD-FAST, given-data Sobol, and HDMR, at the locations above.

   **Exception:** This does not extend to the raw second-order formulas in Saltelli, Janon, or Jansen.

   **Test:** Compare \(f\) against \(2f+3\) on identical samples. Check only the fields listed. Use moderate shifts: the pick-freeze implementations often compute variance as \(E[Y^2]-E[Y]^2\), which can suffer cancellation.

3. **Saltelli’s shift dependence has an exact correction oracle.**

   Let \(A=f(A)\), \(B=f(B)\), \(H_i=f(A_B^i)\), and \(D=\operatorname{Var}_N(A)\). For \(g=af+b\),
   \[
   \widehat S_i(g)-\widehat S_i(f)
   =\frac{b}{aD}\,\overline{H_i-A}.
   \]

   **Applies:** All three Saltelli functions in the table.

   **Status:** Exact finite-sample identity. Shift invariance holds only when \(\overline{H_i-A}=0\), or approximately as sampling error vanishes.

   **Test:** Supply cached arrays with deliberately unequal means. Transform every output array and assert the correction above. A simple one-factor witness is
   \[
   A=[0,1,2,3],\quad B=H=[1,2,4,5].
   \]
   The formula gives \(\widehat S=4.2\); adding \(10\) gives \(16.2\). These are calculations of the source formula, illustrating why neither shift invariance nor a finite-sample \([0,1]\) bound is valid here.

4. **Moment-independent indices have stronger output transformation invariance.**

   PAWN satisfies
   \[
   \operatorname{PAWN}(X,h(Y))=\operatorname{PAWN}(X,Y)
   \]
   for strictly increasing \(h\), because every empirical KS statistic depends on ordering and ties.

   Borgonovo’s implementation satisfies affine invariance
   \[
   \widehat\delta_i(X,aY+b)=\widehat\delta_i(X,Y),\quad a\ne0,
   \]
   provided its bandwidth floor is inactive: KDE densities scale by \(1/|a|\), and integration distances by \(|a|\).

   **Applies:** [`estimate_pawn`, pawn.rs:151](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/pawn.rs:151); [`estimate_borgonovo_delta`, borgonovo.rs:146](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/borgonovo.rs:146).

   **Status:** PAWN is exact, potentially bit-identical for increasing transforms preserving comparisons. Borgonovo affine invariance is exact algebraically, approximate numerically.

   **Test:** PAWN: replace \(Y\) with \(\exp(Y)\) on bounded values and compare all summary fields. Borgonovo: compare \(Y\), \(2Y+3\), and \(-2Y+3\). Do not require this KDE implementation to preserve arbitrary nonlinear output transforms exactly.

5. **QOSA preserves positive scaling, but its empirical shift behavior depends on tail counts.**

   Write \(T_g\) for `global_cte`, \(T_i\) for the conditional tail quantity, and \(m=\overline Y\). Let
   \[
   r_g=\frac{\#\{Y>q_\alpha\}}{N(1-\alpha)},\qquad
   r_i=\frac{\#\{Y>q_{\alpha,\mathrm{class}}\}}{N(1-\alpha)}.
   \]
   Under \(Y'=aY+b\), \(a>0\), the pre-clamp result becomes
   \[
   S_i'=
   \frac{a(T_g-T_i)+b(r_g-r_i)}
        {a(T_g-m)+b(r_g-1)}.
   \]

   **Applies:** [`estimate_qosa`, qosa.rs:200](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/qosa.rs:200), especially tail sums at lines 242 and 307.

   **Status:** Exact formula, followed by the implementation’s \([0,1]\) clamp. Positive scaling preserves the index; general shifts do not. Negative scaling changes the upper-tail problem.

   **Test:** Use \(N=64\), \(X_k=Y_k=k+1\), \(\alpha=0.5\). The source formulas produce three classes, global/conditional tail counts \(32/31\), and indices \(0.708984375\) versus \(0.728515625\) after adding \(10\). Also verify
   \[
   q_\alpha'=a q_\alpha+b,\qquad T_g'=aT_g+b r_g.
   \]

6. **Unnormalized effects transform predictably instead of remaining invariant.**

   For \(g=af+b\),
   \[
   \mu_i(g)=a\mu_i(f),\quad
   \mu_i^*(g)=|a|\mu_i^*(f),\quad
   \sigma_i(g)=|a|\sigma_i(f).
   \]
   Fractional-factorial signed effects scale by \(a\), and absolute effects by \(|a|\).

   **Applies:** [`estimate_morris_effects`, morris.rs:225](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/morris.rs:225), [`estimate_grouped_morris_effects`, morris.rs:333](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/morris.rs:333), [`estimate_fractional_factorial`, fractional_factorial.rs:68](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/fractional_factorial.rs:68).

   **Status:** Exact algebraically.

   **Test:** Reuse trajectories/designs for \(f,-2f+7\). Check signed and unsigned scaling separately, including grouped fields.

7. **Factor permutation is equivariance, provided the design is permuted too.**

   Define \(X'_j=X_{\pi(j)}\) and \(f'(x')=f(x)\). Then
   \[
   I_j(X',f')=I_{\pi(j)}(X,f).
   \]

   **Applies:** Pick-freeze S1/ST; given-data Sobol, RBD-FAST, PAWN, Borgonovo, QOSA; [`estimate_regression_indices`, regression.rs:164](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/regression.rs:164); HDMR; Morris; FAST; fractional factorial.

   **Status:** Exact algebraically, with fitting/reduction tolerance where needed.

   **Test:** Permute an existing design, rather than regenerate it using the same seed:

   - Pick-freeze: permute all columns and reorder hybrid vectors consistently.
   - Morris: also remap `factor_order` and permute `deltas`.
   - FAST: permute factor blocks and columns, plus both axes of `omegas`/`phases`.
   - HDMR/fractional factorial: also permute `Problem` factors and group references.

   Regression scalar \(R^2\) fields remain unchanged. Arbitrary renaming/reordering need not preserve `Problem::content_hash`, which serializes ordered metadata at [`problem.rs:106`](/Users/patrickbeam/projects/salib/crates/salib-core/src/problem.rs:106).

8. **Second-order permutation equivariance is only approximate in the current pick-freeze code.**

   Reversing a pair’s order changes its cross term from
   \[
   \overline{f(B_A^j)f(A_B^i)}
   \quad\text{to}\quad
   \overline{f(B_A^i)f(A_B^j)}.
   \]
   Their population expectations agree; their sample means need not.

   **Applies:** Saltelli S2 at [`saltelli2010.rs:143`](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/saltelli2010.rs:143), cached S2 at line 303; Janon at [`janon.rs:201`](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/janon.rs:201); Jansen at [`jansen.rs:156`](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/jansen.rs:156).

   **Status:** Population symmetry; approximate finite-sample equivariance.

   **Test:** Swap two factors and compare S2 statistically, or assert the exact difference of the two cross-product means divided by the estimator’s variance denominator. Do not use machine-epsilon equality.

9. **Joint row permutations preserve point estimates.**

   Apply the same row permutation to every paired output/matrix, or jointly to \((X,Y)\). The estimates remain unchanged.

   **Applies:** Pick-freeze functions; given-data Sobol, Borgonovo, PAWN, QOSA, RBD-FAST, regression, HDMR; DGSM with gradient rows permuted.

   **Status:** Exact algebraically; floating-point summation may change. For rank-based input methods, require distinct values within input columns: stable ordinal tie-breaking can change class membership after row permutation.

   **Test:** Reverse rows and use a random permutation. Keep pick-freeze pairing intact. Arbitrary row permutation is **not** valid for FAST’s ordered search curves or Morris’s within-trajectory steps.

10. **Fully additive models have no interaction variance.**

    For independent inputs and
    \[
    f(X)=b+\sum_i g_i(X_i),\qquad v_i=\operatorname{Var}(g_i(X_i)),
    \]
    \[
    S_i=ST_i=\frac{v_i}{\sum_jv_j},\quad
    \sum_i S_i=1,\quad S_{ij}=0.
    \]

    **Applies:** Pick-freeze functions; FAST/RBD-FAST/given-data Sobol; HDMR. Exact balanced-grid counterparts apply to [`estimate_anova_two_way`, anova.rs:266](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/anova.rs:266) and [`estimate_anova_three_way`, anova.rs:347](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/anova.rs:347).

    **Status:** Population identity for sampling estimators; exact grid identity for ANOVA. HDMR is exact up to fitting error when the additive model lies in its polynomial basis and the fit is identifiable.

    **Test:** Compare separate components \(g_i\) with their sum. For independent \(U(0,1)\) inputs, \(f=X_0+2X_1+3X_2\) has shares \((1,4,9)/14\). For ANOVA use \(Y_{ijk}=u_i+v_j+w_k\): all interaction fractions should vanish and main fractions sum to one.

11. **Changing one additive coefficient gives a monotonicity oracle.**

    For \(f_t=t\,g_i(X_i)+\sum_{j\ne i}g_j(X_j)\), let \(R=\sum_{j\ne i}v_j>0\). Then
    \[
    S_i(t)=ST_i(t)=\frac{t^2v_i}{t^2v_i+R}.
    \]
    Increasing \(|t|\) increases factor \(i\)’s index and decreases every other factor’s index. Changing \(t\) to \(-t\) preserves all population indices.

    **Applies:** Same variance-based estimators as relation 10.

    **Status:** Population identity; approximate for sampled estimates, exact for suitable ANOVA grids/PCE fits.

    **Test:** Reuse samples for \(t=0,1,2,4\) and compare against the formula. Require statistical separation rather than strict ordering of nearly equal noisy estimates.

12. **Adding a pure interaction changes S1/ST in a known direction.**

    For independent \(Z_0,Z_1\sim U(-1,1)\),
    \[
    f_t=aZ_0+bZ_1+tZ_0Z_1,\qquad
    D_t=\frac{a^2+b^2}{3}+\frac{t^2}{9}.
    \]
    Then
    \[
    S_0=\frac{a^2/3}{D_t},\quad S_1=\frac{b^2/3}{D_t},\quad
    S_{01}=\frac{t^2/9}{D_t},
    \]
    \[
    ST_0=S_0+S_{01},\qquad ST_1=S_1+S_{01}.
    \]
    With \(a,b\ne0\), increasing \(|t|\) decreases both S1 values, increases both ST values, and increases the interaction share.

    **Applies:** Pick-freeze estimators, FAST; HDMR with degree at least two; two-way ANOVA on a balanced centered grid.

    **Status:** Population identity; exact decomposition for representable PCE/grid models.

    **Test:** Compare \(t=0,1,2\). This tests additive decomposition, monotonicity, S2, and \(S_i\le ST_i\) together.

13. **Inactive factors yield exact zeros for some estimators.**

    If \(f\) ignores factor \(i\), then \(f(A_B^i)=f(A)\), giving exact Saltelli \(S_i=ST_i=0\). Owen’s first difference also vanishes, so its \(S_i=0\). Morris’s elementary effects for that factor are zero.

    Conversely, if \(f\) depends only on factor \(i\), then \(f(A_B^i)=f(B)\), giving exact **Jansen S1 = Janon S1 = 1**.

    **Applies:** Pick-freeze functions and Morris at the locations above.

    **Status:** Exact, assuming positive denominators.

    **Test:** Start with \(f(x)=x_0^2\), change unused columns arbitrarily, and check zeros and the active-factor ones. Do not demand exact dummy zeros from Janon/Jansen: their finite-sample formulas can retain sampling error.

14. **Grouping additive factors has an especially strong Saltelli oracle.**

    Using the same base \(A,B\), for an additive model and group \(G\),
    \[
    \widehat S_G=\sum_{i\in G}\widehat S_i.
    \]
    This is exact because the group hybrid output difference is the sum of individual hybrid differences.

    **Applies:** [`build_grouped_saltelli_matrix`, saltelli_matrix.rs:230](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/saltelli_matrix.rs:230), consumed by Saltelli.

    **Status:** Exact for Saltelli S1. \(ST_G=\sum_{i\in G}ST_i\) holds at population level for additive models, but not generally exactly for sample squared differences.

    **Test:** Reset RNG state before grouped/ungrouped builds; group factors \(\{0,2\}\). Compare grouped S1 with their sum. Singleton groups should reproduce the ordinary bundle exactly.

15. **Larger factor groups cannot have smaller population first/total effects.**

    For \(G\subseteq H\),
    \[
    S_G\le S_H,\qquad ST_G\le ST_H.
    \]
    Conditioning on more variables increases explained variance; larger groups capture more ANOVA components.

    **Applies:** Grouped Saltelli builder/estimator above.

    **Status:** Population relation; approximate for estimates. General group effects are not sums of individual effects when interactions exist.

    **Test:** On identical base samples compare groups \(\{0\}\) and \(\{0,1\}\), using the interaction model in relation 12.

16. **HDMR has exact internal conservation relations, even for an imperfect fit.**

    Its nonconstant coefficient contributions are nonnegative. Thus, for the fitted PCE,
    \[
    0\le S_i\le ST_i\le1,
    \]
    \[
    \sum_iS_i=\texttt{order\_variance}[0],
    \]
    and, when all supported interaction orders are tracked,
    \[
    \sum_k\texttt{order\_variance}[k]=1,\qquad
    \sum_iST_i=\sum_k(k+1)\texttt{order\_variance}[k].
    \]
    Also \(\sum_{i<j}S_{ij}=\texttt{order\_variance}[1]\).

    **Applies:** [`estimate_hdmr`, hdmr.rs:203](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/hdmr.rs:203).

    **Status:** Exact up to accumulation/clamping error.

    **Test:** Run the same fit with `max_order=1` and `max_order=d`. S1, ST, S2, PCE, and total variance must be unchanged; tracked order fractions extend. `max_order` changes reporting, not fitting or the normalization denominator.

17. **Bounds depend on the estimator; universal finite-sample \([0,1]\) assertions are invalid.**

    | Estimator | Finite-sample constraint, for valid finite outputs |
    |---|---|
    | Saltelli | \(ST_i\ge0\) if \(D>0\); S1/S2 can be negative or exceed 1 |
    | Jansen | \(S_i\le1\); no general lower bound |
    | Janon | \(-1\le S_i\le1\), from its joint moment formula |
    | FAST | \(0\le S_i\le ST_i\le1\), up to roundoff; harmonic and complementary bands are disjoint |
    | RBD-FAST | \(-\lambda/(1-\lambda)\le S_i\le1\), \(\lambda=2M/N\) |
    | Given-data Sobol, QOSA, HDMR | Reported indices clamped to \([0,1]\) |
    | PAWN | KS min/mean/median/max in \([0,1]\); CV is not such an index |

    **Applies:** Functions linked above; FAST band calculations at [`fast.rs:178`](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/fast.rs:178), RBD correction at [`rbd_fast.rs:220`](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/rbd_fast.rs:220).

    **Test:** Check these constraints on original and transformed inputs. At population level, additionally test \(0\le S_i\le ST_i\le1\), \(\sum S_i\le1\), and \(\sum ST_i\ge1\) statistically. Neither PAWN nor Borgonovo/QOSA generally sums to one.

18. **Janon and Jansen share an exact squared-difference relationship.**

    Put \(Y=f(B)\), \(Z_i=f(A_B^i)\),
    \[
    Q_i=\tfrac12\overline{(Y-Z_i)^2},\qquad
    D_i=\tfrac12\overline{Y^2+Z_i^2}
         -\left(\tfrac{\overline Y+\overline Z_i}{2}\right)^2.
    \]
    The code implies
    \[
    (1-\widehat S_i^{Janon})D_i
    =(1-\widehat S_i^{Jansen})\operatorname{Var}_N(Y)
    =Q_i.
    \]

    **Applies:** Janon at [`janon.rs:158`](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/janon.rs:158), Jansen at [`jansen.rs:120`](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/jansen.rs:120).

    **Status:** Exact outside denominator fallback branches.

    **Test:** Evaluate one shared matrix, calculate both denominators independently, and compare these products. The raw S1 estimates need not be equal at finite \(N\).

19. **DGSM transformations and monotonicity are exact when gradients are supplied.**

    With \(\nu_i=\overline{(\partial_i f)^2}\) and \(U_i=C_i\nu_i/V\),
    \[
    \nu_i(af+b)=a^2\nu_i(f),\qquad U_i(af+b)=U_i(f).
    \]
    Holding other inputs fixed, \(U_i\) scales linearly with \(C_i\), inversely with \(V\), and cannot decrease if every squared gradient increases.

    **Applies:** [`estimate_dgsm`, dgsm.rs:175](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/dgsm.rs:175); [`finite_difference_gradients`, dgsm.rs:247](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/dgsm.rs:247).

    **Status:** Exact for supplied gradients; approximate with finite differences.

    **Test:** Compare \((G,C,V)\) with \((2G,C,4V)\), then \((G,2C,V)\) and \((G,C,2V)\). For suitable independent differentiable models, also test the population bound \(ST_i\le U_i\) against Saltelli ST with sampling/FD tolerance. \(U_i\) can exceed one.

20. **Rank-preserving input transforms leave rank-based estimates unchanged.**

    Replace each input column by a strictly increasing function \(h_i(X_i)\), keeping \(Y\) fixed. Then rank partitions and sorting are unchanged.

    **Applies:** Given-data Sobol, Borgonovo, PAWN, QOSA, RBD-FAST; regression **SRRC, PRCC, and rank \(R^2\)**.

    **Status:** Exact; potentially bit-identical because downstream arrays are unchanged. This does not preserve regression’s raw SRC/PCC under nonlinear input transforms.

    **Test:** Compare \(X\) with \(\exp(X)\) or \(X^3\), preserving distinct finite values. Increasing nonlinear transforms of \(Y\) also preserve regression’s rank fields. Avoid blanket decreasing-transform assertions for partition methods with unequal class sizes.

21. **Affine changes of input units commute with sampling and model evaluation.**

    For \(X'_i=s_iX_i+t_i\), \(s_i>0\), transform the distribution accordingly and define
    \[
    f'(x')=f\!\left((x'-t)/s\right).
    \]
    Then each coupled sample has the same model output, hence unchanged sensitivity indices.

    **Applies:** [`Distribution::quantile`, distribution.rs:127](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:127), Saltelli/Owen builders and their estimators; HDMR’s canonical mapping.

    **Status:** Exact coupling, up to quantile/evaluation roundoff.

    **Test:** Use the same unit-cube samples. For Uniform, transform bounds; for Normal transform \((\mu,\sigma)\) to \((s\mu+t,s\sigma)\); for Triangular transform all three locations. Assert \(Q_{D'}(u)=sQ_D(u)+t\), then compare model outputs and indices. Changing a distribution without compensating the model generally changes sensitivity.

22. **Quantiles are monotone, with useful parameter-order oracles.**

    For every supported distribution,
    \[
    u\le v\implies Q(u)\le Q(v).
    \]
    Additional exact relations include
    \[
    Q_{\mathrm{Normal}(\mu+t,\sigma)}(u)
    =Q_{\mathrm{Normal}(\mu,\sigma)}(u)+t,
    \]
    \[
    Q_{\mathrm{Exponential}(k\lambda)}(u)
    =Q_{\mathrm{Exponential}(\lambda)}(u)/k,
    \]
    and analogous scale multiplication for Gamma/Weibull. Bernoulli quantiles are nondecreasing in \(p\).

    **Applies:** `Distribution::quantile` above; concrete helpers at [`distribution.rs:175`](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:175).

    **Status:** Exact mathematically; approximate for numerical inverse CDFs.

    **Test:** Sweep sorted interior probabilities and paired parameters. Use interior probabilities for infinite-support affine checks; test endpoints separately.

23. **Iman–Conover preserves empirical marginal distributions exactly.**

    For every column,
    \[
    \operatorname{sort}(IC(X,R)_i)=\operatorname{sort}(X_i).
    \]
    Also, for strictly increasing column transforms \(h\),
    \[
    IC(h(X),R;rng)=h(IC(X,R;rng)).
    \]

    **Applies:** [`iman_conover_transform`, iman_conover.rs:125](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/iman_conover.rs:125), reordering at line 201.

    **Status:** Exact: the implementation only permutes existing column values.

    **Test:** Clone RNG state, transform inputs, and compare commuting outputs and sorted column multisets. Compare two different target matrices: marginals must still match. Neither target rank correlation nor downstream sensitivity indices are preserved exactly.

24. **Sampler size/dimension transformations have specific deterministic relations.**

    For fixed configuration and initial RNG state:

    - LHS and Sobol: increasing dimension preserves the existing column prefix.
    - Sobol: increasing \(N\) preserves the row prefix; changing seed/stream/position preserves the entire sample because this implementation is unscrambled and consumes no RNG.
    - Morris: increasing trajectory count preserves earlier complete trajectories.
    - Centered LHS: across seeds, each sorted column is exactly \(\{(k+1/2)/N\}_{k=0}^{N-1}\).
    - Classic LHS: across seeds, each column retains one point per stratum, though locations change.

    **Applies:** [`LhsSampler::unit_sample`, lhs.rs:116](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/lhs.rs:116), [`SobolSampler::unit_sample`, sobol.rs:219](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/sobol.rs:219), [`build_morris_trajectories`, morris.rs:111](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/morris.rs:111).

    **Status:** Exact deterministic relations.

    **Test:** Compare paired calls with cloned initial states. Do not expect LHS row-prefix preservation when \(N\) changes, or sequential Sobol calls to continue the sequence.

25. **Enabling second-order sampling preserves existing evaluations.**

    With identical sampler, \(N\), and initial RNG,
    \[
    (A,B,A_B^i)_{\text{S2 off}}=(A,B,A_B^i)_{\text{S2 on}}.
    \]
    Final RNG states also agree; only \(B_A^i\) matrices are added. Consequently S1/ST must remain unchanged.

    **Applies:** [`build_saltelli_matrix`, saltelli_matrix.rs:138](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/saltelli_matrix.rs:138), grouped builder at line 230; Saltelli, Janon, Jansen.

    **Status:** Exact, including bit identity for retained arrays and estimates.

    **Test:** Build both variants from cloned RNG states and compare retained matrices, final RNG, and S1/ST fields.

26. **Bootstrap intervals obey transformation and confidence-level relations.**

    With identical resampling RNG, an exact invariant of every resample carries through to percentile CI endpoints. For example, output scaling preserves Saltelli S1/ST CIs.

    Also, for \(0<\alpha_1<\alpha_2<1\), using the same bootstrap estimates,
    \[
    L_{\alpha_1}\le L_{\alpha_2},\qquad
    U_{\alpha_1}\ge U_{\alpha_2}.
    \]

    **Applies:** [`estimate_saltelli2010_with_bootstrap`, bootstrap.rs:73](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/bootstrap.rs:73), cached wrapper at line 171; [`bootstrap_given_data`, bootstrap_given_data.rs:207](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/bootstrap_given_data.rs:207).

    **Status:** Exact with matching successful resamples and finite estimates.

    **Test:** Reset RNG for transformed data and for \(\alpha=0.05,0.10,0.20\). Compare endpoints and skipped counts. More samples/resamples do **not** guarantee monotonically narrower realized intervals.

27. **ANOVA and G-theory have affine and permutation oracles; G-theory adds conditional monotonicity.**

    For an affine-transformed grid \(Y'=aY+b\), ANOVA fractions stay unchanged and mean squares scale by \(a^2\). G-theory variance components scale by \(a^2\), while \(G,\Phi\) stay unchanged. Level permutations within axes preserve both analyses.

    For **nonnegative** G-theory components and \(\sigma_p>0\),
    \[
    n_i'\ge n_i,\ n_r'\ge n_r
    \implies G'\ge G,\quad\Phi'\ge\Phi,\quad 0\le\Phi\le G\le1.
    \]

    **Applies:** ANOVA functions above; [`estimate_g_theory_pir`, g_theory.rs:162](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/g_theory.rs:162), [`project_g_theory_d_study`, g_theory.rs:456](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/g_theory.rs:456).

    **Status:** Exact algebraically outside fallback thresholds. Estimated G-theory components can be negative, so the monotonicity/bounds require the stated precondition.

    **Test:** Transform and shuffle a balanced grid. Separately construct positive components and increase item/rater counts. Swapping item/rater axes should swap corresponding components and preserve reliability when projected counts are swapped too.

28. **Discrepancy measures have permutation, replication, and reflection invariance.**

    All four discrepancy outputs satisfy
    \[
    D(P_{\rm rows}XP_{\rm cols})=D(X),\qquad
    D(\operatorname{repeat\_rows}(X,k))=D(X).
    \]
    Centered, wrap-around, and modified discrepancy additionally preserve reflection of any coordinate \(x_j\mapsto1-x_j\). Wrap-around discrepancy preserves coordinatewise torus translations \(x_j\mapsto(x_j+t_j)\bmod1\).

    **Applies:** [`compute_discrepancy`, discrepancy.rs:75](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/discrepancy.rs:75).

    **Status:** Exact algebraically; floating-point tolerance required.

    **Test:** Permute, duplicate, reflect, and translate a small point set. Do not apply reflection/translation invariance to anchored L2-star discrepancy.

For algebraic tests, reuse the same design and use roughly \(10^{-12}\)–\(10^{-10}\) tolerances on well-scaled data. For population relations, use paired runs across independent LHS seeds and assess the distribution of differences; changing seeds does not create independent runs of this unscrambled Sobol sampler. I made no source changes or ran Rust tests; the small numerical witnesses above were evaluated directly from the source formulas.