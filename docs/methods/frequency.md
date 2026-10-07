# Frequency-based methods

FAST and RBD-FAST estimate variance contributions from the Fourier spectrum of
model outputs. FAST/eFAST uses one search curve per factor, for $dN$ model
evaluations. RBD-FAST reorders a single sample of $N$ evaluations and returns
first-order indices only.

## Theory

Cukier et al. (1973) *J. Chem. Phys.* 59(8), 3873--3878. [paper](https://doi.org/10.1063/1.1680571) · [reference](../bibliography.md#cukier1973)

FAST assigns each factor a characteristic frequency $\omega_i$ and samples the model along a search curve in the input space. The curve sweeps a single parameter $s$ through $[0, 2\pi)$ and converts it to a $d$-dimensional input via

$$x_j(s) = \tfrac{1}{2} + \tfrac{1}{\pi}\arcsin\!\big(\sin(\omega_j\, s + \varphi_j)\big)$$

where $\varphi_j \sim \mathrm{Uniform}[0, 2\pi)$ is a random phase shift. Because each factor oscillates at a different frequency, the model output's Fourier spectrum at frequency $\omega_i$ reveals $X_i$'s first-order contribution.

The one-sided power spectrum is $\mathrm{Sp}[k] = |Y[k]|^2 / N^2$ for $k \in [1, \lfloor N/2 \rfloor]$, where $Y[k]$ is the DFT of the model output at frequency $k$. The first-order index extracts variance at harmonics of $\omega_i$:

$$S_i = \frac{\sum_{p=1}^{M} w_{p\omega_i}\,\mathrm{Sp}[p\,\omega_i]}{\sum_{k=1}^{\lfloor N/2 \rfloor} w_k\,\mathrm{Sp}[k]}$$

where $M$ is the harmonic truncation order (typically 4). The multiplicity
$w_k$ is 2 for a conjugate frequency pair and 1 for the even-$N$ Nyquist bin
($k=N/2$), which has no conjugate partner. The estimator uses these weights
for both odd and even sample counts.

## FAST / eFAST

Saltelli, Tarantola & Chan (1999) *Technometrics* 41(1), 39–56. [paper](https://doi.org/10.1080/00401706.1999.10485594) · [reference](../bibliography.md#saltelli1999)

Classic FAST (Cukier 1973) computes first-order indices only. The extended variant (eFAST, Saltelli 1999) adds total-effect indices by partitioning the spectrum into "factor $i$" and "everything else" bands.

**First-order.** Factor $i$ gets the maximum frequency $\omega_{\max} = \lfloor(N-1)/(2M)\rfloor$. First-order variance is the spectral power at harmonics $\omega_{\max}, 2\omega_{\max}, \ldots, M\omega_{\max}$:

$$V_{1,i} = \sum_{p=1}^{M} w_{p\omega_i}\,\mathrm{Sp}[p\,\omega_i]$$

**Total-effect.** The complementary variance collects all spectral power in the low-frequency band $[1, \lfloor\omega_i/2\rfloor]$, which excludes factor $i$'s harmonics:

$$V_{\sim i} = \sum_{k=1}^{\lfloor\omega_i/2\rfloor} w_k\,\mathrm{Sp}[k]$$

$$S_{Ti} = 1 - \frac{V_{\sim i}}{V}$$

where $V = \sum_{k=1}^{\lfloor N/2\rfloor} w_k\,\mathrm{Sp}[k]$ is the total spectral variance.

**Frequency assignment.** The remaining $d - 1$ factors receive complementary frequencies bounded above by $\omega_{\max} / (2M)$ to estimate the complementary contribution from low frequencies. Higher harmonics and interactions can still alias. When $m = \lfloor\omega_{\max}/(2M)\rfloor \geq d - 1$, complementary frequencies are drawn from $\mathrm{linspace}(1, m, d-1)$; otherwise they cycle $1, 2, \ldots, m, 1, 2, \ldots$ This frequency reuse is a design tradeoff; it does not eliminate aliasing.

**Cost.** One search curve per factor with $N$ points each requires $dN$ model evaluations.

```rust
use salib::samplers::{build_fast_design, FastDesign};
use salib::estimators::estimate_fast;
use salib::RngState;

let mut rng = RngState::from_seed([0u8; 32]);
let design: FastDesign = build_fast_design(
    3,    // d = 3 factors
    1025, // N per factor
    4,    // harmonic order M
    &mut rng,
).unwrap();

let indices = estimate_fast(&design, |u| {
    let pi = std::f64::consts::PI;
    let x = [2.0*pi*u[0]-pi, 2.0*pi*u[1]-pi, 2.0*pi*u[2]-pi];
    x[0].sin()
        + 7.0 * x[1].sin().powi(2)
        + 0.1 * x[2].powi(4) * x[0].sin()
}).unwrap();

println!("{indices}");
```

Compare the example against the analytic Ishigami indices in the
[quickstart](../quickstart.md#5-interpret-the-results). Frequency truncation
and aliasing can affect the result; check sample count and harmonic order.

The constructor requires $N \geq 4M^2 + 1$, or at least 65 samples per factor
when $M=4$. Values such as 257 or 1025 are starting points; the required size
depends on the model and desired precision.

## RBD-FAST

Tarantola et al. (2006) *Rel. Eng. Sys. Safety* 91(6), 717--727. [paper](https://doi.org/10.1016/j.ress.2005.06.003) · [reference](../bibliography.md#tarantola2006)
Plischke (2010) *Rel. Eng. Sys. Safety* 95(4), 354–360. [paper](https://www-10b015.pages.gwdg.de/papers/easi_ress.pdf) · [reference](../bibliography.md#plischke2010)

RBD-FAST reuses a single random sample for all factors.

The EASI version in [Plischke (2010), §3](https://www-10b015.pages.gwdg.de/papers/easi_ress.pdf)
sorts each input, then arranges odd ranks in ascending order followed by even
ranks in descending order. That triangular traversal reduces the discontinuity
at the periodic boundary. The low-frequency spectrum estimates first-order variance:

salib applies this traversal before the FFT. `scripts/check-frequency-literature.py`
checks a single-factor cosine example and an independent even-$N$ Nyquist fixture.
Input columns must contain distinct finite values. Ties are rejected because
ordering their outputs arbitrarily would invent a rank trace; use a conditioning
method for discrete inputs. Outputs must also be finite.

$$V_{1,i} = \sum_{k=1}^{M} w_k\,\mathrm{Sp}[k]$$

where $\mathrm{Sp}$ is the power spectrum of the permuted $Y$ vector.

**Plischke bias correction.** The naive estimate $\hat{S}_{\mathrm{naive}} = V_{1,i}/V$ overestimates small effects. The 2010 correction subtracts the expected bias:

$$\lambda = \frac{2M}{N}, \qquad S_i = \frac{\hat{S}_{\mathrm{naive}} - \lambda}{1 - \lambda}$$

After bias correction, sampling error can give slightly negative estimates for factors whose true index is zero.

**First-order only.** RBD-FAST does not produce total-effect indices. The reordered spectrum does not separate the complementary band needed for $S_{Ti}$.

**Cost.** $N$ evaluations total. The estimator accepts an input/output dataset from LHS, a Sobol' sequence, or another source.

```rust
use salib::samplers::{LhsSampler, Sampler};
use salib::estimators::estimate_rbd_fast;
use salib::RngState;

let mut rng = RngState::from_seed([0u8; 32]);
let sampler = LhsSampler::classic(3);
let x = sampler.unit_sample(4097, &mut rng);

// Map [0, 1]^3 to [-pi, pi]^3 and evaluate Ishigami
let y: Vec<f64> = (0..4097).map(|i| {
    let u = [x[[i, 0]], x[[i, 1]], x[[i, 2]]];
    let v: Vec<f64> = u.iter()
        .map(|&u| -std::f64::consts::PI + 2.0 * std::f64::consts::PI * u)
        .collect();
    v[0].sin() + 7.0 * v[1].sin().powi(2) + 0.1 * v[2].powi(4) * v[0].sin()
}).collect();

let indices = estimate_rbd_fast(x.view(), &y, 10).unwrap();
println!("{indices}");
```

The example uses $M=10$; pass the harmonic order explicitly to the estimator.
RBD-FAST requires $N \geq 2M+1$. Its reordered samples have a different spectrum
from the FAST search curves, so the same harmonic order need not suit both.

## Choosing between methods

| Method    | $S_i$ | $S_{Ti}$ | Cost               | Best for                                                  |
|-----------|--------|----------|--------------------|------------------------------------------------------------|
| eFAST     | yes    | yes      | $d \times N$       | Both indices via spectral decomposition.                   |
| RBD-FAST  | yes    | no       | $N$                | First-order only, tight budget, distinct continuous inputs. |

Use eFAST when you need total effects from a frequency design. Saltelli is
another option, with $N(d+2)$ model calls and no frequency assignment. Use
RBD-FAST when you have a single sample and only need first-order indices.
