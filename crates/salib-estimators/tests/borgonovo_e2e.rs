//! Borgonovo delta against an analytic Gaussian-mixture contrast.
//!
//! X is a balanced binary factor, Z is an independent standard normal, and
//! Y=mu*(2X-1)+Z. If f0/f1 are the conditional normal densities, the marginal
//! density is (f0+f1)/2. Hence delta_X = TV(f0,f1)/2 = Phi(mu)-1/2.
//! Their densities cross at zero. This oracle follows directly from Borgonovo's
//! definition, not from an untraceable external-package array.
//! Quantile grids integrate the normal input deterministically; finite KDE
//! bandwidth and output quadrature still introduce approximation error.

#![allow(clippy::expect_used, clippy::cast_precision_loss)]

use ndarray::Array2;
use salib_estimators::estimate_borgonovo_delta;
use statrs::distribution::{ContinuousCDF, Normal};

fn gaussian_mixture_data(per_group: usize, mu: f64) -> (Array2<f64>, Vec<f64>) {
    let normal = Normal::new(0.0, 1.0).expect("valid normal distribution");
    let n = 4 * per_group;
    let mut x = Array2::zeros((n, 2));
    let mut y = vec![0.0; n];
    // The second binary input is independent of the first input and output:
    // each of its values sees the identical list of both conditional normals.
    for first in 0..2 {
        for second in 0..2 {
            for k in 0..per_group {
                let row = (2 * first + second) * per_group + k;
                x[[row, 0]] = first as f64;
                x[[row, 1]] = second as f64;
                let z = normal.inverse_cdf((k as f64 + 0.5) / per_group as f64);
                y[row] = mu * (2.0 * first as f64 - 1.0) + z;
            }
        }
    }
    (x, y)
}

#[test]
fn borgonovo_recovers_analytic_gaussian_mixture_contrast() {
    let normal = Normal::new(0.0, 1.0).expect("valid normal distribution");
    for mu in [0.5, 2.0, 4.0] {
        let (x, y) = gaussian_mixture_data(8192, mu);
        let indices = estimate_borgonovo_delta(x.view(), &y).expect("estimate");
        let expected = normal.cdf(mu) - 0.5;
        // Accuracy of this finite KDE/partition configuration, not a universal bound.
        assert!(
            (indices.delta[0] - expected).abs() < 0.03,
            "mu={mu}: delta={}, analytic={expected}",
            indices.delta[0]
        );
        assert!(
            indices.delta[1] < 0.02,
            "independent factor has delta={}",
            indices.delta[1]
        );
    }
}

#[test]
fn borgonovo_tied_inputs_are_invariant_under_joint_row_permutation() {
    let (x, y) = gaussian_mixture_data(256, 2.0);
    let baseline = estimate_borgonovo_delta(x.view(), &y).expect("estimate");
    // Multiplication by an odd integer permutes this power-of-two row count.
    let permuted_x =
        Array2::from_shape_fn(x.dim(), |(row, column)| x[[(row * 137) % y.len(), column]]);
    let permuted_y: Vec<_> = (0..y.len()).map(|row| y[(row * 137) % y.len()]).collect();
    let permuted = estimate_borgonovo_delta(permuted_x.view(), &permuted_y).expect("estimate");
    for factor in 0..2 {
        assert!((baseline.delta[factor] - permuted.delta[factor]).abs() < 1e-12);
    }
}

#[test]
fn borgonovo_is_invariant_under_positive_affine_output_changes() {
    let (x, y) = gaussian_mixture_data(256, 2.0);
    let baseline = estimate_borgonovo_delta(x.view(), &y).expect("estimate");
    let transformed: Vec<_> = y.iter().map(|value| 3.0 * value + 100.0).collect();
    let changed = estimate_borgonovo_delta(x.view(), &transformed).expect("estimate");
    for factor in 0..2 {
        assert!((baseline.delta[factor] - changed.delta[factor]).abs() < 1e-12);
    }
}
