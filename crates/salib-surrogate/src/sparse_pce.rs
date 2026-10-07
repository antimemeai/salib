//! Sparse polynomial chaos expansion using forward selection and OLS refits.
//!
//! Both solvers return [`PolynomialChaos`], so their fitted polynomials
//! can be passed to [`crate::pce::sobol_indices_from_pce`].
//!
//! # Solvers
//!
//! - [`SparseSolver::Omp`] selects the centered, unit-length column with
//!   the largest absolute inner product with the residual, then refits OLS.
//! - [`SparseSolver::Lars`] uses equiangular steps for selection and OLS
//!   refits for the returned coefficients. It centers the response and
//!   centers and normalizes predictors as in
//!   [Efron et al. (2004)](https://arxiv.org/pdf/math/0406456), Eq. 1.1.
//!
//! Both selection paths exclude constant predictor columns. OLS refits use
//! the original polynomial columns and include the intercept. LARS admits
//! tied correlations together at each knot. If a whole tie group would
//! exceed `max_terms`, it stops before that group rather than splitting it.
//!
//! Both keep the fit with the lowest observed PRESS score and stop
//! after three consecutive steps without improvement, or at the term
//! limit. This is a fixed-basis implementation. [Blatman and Sudret
//! (2011)](https://doi.org/10.1016/j.jcp.2010.12.021) also adjust degree and experimental
//! design and uses a corrected error criterion.
//!
//! # Hyperbolic truncation
//!
//! [`TruncationScheme::Hyperbolic`] retains multi-indices satisfying
//! `(Σ αⱼ^q)^(1/q) ≤ p`, for `0 < q ≤ 1`. Smaller `q` excludes more
//! interaction terms; useful interactions can also be excluded. See
//! Blatman and Sudret (2011), §3.2. For `q < 1` this is a quasi-norm.
//!
//! # PRESS score
//!
//! For a fixed OLS basis with nonsingular leave-one-out fits:
//!
//! ```text
//! LOO_err = (1/N) Σᵢ ((yᵢ - ŷᵢ) / (1 - hᵢᵢ))²
//! H = Ψ_A (Ψ_Aᵀ Ψ_A)⁻¹ Ψ_Aᵀ
//! ```
//!
//! If any row has `1-hᵢᵢ <= 1e-10` or nonfinite leverage, the entire
//! candidate receives an infinite score and cannot be selected as best.
//! If no candidate has a finite score, fitting returns an error.
//! The score has squared-output units. It is not corrected for basis
//! selection, normalized by output variance, or a held-out error estimate.
//!
//! # Cost and output
//!
//! At step `k`, correlation scans, OLS refits, and leverage calculations
//! cost `O(N P + N k² + k³)`, where `P` is the candidate basis size.
//! The candidate matrix occupies `O(N P)` memory. Returned coefficients
//! cover the full candidate basis; unselected terms have coefficient zero.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::many_single_char_names,
    clippy::too_many_lines,
    clippy::assigning_clones,
    clippy::manual_let_else,
    clippy::needless_range_loop
)]

use nalgebra::{DMatrix, DVector};
#[cfg(test)]
use ndarray::Array2;
use ndarray::ArrayView2;

use crate::multi_index::{enumerate_hyperbolic, enumerate_total_degree, MultiIndex};
use crate::pce::{validate_finite_data, PceError, PolynomialChaos};
use crate::polynomial::{evaluate, is_in_canonical_domain, PolynomialFamily};

/// Basis truncation scheme for sparse PCE.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TruncationScheme {
    /// `|α| = Σ αⱼ ≤ max_degree`.
    TotalDegree,
    /// Hyperbolic quasi-norm: `(Σ αⱼ^q)^{1/q} ≤ max_degree`,
    /// `q ∈ (0, 1]`. At `q = 1` reduces to total-degree; at `q < 1`
    /// favors low-interaction terms.
    Hyperbolic {
        /// Hyperbolic truncation exponent, in `(0,1]` for valid truncation.
        q: f64,
    },
}

/// Sparse-solver choice. See module docstring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum SparseSolver {
    /// Orthogonal Matching Pursuit.
    Omp,
    /// Equiangular selection with OLS refits. See module limitations.
    Lars,
}

/// Diagnostic carried through the fit. Not part of
/// [`PolynomialChaos`] so the existing analysis surface stays
/// untouched.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct SparseFitDiagnostic {
    /// Solver used.
    pub solver: SparseSolver,
    /// Truncation scheme used.
    pub truncation: TruncationScheme,
    /// Number of selected basis terms, including the constant term.
    pub num_active: usize,
    /// Total candidate basis size before sparse selection.
    pub candidate_basis_size: usize,
    /// Lowest finite PRESS score found during selection, in squared-output
    /// units. Near-unit leverage invalidates an entire candidate; see module docs.
    pub loo_error: f64,
    /// Step at which the minimum LOO was reached (zero-indexed).
    pub best_step: usize,
}

/// Fit a sparse PCE via the chosen forward-selection solver.
///
/// Same `samples_canonical` / `families` / `max_degree` contract as
/// [`crate::pce::fit_full_pce`]. `max_terms` caps the active set
/// size; if `None`, defaults to `min(N - 1, basis_size)`.
/// This leaves residual degrees of freedom but does not guarantee a
/// full-rank or well-conditioned design matrix. LARS stops before a tied
/// group that would exceed the cap, so it can return fewer terms.
///
/// # Errors
///
/// - [`PceError::ShapeMismatch`] / [`PceError::ZeroD`] /
///   [`PceError::FamiliesDimMismatch`] — input shape errors.
/// - [`PceError::InsufficientSamples`] if `N` is too small to fit
///   even a constant via OLS (`N < 2`).
/// - [`PceError::SingularDesignMatrix`] if the active-set Gram matrix
///   becomes singular (collinearity in the chosen subset).
/// - [`PceError::UndefinedLooError`] if no candidate has a finite PRESS score.
/// - [`PceError::NonFiniteInput`] if a sample or response is not finite.
/// - [`PceError::NonFiniteFit`] if basis evaluation or fitting overflows.
pub fn fit_sparse_pce(
    samples_canonical: ArrayView2<'_, f64>,
    y: &[f64],
    families: &[PolynomialFamily],
    max_degree: usize,
    truncation: TruncationScheme,
    solver: SparseSolver,
    max_terms: Option<usize>,
) -> Result<(PolynomialChaos, SparseFitDiagnostic), PceError> {
    let n = samples_canonical.nrows();
    let d = samples_canonical.ncols();
    if d == 0 {
        return Err(PceError::ZeroD);
    }
    if y.len() != n {
        return Err(PceError::ShapeMismatch {
            x_rows: n,
            y_len: y.len(),
        });
    }
    if families.len() != d {
        return Err(PceError::FamiliesDimMismatch {
            families_len: families.len(),
            d,
        });
    }
    if n < 2 {
        return Err(PceError::InsufficientSamples { n, basis_size: 1 });
    }

    validate_finite_data(samples_canonical, y)?;

    // Caller-side canonical-domain debug-assert (mirrors fit_full_pce).
    #[cfg(debug_assertions)]
    {
        for (k, family) in families.iter().enumerate() {
            let xs = samples_canonical.column(k);
            let in_domain = xs.iter().all(|&x| is_in_canonical_domain(*family, x));
            debug_assert!(
                in_domain,
                "sparse PCE: column {k} contains values outside {family:?}'s canonical domain"
            );
        }
    }

    // Build the candidate basis per truncation scheme.
    let multi_indices: Vec<MultiIndex> = match truncation {
        TruncationScheme::TotalDegree => {
            enumerate_total_degree(d, max_degree).map_err(|_| PceError::ZeroD)?
        }
        TruncationScheme::Hyperbolic { q } => {
            enumerate_hyperbolic(d, max_degree, q).map_err(|_| PceError::ZeroD)?
        }
    };
    let basis_size = multi_indices.len();

    // Build the full basis matrix Ψ ∈ R^{N × P}. Sparse solvers will
    // select columns from this; we materialize once.
    let mut psi = DMatrix::<f64>::zeros(n, basis_size);
    for i in 0..n {
        for (j, alpha) in multi_indices.iter().enumerate() {
            let mut value = 1.0;
            for (k, &deg) in alpha.indices.iter().enumerate() {
                value *= evaluate(families[k], deg, samples_canonical[[i, k]]);
            }
            psi[(i, j)] = value;
        }
    }
    if psi.iter().any(|value| !value.is_finite()) {
        return Err(PceError::NonFiniteFit);
    }
    let y_vec = DVector::from_iterator(n, y.iter().copied());

    let max_terms_eff = max_terms
        .unwrap_or(usize::MAX)
        .min(basis_size)
        .min(n.saturating_sub(1));
    if max_terms_eff == 0 {
        return Err(PceError::InsufficientSamples { n, basis_size });
    }

    let (active, coefficients_active, diag) = match solver {
        SparseSolver::Omp => omp_forward_select(&psi, &y_vec, max_terms_eff)?,
        SparseSolver::Lars => lars_forward_select(&psi, &y_vec, max_terms_eff)?,
    };

    // Scatter active coefficients back to the full basis.
    let mut coefficients = vec![0.0_f64; basis_size];
    for (idx, &j) in active.iter().enumerate() {
        coefficients[j] = coefficients_active[idx];
    }

    let pce = PolynomialChaos {
        coefficients,
        multi_indices,
        families: families.to_vec(),
        max_degree,
    };

    let diagnostic = SparseFitDiagnostic {
        solver,
        truncation,
        num_active: active.len(),
        candidate_basis_size: basis_size,
        loo_error: diag.best_loo,
        best_step: diag.best_step,
    };

    Ok((pce, diagnostic))
}

struct ForwardSelectDiag {
    best_loo: f64,
    best_step: usize,
}

/// OMP forward selection. Returns the active-column indices (into
/// `psi`'s columns), their OLS coefficients, and the diagnostic.
fn omp_forward_select(
    psi: &DMatrix<f64>,
    y: &DVector<f64>,
    max_terms: usize,
) -> Result<(Vec<usize>, DVector<f64>, ForwardSelectDiag), PceError> {
    let standardized = centered_unit_predictors(psi)?;

    // The constant column (index 0 in our enumeration) is always
    // active — every PCE has a mean term. Initialize there.
    let mut active: Vec<usize> = vec![0];
    let mut best_loo = f64::INFINITY;
    let mut best_step = 0;
    let mut best_active: Vec<usize> = active.clone();
    let mut best_beta: DVector<f64> = DVector::zeros(1);

    let mut consecutive_increases = 0_usize;

    for step in 0..max_terms {
        // Refit OLS on the current active set.
        let (beta, hat_diag) = refit_active_ols(psi, y, &active)?;
        let loo = loo_error_from_hat(psi, y, &active, &beta, &hat_diag);

        if loo < best_loo {
            best_loo = loo;
            best_step = step;
            best_active = active.clone();
            best_beta = beta.clone();
            consecutive_increases = 0;
        } else {
            consecutive_increases += 1;
        }

        // Stop if LOO has been increasing for several consecutive
        // steps — local noise can cause one-step bumps; require a
        // sustained trend before giving up.
        if consecutive_increases >= 3 {
            break;
        }

        // Compute residual using current OLS fit.
        let psi_a = active_columns(psi, &active);
        let residual = y - &psi_a * beta;

        if active.len() >= max_terms {
            break;
        }

        // Pick the inactive column most correlated with residual.
        let next_idx = match best_inactive_correlation(&standardized, &residual, &active) {
            Some(j) => j,
            None => break,
        };
        active.push(next_idx);

        // Guard against numerical underflow on residual norm.
        if residual.norm() < 1e-14 * y.norm().max(1.0) {
            // Fit is already exact on training data — finalize and stop.
            let (beta, hat_diag) = refit_active_ols(psi, y, &active)?;
            let loo = loo_error_from_hat(psi, y, &active, &beta, &hat_diag);
            if loo < best_loo {
                best_loo = loo;
                best_step = step + 1;
                best_active = active.clone();
                best_beta = beta;
            }
            break;
        }
    }

    if !best_loo.is_finite() {
        return Err(PceError::UndefinedLooError);
    }
    Ok((
        best_active,
        best_beta,
        ForwardSelectDiag {
            best_loo,
            best_step,
        },
    ))
}

/// LARS forward selection (Efron 2004 § 2). Returns the active set,
/// LARS-OLS-hybrid coefficients (refit OLS on the final active set),
/// and the diagnostic.
///
/// Efron 2004 eq. (1.1) standardizes columns to unit `ℓ²` norm and
/// centers both the response and predictor columns. OLS refits use
/// the original columns and a constant term, so
/// the returned coefficients retain the polynomial family's scale.
fn lars_forward_select(
    psi: &DMatrix<f64>,
    y: &DVector<f64>,
    max_terms: usize,
) -> Result<(Vec<usize>, DVector<f64>, ForwardSelectDiag), PceError> {
    let n = psi.nrows();
    let p = psi.ncols();

    // ── Standardize for LARS path-following ──────────────────────
    //
    // Center y. Drop the constant column (j = 0) from the LARS
    // selection pool — its job is captured by the y-centering. The
    // OLS-hybrid refit on the un-standardized columns will pull the
    // mean back in via the constant column, which we manually keep
    // in the active set throughout.
    #[allow(clippy::cast_precision_loss)]
    let y_mean = y.iter().sum::<f64>() / n as f64;
    let y_centered = y.map(|v| v - y_mean);

    let standardized = centered_unit_predictors(psi)?;

    // The constant column always lives in `active` (un-standardized
    // pool index 0); the LARS path operates on standardized columns
    // 1..p only. We track active selections in the un-standardized
    // index space.
    let mut active: Vec<usize> = vec![0];
    // LARS-internal active set (subset of {1..p}, in standardized
    // column space). Distinct from `active` because `active`
    // includes the constant.
    let mut active_lars: Vec<usize> = Vec::new();

    // μ̂ is the LARS path estimate of y_centered, in standardized
    // column space.
    let mut mu = DVector::<f64>::zeros(n);

    let mut best_loo = f64::INFINITY;
    let mut best_step = 0;
    let mut best_active: Vec<usize> = active.clone();
    let mut best_beta: DVector<f64> = {
        let psi_a = active_columns(psi, &active);
        solve_ols(&psi_a, y)?
    };

    let mut consecutive_increases = 0_usize;

    for step in 0..max_terms {
        // Evaluate LARS-OLS hybrid: refit OLS on the
        // un-standardized current active set (constant + LARS picks).
        let (beta, hat_diag) = refit_active_ols(psi, y, &active)?;
        let loo = loo_error_from_hat(psi, y, &active, &beta, &hat_diag);
        if loo < best_loo {
            best_loo = loo;
            best_step = step;
            best_active = active.clone();
            best_beta = beta;
            consecutive_increases = 0;
        } else {
            consecutive_increases += 1;
        }
        if consecutive_increases >= 3 {
            break;
        }
        // `active_lars.len() + 1 >= p` rather than `>= p - 1` to
        // avoid underflow if `p == 0` (unreachable in practice but
        // belt-and-suspenders).
        if active.len() >= max_terms || active_lars.len() + 1 >= p {
            break;
        }

        // Compute current standardized correlations c = Ψ̃ᵀ(ỹ - μ̂).
        let residual = &y_centered - &mu;
        // c[j] for j ≥ 1 is the standardized correlation (j = 0 is
        // skipped). We allocate a length-p vector and zero out j = 0.
        let mut c = DVector::<f64>::zeros(p);
        for j in 1..p {
            c[j] = standardized.column(j).dot(&residual);
        }

        // At each knot, admit all maximally correlated inactive columns
        // together (Efron Eq. 2.9). Ignoring a zero-length joining step
        // lets a tied column lag behind and changes the following knot.
        let active_lars_set: std::collections::HashSet<usize> =
            active_lars.iter().copied().collect();
        let max_inactive = (1..p)
            .filter(|j| !active_lars_set.contains(j))
            .map(|j| c[j].abs())
            .filter(|value| value.is_finite())
            .fold(0.0_f64, f64::max);
        if max_inactive <= 1e-14 {
            break;
        }
        let tie_tolerance = 64.0 * f64::EPSILON * max_inactive;
        let joining: Vec<usize> = (1..p)
            .filter(|j| !active_lars_set.contains(j))
            .filter(|&j| (c[j].abs() - max_inactive).abs() <= tie_tolerance)
            .collect();
        // Splitting a tie merely because of a term cap would make the
        // fit depend on factor order. Retain the previous best instead.
        if joining.len() > max_terms - active.len() {
            break;
        }
        active_lars.extend(joining.iter().copied());
        active.extend(joining);

        // Build sign-flipped standardized active matrix X̃_A.
        // s_j = sign(c[j]) from the correlations at the joining knot.
        let signs: Vec<f64> = active_lars
            .iter()
            .map(|&j| if c[j] >= 0.0 { 1.0 } else { -1.0 })
            .collect();
        let mut psi_a_std_signed = DMatrix::<f64>::zeros(n, active_lars.len());
        for (col, (&j, &s)) in active_lars.iter().zip(signs.iter()).enumerate() {
            let v = standardized.column(j).clone_owned() * s;
            psi_a_std_signed.set_column(col, &v);
        }

        // Gram G_A = X̃_Aᵀ X̃_A.
        let g_a = psi_a_std_signed.transpose() * &psi_a_std_signed;
        let cholesky = match g_a.clone().cholesky() {
            Some(c) => c,
            None => return Err(PceError::SingularDesignMatrix),
        };
        let ones = DVector::<f64>::from_element(active_lars.len(), 1.0);
        let g_inv_ones = cholesky.solve(&ones);
        let denom = ones.dot(&g_inv_ones);
        if !(denom.is_finite() && denom > 1e-15) {
            return Err(PceError::SingularDesignMatrix);
        }
        let a_a = denom.powf(-0.5); // Efron eq. 2.5
        let w_a = a_a * &g_inv_ones; // eq. 2.6
        let u_a = &psi_a_std_signed * &w_a; // eq. 2.6

        // a = X̃ᵀ u_A — but only j ∉ active_lars matters for the step.
        // Recompute c_max (the active-set absolute correlation) from
        // the LARS-internal pool after admitting the joining group.
        let c_max: f64 = active_lars
            .iter()
            .map(|&j| c[j].abs())
            .fold(0.0_f64, f64::max);

        let mut gamma_hat = f64::INFINITY;
        let active_lars_set2: std::collections::HashSet<usize> =
            active_lars.iter().copied().collect();
        for j in 1..p {
            if active_lars_set2.contains(&j) {
                continue;
            }
            let aj = standardized.column(j).dot(&u_a);
            for &candidate in &[(c_max - c[j]) / (a_a - aj), (c_max + c[j]) / (a_a + aj)] {
                if candidate > 1e-12 && candidate < gamma_hat {
                    gamma_hat = candidate;
                }
            }
        }
        // Last-step fallback: if no inactive column gives a finite
        // positive γ̂ (we've effectively exhausted), use the OLS
        // distance |c_max / A_A| to walk fully along u_A — Efron's
        // "final" γ̂ that drives μ̂ to the LS fit on the active set.
        if !gamma_hat.is_finite() {
            gamma_hat = c_max / a_a;
        }

        // Update μ̂ along the equiangular direction by γ̂ · u_A.
        mu += gamma_hat * &u_a;
    }

    if !best_loo.is_finite() {
        return Err(PceError::UndefinedLooError);
    }
    Ok((
        best_active,
        best_beta,
        ForwardSelectDiag {
            best_loo,
            best_step,
        },
    ))
}

/// Efron Eq. 1.1: center predictors and scale to unit Euclidean length.
/// The intercept and degenerate constant predictors remain zero columns.
/// OLS continues to use the original matrix so polynomial coefficients
/// retain their original units.
fn centered_unit_predictors(psi: &DMatrix<f64>) -> Result<DMatrix<f64>, PceError> {
    let mut standardized = DMatrix::zeros(psi.nrows(), psi.ncols());
    for j in 1..psi.ncols() {
        let column = psi.column(j);
        let mean = column.iter().sum::<f64>() / psi.nrows() as f64;
        let centered = column.map(|value| value - mean);
        let norm = centered.norm();
        if !mean.is_finite() || !norm.is_finite() {
            return Err(PceError::NonFiniteFit);
        }
        if norm > 0.0 {
            standardized.set_column(j, &(centered / norm));
        }
    }
    Ok(standardized)
}

/// Stack the columns of `psi` indexed by `active` into a dense
/// `(N, |A|)` matrix.
fn active_columns(psi: &DMatrix<f64>, active: &[usize]) -> DMatrix<f64> {
    let mut out = DMatrix::<f64>::zeros(psi.nrows(), active.len());
    for (col, &j) in active.iter().enumerate() {
        out.set_column(col, &psi.column(j).clone_owned());
    }
    out
}

/// OLS via Cholesky on the normal equations.
fn solve_ols(psi_a: &DMatrix<f64>, y: &DVector<f64>) -> Result<DVector<f64>, PceError> {
    let xtx = psi_a.transpose() * psi_a;
    let xty = psi_a.transpose() * y;
    if xtx.iter().chain(xty.iter()).any(|value| !value.is_finite()) {
        return Err(PceError::NonFiniteFit);
    }
    let cholesky = xtx.cholesky().ok_or(PceError::SingularDesignMatrix)?;
    let beta = cholesky.solve(&xty);
    if beta.iter().any(|value| !value.is_finite()) {
        return Err(PceError::NonFiniteFit);
    }
    Ok(beta)
}

/// OLS coefficients + hat-matrix diagonal for the active set. The
/// hat-matrix diagonal is `hᵢᵢ = ψᵢᵀ (Ψ_Aᵀ Ψ_A)⁻¹ ψᵢ` where `ψᵢ`
/// is the i-th row of `Ψ_A`.
fn refit_active_ols(
    psi: &DMatrix<f64>,
    y: &DVector<f64>,
    active: &[usize],
) -> Result<(DVector<f64>, Vec<f64>), PceError> {
    let psi_a = active_columns(psi, active);
    let xtx = psi_a.transpose() * &psi_a;
    let xty = psi_a.transpose() * y;
    if xtx.iter().chain(xty.iter()).any(|value| !value.is_finite()) {
        return Err(PceError::NonFiniteFit);
    }
    let cholesky = xtx.cholesky().ok_or(PceError::SingularDesignMatrix)?;
    let beta = cholesky.solve(&xty);
    if beta.iter().any(|value| !value.is_finite()) {
        return Err(PceError::NonFiniteFit);
    }

    // Hat-matrix diagonal: h_ii = ψᵢᵀ (XᵀX)⁻¹ ψᵢ. Per row, solve
    // (XᵀX) z = ψᵢ, then h_ii = ψᵢ · z.
    let mut hat_diag = Vec::with_capacity(psi_a.nrows());
    for i in 0..psi_a.nrows() {
        let row = psi_a.row(i).transpose().clone_owned();
        let z = cholesky.solve(&row);
        hat_diag.push(row.dot(&z));
    }
    Ok((beta, hat_diag))
}

/// Allen's PRESS statistic — closed-form leave-one-out CV error.
/// `LOO = (1/N) · Σ ((yᵢ - ŷᵢ) / (1 - hᵢᵢ))²`.
fn loo_error_from_hat(
    psi: &DMatrix<f64>,
    y: &DVector<f64>,
    active: &[usize],
    beta: &DVector<f64>,
    hat_diag: &[f64],
) -> f64 {
    let psi_a = active_columns(psi, active);
    let yhat = &psi_a * beta;
    let n = y.len();
    let mut acc = 0.0_f64;
    for i in 0..n {
        let denom = 1.0 - hat_diag[i];
        // Removing a leverage-one row leaves a singular OLS design.
        // Penalize the whole candidate, rather than hiding that row.
        if !denom.is_finite() || denom <= 1e-10 {
            return f64::INFINITY;
        }
        let resid = y[i] - yhat[i];
        acc += (resid / denom).powi(2);
    }
    let error = acc / n as f64;
    if error.is_finite() {
        error
    } else {
        f64::INFINITY
    }
}

/// Pick the inactive column index with the largest absolute inner
/// product against `residual`. Returns `None` if every inactive
/// column is numerically uncorrelated.
fn best_inactive_correlation(
    psi: &DMatrix<f64>,
    residual: &DVector<f64>,
    active: &[usize],
) -> Option<usize> {
    let p = psi.ncols();
    let active_set: std::collections::HashSet<usize> = active.iter().copied().collect();
    let mut best: Option<(usize, f64)> = None;
    for j in 0..p {
        if active_set.contains(&j) {
            continue;
        }
        let col = psi.column(j);
        let dot = col.dot(residual).abs();
        if dot.is_finite() && best.is_none_or(|(_, prev)| dot > prev) {
            best = Some((j, dot));
        }
    }
    best.and_then(|(j, v)| if v > 1e-14 { Some(j) } else { None })
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {
    use super::*;

    fn linspace_unit_to_canonical(n: usize, d: usize) -> Array2<f64> {
        let mut x = Array2::<f64>::zeros((n, d));
        for j in 0..d {
            let mut perm: Vec<usize> = (0..n).collect();
            let mut state: u64 = 0x9E37_79B9_7F4A_7C15_u64.wrapping_mul((j as u64).wrapping_add(1));
            for i in (1..n).rev() {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1);
                #[allow(clippy::cast_possible_truncation)]
                let k = (state >> 33) as usize % (i + 1);
                perm.swap(i, k);
            }
            for i in 0..n {
                let unit = (perm[i] as f64 + 0.5) / (n as f64);
                x[[i, j]] = 2.0 * unit - 1.0;
            }
        }
        x
    }

    // Efron et al. (2004), Eqs. 1.1 and 2.13: for orthogonal unit
    // predictors and y=3a+2b+0.5c, correlations are sqrt(8)*(3,2,0.5).
    // The first knot is gamma=(3-2)*sqrt(8), where b joins a.
    // Thus the first two nonconstant columns must be a, then b,
    // regardless of the predictors' means or units. OLS retains raw units.
    fn orthogonal_path_fixture() -> (DMatrix<f64>, DVector<f64>) {
        let a = [1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        let b = [1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0];
        let c = [1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0];
        let psi = DMatrix::from_fn(8, 4, |i, j| match j {
            0 => 1.0,
            1 => 20.0 + a[i],
            2 => b[i],
            _ => c[i],
        });
        let y = DVector::from_fn(8, |i, _| 3.0 * a[i] + 2.0 * b[i] + 0.5 * c[i]);
        (psi, y)
    }

    #[test]
    fn lars_path_matches_orthogonal_analytic_knots() {
        let (psi, y) = orthogonal_path_fixture();
        let (active, beta, _) = lars_forward_select(&psi, &y, 2).unwrap();
        assert_eq!(active, vec![0, 1]);
        assert!((beta[0] + 60.0).abs() < 1e-10);
        assert!((beta[1] - 3.0).abs() < 1e-10);
        let (active, beta, _) = lars_forward_select(&psi, &y, 3).unwrap();
        assert_eq!(active, vec![0, 1, 2]);
        assert!((beta[2] - 2.0).abs() < 1e-10);
        let mut transformed = psi.clone();
        for i in 0..8 {
            transformed[(i, 1)] = -2.0 * psi[(i, 1)] + 7.0;
            transformed[(i, 2)] = 0.25 * psi[(i, 2)] - 3.0;
        }
        let (changed, _, _) = lars_forward_select(&transformed, &y, 3).unwrap();
        assert_eq!(changed, active);
    }

    #[test]
    fn lars_tied_knots_admit_all_maximal_correlations_before_moving() {
        // Efron Eq.2.9 admits both a,b at the initial knot. Their bisector
        // has z projection .32/sqrt(2), vs A=1/sqrt(2). In units of the
        // common a,b coefficient increment, z joins at (1-c_z)/.68 ~.3925.
        // Its joining correlation ~.6075 exceeds |c_c|=.43, so z joins first.
        let walsh = |i: usize, j: usize| if (i >> j) & 1 == 0 { 1.0 } else { -1.0 };
        let zcoef = (1.0_f64 - 0.36_f64.powi(2) - 0.04_f64.powi(2) - 0.65_f64.powi(2)).sqrt();
        let fixture = Array2::from_shape_fn((64, 4), |(i, j)| {
            if j < 3 {
                walsh(i, j)
            } else {
                (0.36 * walsh(i, 0) - 0.04 * walsh(i, 1) - 0.65 * walsh(i, 2) - zcoef * walsh(i, 3))
                    / 2.0
            }
        });
        let y: Vec<_> = (0..64)
            .map(|i| walsh(i, 0) + walsh(i, 1) - 0.43 * walsh(i, 2) - 0.2 * walsh(i, 3))
            .collect();
        let fit = |permutation: [usize; 4]| {
            let x = Array2::from_shape_fn((64, 4), |(i, j)| fixture[[i, permutation[j]]]);
            let (pce, diagnostic) = fit_sparse_pce(
                x.view(),
                &y,
                &[PolynomialFamily::Legendre; 4],
                1,
                TruncationScheme::TotalDegree,
                SparseSolver::Lars,
                Some(4),
            )
            .unwrap();
            let mut support: Vec<_> = pce
                .multi_indices
                .iter()
                .zip(&pce.coefficients)
                .filter(|(alpha, coefficient)| !alpha.is_zero() && coefficient.abs() > 1e-9)
                .map(|(alpha, _)| permutation[alpha.active_factors()[0]])
                .collect();
            support.sort_unstable();
            assert_eq!(support, vec![0, 1, 3]);
            assert_eq!(diagnostic.num_active, 4);
            (pce, diagnostic, permutation)
        };
        let (original, diagnostic, _) = fit([0, 1, 2, 3]);
        let (permuted, changed, permutation) = fit([1, 0, 2, 3]);
        assert!((diagnostic.loo_error - changed.loo_error).abs() < 1e-12);
        for row in fixture.rows() {
            let physical = row.to_vec();
            let reordered: Vec<_> = permutation.iter().map(|&j| physical[j]).collect();
            assert!((original.evaluate(&physical) - permuted.evaluate(&reordered)).abs() < 1e-12);
        }
    }

    #[test]
    fn lars_stops_before_tie_group_that_exceeds_term_limit() {
        let (psi, _) = orthogonal_path_fixture();
        let y = DVector::from_fn(8, |i, _| psi[(i, 1)] - 20.0 + psi[(i, 2)]);
        let (active, _, _) = lars_forward_select(&psi, &y, 2).unwrap();
        // Splitting the a,b tie would choose an arbitrary basis coordinate.
        assert_eq!(active, vec![0]);
    }

    #[test]
    fn lars_correlated_predictor_path_matches_equiangular_oracle() {
        // a,b,c are orthogonal mean-zero +/-1 columns; the second predictor
        // is z=0.8a+0.6b, and y=3a+0.5b+c. Unit-length correlations are
        // sqrt(N)*(3,2.7,1). Eq.2.13 gives first knot gamma=1.5sqrt(N):
        // z joins a before c. Greedy OMP fits a fully, leaving correlations
        // proportional to (0.3,1), so it chooses c instead of z.
        // Replication keeps PRESS from rejecting the analytically chosen
        // two-predictor model solely for its extra leverage.
        let (base, _) = orthogonal_path_fixture();
        let psi = DMatrix::from_fn(32, 4, |i, j| {
            let a = base[(i % 8, 1)] - 20.0;
            let b = base[(i % 8, 2)];
            let c = base[(i % 8, 3)];
            match j {
                0 => 1.0,
                1 => 10.0 + a,
                2 => 5.0 + 0.8 * a + 0.6 * b,
                _ => -3.0 + c,
            }
        });
        let y = DVector::from_fn(32, |i, _| {
            3.0 * (base[(i % 8, 1)] - 20.0) + 0.5 * base[(i % 8, 2)] + base[(i % 8, 3)]
        });
        let (active, beta, _) = lars_forward_select(&psi, &y, 3).unwrap();
        assert_eq!(active, vec![0, 1, 2]);
        assert!((beta[1] - 7.0 / 3.0).abs() < 1e-10);
        assert!((beta[2] - 5.0 / 6.0).abs() < 1e-10);
        let (omp_active, _, _) = omp_forward_select(&psi, &y, 3).unwrap();
        assert_eq!(omp_active, vec![0, 1, 3]);
    }

    #[test]
    fn omp_selection_is_invariant_to_column_units() {
        let (mut psi, y) = orthogonal_path_fixture();
        let (active, _, _) = omp_forward_select(&psi, &y, 2).unwrap();
        assert_eq!(active, vec![0, 1]);
        for i in 0..8 {
            psi[(i, 2)] = 100.0 * psi[(i, 2)] + 10.0;
        }
        let (changed, _, _) = omp_forward_select(&psi, &y, 2).unwrap();
        assert_eq!(changed, active);
    }

    #[test]
    fn sparse_fit_rejects_nonfinite_inputs_and_outputs() {
        for solver in [SparseSolver::Omp, SparseSolver::Lars] {
            let mut x = linspace_unit_to_canonical(8, 1);
            assert!(matches!(
                fit_sparse_pce(
                    x.view(),
                    &[f64::NAN; 8],
                    &[PolynomialFamily::Legendre],
                    1,
                    TruncationScheme::TotalDegree,
                    solver,
                    None
                ),
                Err(PceError::NonFiniteInput)
            ));
            x[[0, 0]] = f64::INFINITY;
            assert!(matches!(
                fit_sparse_pce(
                    x.view(),
                    &[1.0; 8],
                    &[PolynomialFamily::Legendre],
                    1,
                    TruncationScheme::TotalDegree,
                    solver,
                    None
                ),
                Err(PceError::NonFiniteInput)
            ));
        }
    }

    #[test]
    fn sparse_fit_errors_when_every_press_candidate_is_invalid() {
        let x = linspace_unit_to_canonical(8, 1);
        // Finite data, but every candidate's squared prediction error overflows.
        let y: Vec<_> = (0..8)
            .map(|i| if i & 1 == 0 { 1e200 } else { -1e200 })
            .collect();
        for solver in [SparseSolver::Omp, SparseSolver::Lars] {
            assert!(matches!(
                fit_sparse_pce(
                    x.view(),
                    &y,
                    &[PolynomialFamily::Legendre],
                    1,
                    TruncationScheme::TotalDegree,
                    solver,
                    None
                ),
                Err(PceError::UndefinedLooError)
            ));
        }
    }

    #[test]
    fn press_matches_explicit_leave_one_out_refits() {
        let psi = DMatrix::from_fn(6, 2, |i, j| if j == 0 { 1.0 } else { i as f64 });
        let y = DVector::from_vec(vec![0.0, 0.5, 3.0, 2.0, 4.5, 7.0]);
        let active = [0, 1];
        let (beta, hat) = refit_active_ols(&psi, &y, &active).unwrap();
        let press = loo_error_from_hat(&psi, &y, &active, &beta, &hat);
        let mut deletion_error = 0.0;
        for omitted in 0..6 {
            let rows: Vec<_> = (0..6).filter(|&i| i != omitted).collect();
            let training = DMatrix::from_fn(5, 2, |i, j| psi[(rows[i], j)]);
            let response = DVector::from_fn(5, |i, _| y[rows[i]]);
            let fit = solve_ols(&training, &response).unwrap();
            let prediction = psi.row(omitted).transpose().dot(&fit);
            deletion_error += (y[omitted] - prediction).powi(2);
        }
        assert!((press - deletion_error / 6.0).abs() < 1e-12);
    }

    #[test]
    fn press_invalidates_whole_candidate_when_one_deletion_is_singular() {
        // Leaving out row 0 destroys the only observation of predictor 1.
        // The other three deletion fits are valid; omitting only row 0 from
        // PRESS would conceal this undefined part of the full diagnostic.
        let psi = DMatrix::from_row_slice(4, 2, &[1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
        let y = DVector::from_vec(vec![4.0, 0.0, 1.0, 2.0]);
        let (beta, hat) = refit_active_ols(&psi, &y, &[0, 1]).unwrap();
        assert!((hat[0] - 1.0).abs() < 1e-12);
        let deletion = psi.rows(1, 3).into_owned();
        assert!(solve_ols(&deletion, &y.rows(1, 3).into_owned()).is_err());
        assert_eq!(
            loo_error_from_hat(&psi, &y, &[0, 1], &beta, &hat),
            f64::INFINITY
        );
    }

    // ── Validation ────────────────────────────────────────────────

    #[test]
    fn omp_zero_d_errors() {
        let x = Array2::<f64>::zeros((10, 0));
        let y = vec![0.0; 10];
        let err = fit_sparse_pce(
            x.view(),
            &y,
            &[],
            3,
            TruncationScheme::TotalDegree,
            SparseSolver::Omp,
            None,
        )
        .unwrap_err();
        assert_eq!(err, PceError::ZeroD);
    }

    #[test]
    fn lars_shape_mismatch_errors() {
        let x = Array2::<f64>::zeros((10, 3));
        let y = vec![0.0; 5];
        let err = fit_sparse_pce(
            x.view(),
            &y,
            &[PolynomialFamily::Legendre; 3],
            3,
            TruncationScheme::TotalDegree,
            SparseSolver::Lars,
            None,
        )
        .unwrap_err();
        assert!(matches!(err, PceError::ShapeMismatch { .. }));
    }

    // ── OMP recovery on closed-form polynomial models ────────────

    #[test]
    fn omp_fits_constant() {
        let n = 64;
        let x = linspace_unit_to_canonical(n, 2);
        let y = vec![5.0; n];
        let (pce, _) = fit_sparse_pce(
            x.view(),
            &y,
            &[PolynomialFamily::Legendre; 2],
            3,
            TruncationScheme::TotalDegree,
            SparseSolver::Omp,
            None,
        )
        .unwrap();
        assert!((pce.mean() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn omp_picks_out_sparse_additive_active_factors() {
        // Y = ξ_0 + 0.5·ξ_2 + 2·ξ_4 on d = 5. Sparse PCE should pick
        // only the (1,0,0,0,0), (0,0,1,0,0), (0,0,0,0,1) main-effects.
        let n = 256;
        let x = linspace_unit_to_canonical(n, 5);
        let y: Vec<f64> = (0..n)
            .map(|i| x[[i, 0]] + 0.5 * x[[i, 2]] + 2.0 * x[[i, 4]])
            .collect();
        let (pce, diag) = fit_sparse_pce(
            x.view(),
            &y,
            &[PolynomialFamily::Legendre; 5],
            4,
            TruncationScheme::TotalDegree,
            SparseSolver::Omp,
            None,
        )
        .unwrap();
        // Coefficient at (1,0,0,0,0) should be 1.0; at (0,0,1,0,0)
        // should be 0.5; at (0,0,0,0,1) should be 2.0.
        let find = |target: Vec<usize>| {
            pce.multi_indices
                .iter()
                .position(|a| a.indices == target)
                .map(|i| pce.coefficients[i])
                .unwrap()
        };
        assert!((find(vec![1, 0, 0, 0, 0]) - 1.0).abs() < 1e-3);
        assert!((find(vec![0, 0, 1, 0, 0]) - 0.5).abs() < 1e-3);
        assert!((find(vec![0, 0, 0, 0, 1]) - 2.0).abs() < 1e-3);
        // Sparse: total active terms way under the 126-term basis.
        assert!(
            diag.num_active <= 10,
            "OMP kept {} terms, expected ≤ 10",
            diag.num_active
        );
    }

    // ── LARS recovery ────────────────────────────────────────────

    #[test]
    fn lars_picks_out_sparse_additive_active_factors() {
        let n = 256;
        let x = linspace_unit_to_canonical(n, 5);
        let y: Vec<f64> = (0..n)
            .map(|i| x[[i, 0]] + 0.5 * x[[i, 2]] + 2.0 * x[[i, 4]])
            .collect();
        let (pce, diag) = fit_sparse_pce(
            x.view(),
            &y,
            &[PolynomialFamily::Legendre; 5],
            4,
            TruncationScheme::TotalDegree,
            SparseSolver::Lars,
            None,
        )
        .unwrap();
        let find = |target: Vec<usize>| {
            pce.multi_indices
                .iter()
                .position(|a| a.indices == target)
                .map(|i| pce.coefficients[i])
                .unwrap()
        };
        // Coefficients should be near closed-form.
        assert!((find(vec![1, 0, 0, 0, 0]) - 1.0).abs() < 1e-3);
        assert!((find(vec![0, 0, 1, 0, 0]) - 0.5).abs() < 1e-3);
        assert!((find(vec![0, 0, 0, 0, 1]) - 2.0).abs() < 1e-3);
        // LARS may keep a couple more terms than OMP near the LOO
        // minimum (equiangular movement is gentler than greedy).
        assert!(diag.num_active <= 15, "LARS kept {} terms", diag.num_active);
    }

    // ── Hyperbolic truncation ────────────────────────────────────

    #[test]
    fn hyperbolic_truncation_used_at_q_0p75() {
        let n = 256;
        let x = linspace_unit_to_canonical(n, 5);
        let y: Vec<f64> = (0..n).map(|i| x[[i, 0]] + 2.0 * x[[i, 4]]).collect();
        let (_pce, diag) = fit_sparse_pce(
            x.view(),
            &y,
            &[PolynomialFamily::Legendre; 5],
            4,
            TruncationScheme::Hyperbolic { q: 0.75 },
            SparseSolver::Omp,
            None,
        )
        .unwrap();
        // Hyperbolic at d=5, p=4, q=0.75 should be substantially
        // smaller than total-degree (126).
        assert!(
            diag.candidate_basis_size < 126,
            "hyperbolic basis size = {} should be < 126",
            diag.candidate_basis_size
        );
    }

    // ── Determinism ──────────────────────────────────────────────

    #[test]
    fn omp_is_deterministic() {
        let n = 128;
        let x = linspace_unit_to_canonical(n, 3);
        let y: Vec<f64> = (0..n).map(|i| x[[i, 0]] + x[[i, 1]] + x[[i, 2]]).collect();
        let a = fit_sparse_pce(
            x.view(),
            &y,
            &[PolynomialFamily::Legendre; 3],
            3,
            TruncationScheme::TotalDegree,
            SparseSolver::Omp,
            None,
        )
        .unwrap()
        .0;
        let b = fit_sparse_pce(
            x.view(),
            &y,
            &[PolynomialFamily::Legendre; 3],
            3,
            TruncationScheme::TotalDegree,
            SparseSolver::Omp,
            None,
        )
        .unwrap()
        .0;
        assert_eq!(a.coefficients, b.coefficients);
    }

    #[test]
    fn lars_is_deterministic() {
        let n = 128;
        let x = linspace_unit_to_canonical(n, 3);
        let y: Vec<f64> = (0..n).map(|i| x[[i, 0]] + x[[i, 1]] + x[[i, 2]]).collect();
        let a = fit_sparse_pce(
            x.view(),
            &y,
            &[PolynomialFamily::Legendre; 3],
            3,
            TruncationScheme::TotalDegree,
            SparseSolver::Lars,
            None,
        )
        .unwrap()
        .0;
        let b = fit_sparse_pce(
            x.view(),
            &y,
            &[PolynomialFamily::Legendre; 3],
            3,
            TruncationScheme::TotalDegree,
            SparseSolver::Lars,
            None,
        )
        .unwrap()
        .0;
        assert_eq!(a.coefficients, b.coefficients);
    }
}
