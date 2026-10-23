#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

mod g_theory_fixtures;

use g_theory_fixtures::synthetic3;
use ndarray::Array3;
use salib_core::RngState;
use salib_estimators::{
    estimate_g_theory_pir, estimate_g_theory_pir_with_bootstrap, GTheoryDesign,
};

fn grid() -> Array3<f64> {
    let mut grid = Array3::<f64>::zeros((2, 2, 2));
    let levels = [-1.0_f64, 1.0_f64];
    for (ip, &p) in levels.iter().enumerate() {
        for (ii, &i) in levels.iter().enumerate() {
            for (ir, &r) in levels.iter().enumerate() {
                grid[[ip, ii, ir]] = 50.0
                    + 6.0 * p
                    + 4.0 * i
                    + 2.0 * r
                    + 3.0 * p * i
                    + 1.5 * p * r
                    + 1.0 * i * r
                    + 0.5 * p * i * r;
            }
        }
    }
    grid
}

#[test]
fn g_theory_recovers_expected_components_and_coefficients() {
    let r = estimate_g_theory_pir(grid().view(), GTheoryDesign::Crossed).unwrap();
    let tol = 1.0e-12;
    assert!((r.sigma_p - 50.0).abs() < tol);
    assert!((r.sigma_i - 12.5).abs() < tol);
    assert!((r.sigma_r - 2.0).abs() < tol);
    assert!((r.sigma_pi - 35.0).abs() < tol);
    assert!((r.sigma_pr - 8.0).abs() < tol);
    assert!((r.sigma_ir - 3.0).abs() < tol);
    assert!((r.sigma_pir - 2.0).abs() < tol);
    assert!((r.g_coefficient.unwrap() - 0.694_444_444_444_444_4).abs() < tol);
    assert!((r.phi_coefficient.unwrap() - 0.625).abs() < tol);
}

#[test]
fn g_theory_bootstrap_is_deterministic_and_contains_point_estimate() {
    let mut rng_a = RngState::from_seed([0x44; 32]);
    let mut rng_b = RngState::from_seed([0x44; 32]);
    let a = estimate_g_theory_pir_with_bootstrap(
        grid().view(),
        GTheoryDesign::Crossed,
        128,
        0.05,
        &mut rng_a,
    )
    .expect("g-theory bootstrap estimate");
    let b = estimate_g_theory_pir_with_bootstrap(
        grid().view(),
        GTheoryDesign::Crossed,
        128,
        0.05,
        &mut rng_b,
    )
    .expect("g-theory bootstrap estimate");
    assert_eq!(a.variance_component_ci_low, b.variance_component_ci_low);
    assert_eq!(a.variance_component_ci_high, b.variance_component_ci_high);
    assert_eq!(a.g_coefficient_ci_low, b.g_coefficient_ci_low);
    assert_eq!(a.g_coefficient_ci_high, b.g_coefficient_ci_high);
    assert_eq!(a.phi_coefficient_ci_low, b.phi_coefficient_ci_low);
    assert_eq!(a.phi_coefficient_ci_high, b.phi_coefficient_ci_high);
    let low = a.variance_component_ci_low.as_ref().unwrap();
    let high = a.variance_component_ci_high.as_ref().unwrap();
    for (k, value) in a.components().iter().enumerate() {
        let (low, high) = (low[k].unwrap(), high[k].unwrap());
        assert!(low <= *value && *value <= high, "component {k}");
    }
    // Two persons: about half the resamples draw one person twice, leaving no
    // person variance, so G is 0/0 there and its percentiles are unidentifiable.
    // Phi is 0/0 when the resample is constant (one level drawn on every axis).
    let g_failed = a.g_coefficient_failed.unwrap();
    let phi_failed = a.phi_coefficient_failed.unwrap();
    assert!(g_failed as f64 > 0.025 * 127.0 && phi_failed as f64 > 0.025 * 127.0);
    assert_eq!(
        (a.g_coefficient_ci_low, a.g_coefficient_ci_high),
        (None, None)
    );
    assert_eq!(
        (a.phi_coefficient_ci_low, a.phi_coefficient_ci_high),
        (None, None)
    );
}

#[test]
fn g_theory_bootstrap_interval_contains_point_when_identifiable() {
    let mut rng = RngState::from_seed([0x44; 32]);
    // A person effect of one point per person keeps σ²(p) well clear of zero,
    // so the clipped bounds are informative rather than pinned at 0.
    let grid = synthetic3() + Array3::from_shape_fn((10, 4, 2), |(p, _, _)| p as f64);
    let r = estimate_g_theory_pir_with_bootstrap(
        grid.view(),
        GTheoryDesign::Crossed,
        400,
        0.05,
        &mut rng,
    )
    .expect("g-theory bootstrap estimate");
    for (point, low, high) in [
        (
            r.g_coefficient,
            r.g_coefficient_ci_low,
            r.g_coefficient_ci_high,
        ),
        (
            r.phi_coefficient,
            r.phi_coefficient_ci_low,
            r.phi_coefficient_ci_high,
        ),
    ] {
        let (point, low, high) = (point.unwrap(), low.unwrap(), high.unwrap());
        assert!(
            0.0 < low && low < point && point < high && high < 1.0,
            "{low} {point} {high}"
        );
    }
}

#[test]
fn g_theory_bootstrap_records_all_component_intervals_and_metadata() {
    let mut rng = RngState::from_seed([0x44; 32]);
    let r = estimate_g_theory_pir_with_bootstrap(
        grid().view(),
        GTheoryDesign::Crossed,
        128,
        0.05,
        &mut rng,
    )
    .expect("g-theory bootstrap estimate");
    assert_eq!(
        r.variance_component_ci_low.as_ref().expect("CI lows").len(),
        7
    );
    assert_eq!(
        r.variance_component_ci_high
            .as_ref()
            .expect("CI highs")
            .len(),
        7
    );
    assert_eq!(r.bootstrap_iterations, Some(128));
    assert_eq!(r.bootstrap_alpha, Some(0.05));
    assert!(r.bootstrap_skipped.is_some());
    assert!(r.g_coefficient_failed.is_some() && r.phi_coefficient_failed.is_some());
}
