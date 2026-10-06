//! Distribution quantile boundaries, medians, monotonicity, and identities.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::float_cmp,
    clippy::approx_constant
)]

use salib_core::Distribution;

fn assert_close(got: f64, expected: f64, tolerance: f64) {
    assert!(got.is_finite() && expected.is_finite());
    assert!(
        (got - expected).abs() <= tolerance,
        "got {got}, expected {expected} within {tolerance}"
    );
}

fn assert_monotone(distribution: Distribution) {
    let mut previous = f64::NEG_INFINITY;
    for u in [
        0.0, 0.001, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 0.99, 0.999, 1.0,
    ] {
        let q = distribution.quantile(u);
        // Unbounded distributions may have infinite endpoint quantiles.
        assert!(!q.is_nan());
        if u > 0.0 && u < 1.0 {
            assert!(q.is_finite());
        }
        assert!(q >= previous, "quantile({u}) = {q} < {previous}");
        previous = q;
    }
}

fn assert_agreement(a: Distribution, b: Distribution, tolerance: f64) {
    for u in [0.1, 0.3, 0.5, 0.7, 0.9] {
        assert_close(a.quantile(u), b.quantile(u), tolerance);
    }
}

#[test]
fn uniform_maps_unit_interval_linearly() {
    let d = Distribution::Uniform { lo: 10.0, hi: 30.0 };
    for (u, expected) in [(0.0, 10.0), (0.5, 20.0), (1.0, 30.0)] {
        assert_eq!(d.quantile(u), expected);
    }
}

#[test]
fn kani_regression_discrete_uniform_inclusive_size_overflow() {
    let d = Distribution::DiscreteUniform {
        lo: i64::MIN / 2,
        hi: i64::MAX / 2,
    };
    assert_eq!(d.quantile(0.5), 0.0);

    let full = Distribution::DiscreteUniform {
        lo: i64::MIN,
        hi: i64::MAX,
    };
    assert_eq!(full.quantile(0.25), (i64::MIN / 2) as f64);
    assert_eq!(full.quantile(0.5), 0.0);
    assert_eq!(full.quantile(0.75), (1_i64 << 62) as f64);
    assert_monotone(full);
}

#[test]
fn kani_regression_discrete_uniform_rounded_upper_endpoint() {
    let d = Distribution::DiscreteUniform {
        lo: -18_014_398_509_481_985,
        hi: 0,
    };
    assert_eq!(d.quantile(0.0), -18_014_398_509_481_985_i64 as f64);
    assert_eq!(d.quantile(1.0), 0.0);
    assert_eq!(d.quantile(1.5), 0.0);
    assert_monotone(d);
}

#[test]
fn kani_regression_uniform_extreme_finite_bounds() {
    let d = Distribution::Uniform {
        lo: -f64::MAX,
        hi: f64::MAX,
    };
    assert_eq!(d.quantile(0.0), -f64::MAX);
    assert_eq!(d.quantile(0.5), 0.0);
    assert_eq!(d.quantile(1.0), f64::MAX);
    assert_monotone(d);
}

#[test]
fn triangular_hits_support_boundaries() {
    let d = Distribution::Triangular {
        lo: -1.0,
        mode: 0.0,
        hi: 1.0,
    };
    assert_eq!(d.quantile(0.0), -1.0);
    assert_eq!(d.quantile(1.0), 1.0);
}

#[test]
fn beta_on_non_unit_interval_hits_boundaries() {
    let d = Distribution::Beta {
        alpha: 2.0,
        beta: 5.0,
        lo: 0.5,
        hi: 1.5,
    };
    assert_close(d.quantile(0.0), 0.5, 1e-9);
    assert_close(d.quantile(1.0), 1.5, 1e-9);
}

#[test]
fn normal_median_is_mu() {
    assert_close(
        Distribution::Normal {
            mu: 7.0,
            sigma: 2.0,
        }
        .quantile(0.5),
        7.0,
        1e-9,
    );
}

#[test]
fn symmetric_beta_median_is_midpoint() {
    assert_close(
        Distribution::Beta {
            alpha: 3.0,
            beta: 3.0,
            lo: 0.0,
            hi: 1.0,
        }
        .quantile(0.5),
        0.5,
        1e-9,
    );
}

#[test]
fn lognormal_median_is_exp_mu_log() {
    assert_close(
        Distribution::LogNormal {
            mu_log: 1.0,
            sigma_log: 0.5,
        }
        .quantile(0.5),
        2.718_281_828,
        1e-6,
    );
}

#[test]
fn uniform_quantile_is_monotone() {
    assert_monotone(Distribution::Uniform { lo: 0.0, hi: 1.0 });
}
#[test]
fn normal_quantile_is_monotone() {
    assert_monotone(Distribution::Normal {
        mu: 0.0,
        sigma: 1.0,
    });
}
#[test]
fn beta_quantile_is_monotone() {
    assert_monotone(Distribution::Beta {
        alpha: 2.0,
        beta: 5.0,
        lo: 0.0,
        hi: 1.0,
    });
}
#[test]
fn bernoulli_quantile_is_monotone() {
    assert_monotone(Distribution::Bernoulli { p: 0.4 });
}

#[test]
fn beta_one_one_is_uniform() {
    assert_agreement(
        Distribution::Beta {
            alpha: 1.0,
            beta: 1.0,
            lo: 0.0,
            hi: 1.0,
        },
        Distribution::Uniform { lo: 0.0, hi: 1.0 },
        1e-9,
    );
}
#[test]
fn weibull_shape_one_is_exponential() {
    assert_agreement(
        Distribution::Weibull {
            shape: 1.0,
            scale: 4.0,
        },
        Distribution::Exponential { lambda: 0.25 },
        1e-12,
    );
}
#[test]
fn gamma_shape_one_is_exponential() {
    assert_agreement(
        Distribution::Gamma {
            shape: 1.0,
            scale: 2.0,
        },
        Distribution::Exponential { lambda: 0.5 },
        1e-7,
    );
}
#[test]
fn out_of_range_probabilities_saturate_to_support() {
    let d = Distribution::Uniform { lo: 10.0, hi: 30.0 };
    assert_eq!(d.quantile(-0.5), 10.0);
    assert_eq!(d.quantile(1.5), 30.0);
}
