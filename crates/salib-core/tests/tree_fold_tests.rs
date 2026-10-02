//! Parallel tree reductions are bit identical across thread counts.
#![allow(clippy::unwrap_used, clippy::expect_used)]
#![cfg(feature = "parallel")]

use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use salib_core::{par_tree_dot, par_tree_sum, par_tree_var, tree_dot, tree_sum, tree_var};

const FIXTURE_SEED: [u8; 32] = [0x42; 32];

fn fixture_vec(stream: u64, n: usize) -> Vec<f64> {
    let mut rng = ChaCha20Rng::from_seed(FIXTURE_SEED);
    rng.set_stream(stream);
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        // Sample u64, normalize to [-1, 1] via two-step f64 widening.
        // Deterministic across platforms because the integer-to-f64
        // conversion is exact for `u64::MAX`-bounded values.
        let x = rng.next_u64();
        #[allow(clippy::cast_precision_loss)]
        let f = (x as f64) / (u64::MAX as f64);
        out.push(f.mul_add(2.0, -1.0));
    }
    out
}

fn install_pool<R, F>(threads: usize, f: F) -> R
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("build rayon pool");
    pool.install(f)
}

fn assert_bits_eq(a: f64, b: f64) {
    assert!(a.is_finite() && b.is_finite());
    assert_eq!(a.to_bits(), b.to_bits());
}
#[test]
fn parallel_sum_matches_sequential_at_each_thread_count() {
    for n in [4096, 65536] {
        let xs = fixture_vec(0, n);
        for threads in [1, 2, 8, 32] {
            assert_bits_eq(install_pool(threads, || par_tree_sum(&xs)), tree_sum(&xs));
        }
    }
}
#[test]
fn parallel_dot_matches_sequential() {
    for (n, threads) in [(4096, 8), (65536, 32)] {
        let a = fixture_vec(1, n);
        let b = fixture_vec(2, n);
        assert_bits_eq(
            install_pool(threads, || par_tree_dot(&a, &b)),
            tree_dot(&a, &b),
        );
    }
}
#[test]
fn parallel_sum_is_identical_across_reruns() {
    let xs = fixture_vec(0, 65536);
    assert_bits_eq(
        install_pool(8, || par_tree_sum(&xs)),
        install_pool(8, || par_tree_sum(&xs)),
    );
}
#[test]
fn parallel_variance_matches_sequential() {
    let xs = fixture_vec(0, 65536);
    assert_bits_eq(install_pool(32, || par_tree_var(&xs)), tree_var(&xs));
}
#[test]
fn empty_input_reduces_to_positive_zero() {
    assert_bits_eq(install_pool(8, || par_tree_sum(&[])), 0.0);
}
#[test]
fn single_element_reduces_to_itself() {
    assert_bits_eq(install_pool(8, || par_tree_sum(&[1.5])), 1.5);
}
