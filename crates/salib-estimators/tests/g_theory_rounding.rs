//! Rounding-residue snapping against exact rational ANOVA (#3, #4).
//!
//! Integer grids admit exact variance components: with T terms
//! `N·T_e = Σ_levels (level sum)² · (N / cells per level)`, every `N·SS` is an
//! integer. An exactly zero component must be reported as exactly 0.0; every
//! other estimate must lie within the documented snap bound of its exact value.
#![allow(
    clippy::unwrap_used,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]

mod g_theory_fixtures;

use g_theory_fixtures::XorShift;
use ndarray::Array3;
use salib_estimators::{estimate_g_theory_pir, GTheoryDesign};

fn sq(v: i128) -> i128 {
    v.checked_mul(v).unwrap()
}

/// Exact `(numerator, denominator)` of each component, plus the mean squares
/// as f64 for the bound.
fn exact_components(y: &Array3<i64>) -> ([(i128, i128); 7], [f64; 7]) {
    let (p, i, r) = y.dim();
    let n = (p * i * r) as i128;
    // N·T for one effect: Σ over its levels of (level sum)² × (number of levels).
    let t = |levels: usize, level: &dyn Fn(usize, usize, usize) -> usize| -> i128 {
        let mut sums = vec![0_i128; levels];
        for ((a, b, c), v) in y.indexed_iter() {
            sums[level(a, b, c)] += i128::from(*v);
        }
        sums.iter().fold(0_i128, |acc, s| {
            acc.checked_add(sq(*s).checked_mul(levels as i128).unwrap())
                .unwrap()
        })
    };
    let t0 = t(1, &|_, _, _| 0);
    let tp = t(p, &|a, _, _| a);
    let ti = t(i, &|_, b, _| b);
    let tr = t(r, &|_, _, c| c);
    let tpi = t(p * i, &|a, b, _| a * i + b);
    let tpr = t(p * r, &|a, _, c| a * r + c);
    let tir = t(i * r, &|_, b, c| b * r + c);
    let tpir = t(p * i * r, &|a, b, c| (a * i + b) * r + c);
    // N·SS for p, i, r, pi, pr, ir, pir.
    let ss = [
        tp - t0,
        ti - t0,
        tr - t0,
        tpi - tp - ti + t0,
        tpr - tp - tr + t0,
        tir - ti - tr + t0,
        tpir - tpi - tpr - tir + tp + ti + tr - t0,
    ];
    let (p1, i1, r1) = ((p - 1) as i128, (i - 1) as i128, (r - 1) as i128);
    let df = [p1, i1, r1, p1 * i1, p1 * r1, i1 * r1, p1 * i1 * r1];
    let l: i128 = df.iter().product();
    // (signs over [p, i, r, pi, pr, ir, pir] mean squares, divisor) per component.
    let rows: [([i128; 7], i128); 7] = [
        ([1, 0, 0, -1, -1, 0, 1], (i * r) as i128),
        ([0, 1, 0, -1, 0, -1, 1], (p * r) as i128),
        ([0, 0, 1, 0, -1, -1, 1], (p * i) as i128),
        ([0, 0, 0, 1, 0, 0, -1], r as i128),
        ([0, 0, 0, 0, 1, 0, -1], i as i128),
        ([0, 0, 0, 0, 0, 1, -1], p as i128),
        ([0, 0, 0, 0, 0, 0, 1], 1),
    ];
    let components = rows.map(|(signs, divisor)| {
        let num = (0..7).fold(0_i128, |acc, j| {
            acc.checked_add(signs[j] * ss[j].checked_mul(l / df[j]).unwrap())
                .unwrap()
        });
        (num, n * l * divisor)
    });
    let ms = std::array::from_fn(|j| ss[j] as f64 / n as f64 / df[j] as f64);
    (components, ms)
}

fn check(y: &Array3<i64>) {
    let (components, ms) = exact_components(y);
    let (p, i, r) = y.dim();
    let n = (p * i * r) as f64;
    let max_abs_y = y.iter().map(|v| v.unsigned_abs()).max().unwrap() as f64;
    let mean = y.iter().map(|v| *v as f64).sum::<f64>() / n;
    let max_abs_z = y
        .iter()
        .map(|v| (*v as f64 - mean).abs())
        .fold(0.0, f64::max);
    let estimate =
        estimate_g_theory_pir(y.mapv(|v| v as f64).view(), GTheoryDesign::Crossed).unwrap();
    let (k8, eps) = (8.0, f64::EPSILON);
    let delta = k8 * n * eps * max_abs_z;
    let terms: [(&[usize], f64); 7] = [
        (&[0, 3, 4, 6], (i * r) as f64),
        (&[1, 3, 5, 6], (p * r) as f64),
        (&[2, 4, 5, 6], (p * i) as f64),
        (&[3, 6], r as f64),
        (&[4, 6], i as f64),
        (&[5, 6], p as f64),
        (&[6], 1.0),
    ];
    for (k, ((num, den), (used, divisor))) in components.iter().zip(terms).enumerate() {
        let actual = estimate.components()[k];
        if *num == 0 {
            assert_eq!(
                actual.to_bits(),
                0.0_f64.to_bits(),
                "{:?} component {k}",
                y.dim()
            );
            continue;
        }
        let exact = *num as f64 / *den as f64;
        let magnitude: f64 = used.iter().map(|&j| ms[j]).sum::<f64>() / divisor;
        let bound = delta * delta
            + delta * magnitude.sqrt()
            + k8 * n * eps * magnitude
            + (4.0 * eps * max_abs_y).powi(2);
        assert!(
            (actual - exact).abs() <= bound * (1.0 + 1e-9) + 4.0 * eps * exact.abs(),
            "{:?} component {k}: actual {actual:e}, exact {exact:e}, bound {bound:e}",
            y.dim()
        );
    }
}

const SHAPES: [(usize, usize, usize); 8] = [
    (2, 2, 2),
    (4, 3, 2),
    (10, 4, 2),
    (30, 20, 10),
    (300, 2, 2),
    (2, 2, 500),
    (2, 300, 2),
    (40, 40, 3),
];

#[test]
fn additive_integer_grids_have_exactly_zero_interactions() {
    let mut rng = XorShift(0x2545_F491_4F6C_DD1D);
    for (p, i, r) in SHAPES {
        for offset in [0_i64, 7, -123_457, 1_000_000_007] {
            for scales in [(1_i64, 1, 1), (1000, 1, 1), (0, 5, 3), (9, 0, 0)] {
                let a: Vec<i64> = (0..p)
                    .map(|_| (rng.next() % 50) as i64 * scales.0)
                    .collect();
                let b: Vec<i64> = (0..i)
                    .map(|_| (rng.next() % 50) as i64 * scales.1)
                    .collect();
                let c: Vec<i64> = (0..r)
                    .map(|_| (rng.next() % 50) as i64 * scales.2)
                    .collect();
                let y = Array3::from_shape_fn((p, i, r), |(x, u, v)| offset + a[x] + b[u] + c[v]);
                if y.iter().all(|v| *v == y[[0, 0, 0]]) {
                    continue;
                }
                check(&y);
            }
        }
    }
}

#[test]
fn integer_grids_with_interactions_stay_within_bound_of_exact() {
    let mut rng = XorShift(0x9E37_79B9_7F4A_7C15);
    for (p, i, r) in SHAPES {
        for offset in [0_i64, -999, 1_000_000_007] {
            for levels in [2_u64, 7, 1000] {
                let y = Array3::from_shape_fn((p, i, r), |_| offset + (rng.next() % levels) as i64);
                check(&y);
            }
        }
    }
}

/// A dominant effect must not hide small real components: every nonzero exact
/// component is reported nonzero, and within the error bound checked above.
#[test]
fn dominant_effects_do_not_snap_real_components() {
    let mut rng = XorShift(0x6A09_E667_F3BC_C908);
    for ((p, i, r), mult) in [
        ((50, 20, 10), 100_000_i64),
        ((10, 4, 2), 10_000_000),
        ((200, 50, 3), 100_000),
        ((4, 3, 300), 1_000_000),
    ] {
        let a: Vec<i64> = (0..p).map(|_| (rng.next() % 10) as i64 * mult).collect();
        let y = Array3::from_shape_fn((p, i, r), |(x, _, _)| a[x] + (rng.next() % 3) as i64);
        check(&y);
        let (components, _) = exact_components(&y);
        let estimate =
            estimate_g_theory_pir(y.mapv(|v| v as f64).view(), GTheoryDesign::Crossed).unwrap();
        for (k, (num, _)) in components.iter().enumerate() {
            assert_eq!(
                *num == 0,
                estimate.components()[k] == 0.0,
                "{:?} x{mult} component {k}: exact numerator {num}, reported {}",
                y.dim(),
                estimate.components()[k]
            );
        }
    }
}
