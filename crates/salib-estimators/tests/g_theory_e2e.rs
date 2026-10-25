#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

mod g_theory_fixtures;

use g_theory_fixtures::synthetic3;
use ndarray::Array3;
use salib_core::RngState;
use salib_estimators::{
    estimate_g_theory_pir, estimate_g_theory_pir_with_bootstrap, g_theory_resampling_plan,
    GTheoryDesign,
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
fn g_theory_bootstrap_is_deterministic_and_records_its_procedures() {
    let mut rng_a = RngState::from_seed([0x44; 32]);
    let mut rng_b = RngState::from_seed([0x44; 32]);
    let run = |rng: &mut RngState| {
        estimate_g_theory_pir_with_bootstrap(grid().view(), GTheoryDesign::Crossed, 128, 0.05, rng)
            .expect("g-theory bootstrap estimate")
    };
    let (a, b) = (run(&mut rng_a), run(&mut rng_b));
    let boot = a.bootstrap.as_ref().expect("bootstrap results");
    assert_eq!(Some(boot), b.bootstrap.as_ref());
    assert_eq!(boot.plan, g_theory_resampling_plan([2, 2, 2]));
    assert_eq!((boot.n_resamples, boot.alpha), (128, 0.05));
    assert_eq!((boot.facet_skipped, boot.parametric_skipped), ([0; 3], 0));
    assert!(boot.component_se.iter().all(Option::is_some));
}

#[test]
fn g_theory_intervals_contain_point_when_available() {
    let mut rng = RngState::from_seed([0x44; 32]);
    // A person effect of one point per person keeps σ²(p) well clear of zero.
    let grid = synthetic3() + Array3::from_shape_fn((10, 4, 2), |(p, _, _)| p as f64);
    let r = estimate_g_theory_pir_with_bootstrap(
        grid.view(),
        GTheoryDesign::Crossed,
        400,
        0.05,
        &mut rng,
    )
    .expect("g-theory bootstrap estimate");
    for (point, interval) in [
        (r.g_coefficient, r.g_coefficient_interval()),
        (r.phi_coefficient, r.phi_coefficient_interval()),
    ] {
        let (point, (low, high)) = (point.unwrap(), interval.unwrap());
        assert!(
            0.0 < low && low < point && point < high && high < 1.0,
            "{low} {point} {high}"
        );
    }
    let boot = r.bootstrap.as_ref().unwrap();
    for (k, value) in r.components().iter().enumerate() {
        let (low, high) = boot.component_ci[k].unwrap();
        assert!(0.0 < low && low < high, "component {k}: {low} {high}");
        // Satterthwaite contains the estimate only when ν is not tiny; it is
        // asserted for the components the coverage study supports (pi, pr,
        // ir, pir). With 2 occasions, σ²(r) has ν ≈ 0.007 and its interval
        // lies above the estimate.
        if k >= 3 {
            assert!(
                low < *value && *value < high,
                "component {k}: {low} {value} {high}"
            );
        }
        assert!(boot.component_se[k].unwrap() > 0.0);
    }
}
