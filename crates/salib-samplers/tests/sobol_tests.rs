//! Sobol canonical values, structure, and determinism.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::float_cmp,
    clippy::cast_precision_loss
)]

use ndarray::Array2;
use salib_core::RngState;
use salib_samplers::{Sampler, SobolDimSet, SobolSampler};

fn draw(
    dim: usize,
    set: SobolDimSet,
    skip: bool,
    n: usize,
    stream: u64,
) -> (Array2<f64>, RngState) {
    let mut rng = RngState::from_parts([0; 32], stream, 0);
    (
        SobolSampler::with(dim, set, skip).unit_sample(n, &mut rng),
        rng,
    )
}
fn standard(dim: usize, skip: bool, n: usize) -> Array2<f64> {
    draw(dim, SobolDimSet::Standard, skip, n, 0).0
}
fn assert_unit_interval(m: &Array2<f64>) {
    for &value in m {
        assert!(value.is_finite() && (0.0..1.0).contains(&value));
    }
}

#[test]
fn dim_one_matches_canonical_halving_sequence() {
    assert_eq!(
        standard(1, false, 8).column(0).to_vec(),
        [0.0, 0.5, 0.75, 0.25, 0.375, 0.875, 0.625, 0.125]
    );
}
#[test]
fn dim_two_matches_canonical_joe_kuo_sequence() {
    assert_eq!(
        standard(2, false, 8).column(1).to_vec(),
        [0.0, 0.5, 0.25, 0.75, 0.375, 0.875, 0.125, 0.625]
    );
}
#[test]
fn skip_first_drops_origin() {
    assert_eq!(
        standard(1, true, 7).column(0).to_vec(),
        [0.5, 0.75, 0.25, 0.375, 0.875, 0.625, 0.125]
    );
}
#[test]
fn skip_first_false_keeps_origin() {
    assert_eq!(standard(3, false, 4).row(0).to_vec(), [0.0; 3]);
}
#[test]
fn output_shape_is_n_by_dim() {
    assert_eq!(standard(4, true, 64).shape(), [64, 4]);
}
#[test]
fn zero_rows_preserves_dim() {
    assert_eq!(standard(3, true, 0).shape(), [0, 3]);
}
#[test]
fn skipping_origin_keeps_values_in_range() {
    assert_unit_interval(&standard(5, true, 256));
}
#[test]
fn including_origin_keeps_values_in_range() {
    assert_unit_interval(&standard(3, false, 8));
}
#[test]
fn power_of_two_samples_stratify_each_column() {
    let m = standard(3, false, 1024);
    assert_unit_interval(&m);
    for column in m.columns() {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let mut bins: Vec<usize> = column
            .iter()
            .map(|x| (x * 1024.0).floor() as usize)
            .collect();
        bins.sort_unstable();
        assert_eq!(bins, (0..1024).collect::<Vec<_>>());
    }
}
#[test]
fn minimal_set_supports_100_dimensions() {
    assert_eq!(
        draw(100, SobolDimSet::Minimal, true, 16, 0).0.shape(),
        [16, 100]
    );
}
#[test]
fn standard_set_supports_500_dimensions() {
    assert_eq!(standard(500, true, 8).shape(), [8, 500]);
}
#[test]
fn same_config_produces_identical_output() {
    assert_eq!(
        standard(4, true, 128).mapv(f64::to_bits),
        standard(4, true, 128).mapv(f64::to_bits)
    );
}
#[test]
fn unscrambled_output_is_independent_of_rng_stream() {
    let a = draw(4, SobolDimSet::Standard, true, 64, 1).0;
    let b = draw(4, SobolDimSet::Standard, true, 64, 999).0;
    assert_eq!(a.mapv(f64::to_bits), b.mapv(f64::to_bits));
}
#[test]
fn unscrambled_draw_does_not_advance_word_position() {
    assert_eq!(draw(3, SobolDimSet::Standard, true, 64, 0).1.word_pos, 0);
}
