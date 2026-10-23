//! Meaning of emitted G-theory coefficients: admissibility, units and 0/0.
//!
//! The first three tests failed on 0.3.0 with the same predicates on its `f64`
//! coefficients (#3, #4): G = -69 and Phi = 2.875 on a random grid,
//! `ZeroVariance` at scale 1e-150, and G = 0 for 0/0.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

mod g_theory_fixtures;

use g_theory_fixtures::{gate1, synthetic3, XorShift};
use ndarray::Array3;
use salib_estimators::{
    estimate_g_theory_pir, project_g_theory_d_study, GTheoryCoefficient, GTheoryCoefficientError,
    GTheoryComponent, GTheoryDesign, GTheoryResult,
};

const G_USES: [usize; 4] = [0, 3, 4, 6];

fn first_negative(c: &[f64; 7], used: &[usize]) -> Option<GTheoryComponent> {
    used.iter()
        .find(|&&k| c[k] < 0.0)
        .map(|&k| GTheoryComponent::ALL[k])
}

/// An available coefficient must be in [0, 1]; an unavailable one must name
/// the first negative component it uses.
fn check(c: &[f64; 7], used: &[usize], value: &GTheoryCoefficient) -> bool {
    match (first_negative(c, used), value) {
        (Some(component), Err(e)) => {
            assert_eq!(*e, GTheoryCoefficientError::NegativeComponent { component });
            false
        }
        (None, Ok(v)) => {
            assert!((0.0..=1.0).contains(v), "coefficient {v} outside [0, 1]");
            true
        }
        (None, Err(GTheoryCoefficientError::ZeroDenominator)) => false,
        (expected, got) => panic!("expected negative {expected:?}, got {got:?}"),
    }
}

#[test]
fn reported_coefficients_lie_in_unit_interval_with_phi_at_most_g() {
    let mut rng = XorShift(0x9E37_79B9_7F4A_7C15);
    let (mut available, mut unavailable) = (0, 0);
    for _ in 0..5000 {
        let grid = Array3::from_shape_fn((3, 3, 2), |_| (rng.next() % 5) as f64);
        let Ok(r) = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed) else {
            continue;
        };
        let c = r.components();
        let counts = [(3, 2), (1, 1), (7, 3)];
        for (ni, nr) in counts {
            let point = project_g_theory_d_study(&r, ni, nr).unwrap();
            for (g, phi) in [
                (&point.g_coefficient, &point.phi_coefficient),
                (&r.g_coefficient, &r.phi_coefficient),
            ] {
                let g_ok = check(&c, &G_USES, g);
                let phi_ok = check(&c, &[0, 1, 2, 3, 4, 5, 6], phi);
                if g_ok && phi_ok {
                    assert!(phi.unwrap() <= g.unwrap(), "Phi > G for {c:?}");
                }
                if g_ok {
                    available += 1;
                } else {
                    unavailable += 1;
                }
            }
        }
    }
    assert!(
        available > 1000 && unavailable > 1000,
        "{available} / {unavailable}"
    );
}

#[test]
fn coefficients_do_not_depend_on_measurement_units() {
    for grid in [synthetic3(), gate1()] {
        let base = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
        for s in [1e-150, 1e-9, 1e-8, 1.0, 1e150] {
            let r = estimate_g_theory_pir(grid.mapv(|y| y * s).view(), GTheoryDesign::Crossed)
                .unwrap_or_else(|e| panic!("scale {s:e}: {e}"));
            for (a, b) in [
                (&r.g_coefficient, &base.g_coefficient),
                (&r.phi_coefficient, &base.phi_coefficient),
            ] {
                match (a, b) {
                    (Ok(a), Ok(b)) => {
                        assert!((a - b).abs() <= 1e-13 * b, "scale {s:e}: {a} vs {b}")
                    }
                    _ => assert_eq!(a, b, "scale {s:e}"),
                }
            }
            for (a, b) in r.components().iter().zip(base.components()) {
                assert!(
                    (a / (s * s) - b).abs() <= 1e-12 * b.abs(),
                    "scale {s:e}: {a} vs {b}"
                );
            }
        }
    }
}

#[test]
fn zero_universe_and_zero_relative_error_is_not_reported_as_zero() {
    // σ²(p) = σ²(pi) = σ²(pr) = σ²(pir) = 0, so G = 0/0; σ²(i) > 0 gives Phi = 0.
    let grid = Array3::from_shape_fn((2, 2, 2), |(_, i, _)| 50.0 + 4.0 * i as f64);
    let r = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
    assert_eq!(
        r.g_coefficient,
        Err(GTheoryCoefficientError::ZeroDenominator)
    );
    assert_eq!(r.phi_coefficient, Ok(0.0));
}

#[test]
fn additive_data_has_exactly_zero_interactions_despite_offsets() {
    let mut rng = XorShift(0xD1B5_4A32_D192_ED03);
    for _ in 0..500 {
        let (np, ni, nr) = (
            2 + rng.next() as usize % 10,
            2 + rng.next() as usize % 6,
            2 + rng.next() as usize % 4,
        );
        let spread = 10_f64.powf(rng.unit() * 6.0 - 3.0);
        let offset = spread * 10_f64.powf(rng.unit() * 9.0);
        let a: Vec<f64> = (0..np).map(|_| rng.unit() * spread).collect();
        let b: Vec<f64> = (0..ni).map(|_| rng.unit() * spread).collect();
        let c: Vec<f64> = (0..nr).map(|_| rng.unit() * spread).collect();
        let grid = Array3::from_shape_fn((np, ni, nr), |(p, i, r)| offset + a[p] + b[i] + c[r]);
        let r = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
        assert_eq!(
            [r.sigma_pi, r.sigma_pr, r.sigma_ir, r.sigma_pir],
            [0.0; 4],
            "offset {offset:e}, spread {spread:e}"
        );
        assert!(r.phi_coefficient.is_ok() || r.sigma_p == 0.0);
    }
}

#[test]
fn issue_3_grid_no_longer_reports_g_above_one() {
    // 0.3.0 reported G = 2.25 here.
    let values = [
        0., 1., 3., 4., 1., 3., 1., 3., 3., 1., 2., 2., 4., 1., 0., 3., 2., 4.,
    ];
    let grid = Array3::from_shape_vec((3, 3, 2), values.to_vec()).unwrap();
    let r = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
    let pi = GTheoryCoefficientError::NegativeComponent {
        component: GTheoryComponent::Pi,
    };
    assert_eq!(r.g_coefficient, Err(pi));
    assert_eq!(r.phi_coefficient, Err(pi));
}

#[test]
fn unavailable_coefficient_names_first_negative_component() {
    let r = GTheoryResult::from_components(1.0, 1.0, 1.0, -0.1, -0.2, 1.0, 1.0, 2, 2).unwrap();
    let pi = GTheoryCoefficientError::NegativeComponent {
        component: GTheoryComponent::Pi,
    };
    assert_eq!(r.g_coefficient, Err(pi));
    assert_eq!(r.phi_coefficient, Err(pi));
}
