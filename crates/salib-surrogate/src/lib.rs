#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod active_subspace;
pub mod multi_index;
pub mod pce;
pub mod polynomial;
pub mod sparse_pce;

pub use active_subspace::{compute_active_subspace, ActiveSubspace, ActiveSubspaceError};
pub use multi_index::{enumerate_hyperbolic, enumerate_total_degree, MultiIndex, MultiIndexError};
pub use pce::{fit_full_pce, sobol_indices_from_pce, PceError, PolynomialChaos, SobolFromPce};
pub use polynomial::{evaluate, norm_squared, PolynomialFamily};
pub use sparse_pce::{fit_sparse_pce, SparseFitDiagnostic, SparseSolver, TruncationScheme};
