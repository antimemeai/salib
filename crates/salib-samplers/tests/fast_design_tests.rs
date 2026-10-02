//! FAST search curve bounds, frequency structure, and determinism.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use salib_core::RngState;
use salib_samplers::{build_fast_design, FastDesign};

fn design(dim: usize) -> FastDesign {
    build_fast_design(dim, 129, 4, &mut RngState::from_seed([0x42; 32])).expect("FAST design")
}
#[test]
fn samples_are_in_closed_unit_interval() {
    for &value in &design(6).samples {
        assert!(value.is_finite() && (0.0..=1.0).contains(&value));
    }
}
#[test]
fn factor_of_interest_has_maximum_frequency() {
    let d = design(6);
    for i in 0..d.d {
        for j in 0..d.d {
            if j != i {
                assert!(d.omegas[[i, j]] < d.omegas[[i, i]]);
            }
        }
    }
}
#[test]
fn complementary_frequencies_obey_harmonic_bandwidth_bound() {
    let d = design(6);
    let bound = d.omegas[[0, 0]] / (2 * d.harmonic);
    for i in 0..d.d {
        for j in 0..d.d {
            if j != i {
                assert!(d.omegas[[i, j]] <= bound);
            }
        }
    }
}
#[test]
fn complementary_frequencies_are_distinct_in_linspace_regime() {
    let d = design(3);
    for i in 0..d.d {
        let mut frequencies: Vec<_> = (0..d.d)
            .filter(|&j| j != i)
            .map(|j| d.omegas[[i, j]])
            .collect();
        frequencies.sort_unstable();
        frequencies.dedup();
        assert_eq!(frequencies.len(), d.d - 1);
    }
}
#[test]
fn same_seed_produces_bit_identical_design() {
    let a = design(6);
    let b = design(6);
    assert_eq!(a.samples.mapv(f64::to_bits), b.samples.mapv(f64::to_bits));
    assert_eq!(a.omegas, b.omegas);
    assert_eq!(
        a.phases.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        b.phases.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
}
