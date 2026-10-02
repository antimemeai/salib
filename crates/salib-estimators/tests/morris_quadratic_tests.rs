//! Morris elementary effects for the eight-factor quadratic additive model.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use salib_core::RngState;
use salib_estimators::{estimate_morris_effects, MorrisEffects};
use salib_samplers::build_morris_trajectories;
use salib_validation::morris_test;

fn effects(r: usize) -> MorrisEffects {
    let trajectories = build_morris_trajectories(8, r, 4, &mut RngState::from_seed([0; 32]))
        .expect("trajectories");
    estimate_morris_effects(&trajectories, |x| {
        morris_test::morris_quadratic_additive_with_dim(x, 8)
    })
    .expect("effects")
}
fn assert_close(got: f64, expected: f64, tolerance: f64) {
    assert!(got.is_finite() && expected.is_finite());
    assert!(
        (got - expected).abs() <= tolerance,
        "got {got}, expected {expected} within {tolerance}"
    );
}
#[test]
fn recovers_analytic_mu_and_sigma_at_1000_trajectories() {
    let e = effects(1000);
    assert_eq!(e.d, 8);
    let mu = [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0];
    let sigma = [
        1.0 / 3.0,
        2.0 / 3.0,
        1.0,
        4.0 / 3.0,
        5.0 / 3.0,
        2.0,
        7.0 / 3.0,
        8.0 / 3.0,
    ];
    for i in 0..8 {
        assert_close(e.mu[i], mu[i], 0.1);
        assert_close(e.sigma[i], sigma[i], 0.15);
    }
}
#[test]
fn mu_star_is_at_least_absolute_mu() {
    let e = effects(100);
    for i in 0..e.d {
        assert!(e.mu_star[i].is_finite() && e.mu[i].is_finite());
        assert!(e.mu_star[i] >= e.mu[i].abs() - 1e-12);
    }
}
#[test]
fn largest_sigma_factor_errors_are_below_015_at_1000_trajectories() {
    let e = effects(1000);
    let analytic = morris_test::analytic_quadratic_effects(e.d);
    for (got, expected) in [(e.mu[7], analytic.mu[7]), (e.sigma[7], analytic.sigma[7])] {
        assert!(got.is_finite() && expected.is_finite());
        assert!((got - expected).abs() < 0.15);
    }
}
