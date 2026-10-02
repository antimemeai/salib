//! ChaCha20 determinism, stream forking, and snapshot resumption.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use rand_chacha::{rand_core::RngCore, ChaCha20Rng};
use salib_core::RngState;

const SEED: [u8; 32] = [0x42; 32];

fn draw(rng: &mut ChaCha20Rng, n: usize) -> Vec<u8> {
    let mut bytes = vec![0; n];
    rng.fill_bytes(&mut bytes);
    bytes
}

#[test]
fn same_seed_stream_and_position_produce_identical_bytes() {
    for position in [0, 1234] {
        let mut a = RngState::from_parts(SEED, 7, position).into_chacha();
        let mut b = RngState::from_parts(SEED, 7, position).into_chacha();
        assert_eq!(draw(&mut a, 1024), draw(&mut b, 1024));
    }
}
#[test]
fn same_salt_produces_equal_forks() {
    let parent = RngState::from_parts(SEED, 0, 0);
    assert_eq!(
        parent.fork(b"saltelli-block-0"),
        parent.fork(b"saltelli-block-0")
    );
}
#[test]
fn distinct_salts_produce_distinct_streams_and_bytes() {
    let parent = RngState::from_parts(SEED, 0, 0);
    let a = parent.fork(b"saltelli-block-0");
    let b = parent.fork(b"saltelli-block-1");
    assert_ne!(a.stream, b.stream);
    assert_ne!(
        draw(&mut a.into_chacha(), 1024),
        draw(&mut b.into_chacha(), 1024)
    );
}
#[test]
fn fork_is_pure_under_parent_stream_and_seed() {
    let parent = RngState::from_parts(SEED, 12345, 0);
    let fresh = RngState::from_parts(parent.seed, parent.stream, parent.word_pos);
    assert_eq!(
        parent.fork(b"saltelli-block-0"),
        fresh.fork(b"saltelli-block-0")
    );
}
#[test]
fn word_position_snapshot_resumes_unbroken_draw() {
    let state = RngState::from_parts(SEED, 0, 0);
    let mut original = state.clone().into_chacha();
    let _ = draw(&mut original, 8192);
    let snapshot = RngState::snapshot(&original, &state);
    assert_eq!(snapshot.word_pos, original.get_word_pos());
    let mut resumed = snapshot.into_chacha();
    assert_eq!(draw(&mut resumed, 1024), draw(&mut original, 1024));
}
