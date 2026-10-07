//! Optional Arrow and Polars conversions for result tables.
//! Enabled by the facade’s `arrow` and `polars` features.

#[cfg(feature = "arrow")]
pub mod arrow;

#[cfg(feature = "polars")]
pub mod polars;
