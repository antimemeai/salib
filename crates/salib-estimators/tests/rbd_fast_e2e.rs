//! RBD-FAST checks on Ishigami with fixed LHS seeds and harmonic order.
//!
//! Analytic first-order indices provide the population targets. Tolerances
//! and comparisons across sample counts apply to these configurations;
//! they do not establish unbiasedness or an asymptotic convergence rate.
//!
//! Separate exact Fourier fixtures check EASI traversal and Nyquist multiplicity.

#![allow(
    clippy::float_cmp,
    clippy::approx_constant,
    clippy::unreadable_literal,
    clippy::expect_used,
    clippy::cast_precision_loss,
    clippy::similar_names,
    clippy::items_after_statements,
    clippy::needless_range_loop,
    clippy::doc_markdown
)]

use std::f64::consts::PI;

use ndarray::Array2;
use salib_core::RngState;
use salib_estimators::{estimate_rbd_fast, RbdFastIndices};
use salib_samplers::{LhsSampler, Sampler};
use salib_validation::ishigami;

const FIXTURE_SEED: [u8; 32] = [0; 32];
const HARMONIC: u32 = 10;

fn lhs_ishigami_inputs(n: usize) -> (Array2<f64>, Vec<f64>) {
    let mut rng = RngState::from_seed(FIXTURE_SEED);
    let sampler = LhsSampler::classic(3);
    let unit = sampler.unit_sample(n, &mut rng);
    // Map [0, 1]^3 to Ishigami's [-π, π]^3.
    let mut x = Array2::<f64>::zeros((n, 3));
    for i in 0..n {
        for j in 0..3 {
            x[[i, j]] = -PI + 2.0 * PI * unit[[i, j]];
        }
    }
    let y: Vec<f64> = (0..n)
        .map(|i| {
            let row = [x[[i, 0]], x[[i, 1]], x[[i, 2]]];
            ishigami::ishigami(&row)
        })
        .collect();
    (x, y)
}

fn run_rbd_fast(n: usize) -> RbdFastIndices {
    let (x, y) = lhs_ishigami_inputs(n);
    estimate_rbd_fast(x.view(), &y, HARMONIC).expect("estimate")
}

// Analytic targets.

#[test]
fn rbd_fast_ishigami_recovers_analytic_on_selected_design() {
    // Check this N=4096, M=10 fixture against the analytic first-order indices.
    // The tolerance does not describe a general bias floor.
    let estimate = run_rbd_fast(4096);
    let analytic = ishigami::analytic_indices(7.0, 0.1);
    const TOL: f64 = 0.06;
    for i in 0..3 {
        let err = (estimate.s[i] - analytic.first_order[i]).abs();
        assert!(
            err < TOL,
            "S_{i}: got {:.4}, analytic {:.4}, err {err:.4} > {TOL}",
            estimate.s[i],
            analytic.first_order[i]
        );
    }
}

// Index ordering and ranges on the selected fixture.

#[test]
fn rbd_fast_ishigami_indices_in_unit_with_bias_slack() {
    // Bias correction can produce negative estimates. This interval is a
    // fixture-specific acceptance range, not a bound for the estimator.
    let estimate = run_rbd_fast(4096);
    for i in 0..3 {
        assert!(
            (-0.05..=1.05).contains(&estimate.s[i]),
            "S_{i} = {} outside [-0.05, 1.05]",
            estimate.s[i]
        );
    }
}

// Errors at selected sample counts.

#[test]
fn rbd_fast_ishigami_errors_at_selected_sample_counts() {
    // Compare maximum absolute errors at N=256 and 4096 for this fixed seed.
    // This fixture does not establish a general convergence rate.
    let analytic = ishigami::analytic_indices(7.0, 0.1);

    let max_err = |est: &RbdFastIndices| -> f64 {
        (0..3)
            .map(|i| (est.s[i] - analytic.first_order[i]).abs())
            .fold(0.0, f64::max)
    };

    let est_low = run_rbd_fast(256);
    let est_high = run_rbd_fast(4096);

    let err_low = max_err(&est_low);
    let err_high = max_err(&est_high);

    assert!(
        err_high < err_low,
        "max error should decay: N=256 → 4096: {err_low:.4} → {err_high:.4}"
    );
    assert!(
        err_high < 0.05,
        "N=4096 max err = {err_high:.4} should be < 0.05"
    );
}

// Factor ranking on the selected fixture.

#[test]
fn rbd_fast_ishigami_ranks_factors_by_first_order_correctly() {
    // Analytic ranking by S: factor 2 (0.442) > factor 1 (0.314) > factor 3 (0).
    let estimate = run_rbd_fast(4096);
    assert!(
        estimate.s[1] > estimate.s[0],
        "S_2 = {} should exceed S_1 = {}",
        estimate.s[1],
        estimate.s[0]
    );
    assert!(
        estimate.s[0] > estimate.s[2],
        "S_1 = {} should exceed S_3 = {}",
        estimate.s[0],
        estimate.s[2]
    );
}
