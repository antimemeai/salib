//! Closed-form Ishigami indices and input distribution.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

use salib_core::Distribution;
use salib_validation::{ishigami, SobolIndicesAnalytic};
use std::f64::consts::PI;

fn canonical() -> SobolIndicesAnalytic {
    ishigami::analytic_indices(7.0, 0.1)
}
fn assert_close(got: f64, expected: f64, tolerance: f64) {
    assert!(got.is_finite() && expected.is_finite());
    assert!(
        (got - expected).abs() <= tolerance,
        "got {got}, expected {expected} within {tolerance}"
    );
}
#[test]
fn x3_first_order_is_exactly_zero() {
    assert_eq!(canonical().first_order[2], 0.0);
}
#[test]
fn x2_has_no_interactions() {
    let s = canonical();
    assert_close(s.total_order[1], s.first_order[1], 1e-12);
}
#[test]
fn total_order_bounds_first_order_per_factor() {
    let s = canonical();
    for i in 0..s.dim() {
        assert!(s.first_order[i].is_finite() && s.total_order[i].is_finite());
        assert!(s.total_order[i] >= s.first_order[i] - 1e-12);
    }
}
#[test]
fn first_order_is_nonnegative_and_sums_to_at_most_one() {
    let s = canonical();
    for &value in &s.first_order {
        assert!(value.is_finite() && value >= 0.0);
    }
    let sum: f64 = s.first_order.iter().sum();
    assert!(sum.is_finite() && sum <= 1.0 + 1e-12);
}
#[test]
fn canonical_first_order_matches_published_values() {
    let s = canonical();
    assert_close(s.first_order[0], 0.3139, 5e-4);
    assert_close(s.first_order[1], 0.4424, 5e-4);
    assert_eq!(s.first_order[2], 0.0);
}
#[test]
fn canonical_total_order_matches_published_values() {
    let s = canonical();
    for (got, expected) in s.total_order.iter().zip([0.5576, 0.4424, 0.2436]) {
        assert_close(*got, expected, 5e-4);
    }
}
#[test]
fn zero_a_removes_x2_contribution() {
    assert_eq!(ishigami::analytic_indices(0.0, 0.1).first_order[1], 0.0);
}
#[test]
fn zero_b_removes_x1_x3_interaction() {
    assert_eq!(ishigami::analytic_indices(7.0, 0.0).total_order[2], 0.0);
}
#[test]
fn zero_a_and_b_leave_only_x1_sine() {
    let s = ishigami::analytic_indices(0.0, 0.0);
    assert_close(s.first_order[0], 1.0, 1e-12);
    assert_close(s.total_order[0], 1.0, 1e-12);
}
#[test]
fn canonical_total_variance_is_positive() {
    let variance = canonical().total_variance;
    assert!(variance.is_finite() && variance > 0.0);
}
#[test]
fn input_distribution_has_three_uniform_factors() {
    let p = ishigami::input_distribution();
    assert_eq!(
        p.factors()
            .iter()
            .map(|f| f.name.as_str())
            .collect::<Vec<_>>(),
        ["x1", "x2", "x3"]
    );
    for factor in p.factors() {
        match factor.distribution {
            Distribution::Uniform { lo, hi } => {
                assert_close(lo, -PI, 1e-12);
                assert_close(hi, PI, 1e-12);
            }
            ref other => panic!("expected Uniform, got {other:?}"),
        }
    }
}
