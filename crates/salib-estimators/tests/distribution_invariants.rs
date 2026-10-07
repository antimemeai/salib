//! Analytic and metamorphic checks for conditioning on tied input values.
#![allow(clippy::unwrap_used)]
use ndarray::Array2;
use salib_estimators::{
    estimate_borgonovo_delta, estimate_given_data_sobol, estimate_pawn, estimate_qosa,
};

fn balanced_data() -> (Array2<f64>, Vec<f64>) {
    let n = 64;
    // Four equally weighted combinations of independent binary X_1 and Y.
    // X_0 is constant and X_2 determines Y exactly.
    let y: Vec<_> = (0..n).map(|j| (j / 32) as f64).collect();
    let x = Array2::from_shape_fn((n, 3), |(j, i)| match i {
        0 => 0.0,
        1 => ((j / 16) % 2) as f64,
        _ => y[j],
    });
    (x, y)
}

fn assert_close(a: &[f64], b: &[f64]) {
    assert_eq!(a.len(), b.len());
    for (left, right) in a.iter().zip(b) {
        assert!((left - right).abs() < 1e-12, "{left} != {right}");
    }
}

#[test]
fn tied_classes_preserve_independence_and_full_information() {
    let (x, y) = balanced_data();
    assert_close(
        &estimate_given_data_sobol(x.view(), &y).unwrap().s1,
        &[0.0, 0.0, 1.0],
    );
    for alpha in [0.1, 0.5, 0.9] {
        assert_close(
            &estimate_qosa(x.view(), &y, alpha).unwrap().s,
            &[0.0, 0.0, 1.0],
        );
    }
    let pawn = estimate_pawn(x.view(), &y, 8).unwrap();
    assert_close(&pawn.median, &[0.0, 0.0, 0.5]);
    assert_close(&pawn.maximum, &[0.0, 0.0, 0.5]);
    // A constant input must use the unconditional KDE, hence delta = 0.
    assert!(estimate_borgonovo_delta(x.view(), &y).unwrap().delta[0].abs() < 1e-12);
}

#[test]
fn tied_conditioning_is_invariant_under_joint_row_permutation() {
    let (x, y) = balanced_data();
    let xp = Array2::from_shape_fn(x.raw_dim(), |(j, i)| x[[(13 * j + 7) % y.len(), i]]);
    let yp: Vec<_> = (0..y.len()).map(|j| y[(13 * j + 7) % y.len()]).collect();
    assert_close(
        &estimate_given_data_sobol(x.view(), &y).unwrap().s1,
        &estimate_given_data_sobol(xp.view(), &yp).unwrap().s1,
    );
    assert_close(
        &estimate_borgonovo_delta(x.view(), &y).unwrap().delta,
        &estimate_borgonovo_delta(xp.view(), &yp).unwrap().delta,
    );
    let pawn = estimate_pawn(x.view(), &y, 8).unwrap();
    let pawnp = estimate_pawn(xp.view(), &yp, 8).unwrap();
    assert_close(&pawn.median, &pawnp.median);
    assert_close(&pawn.mean, &pawnp.mean);
    for alpha in [0.1, 0.5, 0.9] {
        assert_close(
            &estimate_qosa(x.view(), &y, alpha).unwrap().s,
            &estimate_qosa(xp.view(), &yp, alpha).unwrap().s,
        );
    }
}

#[test]
fn qosa_pinball_contrast_handles_atoms_and_positive_affine_output_changes() {
    let n = 64;
    let x = Array2::from_shape_fn((n, 1), |(j, _)| (j % 4) as f64);
    let atoms: Vec<_> = (0..n).map(|j| [0.0, 1.0, 1.0, 2.0][j % 4]).collect();
    assert_close(&estimate_qosa(x.view(), &atoms, 0.5).unwrap().s, &[1.0]);
    let x = Array2::from_shape_fn((n, 1), |(j, _)| (j as f64 + 0.5) / n as f64);
    let y = x.column(0).to_vec();
    for alpha in [0.1, 0.5, 0.9, 0.99] {
        let result = estimate_qosa(x.view(), &y, alpha).unwrap();
        for (scale, offset) in [(1.0, 100.0), (3.0, -20.0), (1e-8, 0.0)] {
            let transformed: Vec<_> = y.iter().map(|v| scale * v + offset).collect();
            assert_close(
                &result.s,
                &estimate_qosa(x.view(), &transformed, alpha).unwrap().s,
            );
        }
    }
}

#[test]
fn qosa_atom_diagnostics_match_exact_empirical_contrast() {
    let n = 64;
    let x = Array2::from_shape_fn((n, 1), |(j, _)| (j % 4) as f64);
    let y: Vec<_> = (0..n).map(|j| [0.0, 1.0, 1.0, 2.0][j % 4]).collect();
    let median = estimate_qosa(x.view(), &y, 0.5).unwrap();
    // At the median q=1, half the observations incur loss 1/2.
    assert!((median.global_loss - 0.25).abs() < 1e-12);
    assert!((median.global_quantile - 1.0).abs() < 1e-12);
    assert!((median.global_cte - 1.5).abs() < 1e-12);
    for alpha in [0.1, 0.9] {
        let estimate = estimate_qosa(x.view(), &y, alpha).unwrap();
        assert!((estimate.global_loss - 0.1).abs() < 1e-12);
    }
    // For 64 equally spaced Y=X values the 3 classes have 21,21,22
    // observations. Their median losses sum to 341/128; global sum=8.
    let x = Array2::from_shape_fn((n, 1), |(j, _)| (j as f64 + 0.5) / n as f64);
    let y = x.column(0).to_vec();
    let estimate = estimate_qosa(x.view(), &y, 0.5).unwrap();
    assert!((estimate.global_loss - 0.125).abs() < 1e-12);
    assert_close(&estimate.s, &[683.0 / 1024.0]);
}

#[test]
fn nonfinite_observations_are_rejected_before_conditioning() {
    use salib_estimators::{BorgonovoError, GivenDataSobolError, PawnError, QosaError};
    let (mut x, mut y) = balanced_data();
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        x[[7, 1]] = bad;
        assert!(matches!(
            estimate_qosa(x.view(), &y, 0.5),
            Err(QosaError::NonfiniteInput { row: 7, column: 1 })
        ));
        assert!(matches!(
            estimate_given_data_sobol(x.view(), &y),
            Err(GivenDataSobolError::NonfiniteInput { row: 7, column: 1 })
        ));
        assert!(matches!(
            estimate_pawn(x.view(), &y, 8),
            Err(PawnError::NonfiniteInput { row: 7, column: 1 })
        ));
        assert!(matches!(
            estimate_borgonovo_delta(x.view(), &y),
            Err(BorgonovoError::NonfiniteInput { row: 7, column: 1 })
        ));
        x[[7, 1]] = 0.0;
        y[5] = bad;
        assert!(matches!(
            estimate_qosa(x.view(), &y, 0.5),
            Err(QosaError::NonfiniteOutput { index: 5 })
        ));
        assert!(matches!(
            estimate_given_data_sobol(x.view(), &y),
            Err(GivenDataSobolError::NonfiniteOutput { index: 5 })
        ));
        assert!(matches!(
            estimate_pawn(x.view(), &y, 8),
            Err(PawnError::NonfiniteOutput { index: 5 })
        ));
        assert!(matches!(
            estimate_borgonovo_delta(x.view(), &y),
            Err(BorgonovoError::NonfiniteOutput { index: 5 })
        ));
        y[5] = 0.0;
    }
}

#[test]
fn qosa_expected_shortfall_stays_finite_when_an_unscaled_tail_sum_overflows() {
    let x = Array2::from_shape_fn((16, 1), |(j, _)| (j / 8) as f64);
    let y: Vec<_> = (0..16).map(|j| if j < 8 { 0.0 } else { 1e308 }).collect();
    let result = estimate_qosa(x.view(), &y, 1e-308).unwrap();
    assert!((result.global_loss - 0.5).abs() < 1e-12);
    assert!(result.global_cte.is_finite());
    assert!((result.global_cte / 5e307 - 1.0).abs() < 1e-12);
    let opposite: Vec<_> = (0..16)
        .map(|j| if j < 8 { -1e308 } else { 1e308 })
        .collect();
    let result = estimate_qosa(x.view(), &opposite, 1e-308).unwrap();
    assert!((result.global_loss - 1.0).abs() < 1e-12);
    assert_eq!(result.global_cte, 0.0);
}

#[test]
fn borgonovo_density_distance_is_invariant_to_extreme_finite_output_scales() {
    let x = Array2::from_shape_fn((64, 1), |(j, _)| (j / 32) as f64);
    let y: Vec<_> = (0..64)
        .map(|j| 2.0 * x[[j, 0]] + (j % 32) as f64 / 32.0)
        .collect();
    let baseline = estimate_borgonovo_delta(x.view(), &y).unwrap();
    assert!(
        baseline.delta[0] > 0.4,
        "fixture has strongly separated conditional laws"
    );
    for scale in [1e200, 1e-200] {
        let scaled: Vec<_> = y.iter().map(|v| v * scale).collect();
        assert_close(
            &baseline.delta,
            &estimate_borgonovo_delta(x.view(), &scaled).unwrap().delta,
        );
    }
    let wide: Vec<_> = y.iter().map(|v| (v - 1.5) * 1e308).collect();
    assert!(wide.iter().all(|v| v.is_finite()));
    assert_close(
        &baseline.delta,
        &estimate_borgonovo_delta(x.view(), &wide).unwrap().delta,
    );
}
