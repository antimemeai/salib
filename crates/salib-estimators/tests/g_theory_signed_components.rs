//! Signed moment estimates and the named truncation policy (#3).
//!
//! The 4p × 3i × 2r grid is self-derived, not from a textbook; its expected
//! values come from exact mean squares in fixtures/gate1_pir_4x3x2.json. It
//! is kept because σ²(r) and σ²(ir) are negative. Brennan's Synthetic Data
//! Set No. 3 supplies the independent conformance oracle (g_theory_brennan.rs).

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod g_theory_fixtures;

use g_theory_fixtures::gate1;
use salib_core::RngState;
use salib_estimators::{
    bootstrap_g_theory_pir, estimate_g_theory_pir, estimate_g_theory_pir_with_bootstrap,
    find_minimum_design, project_g_theory_d_study, GTheoryCoefficientError, GTheoryComponent,
    GTheoryDesign, GTheoryError,
};

const TOL: f64 = 1e-12;
const NEGATIVE_R: GTheoryCoefficientError = GTheoryCoefficientError::NegativeComponent {
    component: GTheoryComponent::R,
};

fn close(name: &str, actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < TOL,
        "{name}: actual {actual}, expected {expected}"
    );
}

#[test]
fn components_are_signed_moment_estimates() {
    let r = estimate_g_theory_pir(gate1().view(), GTheoryDesign::Crossed).unwrap();
    let expected = [
        9.0 / 4.0,
        3.0 / 4.0,
        -1.0 / 24.0,
        1.0 / 4.0,
        3.0 / 8.0,
        -1.0 / 72.0,
        7.0 / 72.0,
    ];
    for (k, (a, e)) in r.components().iter().zip(expected).enumerate() {
        close(&GTheoryComponent::ALL[k].to_string(), *a, e);
    }
    assert!(!r.negative_components_zeroed);
    assert_eq!((r.n_items, r.n_raters), (3, 2));
}

#[test]
fn g_is_available_and_phi_names_the_negative_component() {
    let r = estimate_g_theory_pir(gate1().view(), GTheoryDesign::Crossed).unwrap();
    // G uses σ²(p), σ²(pi), σ²(pr), σ²(pir), all positive: (9/4) / (1096/432).
    close("G", r.g_coefficient.unwrap(), 243.0 / 274.0);
    assert_eq!(r.phi_coefficient, Err(NEGATIVE_R));

    let d = project_g_theory_d_study(&r, 6, 4).unwrap();
    close("G(6,4)", d.g_coefficient.unwrap(), 3888.0 / 4129.0);
    assert_eq!(d.phi_coefficient, Err(NEGATIVE_R));
}

#[test]
fn zeroing_is_explicit_and_recomputes_coefficients() {
    let signed = estimate_g_theory_pir(gate1().view(), GTheoryDesign::Crossed).unwrap();
    let zeroed = signed.with_negative_components_zeroed();
    assert!(zeroed.negative_components_zeroed);
    assert_eq!((zeroed.sigma_r, zeroed.sigma_ir), (0.0, 0.0));
    close("sigma_i unchanged", zeroed.sigma_i, signed.sigma_i);
    close("G unchanged", zeroed.g_coefficient.unwrap(), 243.0 / 274.0);
    // Absolute error 1/4 + 1/12 + 3/16 + 7/432 = 29/54, so Phi = (9/4) / (9/4 + 29/54).
    close("Phi", zeroed.phi_coefficient.unwrap(), 243.0 / 301.0);
    let d = project_g_theory_d_study(&zeroed, 6, 4).unwrap();
    close("Phi(6,4)", d.phi_coefficient.unwrap(), 3888.0 / 4345.0);
}

#[test]
fn design_search_refuses_signed_components_and_accepts_zeroed() {
    let signed = estimate_g_theory_pir(gate1().view(), GTheoryDesign::Crossed).unwrap();
    let cost = |ni: usize, nr: usize| (ni * nr) as f64;
    assert_eq!(
        find_minimum_design(&signed, 0.8, 5, 5, cost),
        Err(GTheoryError::Coefficient(NEGATIVE_R))
    );
    let zeroed = signed.with_negative_components_zeroed();
    let best = find_minimum_design(&zeroed, 0.8, 5, 5, cost)
        .unwrap()
        .unwrap();
    assert!(best.phi_coefficient.unwrap() >= 0.8);
}

#[test]
fn bootstrap_withholds_interval_for_unavailable_coefficient() {
    let mut rng = RngState::from_seed([0x44; 32]);
    let r = estimate_g_theory_pir_with_bootstrap(
        gate1().view(),
        GTheoryDesign::Crossed,
        200,
        0.1,
        &mut rng,
    )
    .unwrap();
    assert_eq!(r.phi_coefficient, Err(NEGATIVE_R));
    assert_eq!(
        (r.phi_coefficient_ci_low, r.phi_coefficient_ci_high),
        (None, None)
    );
    let (low, high) = (
        r.g_coefficient_ci_low.unwrap(),
        r.g_coefficient_ci_high.unwrap(),
    );
    assert!((0.0..=1.0).contains(&low) && low <= high && high <= 1.0);
    let fitted = 200 - r.bootstrap_skipped.unwrap();
    assert!(r.g_coefficient_failed.unwrap() <= fitted);
    assert!(r.phi_coefficient_failed.unwrap() <= fitted);

    let zeroed = r.with_negative_components_zeroed();
    assert_eq!(zeroed.bootstrap_iterations, None);
    assert_eq!(zeroed.g_coefficient_ci_low, None);

    // The resampled estimator is the clipped one; its Phi interval is exposed
    // directly and brackets the zeroed point Phi = 243/301.
    let mut rng = RngState::from_seed([0x44; 32]);
    let boot =
        bootstrap_g_theory_pir(gate1().view(), GTheoryDesign::Crossed, 200, 0.1, &mut rng).unwrap();
    let (low, high) = boot.phi_coefficient_ci.unwrap();
    assert!(low <= 243.0 / 301.0 && 243.0 / 301.0 <= high);
    assert_eq!(
        boot.g_coefficient_ci,
        Some((
            r.g_coefficient_ci_low.unwrap(),
            r.g_coefficient_ci_high.unwrap()
        ))
    );
}
