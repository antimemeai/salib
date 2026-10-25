//! Fiducial generalized-pivotal-quantity (GPQ) intervals (salib #10).
//!
//! Hannig, Iyer & Patterson (2006, JASA 101:254), Theorem 2: in the balanced
//! normal random model `SS_j / EMS_j ~ χ²(df_j)` independently, so
//! `R_j = SS_j / U_j` with `U_j ~ χ²(df_j)` is a fiducial GPQ for `EMS_j`.
//! Each draw maps the `R_j` through the expected-mean-square equations to
//! components, clipped at zero (their `max(0, R)`), and evaluates Phi, G and
//! the components. The interval is the percentile range of the draws.
//!
//! The harness computes the mean squares from the data itself and samples χ²
//! with its own Marsaglia–Tsang generator, independent of salib.
//!
//! Usage: `salib-gtheory-coverage gpq <n_sim> <draws> <out.md> [threads] [first_cell] [end_cell]`.

use std::error::Error;
use std::fmt::Write as _;
use std::io::Write as _;
use std::thread;

use ndarray::Array3;
use rand::RngCore;
use salib_core::RngState;

use super::{
    cells, coefficients, generate, open_unit, percentile, row, standard_normal, summarize, truth,
    Cell, Data, ALPHA, NAMES,
};

/// Mean squares of a balanced p × i × r grid, in (p, i, r, pi, pr, ir, pir) order,
/// and their degrees of freedom.
fn anova(y: &Array3<f64>) -> ([f64; 7], [f64; 7]) {
    let (np, ni, nr) = y.dim();
    let n = (np * ni * nr) as f64;
    let grand = y.sum() / n;
    let mut mp = vec![0.0; np];
    let mut mi = vec![0.0; ni];
    let mut mr = vec![0.0; nr];
    let mut mpi = vec![0.0; np * ni];
    let mut mpr = vec![0.0; np * nr];
    let mut mir = vec![0.0; ni * nr];
    for ((a, b, c), &v) in y.indexed_iter() {
        let d = v - grand;
        mp[a] += d;
        mi[b] += d;
        mr[c] += d;
        mpi[a * ni + b] += d;
        mpr[a * nr + c] += d;
        mir[b * nr + c] += d;
    }
    let (fp, fi, fr) = (np as f64, ni as f64, nr as f64);
    mp.iter_mut().for_each(|m| *m /= fi * fr);
    mi.iter_mut().for_each(|m| *m /= fp * fr);
    mr.iter_mut().for_each(|m| *m /= fp * fi);
    mpi.iter_mut().for_each(|m| *m /= fr);
    mpr.iter_mut().for_each(|m| *m /= fi);
    mir.iter_mut().for_each(|m| *m /= fp);
    let mut ss = [0.0_f64; 7];
    ss[0] = fi * fr * mp.iter().map(|m| m * m).sum::<f64>();
    ss[1] = fp * fr * mi.iter().map(|m| m * m).sum::<f64>();
    ss[2] = fp * fi * mr.iter().map(|m| m * m).sum::<f64>();
    for a in 0..np {
        for b in 0..ni {
            ss[3] += fr * (mpi[a * ni + b] - mp[a] - mi[b]).powi(2);
        }
        for c in 0..nr {
            ss[4] += fi * (mpr[a * nr + c] - mp[a] - mr[c]).powi(2);
        }
    }
    for b in 0..ni {
        for c in 0..nr {
            ss[5] += fp * (mir[b * nr + c] - mi[b] - mr[c]).powi(2);
        }
    }
    for ((a, b, c), &v) in y.indexed_iter() {
        let e =
            v - grand - mpi[a * ni + b] - mpr[a * nr + c] - mir[b * nr + c] + mp[a] + mi[b] + mr[c];
        ss[6] += e * e;
    }
    let df = [
        fp - 1.0,
        fi - 1.0,
        fr - 1.0,
        (fp - 1.0) * (fi - 1.0),
        (fp - 1.0) * (fr - 1.0),
        (fi - 1.0) * (fr - 1.0),
        (fp - 1.0) * (fi - 1.0) * (fr - 1.0),
    ];
    (std::array::from_fn(|k| ss[k] / df[k]), df)
}

/// Components from expected mean squares (the EMS equations, inverted).
fn components(ems: &[f64; 7], s: [usize; 3]) -> [f64; 7] {
    let [np, ni, nr] = s.map(|v| v as f64);
    let [p, i, r, pi, pr, ir, pir] = *ems;
    [
        (p - pi - pr + pir) / (ni * nr),
        (i - pi - ir + pir) / (np * nr),
        (r - pr - ir + pir) / (np * ni),
        (pi - pir) / nr,
        (pr - pir) / ni,
        (ir - pir) / np,
        pir,
    ]
}

/// Gamma(shape, 1) by Marsaglia & Tsang (2000), with the `U^(1/a)` boost
/// for shape < 1.
fn gamma(shape: f64, rng: &mut impl RngCore) -> f64 {
    if shape < 1.0 {
        return gamma(shape + 1.0, rng) * open_unit(rng).powf(1.0 / shape);
    }
    let d = shape - 1.0 / 3.0;
    let c = 1.0 / (9.0 * d).sqrt();
    loop {
        let x = standard_normal(rng);
        let v = (1.0 + c * x).powi(3);
        if v <= 0.0 {
            continue;
        }
        if open_unit(rng).ln() < 0.5 * x * x + d - d * v + d * v.ln() {
            return d * v;
        }
    }
}

fn chi_square(df: f64, rng: &mut impl RngCore) -> f64 {
    2.0 * gamma(df / 2.0, rng)
}

#[derive(Default, Clone)]
struct GpqRep {
    error: bool,
    /// Clipped point components from the harness's own ANOVA.
    point: Option<[f64; 7]>,
    /// Intervals for the seven components, then G and Phi, then G and Phi
    /// with calibrated percentile levels.
    intervals: [Option<(f64, f64)>; 11],
}

/// Simulated datasets and GPQ draws per dataset for the level calibration.
const CAL_OUTER: usize = 400;
const CAL_INNER: usize = 500;

/// Expected mean squares of components (the EMS equations).
fn expected_mean_squares(c: &[f64; 7], s: [usize; 3]) -> [f64; 7] {
    let [p, i, r, pi, pr, ir, pir] = *c;
    let [np, ni, nr] = s.map(|v| v as f64);
    [
        pir + nr * pi + ni * pr + ni * nr * p,
        pir + nr * pi + np * ir + np * nr * i,
        pir + ni * pr + np * ir + np * ni * r,
        pir + nr * pi,
        pir + ni * pr,
        pir + np * ir,
        pir,
    ]
}

/// One GPQ draw of (G, Phi) from observed mean squares.
fn gpq_draw(ms: &[f64; 7], df: &[f64; 7], s: [usize; 3], rng: &mut impl RngCore) -> [f64; 2] {
    let ems: [f64; 7] = std::array::from_fn(|j| ms[j] * df[j] / chi_square(df[j], rng));
    let c = components(&ems, s).map(|v| v.max(0.0));
    let (g, phi) = coefficients(&c, s[1] as f64, s[2] as f64);
    [g, phi]
}

/// Percentile levels that make the GPQ interval calibrated at the clipped
/// point estimate (prepivoting). Under the normal model a dataset's mean
/// squares are `EMS_j χ²(df_j)/df_j`, so no data are generated. For each
/// simulated dataset, `u` is the share of its GPQ draws at or below the
/// coefficient's value at the estimate; the levels are the α/2 and 1 − α/2
/// quantiles of `u`, for (G, Phi).
fn calibrated_levels(
    point: &[f64; 7],
    df: &[f64; 7],
    s: [usize; 3],
    rng: &mut impl RngCore,
) -> [(f64, f64); 2] {
    let ems = expected_mean_squares(point, s);
    let (g0, phi0) = coefficients(point, s[1] as f64, s[2] as f64);
    let mut u = [Vec::with_capacity(CAL_OUTER), Vec::with_capacity(CAL_OUTER)];
    for _ in 0..CAL_OUTER {
        let ms: [f64; 7] = std::array::from_fn(|j| ems[j] * chi_square(df[j], rng) / df[j]);
        let mut below = [0usize; 2];
        for _ in 0..CAL_INNER {
            let d = gpq_draw(&ms, df, s, rng);
            below[0] += usize::from(d[0] <= g0);
            below[1] += usize::from(d[1] <= phi0);
        }
        for k in 0..2 {
            u[k].push(below[k] as f64 / CAL_INNER as f64);
        }
    }
    u.map(|mut v| {
        v.sort_by(f64::total_cmp);
        (
            percentile(&v, ALPHA / 2.0),
            percentile(&v, 1.0 - ALPHA / 2.0),
        )
    })
}

fn replicate(cell: &Cell, rep: usize, draws: usize) -> GpqRep {
    let root = RngState::from_seed(*b"salib-gtheory-coverage/2026-10-2");
    let data = if cell.data == Data::Normal {
        "normal"
    } else {
        "binary"
    };
    let [np, ni, nr] = cell.sizes;
    let salt = |what: &str| format!("gpq/{np}x{ni}x{nr}/{}/{data}/rep{rep}/{what}", cell.pattern);
    let y = generate(cell, &mut root.fork(salt("data").as_bytes()).into_chacha());
    let (ms, df) = anova(&y);
    if ms[6] <= 0.0 {
        return GpqRep::default();
    }
    let point = components(&ms, cell.sizes).map(|c| c.max(0.0));
    let mut rng = root.fork(salt("pivots").as_bytes()).into_chacha();
    let mut samples: Vec<Vec<f64>> = (0..11).map(|_| Vec::with_capacity(draws)).collect();
    let (fi, fr) = (ni as f64, nr as f64);
    for _ in 0..draws {
        let ems: [f64; 7] = std::array::from_fn(|j| ms[j] * df[j] / chi_square(df[j], &mut rng));
        let c = components(&ems, cell.sizes).map(|v| v.max(0.0));
        let (g, phi) = coefficients(&c, fi, fr);
        for k in 0..7 {
            samples[k].push(c[k]);
        }
        samples[7].push(g);
        samples[8].push(phi);
    }
    let mut intervals = [None; 11];
    for (k, s) in samples.iter_mut().enumerate() {
        s.retain(|v| v.is_finite());
        if s.is_empty() {
            continue;
        }
        s.sort_by(f64::total_cmp);
        intervals[k] = Some((percentile(s, ALPHA / 2.0), percentile(s, 1.0 - ALPHA / 2.0)));
    }
    let mut cal_rng = root.fork(salt("calibration").as_bytes()).into_chacha();
    let levels = calibrated_levels(&point, &df, cell.sizes, &mut cal_rng);
    for (k, (low, high)) in levels.into_iter().enumerate() {
        let s = &samples[7 + k];
        if !s.is_empty() {
            intervals[9 + k] = Some((percentile(s, low), percentile(s, high)));
        }
    }
    GpqRep {
        error: false,
        point: Some(point),
        intervals,
    }
}

pub fn run(args: &[String]) -> Result<(), Box<dyn Error + Send + Sync>> {
    let usage =
        "usage: salib-gtheory-coverage gpq <n_sim> <draws> <out.md> [threads] [first_cell] [end_cell]";
    if args.len() < 5 {
        return Err(usage.into());
    }
    let n_sim: usize = args[2].parse()?;
    let draws: usize = args[3].parse()?;
    let threads: usize = match args.get(5) {
        Some(t) => t.parse()?,
        None => thread::available_parallelism().map_or(1, usize::from),
    };
    let all = cells(true);
    let first: usize = args.get(6).map_or(Ok(0), |v| v.parse())?;
    let end: usize = args
        .get(7)
        .map_or(Ok(all.len()), |v| v.parse())?
        .min(all.len());
    let mut file = std::fs::File::create(&args[4])?;
    let mut header = String::new();
    let _ = writeln!(
        header,
        "# Fiducial GPQ intervals, cells {first}..{end} of {}\n",
        all.len()
    );
    let _ = writeln!(header, "n_sim = {n_sim}, draws = {draws}, nominal {:.2}. Pass: coverage in [.94, .96] and each tail miss in [.015, .035]. Lower miss: the interval lies entirely below the truth; upper miss: entirely above.\n", 1.0 - ALPHA);
    let _ = writeln!(header, "| cell | pattern | data | estimand | method | truth | avail | coverage (MCSE) | lower miss | upper miss | conditional | bias-elim | mean width | notes | verdict |");
    let _ = writeln!(
        header,
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
    );
    file.write_all(header.as_bytes())?;
    for (id, cell) in all.iter().enumerate().take(end).skip(first) {
        let started = std::time::Instant::now();
        let chunks: Vec<Vec<GpqRep>> = thread::scope(|scope| {
            let handles: Vec<_> = (0..threads)
                .map(|t| {
                    scope.spawn(move || {
                        (t..n_sim)
                            .step_by(threads)
                            .map(|rep| {
                                std::panic::catch_unwind(|| replicate(cell, rep, draws))
                                    .unwrap_or_else(|_| GpqRep {
                                        error: true,
                                        ..GpqRep::default()
                                    })
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|h| h.join().map_err(|_| "worker panicked"))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let reps: Vec<GpqRep> = chunks.into_iter().flatten().collect();
        let t = truth(cell);
        let (gt, phit) = coefficients(&t, cell.sizes[1] as f64, cell.sizes[2] as f64);
        let errors = reps.iter().filter(|r| r.error).count();
        let mut rows = String::new();
        let estimands: Vec<(String, f64)> = NAMES
            .iter()
            .zip(t)
            .map(|(n, v)| ((*n).to_string(), v))
            .chain([
                ("G".to_string(), gt),
                ("Phi".to_string(), phit),
                ("G".to_string(), gt),
                ("Phi".to_string(), phit),
            ])
            .collect();
        for (k, (name, value)) in estimands.iter().enumerate() {
            let points: Vec<Option<f64>> = reps
                .iter()
                .map(|r| {
                    r.point.map(|c| match k {
                        7 | 9 => coefficients(&c, cell.sizes[1] as f64, cell.sizes[2] as f64).0,
                        8 | 10 => coefficients(&c, cell.sizes[1] as f64, cell.sizes[2] as f64).1,
                        _ => c[k],
                    })
                })
                .collect();
            let ivs: Vec<Option<(f64, f64)>> = reps.iter().map(|r| r.intervals[k]).collect();
            row(
                &mut rows,
                cell,
                name,
                if k >= 9 {
                    "fiducial GPQ, calibrated"
                } else {
                    "fiducial GPQ"
                },
                *value,
                &summarize(*value, &ivs, &points),
                &format!("errors {errors}"),
            );
        }
        file.write_all(rows.as_bytes())?;
        file.flush()?;
        eprintln!(
            "gpq cell {id} {:?} {} done in {:.1}s",
            cell.sizes,
            cell.pattern,
            started.elapsed().as_secs_f64()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// χ²(df) has mean df and variance 2·df; checked within 5 MCSE.
    #[test]
    fn chi_square_sampler_has_the_right_moments() {
        let mut rng = RngState::from_seed([7; 32]).into_chacha();
        let n = 200_000;
        for df in [1.0, 2.0, 3.0, 19.0, 3781.0] {
            let x: Vec<f64> = (0..n).map(|_| chi_square(df, &mut rng)).collect();
            let mean = x.iter().sum::<f64>() / n as f64;
            let var = x.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
            let se_mean = (2.0 * df / n as f64).sqrt();
            // Var of the sample variance: (μ4 − σ⁴)/n with μ4 = 12df + 3(2df)².
            let se_var = ((12.0 * df + 12.0 * df * df - 4.0 * df * df) / n as f64).sqrt();
            assert!((mean - df).abs() < 5.0 * se_mean, "df {df}: mean {mean}");
            assert!((var - 2.0 * df).abs() < 5.0 * se_var, "df {df}: var {var}");
        }
    }

    /// The harness's ANOVA recovers exact components on ±1 effect codes.
    #[test]
    fn anova_matches_effect_codes() {
        let y = Array3::from_shape_fn((2, 2, 2), |(p, i, r)| {
            let [p, i, r] = [p, i, r].map(|k| if k == 0 { -1.0 } else { 1.0 });
            50.0 + 6.0 * p + 4.0 * i + 2.0 * r + 3.0 * p * i + 1.5 * p * r + i * r + 0.5 * p * i * r
        });
        let (ms, _) = anova(&y);
        let c = components(&ms, [2, 2, 2]);
        let want = [50.0, 12.5, 2.0, 35.0, 8.0, 3.0, 2.0];
        for k in 0..7 {
            assert!(
                (c[k] - want[k]).abs() < 1e-9,
                "{k}: {} vs {}",
                c[k],
                want[k]
            );
        }
    }
}
