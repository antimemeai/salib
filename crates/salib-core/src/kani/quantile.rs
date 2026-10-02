//! Quantile properties over explicitly constrained symbolic domains.

use crate::Distribution;

#[kani::proof]
fn discrete_uniform_quantile_endpoints() {
    let lo: i64 = kani::any();
    let hi: i64 = kani::any();
    kani::assume(lo <= hi);
    let distribution = Distribution::DiscreteUniform { lo, hi };
    assert_eq!(distribution.quantile(0.0), lo as f64);
    assert_eq!(distribution.quantile(1.0), hi as f64);
}

#[kani::proof]
fn discrete_uniform_quantile_no_overflow() {
    let u: f64 = kani::any();
    kani::assume(u.is_finite() && (0.0..=1.0).contains(&u));
    let distribution = Distribution::DiscreteUniform {
        lo: i64::MIN / 2,
        hi: i64::MAX / 2,
    };
    let _ = distribution.quantile(u);
}

#[kani::proof]
fn bernoulli_quantile() {
    let p: f64 = kani::any();
    let u: f64 = kani::any();
    kani::assume(p.is_finite() && (0.0..=1.0).contains(&p));
    kani::assume(u.is_finite() && (0.0..=1.0).contains(&u));
    let result = Distribution::Bernoulli { p }.quantile(u);
    assert!(result == 0.0 || result == 1.0);
}

#[kani::proof]
fn uniform_quantile_monotone() {
    let lo: f64 = kani::any();
    let hi: f64 = kani::any();
    let u1: f64 = kani::any();
    let u2: f64 = kani::any();
    kani::assume(lo.is_finite() && hi.is_finite() && lo < hi);
    kani::assume(u1.is_finite() && (0.0..=1.0).contains(&u1));
    kani::assume(u2.is_finite() && (u1..=1.0).contains(&u2));
    let distribution = Distribution::Uniform { lo, hi };
    assert!(distribution.quantile(u1) <= distribution.quantile(u2));
}
