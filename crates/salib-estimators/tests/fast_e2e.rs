//! FAST/eFAST checks on Ishigami with fixed seeds and harmonic order.
//!
//! Analytic indices provide the population targets. Error tolerances and
//! comparisons across sample counts apply to these configurations; they do
//! not establish a convergence rate or a permanent bias for the method.
//!
//! Discrete Fourier fixtures in the unit tests separately check spectral arithmetic.

#![allow(
    clippy::float_cmp,
    clippy::approx_constant,
    clippy::unreadable_literal,
    clippy::expect_used,
    clippy::cast_precision_loss,
    clippy::similar_names,
    clippy::items_after_statements,
    clippy::doc_markdown,
    clippy::needless_range_loop
)]

use std::f64::consts::PI;

use salib_core::RngState;
use salib_estimators::{estimate_fast, FastIndices};
use salib_samplers::build_fast_design;
use salib_validation::ishigami;

const FIXTURE_SEED: [u8; 32] = [0; 32];

/// Map a `[0, 1]^3` sample to Ishigami's `Uniform[-π, π]^3` input,
/// then evaluate.
fn ishigami_on_unit_cube(u: &[f64]) -> f64 {
    let x: [f64; 3] = [
        -PI + 2.0 * PI * u[0],
        -PI + 2.0 * PI * u[1],
        -PI + 2.0 * PI * u[2],
    ];
    ishigami::ishigami(&x)
}

fn run_fast_at_n(n: usize) -> FastIndices {
    let mut rng = RngState::from_seed(FIXTURE_SEED);
    let design = build_fast_design(3, n, 4, &mut rng).expect("valid design");
    estimate_fast(&design, ishigami_on_unit_cube).expect("estimate")
}

// Analytic targets.

#[test]
fn fast_ishigami_recovers_analytic_on_selected_design() {
    // Check all indices against the analytic targets at this seed and N=1025.
    // The tolerances cover this fixture; they are not general error bounds.
    let estimate = run_fast_at_n(1025);
    let analytic = ishigami::analytic_indices(7.0, 0.1);
    const S_TOL: f64 = 0.05;
    const ST_TOL: f64 = 0.10;
    for i in 0..3 {
        let s_err = (estimate.s[i] - analytic.first_order[i]).abs();
        let st_err = (estimate.st[i] - analytic.total_order[i]).abs();
        assert!(
            s_err < S_TOL,
            "S_{i}: got {:.4}, analytic {:.4}, err {s_err:.4} > {S_TOL}",
            estimate.s[i],
            analytic.first_order[i]
        );
        assert!(
            st_err < ST_TOL,
            "ST_{i}: got {:.4}, analytic {:.4}, err {st_err:.4} > {ST_TOL}",
            estimate.st[i],
            analytic.total_order[i]
        );
    }
}

// Index ordering and ranges on the selected fixture.

#[test]
fn fast_ishigami_total_at_least_first_order() {
    // Population Sobol' indices satisfy ST_i >= S_i for independent inputs.
    // Check whether this finite-sample estimate also respects that ordering.
    let estimate = run_fast_at_n(1025);
    for i in 0..3 {
        assert!(
            estimate.st[i] + 1e-9 >= estimate.s[i],
            "factor {i}: ST = {} < S = {}",
            estimate.st[i],
            estimate.s[i]
        );
    }
}

// Errors at selected sample counts.

#[test]
fn fast_ishigami_errors_at_selected_sample_counts() {
    // Compare selected errors at N=65, 257 and 1025 for this fixed seed.
    // Monotone error reduction is not guaranteed for arbitrary designs,
    // and three sample counts do not establish an asymptotic rate or bias.
    let analytic = ishigami::analytic_indices(7.0, 0.1);
    let est_low = run_fast_at_n(65);
    let est_mid = run_fast_at_n(257);
    let est_high = run_fast_at_n(1025);

    let s2_low = (est_low.s[1] - analytic.first_order[1]).abs();
    let s2_mid = (est_mid.s[1] - analytic.first_order[1]).abs();
    let s2_high = (est_high.s[1] - analytic.first_order[1]).abs();

    assert!(
        s2_mid <= s2_low + 1e-6,
        "S_2 should not regress: N=65 → 257 err {s2_low:.4} → {s2_mid:.4}"
    );
    assert!(
        s2_high <= s2_mid + 1e-6,
        "S_2 should not regress: N=257 → 1025 err {s2_mid:.4} → {s2_high:.4}"
    );

    let st1_low = (est_low.st[0] - analytic.total_order[0]).abs();
    let st1_mid = (est_mid.st[0] - analytic.total_order[0]).abs();
    let st1_high = (est_high.st[0] - analytic.total_order[0]).abs();

    assert!(
        st1_mid <= st1_low + 1e-3,
        "ST_1 should not regress materially: N=65 → 257 err {st1_low:.4} → {st1_mid:.4}"
    );
    assert!(
        st1_high <= st1_mid + 1e-3,
        "ST_1 should not regress materially: N=257 → 1025 err {st1_mid:.4} → {st1_high:.4}"
    );
}

// Factor ranking on the selected fixture.

#[test]
fn fast_ishigami_ranks_factors_by_total_order_correctly() {
    // Analytic ranking by ST: factor 1 (0.558) > factor 2 (0.442) > factor 3 (0.244).
    let estimate = run_fast_at_n(1025);
    assert!(
        estimate.st[0] > estimate.st[1],
        "ST_1 = {} should exceed ST_2 = {}",
        estimate.st[0],
        estimate.st[1]
    );
    assert!(
        estimate.st[1] > estimate.st[2],
        "ST_2 = {} should exceed ST_3 = {}",
        estimate.st[1],
        estimate.st[2]
    );
}
