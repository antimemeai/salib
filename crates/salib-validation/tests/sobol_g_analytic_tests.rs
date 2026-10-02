//! Closed-form Sobol G first-order indices and input distribution.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

use salib_core::Distribution;
use salib_validation::sobol_g;

fn assert_close(got: f64, expected: f64, tolerance: f64) {
    assert!(got.is_finite() && expected.is_finite());
    assert!(
        (got - expected).abs() <= tolerance,
        "got {got}, expected {expected} within {tolerance}"
    );
}
#[test]
fn individual_variance_matches_closed_form() {
    let a = [0.0, 1.0, 9.0];
    let s = sobol_g::analytic_indices(&a);
    for (i, ai) in a.iter().enumerate() {
        assert_close(
            s.first_order[i] * s.total_variance,
            (1.0 / 3.0) / (1.0 + ai).powi(2),
            1e-9,
        );
    }
}
#[test]
fn total_variance_matches_product_form() {
    let s = sobol_g::analytic_indices(&[0.0, 1.0, 9.0]);
    let expected = (1.0 + 1.0 / 3.0) * (1.0 + 1.0 / 12.0) * (1.0 + 1.0 / 300.0) - 1.0;
    assert_close(s.total_variance, expected, 1e-12);
}
#[test]
fn smaller_a_means_larger_first_order_index() {
    let s = sobol_g::analytic_indices(&[0.0, 1.0, 9.0, 99.0]);
    assert!(s.first_order.iter().all(|v| v.is_finite()));
    for pair in s.first_order.windows(2) {
        assert!(pair[0] > pair[1]);
    }
}
#[test]
fn first_order_indices_are_positive_and_sum_to_at_most_one() {
    let s = sobol_g::analytic_indices(&[0.0, 1.0, 4.5, 9.0, 99.0]);
    for &value in &s.first_order {
        assert!(value.is_finite() && value > 0.0);
    }
    let sum: f64 = s.first_order.iter().sum();
    assert!(sum.is_finite() && sum <= 1.0 + 1e-12);
}
#[test]
fn high_a_contributes_negligibly() {
    let s2 = sobol_g::analytic_indices(&[0.0, 99.0]).first_order[1];
    assert!(s2.is_finite() && s2 < 0.01);
}
#[test]
fn total_order_uses_nan_sentinel() {
    let s = sobol_g::analytic_indices(&[1.0, 2.0]);
    assert_eq!(s.total_order.len(), 2);
    for value in &s.total_order {
        assert!(value.is_nan());
    }
}
#[test]
fn canonical_screening_case_ranks_factors() {
    let s = sobol_g::analytic_indices(&[0.0, 1.0, 4.5, 9.0, 99.0, 99.0, 99.0, 99.0]);
    assert_eq!(s.first_order.len(), 8);
    assert!(s.first_order.iter().all(|v| v.is_finite()));
    for &strong in &s.first_order[..4] {
        for &weak in &s.first_order[4..] {
            assert!(strong > weak);
        }
    }
    for &value in &s.first_order[4..] {
        assert!(value < 1e-3);
    }
}
#[test]
fn input_distribution_has_requested_dimensionality() {
    let p = sobol_g::input_distribution(5);
    assert_eq!(
        p.factors()
            .iter()
            .map(|f| f.name.as_str())
            .collect::<Vec<_>>(),
        ["x1", "x2", "x3", "x4", "x5"]
    );
    for factor in p.factors() {
        assert!(matches!(
            factor.distribution,
            Distribution::Uniform { lo: 0.0, hi: 1.0 }
        ));
    }
}
