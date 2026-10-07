#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(clippy::disallowed_methods)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod distribution;
#[cfg(kani)]
pub mod kani;
pub mod problem;
pub mod reduce;
pub mod rng;
pub mod types;

pub use distribution::Distribution;
pub use problem::{BuildError, Factor, FactorKind, Group, Problem, ProblemBuilder};
pub use reduce::{par_tree_dot, par_tree_sum, par_tree_var, tree_dot, tree_sum, tree_var, BLOCK};
pub use rng::{RngAlgorithm, RngState};
pub use types::{FiniteError, FiniteF64, Probability, ProbabilityError};
