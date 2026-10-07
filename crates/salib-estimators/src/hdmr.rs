//! HDMR component variances obtained by grouping a fitted PCE.
//!
//! This implementation fits all total-degree polynomials up to
//! `max_degree` by OLS, then groups coefficient contributions by their
//! active factor sets. It returns first-, second-, and total-order Sobol'
//! indices and variance fractions by interaction order.
//!
//! The coefficient identities require independent inputs and polynomial
//! families orthogonal under the input measure. The current automatic
//! mapping supports Uniform and Normal factors. Other distributions are
//! rejected: an affine support transformation does not turn a nonuniform
//! measure into the uniform reference measure of Legendre polynomials.
//!
//! `max_order` limits the length of the reported `order_variance` vector.
//! It does not remove higher-order terms from the fit or total indices.
//! Reported order fractions can therefore sum to less than one.
//!
//! For the same fitted PCE, first- and total-order indices use the same
//! coefficient grouping as `sobol_indices_from_pce`. Pairwise indices and
//! per-order fractions provide additional detail. These describe the
//! fitted polynomial; truncation and fit errors affect their accuracy
//! for the original model.
//!
//! # References
//!
//! - [Xiu and Karniadakis (2002)](https://doi.org/10.1137/S1064827501387826),
//!   Table 4.1, identifies probability-matched orthogonal families.
//! - [Li, Rosenthal and Rabitz (2001)](https://doi.org/10.1021/jp010450t),
//!   "High Dimensional Model Representations," provides the HDMR background.
//! - [Sudret (2008)](https://doi.org/10.1016/j.ress.2007.04.002),
//!   §5.4, Eqs. 51 and 53, derives PCE-based indices.
//!   The present routine uses a full PCE regression, not a separate
//!   implementation of the component-fitting algorithms in the HDMR paper.

#![allow(clippy::similar_names, clippy::cast_precision_loss)]

use std::fmt;

use ndarray::{Array2, ArrayView2};
use salib_core::Problem;
use salib_surrogate::{fit_full_pce, norm_squared, PceError, PolynomialChaos, PolynomialFamily};
use thiserror::Error;

/// Result of PCE-based HDMR variance decomposition.
///
/// Contains the fitted PCE, first- and second-order Sobol' indices,
/// total-order indices, and per-interaction-order variance fractions.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct HdmrResult {
    /// Factor count `d`.
    pub dim: usize,
    /// Total output variance estimated from the PCE (unnormalized).
    pub total_variance: f64,
    /// Variance fraction per interaction order, normalized by total
    /// variance. `order_variance[0]` = sum of all first-order
    /// component variances / D, `order_variance[1]` = sum of all
    /// second-order / D, etc. Length = `max_order`.
    pub order_variance: Vec<f64>,
    /// First-order Sobol' indices `S_i`, length `d`.
    pub first_order: Vec<f64>,
    /// Second-order Sobol' indices. `second_order[i][k] = S2_{i, i+k+1}`
    /// (upper triangle, row-major). For `d = 3`:
    /// `second_order[0] = [S2_{0,1}, S2_{0,2}]`,
    /// `second_order[1] = [S2_{1,2}]`.
    pub second_order: Vec<Vec<f64>>,
    /// Total-order Sobol' indices `S_{T_i}`, length `d`.
    pub total_order: Vec<f64>,
    /// The fitted PCE (for inspection/reuse).
    pub pce: PolynomialChaos,
}

impl fmt::Display for HdmrResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "HDMR decomposition (d={})", self.dim)?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        for (order, frac) in self.order_variance.iter().enumerate() {
            writeln!(f, "  Order {} variance fraction: {:.4}", order + 1, frac)?;
        }
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S1", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..self.dim {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}",
                i, self.first_order[i], self.total_order[i]
            )?;
        }
        Ok(())
    }
}

/// Errors from [`estimate_hdmr`].
#[derive(Debug, Clone, Error)]
pub enum HdmrError {
    /// Total variance is zero or negative — model output is constant
    /// or the PCE fit collapsed.
    #[error("total variance is zero or negative")]
    ZeroVariance,
    /// PCE fit failed (delegated from `salib-surrogate`).
    #[error("PCE fit failed: {0}")]
    PceFitFailed(#[from] PceError),
    /// Factor distribution has no supported probability-matched
    /// polynomial family and canonical mapping (only Uniform/Normal supported).
    #[error("factor {index} has unsupported distribution for HDMR: {reason}")]
    UnsupportedDistribution {
        /// Zero-based factor index.
        index: usize,
        /// Explanation of why the distribution has no supported canonical mapping.
        reason: String,
    },
}

/// HDMR variance decomposition of a fitted PCE.
///
/// Fits a full polynomial chaos expansion to `(x, y)` data, then
/// decomposes the PCE coefficients by interaction order to produce
/// HDMR component variances and Sobol' indices.
///
/// # Arguments
///
/// * `x` — `N × d` sample matrix in the physical domain (each
///   factor's support).
/// * `y` — model output vector of length `N`.
/// * `problem` — defines factor distributions. The current mapping is
///   valid for independent Uniform and Normal factors only; see module docs.
/// * `max_order` — maximum order reported in `order_variance`.
///   Higher-order terms remain in the fit and total-order indices.
/// * `max_degree` — PCE polynomial truncation degree.
///
/// # Errors
///
/// - [`HdmrError::PceFitFailed`] — delegated from `fit_full_pce`, or
///   [`PceError::NonFiniteFit`] if variance contributions overflow.
/// - [`HdmrError::ZeroVariance`] — total PCE variance is zero.
/// - [`HdmrError::UnsupportedDistribution`] — factor is not Uniform or Normal.
pub fn estimate_hdmr(
    x: ArrayView2<'_, f64>,
    y: &[f64],
    problem: &Problem,
    max_order: usize,
    max_degree: usize,
) -> Result<HdmrResult, HdmrError> {
    let d = problem.dim();
    let n = x.nrows();

    if x.ncols() != d {
        return Err(PceError::FamiliesDimMismatch {
            families_len: d,
            d: x.ncols(),
        }
        .into());
    }
    if y.len() != n {
        return Err(PceError::ShapeMismatch {
            x_rows: n,
            y_len: y.len(),
        }
        .into());
    }

    // Choose only probability-matched families (Xiu/Karniadakis, Table 4.1).
    let mut families = Vec::with_capacity(d);
    for (j, f) in problem.factors().iter().enumerate() {
        match f.distribution {
            salib_core::Distribution::Normal { .. } => {
                families.push(PolynomialFamily::Hermite);
            }
            salib_core::Distribution::Uniform { .. } => {
                families.push(PolynomialFamily::Legendre);
            }
            _ => return Err(HdmrError::UnsupportedDistribution {
                index: j,
                reason:
                    "only Uniform and Normal factors have supported probability-matched mappings"
                        .into(),
            }),
        }
    }

    // Map physical inputs to canonical domain.
    let mut x_canonical = Array2::<f64>::zeros((n, d));
    for i in 0..n {
        for j in 0..d {
            x_canonical[[i, j]] = match families[j] {
                PolynomialFamily::Hermite => {
                    if let salib_core::Distribution::Normal { mu, sigma } =
                        problem.factors()[j].distribution
                    {
                        (x[[i, j]] - mu) / sigma
                    } else {
                        x[[i, j]]
                    }
                }
                _ => {
                    let (lo, hi) = problem.factors()[j].distribution.support();
                    2.0 * (x[[i, j]] - lo) / (hi - lo) - 1.0
                }
            };
        }
    }

    // Fit PCE.
    let pce = fit_full_pce(x_canonical.view(), y, &families, max_degree)?;

    // Per-basis-function variance contribution: β_α² · ∏_k ⟨Ψ_{α_k}, Ψ_{α_k}⟩
    let contributions: Vec<f64> = pce
        .multi_indices
        .iter()
        .zip(pce.coefficients.iter())
        .map(|(alpha, &beta)| {
            let mut ns = 1.0;
            for (k, &deg) in alpha.indices.iter().enumerate() {
                ns *= norm_squared(pce.families[k], deg);
            }
            beta * beta * ns
        })
        .collect();

    // Total variance = sum over all non-constant basis functions.
    let total_variance: f64 = pce
        .multi_indices
        .iter()
        .zip(contributions.iter())
        .filter(|(alpha, _)| !alpha.is_zero())
        .map(|(_, &c)| c)
        .sum();

    if !total_variance.is_finite() {
        return Err(PceError::NonFiniteFit.into());
    }
    if total_variance < 1e-15 {
        return Err(HdmrError::ZeroVariance);
    }

    // Accumulate by interaction order and by factor.
    let mut first_order = vec![0.0_f64; d];
    let mut total_order = vec![0.0_f64; d];
    let mut order_variance = vec![0.0_f64; max_order];
    let mut s2: Vec<Vec<f64>> = (0..d).map(|i| vec![0.0_f64; d - i - 1]).collect();

    for (alpha, &c) in pce.multi_indices.iter().zip(contributions.iter()) {
        if alpha.is_zero() {
            continue;
        }
        let active = alpha.active_factors();
        let order = active.len();

        // Accumulate order variance (capped at max_order).
        if order <= max_order {
            order_variance[order - 1] += c;
        }

        // First-order: exactly one active factor.
        if order == 1 {
            first_order[active[0]] += c;
        }

        // Second-order: exactly two active factors.
        if order == 2 {
            let (i, j) = (active[0], active[1]);
            s2[i][j - i - 1] += c;
        }

        // Total-order: every active factor gets the contribution.
        for &i in &active {
            total_order[i] += c;
        }
    }

    // Normalize by total variance.
    for v in &mut first_order {
        *v = (*v / total_variance).clamp(0.0, 1.0);
    }
    for v in &mut total_order {
        *v = (*v / total_variance).clamp(0.0, 1.0);
    }
    for row in &mut s2 {
        for v in row.iter_mut() {
            *v = (*v / total_variance).clamp(0.0, 1.0);
        }
    }
    for v in &mut order_variance {
        *v /= total_variance;
    }

    Ok(HdmrResult {
        dim: d,
        total_variance,
        order_variance,
        first_order,
        second_order: s2,
        total_order,
        pce,
    })
}
