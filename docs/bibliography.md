# Bibliography

References for salib's methods. Paper links lead to publishers or author copies;
the Box textbook link is a catalogue record. Method guides distinguish the
published results from salib's implementation choices. The
[literature audit](../papers/2026-10-07-literature-audit.md) records corrections,
reproduced failures, and sources we could not acquire.

---

## Variance-based (Sobol')

<a id="sobol1993"></a>
**Sobol' (1993)** · [source](https://www.andreasaltelli.eu/file/repository/sobol1993.pdf)
Sobol', I. M. "Sensitivity estimates for nonlinear mathematical models." *Mathematical Modelling and Computational Experiments* 1(4), 407–414.
Defines variance-based sensitivity through a functional decomposition for independent inputs.

<a id="saltelli2010"></a>
**Saltelli et al. (2010)** → [method page](methods/variance-based.md#saltelli-2010) · [source](https://doi.org/10.1016/j.cpc.2009.09.018)
Saltelli, A., Annoni, P., Azzini, I., Campolongo, F., Ratto, M., & Tarantola, S. "Variance based sensitivity analysis of model output. Design and estimator for the total sensitivity index." *Computer Physics Communications* 181(2), 259–270.
Table 2(b) gives the implemented first-order product-difference estimator; Table 2(f) gives the squared-difference total-effect estimator, attributed to Jansen.

<a id="jansen1999"></a>
**Jansen (1999)** → [method page](methods/variance-based.md#jansen-1999) · [source](https://doi.org/10.1016/S0010-4655%2898%2900154-4)
Jansen, M. J. W. "Analysis of variance designs for model output." *Computer Physics Communications* 117(1–2), 35–43.
Squared-difference estimators. salib uses the first-order complement comparing $f(B)$ with $f(A_B^{(i)})$; see also Saltelli et al. (2010), Table 2(c). Its Jansen result has no total-effect vector.

<a id="janon2014"></a>
**Janon et al. (2014)** → [method page](methods/variance-based.md#janon-2014) · [source](https://doi.org/10.1051/ps/2013040)
Janon, A., Klein, T., Lagnoux, A., Nodet, M., & Prieur, C. "Asymptotic normality and efficiency of two Sobol index estimators." *ESAIM: Probability and Statistics* 18, 342–364.
Propositions 3.2–3.5 give asymptotic results for i.i.d. exchangeable pick-freeze pairs, including efficiency among regular estimators in that model. They do not imply finite-sample superiority or a result for deterministic QMC.

<a id="owen2013"></a>
**Owen (2013)** → [method page](methods/variance-based.md#owen-2013) · [source](https://doi.org/10.1145/2457459.2457460)
Owen, A. B. "Better estimation of small Sobol' sensitivity indices." *ACM Transactions on Modeling and Computer Simulation* 23(2), Article 11.
Studies estimation of small closed Sobol' indices. salib applies the Correlation 2 formula to singleton factors and returns first-order indices only.

<a id="plischke2013"></a>
**Plischke et al. (2013)** → [method page](methods/variance-based.md#given-data-sobol) · [source](https://doi.org/10.1016/j.ejor.2012.11.047)
Plischke, E., Borgonovo, E., & Smith, C. L. "Global sensitivity measures from given data." *European Journal of Operational Research* 226(3), 536–550.
Estimate first-order Sobol' indices from observational data by binning inputs and computing between-bin variance of conditional means. No designed experiment required.

<a id="homma1996"></a>
**Homma & Saltelli (1996)** · [source](https://doi.org/10.1016/0951-8320%2896%2900002-6)
Homma, T., & Saltelli, A. "Importance measures in global sensitivity analysis of nonlinear models." *Reliability Engineering & System Safety* 52(1), 1–17.
Introduces the total-effect index $S_{Ti}$ as a complement to first-order indices. Shows that $S_{Ti} = 0$ is necessary and sufficient for $X_i$ to be non-influential.

---

## Elementary effects

<a id="morris1991"></a>
**Morris (1991)** → [method page](methods/elementary-effects.md#morris-1991) · [source](https://www.stat.cmu.edu/technometrics/90-00/vol-33-02/v3302161.pdf)
Morris, M. D. "Factorial sampling plans for preliminary computational experiments." *Technometrics* 33(2), 161–174.
The original OAT screening design. Defines elementary effects on a $p$-level grid and proposes $r$ random trajectories to estimate $\mu_i$ and $\sigma_i$.

<a id="campolongo2007"></a>
**Campolongo et al. (2007)** → [method page](methods/elementary-effects.md#grouped-morris) · [source](https://doi.org/10.1016/j.envsoft.2006.10.004)
Campolongo, F., Cariboni, J., & Saltelli, A. "An effective screening design for sensitivity analysis of large models." *Environmental Modelling & Software* 22(10), 1509–1518.
Introduces $\mu_i^*$ (mean of absolute elementary effects) to fix the cancellation problem in Morris's signed mean. Also extends Morris to grouped factors.

---

## Frequency-based

<a id="cukier1973"></a>
**Cukier et al. (1973)** → [method page](methods/frequency.md#fast--efast) · [source](https://doi.org/10.1063/1.1680571)
Cukier, R. I., Fortuin, C. M., Shuler, K. E., Petschek, A. G., & Schaibly, J. H. "Study of the sensitivity of coupled reaction systems to uncertainties in rate coefficients. I. Theory." *Journal of Chemical Physics* 59(8), 3873–3878.
The original FAST method. Maps each input to a characteristic frequency and uses Fourier decomposition to extract first-order sensitivity from the model output's power spectrum.

<a id="saltelli1999"></a>
**Saltelli et al. (1999)** → [method page](methods/frequency.md#fast--efast) · [source](https://doi.org/10.1080/00401706.1999.10485594)
Saltelli, A., Tarantola, S., & Chan, K. P.-S. "A quantitative model-independent method for global sensitivity analysis of model output." *Technometrics* 41(1), 39–56.
Extended FAST (eFAST): estimates complementary variance from the assigned low-frequency band and obtains total effects by subtraction from total variance.

<a id="tarantola2006"></a>
**Tarantola et al. (2006)** → [method page](methods/frequency.md#rbd-fast) · [source](https://doi.org/10.1016/j.ress.2005.06.003)
Tarantola, S., Gatelli, D., & Mara, T. A. "Random balance designs for the estimation of first order global sensitivity indices." *Reliability Engineering & System Safety* 91(6), 717–727.
RBD-FAST: uses a single random sample for all factors by permuting columns independently, reducing cost from $d \times N$ to $N$.

<a id="plischke2010"></a>
**Plischke (2010)** · [source](https://www-10b015.pages.gwdg.de/papers/easi_ress.pdf)
Plischke, E. "An effective algorithm for computing global sensitivity indices (EASI)." *Reliability Engineering & System Safety* 95(4), 354–360.
Bias correction for RBD-FAST via the $\lambda = 2M/N$ bandwidth factor.

---

## Distribution-based

<a id="borgonovo2007"></a>
**Borgonovo (2007)** → [method page](methods/distribution.md#borgonovo-delta) · [source](https://doi.org/10.1016/j.ress.2006.04.015)
Borgonovo, E. "A new uncertainty importance measure." *Reliability Engineering & System Safety* 92(6), 771–784.
The $\delta$ moment-independent importance measure: the expected $L^1$ distance between conditional and unconditional output densities. Captures distributional shifts that variance-based indices miss.

<a id="pianosi2015"></a>
**Pianosi & Wagener (2015)** → [method page](methods/distribution.md#pawn) · [source](https://doi.org/10.1016/j.envsoft.2015.01.004)
Pianosi, F., & Wagener, T. "A simple and efficient method for global sensitivity analysis based on cumulative distribution functions." *Environmental Modelling & Software* 67, 1–11.
Defines PAWN using conditional and unconditional CDFs. Section 3.4 recommends the median KS distance, with the maximum as complementary information.

<a id="pianosi2018"></a>
**Pianosi & Wagener (2018)** → [method page](methods/distribution.md#pawn) · [source](https://research-information.bris.ac.uk/ws/portalfiles/portal/175128594/Pianosi_PAWN_givendata_final.pdf) · [DOI](https://doi.org/10.1016/j.envsoft.2018.07.019)
Pianosi, F., & Wagener, T. "Distribution-based sensitivity analysis from a generic input-output sample." *Environmental Modelling & Software* 108, 197–207.
Estimates PAWN from generic input-output observations using conditioning intervals. Section 4 recommends starting with ten intervals and checking sensitivity to interval count and sample size.

<a id="maumedeschamps2018"></a>
**Maume-Deschamps & Niang (2018)** · [source](https://arxiv.org/abs/1702.00925)
Maume-Deschamps, V., & Niang, I. "Estimation of quantile oriented sensitivity indices." *Statistics & Probability Letters* 134, 122–127.
Derives a tail-expectation identity and studies a kernel conditional-quantile estimator with independent fitting and evaluation samples. Its consistency result does not cover salib's capped, same-sample binning approximation.

<a id="fort2016"></a>
**Fort et al. (2016)** → [method page](methods/distribution.md#qosa) · [source](https://arxiv.org/abs/1305.2329)
Fort, J.-C., Klein, T., & Rachdi, N. "New sensitivity analysis subordinated to a contrast." *Communications in Statistics — Theory and Methods* 45(15), 4349–4364.
QOSA: quantile-oriented sensitivity analysis. Measures factor importance at a specific quantile level, capturing tail sensitivity that variance-based methods average away.

---

## Derivative-based

<a id="sobol-kucherenko2009"></a>
<a id="sobolkucherenko2009"></a>
**Sobol' & Kucherenko (2009)** → [method page](methods/derivative.md#dgsm) · [source](https://doi.org/10.1016/j.matcom.2009.01.023)
Sobol', I. M., & Kucherenko, S. "Derivative based global sensitivity measures and their link with global sensitivity indices." *Mathematics and Computers in Simulation* 79(10), 3009–3017.
DGSM: defines $\nu_i = \mathbb{E}[(\partial f / \partial x_i)^2]$ and proves upper bounds linking $\nu_i$ to total-effect indices $S_{Ti}$. Cheap when gradients are available.

---

## Regression

<a id="saltelli-marivoet1990"></a>
<a id="saltellimarivoet1990"></a>
**Saltelli & Marivoet (1990)** → [method page](methods/regression.md) · [source](https://doi.org/10.1016/0951-8320%2890%2990065-U)
Saltelli, A., & Marivoet, J. "Non-parametric statistics in sensitivity analysis for model output: A comparison of selected techniques." *Reliability Engineering & System Safety* 28(2), 229–253.
Compares regression and rank-based sensitivity measures. For independent linear inputs, SRC² estimates first-order variance shares. Rank indices describe transformed-data associations.

---

## Surrogate

<a id="xiu-karniadakis2002"></a>
<a id="xiukarniadakis2002"></a>
**Xiu & Karniadakis (2002)** → [method page](methods/surrogate.md#polynomial-chaos-expansion) · [source](https://www.sci.utah.edu/~dxiu/Papers/XiuK_SISC02.pdf)
Xiu, D., & Karniadakis, G. E. "The Wiener-Askey polynomial chaos for stochastic differential equations." *SIAM Journal on Scientific Computing* 24(2), 619–644.
Generalized polynomial chaos: match orthogonal polynomial families to input distributions (Legendre for uniform, Hermite for Gaussian). Sobol' indices are analytic from the expansion coefficients.

<a id="blatman-sudret2011"></a>
<a id="blatmansudret2011"></a>
**Blatman & Sudret (2011)** → [method page](methods/surrogate.md#polynomial-chaos-expansion) · [source](https://doi.org/10.1016/j.jcp.2010.12.021)
Blatman, G., & Sudret, B. "Adaptive sparse polynomial chaos expansion based on least angle regression." *Journal of Computational Physics* 230(6), 2345–2367.
Sparse PCE via LARS: automatic basis selection when the full polynomial basis has more terms than data points. Makes PCE practical in moderate-to-high dimensions.

<a id="li2001"></a>
<a id="li2002"></a>
**Li et al. (2001)** → [method page](methods/surrogate.md#hdmr) · [source](https://doi.org/10.1021/jp010450t)
Li, G., Rosenthal, C., & Rabitz, H. "High dimensional model representations." *Journal of Physical Chemistry A* 105(33), 7765–7777.
HDMR: decompose $f$ into a hierarchy of component functions (constant, univariate, bivariate, ...). Cut-HDMR evaluates components along cuts through a reference point.

<a id="constantine2015"></a>
**Constantine (2015)** → [method page](methods/surrogate.md#active-subspaces) · [source](https://doi.org/10.1137/1.9781611973860)
Constantine, P. G. *Active Subspaces: Emerging Ideas for Dimension Reduction in Parameter Studies.* SIAM, Philadelphia.
Develops active-subspace methods from the eigendecomposition of the gradient second-moment matrix. The 2014 paper below supplies the error bound used in this documentation.

---

## Game-theoretic

<a id="song2016"></a>
<a id="songnelsonstaum2016"></a>
**Song, Nelson & Staum (2016)** → [method page](methods/game-theoretic.md) · [source](https://users.iems.northwestern.edu/~nelsonb/Publications/SongNelsonStaum.pdf)
Song, E., Nelson, B. L., & Staum, J. "Shapley effects for global sensitivity analysis: Theory and computation." *SIAM/ASA Journal on Uncertainty Quantification* 4(1), 1060–1083.
Defines Shapley effects for dependent as well as independent inputs. Equation (10) gives their population variance sum; Algorithm 1 estimates them by nested sampling. salib supports independent inputs only.

---

## Experimental design

<a id="fisher1925"></a>
**Fisher (1925)** → [method page](methods/experimental-design.md#anova) · [source](https://www.usablebuildings.co.uk/UsableBuildings/Unprotected/ClassicsFisher1925.pdf)
Fisher, R. A. *Statistical Methods for Research Workers.* Oliver & Boyd, Edinburgh.
The origin of analysis of variance. Two-way and higher-order ANOVA partition total variation into main effects, interactions, and error.

<a id="brennan2001"></a>
**Brennan (2001)** → [method page](methods/experimental-design.md#g-theory) · [source](https://link.springer.com/book/10.1007/978-1-4757-3456-0)
Brennan, R. L. *Generalizability Theory.* Springer, New York.
Develops variance components for measurement facets and projects reliability under alternative measurement designs.

<a id="hickernell1998"></a>
**Hickernell (1998)** → [method page](methods/experimental-design.md#discrepancy) · [source](https://doi.org/10.1090/S0025-5718-98-00894-1)
Hickernell, F. J. "A generalized discrepancy and quadrature error bound." *Mathematics of Computation* 67(221), 299–322.
Derives generalized discrepancy bounds with corresponding variation measures. The classical star-discrepancy bound does not permit substituting any discrepancy returned by salib.

<a id="box1978"></a>
<a id="boxhunterhunter1978"></a>
**Box, Hunter & Hunter (1978)** → [method page](methods/experimental-design.md#fractional-factorial) · [source](https://openlibrary.org/works/OL2181531W/Statistics_for_experimenters)
Box, G. E. P., Hunter, W. G., & Hunter, J. S. *Statistics for Experimenters.* Wiley, New York.
Covers factorial designs, confounding, resolution, and analysis of main effects and interactions.

---

## General references

<a id="saltelli2008"></a>
**Saltelli et al. (2008)** · [source](https://doi.org/10.1002/9780470725184)
Saltelli, A., Ratto, M., Andres, T., Campolongo, F., Cariboni, J., Gatelli, D., Saisana, M., & Tarantola, S. *Global Sensitivity Analysis: The Primer.* Wiley, Chichester.
Introduces variance-based, screening, and moment-independent methods with worked examples.

<a id="iooss2015"></a>
**Iooss & Lemaître (2015)** · [source](https://arxiv.org/abs/1404.2405)
Iooss, B., & Lemaître, P. "A review on global sensitivity analysis methods." In *Uncertainty Management in Simulation-Optimization of Complex Systems*, Operations Research/Computer Science Interfaces Series 59, 101–122. Springer.
Reviews variance-based, screening, moment-independent, surrogate, and derivative methods, including method selection.

## Additional sources used in the audit

<a id="sudret2008"></a>
**Sudret (2008)** · [source](https://doi.org/10.1016/j.ress.2007.04.002)
Sudret, B. "Global sensitivity analysis using polynomial chaos expansions." *Reliability Engineering & System Safety* 93(7), 964–979.
Section 5.4, Eqs. (51) and (53), gives component and total-effect indices for a polynomial expansion orthogonal under the input measure.

<a id="efron2004"></a>
**Efron et al. (2004)** · [source](https://arxiv.org/abs/math/0406456)
Efron, B., Hastie, T., Johnstone, I., & Tibshirani, R. "Least angle regression." *Annals of Statistics* 32(2), 407–451.
Equation (1.1) centers predictors and response and normalizes predictor columns. salib currently omits predictor centering.

<a id="constantine2014"></a>
**Constantine, Dow & Wang (2014)** · [source](https://arxiv.org/abs/1304.2070)
Constantine, P. G., Dow, E., & Wang, Q. "Active subspace methods in theory and practice: applications to kriging surfaces." *SIAM Journal on Scientific Computing* 36(4), A1500–A1524.
Theorem 3.1 bounds conditional-mean approximation error using the sum of discarded eigenvalues, under its assumptions. A gap ratio alone does not establish prediction accuracy.

<a id="marino2008"></a>
**Marino et al. (2008)** · [source](https://pmc.ncbi.nlm.nih.gov/articles/PMC2570191/)
Marino, S., Hogue, I. B., Ray, C. J., & Kirschner, D. E. "A methodology for performing global uncertainty and sensitivity analysis in systems biology." *Journal of Theoretical Biology* 254(1), 178–196.
Section 2.1 discusses linear and monotonic assumptions; footnote 3 describes average ranks for ties.

<a id="roustant2017"></a>
**Roustant, Barthe & Iooss (2017)** · [source](https://arxiv.org/abs/1612.03689)
Roustant, O., Barthe, F., & Iooss, B. "Poincaré inequalities on intervals — application to sensitivity analysis." *Electronic Journal of Statistics* 11(2), 3081–3119.
Section 1 states the independent-input setting and links derivative measures to total effects. Sampled plug-in values have additional estimation error.
