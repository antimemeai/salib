//! Saltelli radial matrix structure, determinism, and validation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use ndarray::Array2;
use salib_core::RngState;
use salib_samplers::{
    build_saltelli_matrix, LhsSampler, SaltelliError, SaltelliMatrix, Sampler, SobolDimSet,
    SobolSampler,
};

fn build(sampler: &dyn Sampler, n: usize, second_order: bool, stream: u64) -> SaltelliMatrix {
    build_saltelli_matrix(
        sampler,
        n,
        second_order,
        &mut RngState::from_parts([0x42; 32], stream, 0),
    )
    .expect("Saltelli matrix")
}
fn assert_bits_eq(a: &Array2<f64>, b: &Array2<f64>) {
    assert_eq!(a.mapv(f64::to_bits), b.mapv(f64::to_bits));
}
fn assert_matrices_identical(a: &SaltelliMatrix, b: &SaltelliMatrix) {
    assert_eq!((a.n, a.dim), (b.n, b.dim));
    assert_bits_eq(&a.a, &b.a);
    assert_bits_eq(&a.b, &b.b);
    assert_eq!(a.a_b.len(), b.a_b.len());
    for (x, y) in a.a_b.iter().zip(&b.a_b) {
        assert_bits_eq(x, y);
    }
    match (&a.b_a, &b.b_a) {
        (Some(x), Some(y)) => {
            assert_eq!(x.len(), y.len());
            for (x, y) in x.iter().zip(y) {
                assert_bits_eq(x, y);
            }
        }
        (None, None) => (),
        _ => panic!("second-order matrices differ"),
    }
}

#[test]
fn lhs_base_dim_six_produces_three_factor_matrices() {
    let m = build(&LhsSampler::classic(6), 64, false, 0);
    assert_eq!((m.n, m.dim), (64, 3));
    assert_eq!(m.a.shape(), [64, 3]);
    assert_eq!(m.b.shape(), [64, 3]);
    assert_eq!(m.a_b.len(), 3);
    for ab in &m.a_b {
        assert_eq!(ab.shape(), [64, 3]);
    }
}
#[test]
fn sobol_base_dim_eight_produces_four_factor_matrices() {
    let m = build(
        &SobolSampler::with(8, SobolDimSet::Standard, true),
        32,
        false,
        0,
    );
    assert_eq!((m.n, m.dim), (32, 4));
    assert_eq!(m.a.shape(), [32, 4]);
    assert_eq!(m.b.shape(), [32, 4]);
}
#[test]
fn a_b_replaces_only_column_i_with_b() {
    let m = build(&LhsSampler::classic(6), 32, false, 0);
    assert_eq!(m.a_b.len(), m.dim);
    for (i, ab) in m.a_b.iter().enumerate() {
        for j in 0..m.dim {
            let expected = if j == i { &m.b } else { &m.a };
            assert_eq!(ab.column(j), expected.column(j));
        }
    }
}
#[test]
fn second_order_b_a_replaces_only_column_i_with_a() {
    let m = build(&LhsSampler::classic(6), 16, true, 0);
    let ba = m.b_a.as_ref().expect("second-order matrices");
    assert_eq!(ba.len(), 3);
    for (i, ba) in ba.iter().enumerate() {
        assert_eq!(ba.shape(), [16, 3]);
        for j in 0..m.dim {
            let expected = if j == i { &m.a } else { &m.b };
            assert_eq!(ba.column(j), expected.column(j));
        }
    }
}
#[test]
fn first_and_total_order_evaluation_count_is_n_times_d_plus_two() {
    assert_eq!(
        build(&LhsSampler::classic(6), 64, false, 0).total_evaluations(),
        320
    );
}
#[test]
fn second_order_evaluation_count_is_n_times_twice_d_plus_two() {
    assert_eq!(
        build(&LhsSampler::classic(6), 32, true, 0).total_evaluations(),
        256
    );
}
#[test]
fn a_and_b_are_disjoint_halves_of_base_sample() {
    let sampler = SobolSampler::with(6, SobolDimSet::Standard, false);
    let m = build(&sampler, 16, false, 0);
    let base = sampler.unit_sample(16, &mut RngState::from_parts([0x42; 32], 0, 0));
    for j in 0..3 {
        assert_eq!(m.a.column(j), base.column(j));
        assert_eq!(m.b.column(j), base.column(j + 3));
    }
}
#[test]
fn lhs_same_rng_produces_identical_matrices() {
    let sampler = LhsSampler::classic(6);
    assert_matrices_identical(
        &build(&sampler, 64, false, 0),
        &build(&sampler, 64, false, 0),
    );
}
#[test]
fn sobol_same_rng_produces_identical_matrices() {
    let sampler = SobolSampler::with(8, SobolDimSet::Standard, true);
    assert_matrices_identical(
        &build(&sampler, 32, false, 0),
        &build(&sampler, 32, false, 0),
    );
}
#[test]
fn distinct_lhs_streams_produce_different_matrices() {
    let sampler = LhsSampler::classic(4);
    assert_ne!(
        build(&sampler, 32, false, 1).a,
        build(&sampler, 32, false, 2).a
    );
}
#[test]
fn zero_rows_returns_zero_n_error() {
    let result = build_saltelli_matrix(
        &LhsSampler::classic(4),
        0,
        false,
        &mut RngState::from_seed([0x42; 32]),
    );
    assert!(matches!(result, Err(SaltelliError::ZeroN)), "{result:?}");
}
#[test]
fn odd_base_dimension_returns_odd_base_dim_error() {
    let result = build_saltelli_matrix(
        &LhsSampler::classic(5),
        32,
        false,
        &mut RngState::from_seed([0x42; 32]),
    );
    assert!(
        matches!(result, Err(SaltelliError::OddBaseDim { dim: 5 })),
        "{result:?}"
    );
}
