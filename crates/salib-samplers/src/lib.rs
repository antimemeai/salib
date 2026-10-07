#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod fast;
pub mod harmonic_budget;
pub mod iman_conover;
#[cfg(kani)]
pub mod kani;
pub mod lhs;
pub mod morris;
pub mod owen_matrix;
pub mod plackett_burman;
pub mod saltelli_matrix;
pub mod sampler;
pub mod sobol;

pub use fast::{build_fast_design, build_fast_design_with_budget, FastDesign, FastError};
pub use harmonic_budget::{HarmonicBudget, HarmonicError};
pub use iman_conover::{iman_conover_transform, ImanConoverError};
pub use lhs::{LhsKind, LhsSampler};
pub use morris::{
    build_grouped_morris_trajectories, build_morris_trajectories, MorrisError, MorrisTrajectories,
};
pub use owen_matrix::{build_owen_matrix, OwenMatrix, OwenMatrixError};
pub use plackett_burman::{build_plackett_burman, PbError, PlackettBurmanDesign};
pub use saltelli_matrix::{
    build_grouped_saltelli_matrix, build_saltelli_matrix, SaltelliError, SaltelliMatrix,
};
pub use sampler::Sampler;
pub use sobol::{SobolDimSet, SobolSampler};
