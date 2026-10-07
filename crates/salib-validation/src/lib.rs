#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod analytic;
pub mod ishigami;
pub mod morris_test;
pub mod sobol_g;

pub use analytic::{MorrisEffectsAnalytic, SobolIndicesAnalytic};
