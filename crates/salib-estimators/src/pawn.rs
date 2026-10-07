//! PAWN sensitivity estimates from conditional and unconditional
//! empirical output CDFs.
//!
//! For each input, the estimator splits observations into `S`
//! approximately equal-frequency slices without splitting tied values, and computes
//!
//! ```text
//! KS_k = max_y |F_Y(y) − F_{Y|slice_k}(y)|.
//! ```
//!
//! It returns the median, maximum, mean, minimum, and coefficient of
//! variation of these slice distances. The median and maximum were
//! both discussed in [Pianosi and Wagener (2015)](https://doi.org/10.1016/j.envsoft.2015.01.004).
//! The median describes a typical slice; the maximum records the
//! largest observed change. Neither is a variance contribution.
//!
//! [Pianosi and Wagener (2018)](https://doi.org/10.1016/j.envsoft.2018.07.019)
//! introduced estimation from a generic input-output sample using
//! equal-width conditioning intervals. This implementation
//! uses equal-frequency rank slices of aligned `(X, Y)` data. The
//! sample must represent the input distribution of interest.
//!
//! No density bandwidth or numerical integration grid is needed.
//! Slice count is still a tuning parameter: more slices provide
//! finer conditioning but fewer observations per conditional CDF.
//! Check sensitivity to slice count and sample size on the model
//! being studied. The 2018 paper suggests starting with ten slices
//! and checking nearby counts. Different sensitivity methods can rank
//! inputs differently, including on the Ishigami function.
//!
//! Tied values are assigned by their sorted rank block midpoint; empty
//! slices are omitted from the summaries. Discrete inputs can therefore
//! produce fewer slices than requested. A constant input gives zero.
//! Computation is deterministic for the same ordered data and slice
//! count; this does not remove sampling or partitioning error.

#![allow(
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::many_single_char_names,
    clippy::items_after_statements,
    clippy::needless_range_loop
)]

use std::cmp::Ordering;
use std::fmt;

use crate::conditioning::classes;
#[cfg(test)]
use ndarray::Array2;
use ndarray::ArrayView2;
use salib_core::tree_sum;

/// PAWN sensitivity index estimates per factor — five aggregation
/// statistics over slice-wise KS values.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct PawnIndices {
    /// Median KS across slices.
    pub median: Vec<f64>,
    /// Maximum KS across slices.
    pub maximum: Vec<f64>,
    /// Mean KS across slices.
    pub mean: Vec<f64>,
    /// Minimum KS across slices.
    pub minimum: Vec<f64>,
    /// Coefficient of variation `std / mean` of the KS slice values.
    pub cv: Vec<f64>,
}

impl PawnIndices {
    /// Factor count.
    #[must_use]
    pub fn d(&self) -> usize {
        self.median.len()
    }
}

impl fmt::Display for PawnIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PAWN indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(
            f,
            "  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}",
            "Factor", "median", "mean", "max", "min", "CV"
        )?;
        writeln!(
            f,
            "  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}",
            "------", "------", "------", "------", "------", "------"
        )?;
        for i in 0..self.d() {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}  {:>8.4}  {:>8.4}  {:>8.4}",
                i, self.median[i], self.mean[i], self.maximum[i], self.minimum[i], self.cv[i]
            )?;
        }
        Ok(())
    }
}

/// Errors from [`estimate_pawn`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum PawnError {
    /// An input observation is NaN or infinite.
    #[error("input at row {row}, column {column} must be finite")]
    NonfiniteInput {
        /// Zero-based input row.
        row: usize,
        /// Zero-based input column.
        column: usize,
    },
    /// An output observation is NaN or infinite.
    #[error("PAWN: output at index {index} must be finite")]
    NonfiniteOutput {
        /// Zero-based index that failed validation.
        index: usize,
    },
    /// Input and output shapes are incompatible.
    #[error("PAWN: shape mismatch — X has {x_rows} rows, y has {y_len} elements")]
    ShapeMismatch {
        /// Number of rows in the input matrix.
        x_rows: usize,
        /// Number of output observations.
        y_len: usize,
    },
    /// At least one input dimension is required.
    #[error("PAWN: d must be ≥ 1, got 0")]
    ZeroD,
    /// At least two conditioning slices are required.
    #[error("PAWN: n_slices must be ≥ 2, got {n_slices}")]
    TooFewSlices {
        /// Number of conditioning slices supplied.
        n_slices: usize,
    },
    /// Need at least 2 samples per slice for the KS statistic to be
    /// meaningful: `N ≥ 2 · n_slices`.
    #[error("PAWN: N must be ≥ 2·n_slices (got N={n}, n_slices={n_slices}, minimum={minimum})")]
    InsufficientSamples {
        /// Sample count supplied by the caller.
        n: usize,
        /// Number of conditioning slices supplied.
        n_slices: usize,
        /// Minimum sample count required by this configuration.
        minimum: usize,
    },
}

/// Estimate PAWN per factor from generic `(X, Y)` data.
///
/// `n_slices` is the conditioning slice count. Larger values give
/// finer conditioning but fewer observations per slice. Compare
/// results across slice counts; the minimum accepted sample size
/// is an input check, not an accuracy guarantee.
///
/// # Errors
///
/// - [`PawnError::NonfiniteInput`] for any NaN or infinite input.
///
/// - [`PawnError::NonfiniteOutput`] if any output is NaN or infinite.
/// - [`PawnError::ShapeMismatch`] if `x.nrows() != y.len()`.
/// - [`PawnError::ZeroD`] if `x.ncols() == 0`.
/// - [`PawnError::TooFewSlices`] if `n_slices < 2`.
/// - [`PawnError::InsufficientSamples`] if `N < 2 · n_slices`.
pub fn estimate_pawn(
    x: ArrayView2<'_, f64>,
    y: &[f64],
    n_slices: usize,
) -> Result<PawnIndices, PawnError> {
    let n = x.nrows();
    let d = x.ncols();
    if d == 0 {
        return Err(PawnError::ZeroD);
    }
    if y.len() != n {
        return Err(PawnError::ShapeMismatch {
            x_rows: n,
            y_len: y.len(),
        });
    }
    if n_slices < 2 {
        return Err(PawnError::TooFewSlices { n_slices });
    }
    let minimum = n_slices.saturating_mul(2);
    if n < minimum {
        return Err(PawnError::InsufficientSamples {
            n,
            n_slices,
            minimum,
        });
    }

    if let Some(index) = y.iter().position(|v| !v.is_finite()) {
        return Err(PawnError::NonfiniteOutput { index });
    }

    if let Some(((row, column), _)) = x.indexed_iter().find(|(_, v)| !v.is_finite()) {
        return Err(PawnError::NonfiniteInput { row, column });
    }

    // Sort Y once for the unconditional empirical CDF.
    let mut y_sorted = y.to_vec();
    y_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let mut median = vec![0.0_f64; d];
    let mut maximum = vec![0.0_f64; d];
    let mut mean = vec![0.0_f64; d];
    let mut minimum_arr = vec![0.0_f64; d];
    let mut cv = vec![0.0_f64; d];

    let mut x_col_buf = vec![0.0_f64; n];
    let mut ks_per_slice = Vec::with_capacity(n_slices);
    let mut slice_y_buf: Vec<f64> = Vec::with_capacity(n);

    for i in 0..d {
        for k in 0..n {
            x_col_buf[k] = x[[k, i]];
        }
        let groups = classes(&x_col_buf, n_slices);
        ks_per_slice.clear();
        for group in groups {
            slice_y_buf.clear();
            slice_y_buf.extend(group.iter().map(|&k| y[k]));
            slice_y_buf.sort_by(f64::total_cmp);
            ks_per_slice.push(ks_two_sample_sorted(&y_sorted, &slice_y_buf));
        }

        // Aggregate.
        let aggregates = summarize(&ks_per_slice);
        median[i] = aggregates.median;
        maximum[i] = aggregates.max;
        mean[i] = aggregates.mean;
        minimum_arr[i] = aggregates.min;
        cv[i] = aggregates.cv;
    }

    Ok(PawnIndices {
        median,
        maximum,
        mean,
        minimum: minimum_arr,
        cv,
    })
}

struct Summary {
    min: f64,
    max: f64,
    mean: f64,
    median: f64,
    cv: f64,
}

fn summarize(values: &[f64]) -> Summary {
    debug_assert!(!values.is_empty());
    let n = values.len() as f64;
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let median = if sorted.len().is_multiple_of(2) {
        0.5 * (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2])
    } else {
        sorted[sorted.len() / 2]
    };
    let mean = tree_sum(values) / n;
    let var: f64 = tree_sum(
        &values
            .iter()
            .map(|&v| (v - mean).powi(2))
            .collect::<Vec<f64>>(),
    ) / n;
    let std = var.sqrt();
    let cv = if mean.abs() > 1e-15 { std / mean } else { 0.0 };
    Summary {
        min,
        max,
        mean,
        median,
        cv,
    }
}

/// Two-sample Kolmogorov-Smirnov statistic between two **sorted**
/// samples. Matches `scipy.stats.ks_2samp(...).statistic` (two-sided)
/// modulo identical tie-handling.
///
/// `KS = max_y |F_a(y) − F_b(y)|` evaluated at every observed `y` in
/// `a ∪ b`.
fn ks_two_sample_sorted(a: &[f64], b: &[f64]) -> f64 {
    debug_assert!(!a.is_empty() && !b.is_empty());
    let n_a = a.len() as f64;
    let n_b = b.len() as f64;
    let mut i = 0;
    let mut j = 0;
    let mut max_diff = 0.0_f64;

    while i < a.len() || j < b.len() {
        // Pick the smaller next observation; on tie advance both
        // sides past the equal value to avoid double-counting at
        // ties (matches `scipy`'s `searchsorted(side='right')`).
        let v = if i >= a.len() {
            b[j]
        } else if j >= b.len() {
            a[i]
        } else {
            a[i].min(b[j])
        };
        while i < a.len() && a[i] <= v {
            i += 1;
        }
        while j < b.len() && b[j] <= v {
            j += 1;
        }
        let f_a = (i as f64) / n_a;
        let f_b = (j as f64) / n_b;
        let diff = (f_a - f_b).abs();
        if diff > max_diff {
            max_diff = diff;
        }
    }
    max_diff
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::approx_constant)]
mod tests {
    use super::*;

    #[test]
    fn pawn_rejects_nonfinite_outputs_promptly() {
        let (tx, rx) = std::sync::mpsc::channel();
        let worker = std::thread::spawn(move || {
            for output in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let x = synthetic_x(8, 1);
                let model = |_: f64| output;
                let y: Vec<f64> = x.column(0).iter().map(|&v| model(v)).collect();
                if estimate_pawn(x.view(), &y, 2).is_ok() {
                    tx.send(false).unwrap();
                    return;
                }
            }
            tx.send(true).unwrap();
        });
        assert!(rx
            .recv_timeout(std::time::Duration::from_secs(2))
            .expect("PAWN must return promptly for nonfinite outputs"));
        worker.join().unwrap();
    }

    fn synthetic_x(n: usize, d: usize) -> Array2<f64> {
        let mut x = Array2::<f64>::zeros((n, d));
        for j in 0..d {
            let mut perm: Vec<usize> = (0..n).collect();
            let mut state: u64 = 0x9E37_79B9_7F4A_7C15_u64.wrapping_mul((j as u64).wrapping_add(1));
            for i in (1..n).rev() {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1);
                let k = (state >> 33) as usize % (i + 1);
                perm.swap(i, k);
            }
            for i in 0..n {
                x[[i, j]] = (perm[i] as f64 + 0.5) / (n as f64);
            }
        }
        x
    }

    // ── Validation ────────────────────────────────────────────────

    #[test]
    fn zero_d_errors() {
        let x = Array2::<f64>::zeros((100, 0));
        let y = vec![0.0; 100];
        assert_eq!(
            estimate_pawn(x.view(), &y, 10).unwrap_err(),
            PawnError::ZeroD
        );
    }

    #[test]
    fn shape_mismatch_errors() {
        let x = Array2::<f64>::zeros((100, 3));
        let y = vec![0.0; 50];
        let err = estimate_pawn(x.view(), &y, 10).unwrap_err();
        assert!(matches!(err, PawnError::ShapeMismatch { .. }));
    }

    #[test]
    fn too_few_slices_errors() {
        let x = synthetic_x(100, 3);
        let y = vec![0.0; 100];
        assert_eq!(
            estimate_pawn(x.view(), &y, 1).unwrap_err(),
            PawnError::TooFewSlices { n_slices: 1 }
        );
    }

    #[test]
    fn insufficient_samples_errors() {
        let x = synthetic_x(15, 3);
        let y = vec![0.0; 15];
        let err = estimate_pawn(x.view(), &y, 10).unwrap_err();
        assert!(matches!(err, PawnError::InsufficientSamples { .. }));
    }

    // ── Output shape ──────────────────────────────────────────────

    #[test]
    fn output_length_matches_d() {
        let n = 256;
        let x = synthetic_x(n, 5);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let est = estimate_pawn(x.view(), &y, 10).unwrap();
        assert_eq!(est.d(), 5);
        assert_eq!(est.median.len(), 5);
        assert_eq!(est.maximum.len(), 5);
        assert_eq!(est.mean.len(), 5);
        assert_eq!(est.minimum.len(), 5);
        assert_eq!(est.cv.len(), 5);
    }

    // ── Indices in [0, 1] ─────────────────────────────────────────

    #[test]
    fn indices_in_unit_interval() {
        // KS statistic is always in [0, 1].
        let n = 256;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + 0.5 * x[[k, 1]]).collect();
        let est = estimate_pawn(x.view(), &y, 10).unwrap();
        for i in 0..3 {
            assert!(
                (0.0..=1.0).contains(&est.median[i]),
                "median_{i} = {} not in [0, 1]",
                est.median[i]
            );
            assert!(
                (0.0..=1.0).contains(&est.maximum[i]),
                "max_{i} = {} not in [0, 1]",
                est.maximum[i]
            );
            assert!(
                est.minimum[i] >= 0.0 && est.minimum[i] <= est.maximum[i],
                "min_{i} {} not in [0, max_{i}={}]",
                est.minimum[i],
                est.maximum[i]
            );
        }
    }

    // ── min ≤ median ≤ max ───────────────────────────────────────

    #[test]
    fn aggregate_ordering_holds() {
        let n = 256;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + x[[k, 1]] * x[[k, 2]]).collect();
        let est = estimate_pawn(x.view(), &y, 10).unwrap();
        for i in 0..3 {
            assert!(
                est.minimum[i] <= est.median[i] + 1e-12,
                "min_{i} {} > median_{i} {}",
                est.minimum[i],
                est.median[i]
            );
            assert!(
                est.median[i] <= est.maximum[i] + 1e-12,
                "median_{i} {} > max_{i} {}",
                est.median[i],
                est.maximum[i]
            );
            assert!(
                est.minimum[i] <= est.mean[i] + 1e-12,
                "min_{i} {} > mean_{i} {}",
                est.minimum[i],
                est.mean[i]
            );
            assert!(
                est.mean[i] <= est.maximum[i] + 1e-12,
                "mean_{i} {} > max_{i} {}",
                est.mean[i],
                est.maximum[i]
            );
        }
    }

    // ── Linear single-factor: factor 0 dominates ─────────────────

    #[test]
    fn linear_single_factor_dominates_pawn() {
        let n = 512;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]]).collect();
        let est = estimate_pawn(x.view(), &y, 10).unwrap();
        assert!(
            est.median[0] > est.median[1],
            "median_0 = {} should exceed median_1 = {}",
            est.median[0],
            est.median[1]
        );
        assert!(
            est.median[0] > est.median[2],
            "median_0 = {} should exceed median_2 = {}",
            est.median[0],
            est.median[2]
        );
    }

    // ── Determinism ───────────────────────────────────────────────

    #[test]
    fn same_input_yields_identical_output() {
        let n = 64;
        let x = synthetic_x(n, 3);
        let y: Vec<f64> = (0..n).map(|k| x[[k, 0]] + x[[k, 1]] * x[[k, 2]]).collect();
        let a = estimate_pawn(x.view(), &y, 8).unwrap();
        let b = estimate_pawn(x.view(), &y, 8).unwrap();
        assert_eq!(a.median, b.median);
        assert_eq!(a.maximum, b.maximum);
    }

    // ── KS statistic unit tests ───────────────────────────────────

    #[test]
    fn ks_identical_samples_zero() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(ks_two_sample_sorted(&a, &a), 0.0);
    }

    #[test]
    fn ks_disjoint_samples_one() {
        let a = vec![0.0, 1.0, 2.0];
        let b = vec![10.0, 11.0, 12.0];
        // F_a(2) = 1, F_b(2) = 0 → KS = 1.
        assert_eq!(ks_two_sample_sorted(&a, &b), 1.0);
    }

    #[test]
    fn ks_known_distance_one_third() {
        // a = [0, 1, 2, 3, 4, 5], b = [3, 4, 5, 6, 7, 8].
        // At y=2: F_a = 3/6 = 0.5, F_b = 0/6 = 0   → diff 0.5.
        // At y=5: F_a = 6/6 = 1.0, F_b = 3/6 = 0.5 → diff 0.5.
        // Max diff = 0.5.
        let a: Vec<f64> = (0..6).map(f64::from).collect();
        let b: Vec<f64> = (3..9).map(f64::from).collect();
        let ks = ks_two_sample_sorted(&a, &b);
        assert!((ks - 0.5).abs() < 1e-12, "KS = {ks}, expected 0.5");
    }

    #[test]
    fn ks_handles_ties() {
        let a = vec![1.0, 1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 2.0, 3.0];
        // KS at observed values: at y=1: F_a=2/4=0.5, F_b=1/4=0.25 → 0.25.
        // At y=2: F_a=3/4=0.75, F_b=3/4=0.75 → 0.
        // At y=3: F_a=1, F_b=1 → 0.
        // Max = 0.25.
        let ks = ks_two_sample_sorted(&a, &b);
        assert!((ks - 0.25).abs() < 1e-12, "KS = {ks}, expected 0.25");
    }

    // ── Helpers ───────────────────────────────────────────────────

    #[test]
    fn summarize_basic_stats() {
        let s = summarize(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(s.min, 1.0);
        assert_eq!(s.max, 5.0);
        assert_eq!(s.median, 3.0);
        assert_eq!(s.mean, 3.0);
    }

    #[test]
    fn summarize_even_count_median_averages() {
        let s = summarize(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(s.median, 2.5);
    }
}
