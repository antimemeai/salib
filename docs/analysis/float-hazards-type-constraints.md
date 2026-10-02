I read all **43 Rust source files, totaling 19,660 lines**, in the four requested directories, including their tests. No files were changed.

The most serious findings are:

- **PAWN can loop indefinitely when outputs contain NaN.**
- **Jacobi normalization returns NaN for some valid parameters.**
- **Several Sobol estimators can lose the entire variance through cancellation.**
- **Several validation checks accept NaN or infinity; others silently convert numerical failures into zero indices.**
- **LHS can produce exactly `1.0`, violating its `[0,1)` contract.**

Category tags below correspond to your list:

| Tag | Category |
|---|---|
| **FC** | 1. Float comparison |
| **CC** | 2. Catastrophic cancellation |
| **NaN** | 3. NaN propagation or masking |
| **EPS** | 4. Epsilon handling |
| **ORDER** | 5. Accumulation order |
| **OU** | 6. Overflow/underflow |
| **INF** | 7. Infinity handling |

One correction to the question’s example: **`(a-b).abs() < tol` is false when either operand is NaN.** The dangerous rejection pattern is `if (a-b).abs() > tol { reject }`, because NaN makes that condition false too.

**Core crate**

Locations in this table are relative to `crates/salib-core/src/`.

| Location | Categories | Finding |
|---|---|---|
| [problem.rs:292](/Users/patrickbeam/projects/salib/crates/salib-core/src/problem.rs:292), `:297` | NaN, INF | Normal validation checks `sigma` but never checks `mu`; LogNormal checks `sigma_log` but never checks `mu_log`. A builder-created problem can therefore contain NaN or infinite location parameters. NaN locations subsequently cause the `statrs` constructor’s `expect()` to panic. |
| [problem.rs:287](/Users/patrickbeam/projects/salib/crates/salib-core/src/problem.rs:287), `:302`, `:318`, `:328`, `:335`, `:345` | INF, OU | Ordered bounds and positive parameters are checked without requiring finiteness. Infinite Uniform/Triangular/Beta bounds and infinite positive scales, shapes, rates, or standard deviations can pass validation. Even finite bounds can have an overflowing `hi-lo`. |
| [problem.rs:55](/Users/patrickbeam/projects/salib/crates/salib-core/src/problem.rs:55), `:76`, `:79`; [distribution.rs:57](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:57) | NaN, INF | Public distribution parameters, mutable public problem fields, and derived deserialization bypass builder validation. `#[non_exhaustive]` does not prevent mutation of existing public fields or invalid deserialization. |
| [distribution.rs:128](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:128) | NaN, INF | `u.clamp(0.0, 1.0)` preserves NaN and silently maps infinite probabilities to endpoints. Subsequent behavior depends on the distribution: propagation, panic, or a plausible finite value. |
| [distribution.rs:176](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:176) | CC, OU, NaN, INF | Uniform uses `lo + u*(hi-lo)`. With finite bounds `[-1e308, 1e308]`, the width overflows; `u=0` then produces `0*Inf`, hence NaN. Narrow intervals at large offsets lose sample resolution. Endpoint reconstruction can differ from the supplied bound. |
| [distribution.rs:185](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:185), `:187`, `:189` | CC, OU, NaN, INF | Triangular forms differences, divides by the width, and multiplies two widths before taking a square root. Intermediate products can overflow or underflow even when the final quantile is representable. Infinite parameters can cause `Inf/Inf` or `0*Inf`; endpoint subtraction can lose precision. |
| [distribution.rs:199](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:199), `:207` | CC, OU | Weibull and Exponential use `-ln(1-u)`. Small positive `u` can round `1-u` to `1`, yielding a zero quantile. Use `-ln_1p(-u)`. Weibull additionally risks overflow in `1/shape`, `powf`, and multiplication by `scale`; Exponential can overflow when dividing by a tiny rate. |
| [distribution.rs:217](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:217) | CC, NaN | Bernoulli uses the threshold `1-p`. A sufficiently small positive `p` rounds that threshold to `1`, eliminating the rare event. A NaN probability argument `u` takes the `else` branch and returns `1.0`, hiding invalid input. |
| [distribution.rs:226](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:226), `:228`, `:232`, `:234`; `:168` | OU, NaN | DiscreteUniform computes `hi-lo+1` in `i64`, which can overflow. Conversion of counts and values to `f64` loses integer distinctions beyond `2^53`. Casting NaN from `scaled.floor()` to `i64` produces zero, silently selecting the lower endpoint. |
| [distribution.rs:251](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:251), `:258` | NaN, INF, OU | Normal and LogNormal constructor calls assume builder validation is sufficient. It is not: NaN locations panic; infinite parameters remain problematic. Extreme finite normal quantiles can overflow; LogNormal adds exponential overflow and underflow. |
| [distribution.rs:265](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:265), `:268`, `:270` | INF, OU, NaN | Beta shape checks accept infinity, whereas the dependency constructor rejects infinite shapes, causing `expect()` to panic. Affine mapping of the inverse CDF also inherits Uniform’s overflowing-width problem. |
| [distribution.rs:275](/Users/patrickbeam/projects/salib/crates/salib-core/src/distribution.rs:275), `:278` | OU, INF | Gamma converts scale to rate using `1/scale`. An infinite scale becomes zero and causes a constructor panic. A tiny positive scale can overflow its reciprocal; extremely large finite scales produce subnormal rates with reduced precision. |
| [reduce.rs:78](/Users/patrickbeam/projects/salib/crates/salib-core/src/reduce.rs:78), `:117` | CC, OU, NaN, INF | Pairwise addition and multiplication improve accumulation accuracy but do not prevent overflowing intermediate sums/products or cancellation between signs. For example, `[1e308,1e308,-1e308,-1e308]` produces NaN despite an exact sum of zero. Neither reducer validates finiteness. |
| [reduce.rs:161](/Users/patrickbeam/projects/salib/crates/salib-core/src/reduce.rs:161), `:165`, `:166`, `:186`, `:191`, `:199` | CC, OU, NaN, INF | Variance uses centered differences, which is better than raw moments, but computes the mean by summing before dividing. That sum can overflow even for constant finite data. Centered squares can overflow or underflow. Large offsets can already have destroyed differences in the input representation. |
| [reduce.rs:156](/Users/patrickbeam/projects/salib/crates/salib-core/src/reduce.rs:156), `:181` | NaN, INF | Variance returns zero for fewer than two observations, including a singleton NaN or infinity. This masks invalid input under the documented small-sample convention. |
| [reduce.rs:427](/Users/patrickbeam/projects/salib/crates/salib-core/src/reduce.rs:427), `:480`, `:518`, `:532`, `:571`, `:677` | EPS, INF | Relative-error test assertions can accept a finite value versus infinity: both the absolute error and tolerance become infinity, so `Inf <= Inf` is true. Check finiteness before computing the tolerance. |

`rng.rs` contains integer RNG/state operations, with no floating-point arithmetic finding. `lib.rs` contains exports and configuration.

**Samplers crate**

Locations are relative to `crates/salib-samplers/src/`.

| Location | Categories | Finding |
|---|---|---|
| [iman_conover.rs:149](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/iman_conover.rs:149), `:155` | NaN, INF, EPS | Diagonal and symmetry rejection use `abs() > 1e-9`. NaN passes both checks. Comparing two infinities also creates NaN through subtraction, bypassing symmetry rejection. Require finite entries first. |
| [iman_conover.rs:236](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/iman_conover.rs:236), `:239`, `:242`, `:244` | CC, NaN, EPS, OU | Cholesky repeatedly subtracts products, losing accuracy near singularity. `s <= 1e-15` does not reject NaN, which then passes through `sqrt` and division. The fixed pivot cutoff rejects sufficiently ill-conditioned positive-definite matrices without a relative conditioning criterion. |
| [iman_conover.rs:181](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/iman_conover.rs:181), `:193`, `:273` | INF, NaN, OU | The uniform RNG conversion can return exactly zero; the normal quantile then returns `-Inf`. Multiplication by zero entries in the Cholesky matrix produces NaN. The transform also uses an uncompensated serial dot product. |
| [iman_conover.rs:213](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/iman_conover.rs:213), `:260` | NaN, INF | Sorting uses `partial_cmp(...).unwrap_or(Equal)`. NaN is treated as equal to every value, violating the intended ordering and corrupting ranks. Input samples are not checked for finiteness. |
| [lhs.rs:156](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/lhs.rs:156), `:157` | OU, INF | `(permutation_index + jitter)/n` can round to the next stratum or exactly `1.0`. Concrete arithmetic example: `n=2_097_153`, final stratum, jitter `(2^32-1)/2^32` produces `1.0`. Mapping through an unbounded quantile then produces infinity. Zero jitter also permits an exact lower endpoint. |
| [lhs.rs:126](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/lhs.rs:126), `:164` | OU | Counts converted to `f64` eventually lose integer precision. Centered LHS’s `index+0.5` loses its half-stratum offset at sufficiently large indices. These are mostly impractical allocation sizes, but the API does not encode limits. |
| [morris.rs:133](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/morris.rs:133), `:136`, `:166`, `:195`, `:259`, `:281`, `:304` | CC, OU | Grid points and theoretical deltas are computed separately. The representable difference between successive coordinates need not equal the stored delta; this introduces derivative normalization error. Endpoint samples are also unsuitable for finite-valued inverse-CDF mapping of unbounded distributions. |
| [morris.rs:235](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/morris.rs:235), `:304` | NaN, INF | Group lists are not validated as a partition. Overlapping or repeated members can receive repeated additions, move outside the unit cube, or leave inconsistent deltas. Later elementary-effect divisions can therefore use invalid or zero steps. |
| [fast.rs:200](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/fast.rs:200), `:201` | CC, OU | Large frequency-phase products lose phase precision before `sin`; trigonometric argument reduction becomes less accurate as frequency grows. Near extrema, `asin(sin(...))` is sensitive to rounding. Endpoint values can occur. |
| [fast.rs:148](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/fast.rs:148), `:157`, `:233` | OU | Related integer hazards affect numerical validity: `4*M*M+1` can overflow, large frequencies are truncated to `u32`, and `2*harmonic` can overflow. The resulting design can violate its spectral preconditions. |
| [sobol.rs:255](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/sobol.rs:255) | INF | When the first point is retained, coordinates are zero. Mapping these through Normal’s inverse CDF produces `-Inf`. This is an endpoint-policy issue rather than incorrect Sobol arithmetic. |
| [sobol.rs:250](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/sobol.rs:250), `:263` | OU | The generator has 32-bit resolution but does not enforce a matching sample-count bound; the direction-number index can eventually exceed the available bits. |
| [sampler.rs:35](/Users/patrickbeam/projects/salib/crates/salib-samplers/src/sampler.rs:35) | NaN, INF | The trait returns an unrestricted `Array2<f64>`. It cannot guarantee the advertised shape, finite entries, or unit interval, so custom implementations can inject invalid values into every design constructor. |

Saltelli and Owen matrix construction mostly copies values; its main numerical weakness is trusting raw sampler output and mutable metadata. Plackett–Burman’s generated `-1.0`/`1.0` values are exactly representable.

**Estimators: designed-sample and derivative methods**

Locations are relative to `crates/salib-estimators/src/`.

| Location | Categories | Finding |
|---|---|---|
| [saltelli2010.rs:97](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/saltelli2010.rs:97), `:196`, `:267` | CC, OU | Variance is `E[Y²]-E[Y]²`. Large offsets can erase variance or produce a negative result; squares can overflow. Arithmetic reproduction with `[1e9,1e9+1,1e9+2,1e9+3]` gives zero instead of population variance `1.25`. Use centered variance. |
| [saltelli2010.rs:106](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/saltelli2010.rs:106), `:115`, `:144`, `:146`, `:303`, `:308` | CC, OU | First/total-order output differences lose small effects on large model baselines. Second-order calculations subtract large products, then subtract first-order indices, creating multiple cancellation stages. Products and squared differences can overflow or underflow. |
| [saltelli2010.rs:108](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/saltelli2010.rs:108), `:118`, `:146` | NaN, INF | Model-based estimation divides without checking variance or callback outputs. Constant outputs intentionally produce NaN, as the tests document, but invalid inputs and arithmetic failures produce the same untyped result. |
| [saltelli2010.rs:204](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/saltelli2010.rs:204), `:217`, `:275`, `:288`, `:305` | NaN, EPS, INF | Output-based variants instead return zero when `abs(variance)<1e-30`. NaN bypasses this guard; negative cancellation-corrupted variance is accepted. Behavior differs from the model-based API and estimators using `1e-15`. Empty aligned output arrays also permit division by zero. |
| [bootstrap.rs:254](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/bootstrap.rs:254), `:265`, `:267`, `:274`, `:276` | CC, NaN, OU, INF | Each resample repeats raw-moment variance and unguarded ratios. A valid original sample can generate a constant resample, putting NaN into the percentile pool. No invalid-resample handling exists here. |
| [bootstrap.rs:133](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/bootstrap.rs:133), `:134`, `:212`, `:213` | NaN, EPS, OU | These public bootstrap APIs do not validate `alpha`. NaN/out-of-range alpha reaches percentile indexing/interpolation. Even valid tiny alpha can underflow in `alpha/2` or make the upper probability round to `1`. |
| [bootstrap.rs:292](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/bootstrap.rs:292), `:304`, `:307`, `:309`, `:312` | NaN, INF, OU | NaN estimates are sorted as `Equal`. NaN percentile positions cast to zero; extreme invalid positions can overflow index arithmetic. Interpolation can produce `0*Inf=NaN` even at an exact order statistic when its neighbor is infinite. |
| [bootstrap_given_data.rs:269](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/bootstrap_given_data.rs:269), `:282`, `:311` | NaN, INF | `Ok(Vec<f64>)` is accepted without checking entries. Numerical failures returned as `Ok(NaN)` count as successful resamples. All-failed resamples deliberately return NaN CIs; a typed unavailable-result variant would distinguish this from corrupted arithmetic. |
| [janon.rs:151](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/janon.rs:151), `:156`, `:165`, `:173`, `:176` | CC, OU | Raw second moments have the same offset problem. Adding two means or two squares before dividing by two can overflow although their average is representable. |
| [janon.rs:178](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/janon.rs:178), `:201`, `:203` | NaN, EPS, INF | `denom > 1e-15` silently returns zero for NaN or negative denominator. Infinity passes. Second-order calculations have no corresponding variance guard and can return NaN/Inf. |
| [jansen.rs:116](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/jansen.rs:116), `:124`, `:129`, `:130`, `:156`, `:158` | CC, NaN, EPS, OU, INF | Raw-moment variance; squared output differences; NaN variance converted to zero first-order indices; cancellation in `1-ratio` when the first-order index is small. Second-order divisions remain unguarded. |
| [owen.rs:140](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/owen.rs:140), `:143`, `:152`, `:155` | CC, NaN, EPS, OU | Centered variance avoids raw-moment cancellation, but summing before division and squaring can overflow. Products of paired output differences lose tiny effects. `variance > 1e-15` converts NaN variance into zero indices. |
| [morris.rs:266](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/morris.rs:266), `:385` | CC, NaN, OU, INF | Elementary effects subtract potentially nearly equal model outputs and divide by unchecked stored deltas. Invalid callback values or zero/nonfinite deltas propagate. |
| [morris.rs:293](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/morris.rs:293), `:298`, `:304`, `:408`, `:416`, `:421`, `:436`, `:440`, `:444` | CC, NaN, OU, INF | Effect means sum before division; centered squares can overflow/underflow. No finite-result checks. Returning zero standard deviation for one trajectory is deliberate, but also masks an invalid singleton effect. |
| [dgsm.rs:214](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/dgsm.rs:214), `:215`, `:218` | NaN, OU, INF | Variance and Poincaré constants are checked, but gradient entries are not. Squaring gradients and forming `vi*cp/variance` can overflow/underflow prematurely. In particular, `cp=0` and overflowing `vi` yield NaN. |
| [dgsm.rs:256](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/dgsm.rs:256), `:271`, `:274`, `:280`, `:282`, `:285` | CC, EPS, NaN, OU, INF | `eps > 0` accepts infinity. Small steps can round `x±eps` back to `x`, yielding a spurious zero derivative. Large steps overflow coordinates or `2*eps`. Output subtraction cancels, and division uses the requested step rather than the actual representable displacement. No finite input/output checks or boundary-aware stepping. |
| [dgsm.rs:319](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/dgsm.rs:319), `:320` | OU, NaN, INF | Poincaré constants square a width or standard deviation without validating the distribution or intermediate result. |
| [fractional_factorial.rs:85](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/fractional_factorial.rs:85), `:86` | NaN, INF, OU | Mapping coded endpoints using distribution support fails for unbounded distributions: infinite widths and `0*Inf` produce NaN/Inf. Finite opposite-sign extreme bounds also overflow the width. |
| [fractional_factorial.rs:100](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/fractional_factorial.rs:100), `:103`, `:107` | CC, OU, NaN, INF | Group sums can overflow before averaging; subtracting nearly equal group means loses weak effects. Model outputs are unchecked. Malformed designs can leave an empty group and divide by zero. |

**Estimators: given-data, spectral, ANOVA, and regression**

| Location | Categories | Finding |
|---|---|---|
| [pawn.rs:180](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/pawn.rs:180), `:208`, `:282`, `:292`, `:295` | NaN | **Potential infinite loop.** Sorting permits NaN. In the KS merge, neither `NaN <= v` nor `value <= NaN` advances an index. Once a NaN remains at the head of a stream, the outer loop can run forever. Reject nonfinite outputs before sorting. |
| [pawn.rs:312](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/pawn.rs:312), `:258` | NaN, INF, EPS | Nonfinite input factors corrupt ordinal ranks. The CV guard returns zero for NaN mean, masking failure. The absolute `1e-15` CV cutoff discards sufficiently small mean KS values. |
| [pawn.rs:301](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/pawn.rs:301) | CC | Subtracting close empirical-CDF fractions loses relative accuracy for very small KS distances. This is usually minor at practical sample counts. |
| [borgonovo.rs:340](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/borgonovo.rs:340), `:166`, `:332` | NaN, INF | `min_max` ignores NaNs because both comparisons are false. Thus a finite range can pass validation while some outputs are NaN. NaN input factors are also sorted as equal. |
| [borgonovo.rs:174](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/borgonovo.rs:174) | CC, OU | Grid construction multiplies the width by the grid index before dividing. That intermediate can overflow. Large offsets with narrow ranges can collapse distinct grid points. |
| [borgonovo.rs:276](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/borgonovo.rs:276), `:286` | NaN, EPS, OU | Bandwidth variance can overflow/underflow. `h.max(1e-15)` turns NaN into the finite floor, hiding the bandwidth failure. The absolute floor makes results depend on output units. |
| [borgonovo.rs:295](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/borgonovo.rs:295), `:299`, `:300`, `:232`, `:321` | CC, OU, NaN, INF | KDE normalization and standardized distances can overflow; Gaussian tails underflow. Subtracting nearly equal densities loses tiny δ signals. Trapezoidal integration adds densities before halving and multiplies by potentially collapsed grid widths. |
| [given_data_sobol.rs:225](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/given_data_sobol.rs:225) | NaN, INF | Global output variance has a finite guard, but factor inputs do not. NaN factor values silently alter class membership through the fallback comparator. |
| [given_data_sobol.rs:217](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/given_data_sobol.rs:217), `:218`, `:195`, `:196` | CC, OU | Mean and centered-square accumulation retain overflow/underflow risks. `1-intra_variance/variance` cancels for weak factors; clamping can conceal numerical excursions. |
| [qosa.rs:239](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/qosa.rs:239), `:289` | OU | Rounded `alpha*n` followed by `ceil` can choose the adjacent order statistic near an integer boundary. |
| [qosa.rs:248](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/qosa.rs:248), `:250`, `:251`, `:310`, `:314`, `:315` | CC, EPS, OU, NaN, INF | Tail sums can overflow before normalization. `CTE-mean` loses precision for a large baseline and small tail excess. The threshold `1e-12*abs(mean)+1e-15` is not translation invariant. The global denominator is checked, but conditional tail arithmetic can still become nonfinite; final clamping can conceal infinities. Factor ranks inherit Borgonovo’s unchecked NaNs. |
| [fast.rs:223](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/fast.rs:223), `:230`; [rbd_fast.rs:237](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/rbd_fast.rs:237), `:244` | OU, CC, NaN, INF | FFT inputs are neither centered nor scaled. A large DC offset can overflow FFT intermediates or obscure small AC components. `re²+im²` can overflow before division by `N²`, or underflow for small signals. Finite guards eventually reject many failures, but report them as zero variance. |
| [fast.rs:174](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/fast.rs:174), `:205`; [rbd_fast.rs:212](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/rbd_fast.rs:212) | EPS, CC | Absolute `1e-15` variance floors depend on output units. FAST total-effect calculation `1-complement/total` loses relative accuracy near zero, followed by clamping. |
| [fast.rs:184](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/fast.rs:184), `:186`, `:190` | OU | Estimator scratch storage supports only 32 harmonics; the bound is a debug assertion, while the sampler accepts larger values. Valid sampler output can therefore panic during release estimation. |
| [rbd_fast.rs:186](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/rbd_fast.rs:186), `:187`, `:224` | CC, OU | Bias correction `(S_naive-lambda)/(1-lambda)` becomes poorly conditioned near the minimum sample size. Both subtractions lose precision and the small denominator amplifies error. |
| [rbd_fast.rs:200](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/rbd_fast.rs:200) | NaN, INF | Factor values are not checked before sorting. NaNs receive an arbitrary fallback ordering. This limitation is documented but remains an API misuse opportunity. |
| [regression.rs:186](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/regression.rs:186), `:191` | NaN, INF, EPS | Variance rejection uses `<1e-15`, allowing NaN and infinity through. |
| [regression.rs:289](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/regression.rs:289), `:290` | CC, OU, NaN, INF | Unscaled normal equations square the design’s condition number. Large-offset or nearly collinear predictors can lose rank numerically; Gram products can overflow. Successful Cholesky/solve is not followed by finite-coefficient validation. |
| [regression.rs:243](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/regression.rs:243), `:248`, `:249`, `:261`, `:355` | CC, EPS, OU, NaN | Residual subtraction loses small errors relative to large fits. `1-ss_res/ss_tot` cancels near zero R²; NaN total sums instead return zero R². Standardized-coefficient multiplication/division can overflow prematurely. |
| [regression.rs:369](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/regression.rs:369), `:375`, `:376` | CC, OU, NaN, INF, EPS | Pearson denominator is `sqrt(var_a*var_b)`. The product can overflow or underflow although the final correlation is representable, producing a false zero or invalid ratio. NaN bypasses the small-denominator rejection. |
| [regression.rs:388](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/regression.rs:388), `:389`, `:398` | OU, NaN, INF | Centered variance still sums before division and squares without scaling. Rank conversion silently masks NaN ordering. Ordinal tie handling also makes results sensitive to input order when rounding collapses distinct values. |
| [anova.rs:282](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/anova.rs:282), `:284`, `:287`, `:290`, `:310`, `:373`, `:453` | CC, OU, NaN, INF | Uncompensated means, centered squares, and multi-term interaction residuals can overflow or lose small effects on a large baseline. No finite grid validation. |
| [anova.rs:291](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/anova.rs:291), `:375`, `:775`, `:778` | NaN, INF, EPS | `ss_total <= 1e-15` and denominator tolerance checks permit NaN. Ratios and inferential statistics then propagate invalid values. Absolute cutoffs vary with output scale. |
| [anova.rs:766](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/anova.rs:766) | CC | `1-dist.cdf(statistic)` loses small upper-tail probabilities when CDF rounds to one. Use the survival-function calculation. |
| [anova.rs:639](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/anova.rs:639), `:692`; [g_theory.rs:426](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/g_theory.rs:426), `:437` | NaN, INF | Bootstrap routines accept successful results containing nonfinite components, then pass them to percentile calculations. |
| [g_theory.rs:183](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/g_theory.rs:183), `:184`, `:185`, `:263`, `:311`, `:316` | CC, OU, NaN, INF, EPS | Means, squares, interaction residuals, and differences of mean squares suffer cancellation/overflow. The total-variance guard does not reject nonfinite values. Negative method-of-moments components can be statistically legitimate; cancellation adds a separate numerical risk. |
| [g_theory.rs:319](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/g_theory.rs:319), `:475`, `:562`, `:569` | CC, NaN, INF, EPS | Reliability denominators combine signed components and can nearly cancel. The absolute-zero guard lets NaN pass; infinite denominators can silently yield zero reliability. |
| [g_theory.rs:534](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/g_theory.rs:534), `:547` | NaN, INF | A NaN target produces no feasible design. A NaN cost can become the initial “best” cost, after which comparisons prevent replacement. Inputs/cost callback results are unchecked. |
| [hdmr.rs:173](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/hdmr.rs:173), `:180` | CC, OU, NaN, INF | Canonical mapping subtracts locations or bounds and divides by widths/scales. Degenerate discrete support produces division by zero; extreme finite widths can overflow. Input dimensionality is not tied to the problem. |
| [hdmr.rs:199](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/hdmr.rs:199), `:212`, `:253` | NaN, EPS, OU, INF | Coefficient-square/norm products can become nonfinite. Unlike the surrogate PCE helper, the total-variance guard lacks `is_finite()`. NaN/Inf can propagate through normalized results and clamping. |
| [discrepancy.rs:108](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/discrepancy.rs:108), `:116`, `:128`, `:146`, `:154`, `:172`, `:180`, `:190`, `:214`, `:222`, `:237` | OU | Dimension-dependent powers and tensor products can overflow or underflow for high-dimensional valid unit-cube inputs. Accumulation can overflow before normalization. Casting dimension to `i32` also imposes an unchecked exponent limit. |
| [discrepancy.rs:135](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/discrepancy.rs:135), `:160`, `:196`, `:243` | CC, NaN, INF | Discrepancy squares subtract comparable large terms, losing accuracy for low-discrepancy designs. `.max(0.0).sqrt()` hides negative results and **maps NaN to zero**, potentially reporting perfect space filling after overflow. Input validation itself correctly rejects NaN and infinity. |

**Surrogate crate**

Locations are relative to `crates/salib-surrogate/src/`.

| Location | Categories | Finding |
|---|---|---|
| [polynomial.rs:85](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/polynomial.rs:85), `:126` | INF | Jacobi parameter assertions reject NaN but accept positive infinity. Public polynomial evaluation does not validate finite arguments or canonical domains. Degree-zero evaluation returns one even for invalid arguments. |
| [polynomial.rs:161](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/polynomial.rs:161), `:184`, `:203`, `:242` | CC, OU, NaN, INF | Three-term recurrences subtract comparable terms near roots and can overflow at high degree or large arguments. Jacobi coefficients contain additional large products and poorly conditioned denominators near parameter boundaries. |
| [polynomial.rs:230](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/polynomial.rs:230), `:233`, `:239` | CC, OU | Jacobi’s first-order expression `(alpha+1)+(alpha+beta+2)*(x-1)/2` unnecessarily cancels. For `alpha=beta=0`, `x=1e-20`, it can return zero although the equivalent Legendre polynomial returns `x`. `alpha²-beta²` also loses accuracy for close parameters and can become `Inf-Inf`. |
| [polynomial.rs:253](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/polynomial.rs:253), `:259`, `:264` | NaN, CC, OU | **Valid-parameter defect:** for degree zero and `alpha+beta <= -1`, the logarithmic norm formula encounters zero/negative arguments and returns NaN. Example: `alpha=beta=-0.75` is valid, but the constant polynomial’s norm should be exactly one. Handle degree zero/removable singularities explicitly. Large log-gamma terms also cancel before exponentiation. |
| [polynomial.rs:273](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/polynomial.rs:273) | OU, INF | Hermite norm uses direct factorial, overflowing at degree 171. Zero coefficients subsequently multiplied by infinite norms produce NaN. |
| [polynomial.rs:304](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/polynomial.rs:304), `:311`, `:314` | CC, OU, NaN, INF | Custom log-gamma reflection and Lanczos expressions encounter poles, inaccurate `sin(pi*x)` near integers, cancellation in coefficient sums, and large-value overflow. No finite/domain-result checks. |
| [multi_index.rs:179](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/multi_index.rs:179) | FC | `sum == 0.0` is the sole explicit production float equality operator found. Here it is appropriate: a sum of nonnegative integer-degree powers is exactly zero only for the zero multi-index. |
| [multi_index.rs:182](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/multi_index.rs:182), `:219` | EPS, OU | Valid very small `q` can overflow `1/q`; powers lose information or overflow. Filtering uses an absolute `1e-12` tolerance. Compare in a suitably scaled/logarithmic form instead of constructing a potentially infinite norm. |
| [multi_index.rs:158](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/multi_index.rs:158), `:162`, `:164` | OU | Basis-size integer arithmetic can overflow `d+degree` or its `u128` intermediate, then truncate on conversion to `usize`. This undermines allocation and shape assumptions. |
| [pce.rs:130](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/pce.rs:130), `:134`, `:143`, `:146`, `:148` | CC, OU, NaN, INF | Evaluation shape/domain checks are debug-only. Publicly mutable coefficient/basis arrays can cause silent zip truncation. Tensor products can overflow/underflow, and the final signed sum is uncompensated and unchecked. |
| [pce.rs:273](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/pce.rs:273), `:291`, `:299`, `:301`, `:303` | CC, OU, NaN, INF | Fit domain checks disappear in release; outputs are not checked. Basis products and normal equations can overflow and amplify conditioning errors. A successful solve can return nonfinite coefficients without a fit error. |
| [pce.rs:336](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/pce.rs:336), `:338`, `:360` | EPS, OU, NaN, INF | Norm products and `beta²*norm` can overflow/underflow or produce `0*Inf`. Total variance is correctly checked for finiteness, but the absolute `1e-15` floor rejects valid small-scale models. A nonfinite constant coefficient is excluded from variance, so finite sensitivity indices can accompany invalid predictions. |
| [sparse_pce.rs:186](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/sparse_pce.rs:186), `:216`, `:491`, `:563`, `:578` | CC, OU, NaN, INF | Same unchecked release domains, outputs, tensor products, and normal-equation conditioning problems as full PCE, repeated throughout selection/refitting. |
| [sparse_pce.rs:278](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/sparse_pce.rs:278), `:281`, `:290`, `:339` | NaN, INF | **OMP can return a zero model after numerical failure.** Best coefficients start at zero; if every LOO error is NaN or infinity, none beats `best_loo=Inf`. The fallback is unreachable because `best_active` starts nonempty. Returned diagnostics can contain infinity. |
| [sparse_pce.rs:323](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/sparse_pce.rs:323), `:382`, `:383`, `:388`, `:389` | CC, EPS, OU, NaN, INF | Residual/column norms can overflow or underflow; output centering loses small variations on a large baseline. Fixed floors and `.max(1.0)` make selection depend on units. |
| [sparse_pce.rs:464](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/sparse_pce.rs:464), `:469`, `:499`, `:643`, `:647` | NaN, INF, EPS | Nonfinite correlations are silently discarded, which can make an arithmetic failure appear to be convergence. Correlation and denominator cutoffs use unrelated absolute tolerances. |
| [sparse_pce.rs:523](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/sparse_pce.rs:523), `:524`, `:533`, `:538` | CC, EPS, OU, NaN, INF | LARS subtracts close correlations and equiangular components before dividing. Invalid candidates are ignored; the fallback step can itself become infinite. Updating the fitted path can overflow. |
| [sparse_pce.rs:589](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/sparse_pce.rs:589), `:609`, `:613`, `:617`, `:621` | CC, EPS, OU, NaN, INF | PRESS divides residuals by `1-h_ii`, which cancels near leverage one. The `1e-10` guard skips singular observations rather than invalidating the fit, potentially biasing LOO downward. NaN bypasses the guard. Squared residuals can overflow; infinity is used as a failure sentinel. |
| [active_subspace.rs:171](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/active_subspace.rs:171), `:173`, `:181`, `:203` | OU, NaN, INF | Gradient products and accumulated covariance can overflow before averaging. Inputs/covariance are not checked before eigendecomposition. The dependency’s default eigensolver permits unlimited iterations, so a nonfinite matrix can fail to terminate before the later spectrum check. This is a potential dependency-path hang, not a runtime reproduction. |
| [active_subspace.rs:210](/Users/patrickbeam/projects/salib/crates/salib-surrogate/src/active_subspace.rs:210), `:212`, `:236`, `:237`, `:251`, `:252` | EPS, OU, INF | `.max(1.0)` introduces an absolute floor in negative-eigenvalue clipping. Gap thresholds can become subnormal; overflowing eigenvalue ratios are ignored, potentially changing the selected dimension. Tiny or zero spectra need an explicit policy. |

**Accumulation order: no thread-count-dependent reduction defect found**

The core parallel implementation is carefully structured:

- [reduce.rs:57](/Users/patrickbeam/projects/salib/crates/salib-core/src/reduce.rs:57) fixes a power-of-two block size of 4096.
- [reduce.rs:101](/Users/patrickbeam/projects/salib/crates/salib-core/src/reduce.rs:101) and `:132` collect indexed parallel chunks in order.
- [reduce.rs:188](/Users/patrickbeam/projects/salib/crates/salib-core/src/reduce.rs:188) computes variance blocks with a fixed subsequent reduction.

I found **no category-5 case where Rayon scheduling or thread count changes sum order**.

There are nevertheless order-sensitive *serial* accumulations: ANOVA/G-theory means, discrepancy sums, KDE/integration, regression covariance, active-subspace covariance, and PCE evaluation. Pairwise reduction remains sensitive to input permutation too. Thus “all sums route through `tree_*`” in [estimators/lib.rs:15](/Users/patrickbeam/projects/salib/crates/salib-estimators/src/lib.rs:15) overstates the implementation.

**Float equality inventory**

Direct operators on floats are limited to:

| Location | Use | Assessment |
|---|---|---|
| `surrogate/multi_index.rs:179` | `sum == 0.0` | Appropriate exact zero test, as described above. |
| `samplers/morris.rs:503,519` | `base == 0.0` | Test-only exact grid endpoint checks. |
| `samplers/plackett_burman.rs:114,146` | `v == -1.0 || v == 1.0` | Appropriate exact checks of generated coded values. |

**Derived `PartialEq` also performs exact float equality**, even where no explicit operator appears:

- Core: `distribution.rs:57`; `problem.rs:55,76`.
- Samplers: `iman_conover.rs:77`.
- Estimators: `sobol_indices.rs:15,63`; `morris.rs:74`; `bootstrap_given_data.rs:106,153`; `anova.rs:51`; `g_theory.rs:53,131`; `dgsm.rs:129`; `qosa.rs:159`.
- Surrogate: `polynomial.rs:49`; `sparse_pce.rs:92`; `multi_index.rs:93`; `active_subspace.rs:103`.

These are suitable for exact parameter identity or deterministic replay, but unsuitable as approximate numerical comparisons. Values containing NaN are nonreflexive; `+0.0` and `-0.0` compare equal despite differing bits.

Tests also use exact equality through `assert_eq!`/`assert_ne!` on float scalars, arrays, or float-bearing results. The relevant locations are listed below; most intentionally verify exact endpoints, copied values, or deterministic output:

| Crate/file | Lines |
|---|---|
| Core `distribution.rs` | 324, 331–333, 339–341, 347, 362–363, 408–409, 449–450, 470, 480, 640, 686, 702, 708, 731, 742, 744, 755–758, 760–762, 776, 784–788, 795–798, 804–805, 808, 856 |
| Core `problem.rs` | 697 |
| Core `reduce.rs` | 223, 228, 233, 242, 262, 289, 335–336, 384, 390, 396, 459, 485, 492, 549 |
| Samplers `fast.rs` | 424, 426, 438 |
| Samplers `iman_conover.rs` | 509 |
| Samplers `lhs.rs` | 326, 338, 442–443, 480 |
| Samplers `sobol.rs` | 376, 400, 418, 431, 454, 552 |
| Samplers `saltelli_matrix.rs` | 395, 407, 425, 432, 451–452, 466–467, 469, 480–481, 491, 539, 571, 626–628, 633–635, 660–662, 667–669, 699–700, 702 |
| Samplers `owen_matrix.rs` | 227, 230, 246, 248, 271–273, 283–285 |
| Estimators `bootstrap.rs` | 399, 481, 489, 496, 501–503 |
| Estimators `bootstrap_given_data.rs` | 385, 490, 506, 631 |
| Estimators `borgonovo.rs` | 498 |
| Estimators `dgsm.rs` | 443, 475, 574–575 |
| Estimators `given_data_sobol.rs` | 380, 395 |
| Estimators `janon.rs` | 257, 297–298 |
| Estimators `jansen.rs` | 208, 238 |
| Estimators `morris.rs` | 553–555, 566, 602 |
| Estimators `owen.rs` | 205, 258 |
| Estimators `pawn.rs` | 494–495, 503, 511, 543–546, 552 |
| Estimators `qosa.rs` | 452, 508–510 |
| Estimators `rbd_fast.rs` | 394 |
| Estimators `regression.rs` | 584–589, 613, 619 |
| Estimators `saltelli2010.rs` | 456 |
| Estimators `sobol_indices.rs` | 191–193 |
| Surrogate `pce.rs` | 628 |
| Surrogate `sparse_pce.rs` | 861, 891 |
| Surrogate `polynomial.rs` | 328–329, 358–362, 368, 410, 457, 465, 526–528 |

Assertions comparing `to_bits()` compare integers and are appropriate for the explicit bitwise determinism contract.

**Type-level constraints that would prevent misuse**

These should use private fields, checked constructors, and validated deserialization. Merely wrapping public fields does not establish an invariant.

| Current location/API | Proposed constraint | Misuse prevented |
|---|---|---|
| `core/distribution.rs:60`; `core/problem.rs:287` | `FiniteF64`, `PositiveFinite`, `Probability`, finite ordered `Bounds`; validated distribution parameter structs | NaN means, infinite scales/shapes, invalid probabilities, reversed bounds. Bounds constructors should also check whether required width arithmetic is representable. |
| `core/problem.rs:55,76,79` | Private validated `Problem`/`Factor` fields; checked serde conversion | Mutation/deserialization bypassing builder invariants; inconsistent factor kinds and distributions. |
| `samplers/sampler.rs:35` | `SampleMatrix<const D: usize, Space>` or a validated dynamic-dimension wrapper | Wrong column counts, invalid values, confusion between unit-cube and physical samples. Space markers could distinguish `UnitCube`, `Physical`, and polynomial-canonical coordinates. |
| `samplers/saltelli_matrix.rs:59`; `owen_matrix.rs:59` | Validated `SaltelliDesign<D, Order>` / `OwenDesign<D>` with private matrices and shared row-count metadata | Misaligned matrices, mutated dimensions, missing hybrids, inconsistent sample counts. An order marker can require second-order hybrids at compile time. |
| `samplers/saltelli_matrix.rs:305` | Separate physical factor dimension and group/index dimension | Grouped designs currently use metadata whose “dimension” differs from the physical column count. |
| `estimators/saltelli2010.rs:180`; `bootstrap.rs:175` | Validated output bundles tied to a design and common sample count | Empty samples, output-length mismatches, mixing outputs from different designs. |
| `samplers/lhs.rs:120`; design constructors; bootstrap APIs | `NonZeroUsize`, plus validated minimum-count types where necessary | Zero sample/resample sizes. Variance needs at least two samples; statistical estimators need stronger bounds than merely nonzero. |
| `samplers/fast.rs:148`; `estimators/fast.rs:184` | Harmonic-budget newtype bounded to `1..=32`, checked sample-budget constructor | Sampler/estimator harmonic mismatch and overflowing bandwidth arithmetic. |
| `samplers/morris.rs:112,235`; `estimators/morris.rs:349` | Validated even grid-level type; `MorrisDesign<D, Grouped/Ungrouped>`; validated group partition | Odd/invalid levels, duplicate/empty groups, grouped API called on ungrouped designs, inconsistent trajectories and deltas. |
| `samplers/iman_conover.rs:149` | `CorrelationMatrix<D, CorrelationKind>` with finite, symmetric, unit-diagonal, positive-definite construction | Invalid correlation matrices and ambiguity between Pearson and rank-correlation semantics. |
| `estimators/dgsm.rs:256` | `PositiveFiniteStep` and finite gradient matrix | Infinite finite-difference steps and nonfinite gradient entries. Step representability still requires a runtime check at each coordinate. |
| `surrogate/multi_index.rs:39`; `pce.rs:84` | `MultiIndex<D>` and a validated basis object owning families, indices, and coefficients | Dimension mismatches and coefficient/basis zip truncation. |
| `surrogate/pce.rs:273`; `estimators/hdmr.rs:144` | Canonical sample types associated with polynomial families/distribution transforms | Physical samples passed as canonical samples; wrong normalization or weight family. HDMR currently selects Legendre for finite-support distributions whose weights need not be uniform. |
| `estimators/sobol_indices.rs:41`; bootstrap result types | Validated finite result constructors; `Result`/`Option` for undefined estimates or unavailable CIs | NaN used interchangeably for invalid input, undefined statistics, solver failure, and all-failed bootstrap. |
| `estimators/g_theory.rs:97,534,547` | Finite component/cost types, bounded target type | Nonfinite reliability inputs and NaN optimization costs. Do **not** require every estimated variance component to be positive: signed estimates can be legitimate. |

Const generics can enforce static dimensions and marker types can enforce coordinate/design semantics. Finiteness, positive definiteness, representable steps, and conditioning still require runtime validation.

The highest-value fixes are finite validation at public boundaries, centered/scaled numerical formulas, explicit failure results, and consistent scale-aware tolerances. I did not run Cargo tests or builds; this was source analysis with small arithmetic reproductions.