//! Latin hypercube structure and RNG determinism.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::float_cmp,
    clippy::cast_precision_loss
)]

use ndarray::Array2;
use salib_core::RngState;
use salib_samplers::{LhsSampler, Sampler};

fn draw(sampler: LhsSampler, n: usize, stream: u64) -> (Array2<f64>, RngState) {
    let mut rng = RngState::from_parts([0x42; 32], stream, 0);
    (sampler.unit_sample(n, &mut rng), rng)
}
fn assert_unit_interval(m: &Array2<f64>) {
    for &value in m {
        assert!(value.is_finite() && (0.0..1.0).contains(&value));
    }
}

#[test]
fn classic_shape_is_n_by_dim() {
    assert_eq!(draw(LhsSampler::classic(4), 64, 0).0.shape(), [64, 4]);
}
#[test]
fn classic_values_are_in_unit_interval() {
    assert_unit_interval(&draw(LhsSampler::classic(3), 128, 0).0);
}
#[test]
fn classic_stratifies_every_column() {
    let m = draw(LhsSampler::classic(5), 32, 0).0;
    assert_unit_interval(&m);
    for column in m.columns() {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let mut cells: Vec<usize> = column.iter().map(|x| (x * 32.0).floor() as usize).collect();
        cells.sort_unstable();
        assert_eq!(cells, (0..32).collect::<Vec<_>>());
    }
}
#[test]
fn centered_values_are_cell_centers() {
    let m = draw(LhsSampler::centered(3), 16, 0).0;
    assert_unit_interval(&m);
    for column in m.columns() {
        let mut sorted = column.to_vec();
        sorted.sort_by(f64::total_cmp);
        for (k, got) in sorted.into_iter().enumerate() {
            let expected = (k as f64 + 0.5) / 16.0;
            assert!(got.is_finite() && expected.is_finite());
            assert!((got - expected).abs() <= 1e-12);
        }
    }
}
#[test]
fn zero_rows_preserves_dim() {
    assert_eq!(draw(LhsSampler::classic(3), 0, 0).0.shape(), [0, 3]);
}
#[test]
fn zero_dim_preserves_rows() {
    assert_eq!(draw(LhsSampler::classic(0), 8, 0).0.shape(), [8, 0]);
}
#[test]
fn single_row_classic_is_in_range() {
    let m = draw(LhsSampler::classic(2), 1, 0).0;
    assert_eq!(m.shape(), [1, 2]);
    assert_unit_interval(&m);
}
#[test]
fn single_row_centered_is_half() {
    let m = draw(LhsSampler::centered(2), 1, 0).0;
    assert_eq!(m.shape(), [1, 2]);
    for &value in &m {
        assert_eq!(value, 0.5);
    }
}
#[test]
fn same_rng_produces_identical_matrix() {
    let a = draw(LhsSampler::classic(4), 64, 7).0;
    let b = draw(LhsSampler::classic(4), 64, 7).0;
    assert_eq!(a.mapv(f64::to_bits), b.mapv(f64::to_bits));
}
#[test]
fn same_rng_advances_identically() {
    assert_eq!(
        draw(LhsSampler::classic(4), 32, 0).1,
        draw(LhsSampler::classic(4), 32, 0).1
    );
}
#[test]
fn distinct_streams_produce_different_matrices() {
    assert_ne!(
        draw(LhsSampler::classic(3), 32, 1).0,
        draw(LhsSampler::classic(3), 32, 2).0
    );
}
#[test]
fn classic_draw_advances_word_position() {
    assert!(draw(LhsSampler::classic(3), 32, 0).1.word_pos > 0);
}
#[test]
fn zero_row_draw_consumes_no_rng() {
    assert_eq!(draw(LhsSampler::classic(3), 0, 0).1.word_pos, 0);
}
#[test]
fn centered_consumes_fewer_bytes_than_classic() {
    assert!(
        draw(LhsSampler::classic(2), 32, 0).1.word_pos
            > draw(LhsSampler::centered(2), 32, 0).1.word_pos
    );
}
