//! G-theory reliability projections over item and rater counts.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use ndarray::Array3;
use salib_estimators::{estimate_g_theory_pir, project_g_theory_d_study, GTheoryDesign};

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
fn increasing_items_and_raters_improves_reliability() {
    let estimate =
        estimate_g_theory_pir(grid().view(), GTheoryDesign::Crossed).expect("crossed estimate");
    let counts = [(2, 2), (4, 2), (2, 4), (4, 4)];
    let points: Vec<_> = counts
        .iter()
        .map(|&(items, raters)| {
            project_g_theory_d_study(&estimate, items, raters).expect("D-study projection")
        })
        .collect();
    assert_eq!(points.len(), 4);
    for (point, &(items, raters)) in points.iter().zip(&counts) {
        assert_eq!((point.n_items, point.n_raters), (items, raters));
        assert!(point.g_coefficient.is_finite() && point.phi_coefficient.is_finite());
    }
    let current = &points[0];
    let expanded = &points[3];
    assert!(expanded.g_coefficient > current.g_coefficient);
    assert!(expanded.phi_coefficient > current.phi_coefficient);
    assert!(current.g_coefficient > current.phi_coefficient);
    assert!(expanded.g_coefficient > expanded.phi_coefficient);
}
