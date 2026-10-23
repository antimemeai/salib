//! Coverage study for salib's G-theory standard errors and intervals (salib #5).
//!
//! Design: `journal/2026-10-23-g-theory-bootstrap-plan.md`. Each cell fixes
//! facet sizes, true components and a data distribution; each replication
//! draws one dataset, fits it, and runs `bootstrap_g_theory_pir`. Data come
//! from this crate's own generator, independent of salib's parametric one.
//! Seeds derive from (cell, replication), so any run reproduces its datasets.
//!
//! Usage: `salib-gtheory-coverage <smoke|full> <n_sim> <B> <out.md> [threads]`.
//! `smoke` checks the harness only and is not evidence.

use std::error::Error;
use std::fmt::Write as _;
use std::io::Write as _;
use std::thread;

use ndarray::Array3;
use rand::RngCore;
use salib_core::RngState;
use salib_estimators::{bootstrap_g_theory_pir, estimate_g_theory_pir, GTheoryDesign};
use statrs::distribution::{ChiSquared, ContinuousCDF};

const ALPHA: f64 = 0.05;
const NAMES: [&str; 7] = ["p", "i", "r", "pi", "pr", "ir", "pir"];
/// Facets of each component as a bit mask over (p, i, r).
const MASKS: [u8; 7] = [0b001, 0b010, 0b100, 0b011, 0b101, 0b110, 0b111];

#[derive(Clone, Copy, PartialEq)]
enum Data {
    Normal,
    Dichotomous,
}

#[derive(Clone, Copy)]
struct Cell {
    sizes: [usize; 3],
    pattern: &'static str,
    theta: [f64; 7],
    data: Data,
    baseline: bool,
}

fn patterns() -> [(&'static str, [f64; 7]); 3] {
    [
        ("all-one", [1.0; 7]),
        ("near-zero-rater", [1.0, 1.0, 0.01, 1.0, 0.01, 0.01, 1.0]),
        ("high-g", [4.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5]),
    ]
}

fn cells(full: bool) -> Vec<Cell> {
    let baseline_sizes = [[20, 8, 2], [20, 8, 5], [50, 20, 3], [200, 5, 2]];
    if !full {
        return vec![
            Cell {
                sizes: [20, 5, 2],
                pattern: "all-one",
                theta: [1.0; 7],
                data: Data::Normal,
                baseline: true,
            },
            Cell {
                sizes: [50, 8, 3],
                pattern: "all-one",
                theta: [1.0; 7],
                data: Data::Dichotomous,
                baseline: false,
            },
        ];
    }
    let mut out = Vec::new();
    for np in [20, 50, 200] {
        for ni in [5, 8, 20] {
            for nr in [2, 3, 5] {
                for (pattern, theta) in patterns() {
                    let sizes = [np, ni, nr];
                    let baseline = pattern == "all-one" && baseline_sizes.contains(&sizes);
                    out.push(Cell {
                        sizes,
                        pattern,
                        theta,
                        data: Data::Normal,
                        baseline,
                    });
                }
            }
        }
    }
    for ni in [5, 8, 20] {
        for nr in [2, 3, 5] {
            out.push(Cell {
                sizes: [50, ni, nr],
                pattern: "all-one",
                theta: [1.0; 7],
                data: Data::Dichotomous,
                baseline: false,
            });
        }
    }
    out
}

/// Components of the observed scores. Dichotomous scores threshold a latent
/// normal at 0: two observations sharing index set M have latent correlation
/// ρ(M) = Σ_{∅≠u⊆M} θ_u / V and covariance asin(ρ)/(2π) (Sheppard); Möbius
/// inversion over the subset lattice gives the binary components.
fn truth(cell: &Cell) -> [f64; 7] {
    if cell.data == Data::Normal {
        return cell.theta;
    }
    let total: f64 = cell.theta.iter().sum();
    let cov = |m: u8| -> f64 {
        let rho: f64 = (0..7)
            .filter(|&k| MASKS[k] & !m == 0)
            .map(|k| cell.theta[k])
            .sum::<f64>()
            / total;
        rho.asin() / (2.0 * std::f64::consts::PI)
    };
    MASKS.map(|u| {
        let mut sum = 0.0;
        for m in 0..=u {
            if m & !u == 0 {
                let sign = if (u.count_ones() - m.count_ones()) % 2 == 0 {
                    1.0
                } else {
                    -1.0
                };
                sum += sign * if m == 0 { 0.0 } else { cov(m) };
            }
        }
        sum
    })
}

fn coefficients(c: &[f64; 7], ni: f64, nr: f64) -> (f64, f64) {
    let [p, i, r, pi, pr, ir, pir] = *c;
    let delta = pi / ni + pr / nr + pir / (ni * nr);
    let big_delta = delta + i / ni + r / nr + ir / (ni * nr);
    (p / (p + delta), p / (p + big_delta))
}

fn open_unit(rng: &mut impl RngCore) -> f64 {
    ((rng.next_u64() >> 11) as f64 + 0.5) / (1_u64 << 53) as f64
}

/// Standard normal by Box–Muller: deliberately different machinery from
/// salib's inverse-CDF generator.
fn standard_normal(rng: &mut impl RngCore) -> f64 {
    let (u1, u2) = (open_unit(rng), open_unit(rng));
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

fn generate(cell: &Cell, rng: &mut impl RngCore) -> Array3<f64> {
    let [np, ni, nr] = cell.sizes;
    // Effects in (p, i, r, pi, pr, ir, pir) order; each interaction array is
    // row-major over its own indices.
    let lens = [np, ni, nr, np * ni, np * nr, ni * nr, np * ni * nr];
    let effects: Vec<Vec<f64>> = (0..7)
        .map(|k| {
            (0..lens[k])
                .map(|_| cell.theta[k].sqrt() * standard_normal(rng))
                .collect()
        })
        .collect();
    Array3::from_shape_fn((np, ni, nr), |(a, b, c)| {
        let y = effects[0][a]
            + effects[1][b]
            + effects[2][c]
            + effects[3][a * ni + b]
            + effects[4][a * nr + c]
            + effects[5][b * nr + c]
            + effects[6][(a * ni + b) * nr + c];
        match cell.data {
            Data::Normal => y,
            Data::Dichotomous => f64::from(u8::from(y > 0.0)),
        }
    })
}

/// Mean squares recovered from components through the expected-mean-square equations.
fn mean_squares(c: &[f64; 7], s: [usize; 3]) -> [f64; 7] {
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

/// Satterthwaite interval for each component (ACT RR 87-7, App. A).
fn satterthwaite(c: &[f64; 7], s: [usize; 3]) -> [Option<(f64, f64)>; 7] {
    let ms = mean_squares(c, s);
    let [np, ni, nr] = s.map(|v| v as f64);
    let df = [
        np - 1.0,
        ni - 1.0,
        nr - 1.0,
        (np - 1.0) * (ni - 1.0),
        (np - 1.0) * (nr - 1.0),
        (ni - 1.0) * (nr - 1.0),
        (np - 1.0) * (ni - 1.0) * (nr - 1.0),
    ];
    // (mean squares used, divisor) for each EMS estimator.
    let terms: [(&[usize], f64); 7] = [
        (&[0, 3, 4, 6], ni * nr),
        (&[1, 3, 5, 6], np * nr),
        (&[2, 4, 5, 6], np * ni),
        (&[3, 6], nr),
        (&[4, 6], ni),
        (&[5, 6], np),
        (&[6], 1.0),
    ];
    std::array::from_fn(|k| {
        let (used, d) = terms[k];
        let var: f64 = used
            .iter()
            .map(|&j| 2.0 * (ms[j] / d).powi(2) / df[j])
            .sum();
        if c[k] <= 0.0 || var <= 0.0 {
            return None;
        }
        let nu = 2.0 * c[k] * c[k] / var;
        let chi = ChiSquared::new(nu).ok()?;
        // statrs' inverse_cdf returns NaN for nu <= 1; bisect its cdf instead.
        let q = |prob: f64| {
            let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
            while chi.cdf(hi) < prob {
                lo = hi;
                hi *= 2.0;
                if !hi.is_finite() {
                    return None;
                }
            }
            for _ in 0..2000 {
                let mid = 0.5 * (lo + hi);
                if mid <= lo || mid >= hi {
                    break;
                }
                if chi.cdf(mid) < prob {
                    lo = mid
                } else {
                    hi = mid
                }
            }
            (hi > 0.0).then_some(hi)
        };
        Some((
            c[k] * nu / q(1.0 - ALPHA / 2.0)?,
            c[k] * nu / q(ALPHA / 2.0)?,
        ))
    })
}

fn percentile(sorted: &[f64], prob: f64) -> f64 {
    let pos = prob * (sorted.len() - 1) as f64;
    let (lo, frac) = (pos.floor() as usize, pos - pos.floor());
    if lo + 1 >= sorted.len() {
        sorted[sorted.len() - 1]
    } else {
        sorted[lo] + frac * (sorted[lo + 1] - sorted[lo])
    }
}

/// Percentile intervals from the 0.3.0 baseline plan.
#[derive(Clone, Copy)]
struct Baseline {
    components: [Option<(f64, f64)>; 7],
    g: Option<(f64, f64)>,
    phi: Option<(f64, f64)>,
}

/// salib 0.3.0's plan, for comparison: persons, items and raters resampled
/// together; signed refits; signed G and Phi; percentiles over successes.
fn baseline(y: &Array3<f64>, b: usize, rng: &mut impl RngCore) -> Baseline {
    let s = [y.shape()[0], y.shape()[1], y.shape()[2]];
    let (ni, nr) = (s[1] as f64, s[2] as f64);
    let mut comps: Vec<Vec<f64>> = vec![Vec::new(); 7];
    let (mut gs, mut phis) = (Vec::new(), Vec::new());
    for _ in 0..b {
        let idx: Vec<Vec<usize>> = s
            .iter()
            .map(|&n| (0..n).map(|_| (rng.next_u32() as usize) % n).collect())
            .collect();
        let rs = Array3::from_shape_fn((s[0], s[1], s[2]), |(a, bb, c)| {
            y[[idx[0][a], idx[1][bb], idx[2][c]]]
        });
        if let Ok(fit) = estimate_g_theory_pir(rs.view(), GTheoryDesign::Crossed) {
            let c = fit.components();
            for k in 0..7 {
                comps[k].push(c[k]);
            }
            let (g, phi) = coefficients(&c, ni, nr);
            if g.is_finite() {
                gs.push(g);
            }
            if phi.is_finite() {
                phis.push(phi);
            }
        }
    }
    let ci = |v: &mut Vec<f64>| -> Option<(f64, f64)> {
        if v.is_empty() {
            return None;
        }
        v.sort_by(f64::total_cmp);
        Some((percentile(v, ALPHA / 2.0), percentile(v, 1.0 - ALPHA / 2.0)))
    };
    let comp = std::array::from_fn(|k| ci(&mut comps[k]));
    Baseline {
        components: comp,
        g: ci(&mut gs),
        phi: ci(&mut phis),
    }
}

#[derive(Default, Clone)]
struct Rep {
    /// The replicate raised an error; its intervals count as misses.
    error: bool,
    fitted: bool,
    est: [f64; 7],
    se: [Option<f64>; 7],
    ci: [Option<(f64, f64)>; 7],
    satt: [Option<(f64, f64)>; 7],
    g_clipped: Option<f64>,
    g_ci: Option<(f64, f64)>,
    phi_clipped: Option<f64>,
    phi_ci: Option<(f64, f64)>,
    phi_ci_attached: Option<(f64, f64)>,
    base: Option<Baseline>,
    failures: usize,
}

fn replicate(cell: &Cell, rep: usize, b: usize) -> Result<Rep, Box<dyn Error + Send + Sync>> {
    let root = RngState::from_seed(
        *b"salib-gtheory-coverage/2026-10-23"[..32]
            .first_chunk::<32>()
            .ok_or("seed")?,
    );
    let data = if cell.data == Data::Normal {
        "normal"
    } else {
        "binary"
    };
    let [np, ni, nr] = cell.sizes;
    let key = format!("{np}x{ni}x{nr}/{}/{data}", cell.pattern);
    let salt = |what: &str| format!("{key}/rep{rep}/{what}");
    let y = generate(cell, &mut root.fork(salt("data").as_bytes()).into_chacha());
    let Ok(point) = estimate_g_theory_pir(y.view(), GTheoryDesign::Crossed) else {
        return Ok(Rep::default());
    };
    let mut boot_rng = root.fork(salt("boot").as_bytes());
    let boot = bootstrap_g_theory_pir(y.view(), GTheoryDesign::Crossed, b, ALPHA, &mut boot_rng)?;
    let clipped = point.with_negative_components_zeroed();
    let mut with_boot = point.clone();
    with_boot.bootstrap = Some(boot.clone());
    let base = cell.baseline.then(|| {
        baseline(
            &y,
            b,
            &mut root.fork(salt("baseline").as_bytes()).into_chacha(),
        )
    });
    Ok(Rep {
        error: false,
        fitted: true,
        est: point.components(),
        se: boot.component_se,
        ci: boot.component_ci,
        satt: satterthwaite(&point.components(), cell.sizes),
        g_clipped: clipped.g_coefficient.ok(),
        g_ci: boot.g_coefficient_ci,
        phi_clipped: clipped.phi_coefficient.ok(),
        phi_ci: boot.phi_coefficient_ci,
        phi_ci_attached: with_boot.phi_coefficient_interval(),
        base,
        failures: boot.facet_skipped.iter().sum::<usize>() + boot.parametric_skipped,
    })
}

struct Summary {
    n: usize,
    available: f64,
    coverage: f64,
    lower_miss: f64,
    upper_miss: f64,
    conditional: f64,
    bias_eliminated: f64,
    width: f64,
}

/// A `None` interval counts as a miss overall; misses are split by side.
fn summarize(truth: f64, intervals: &[Option<(f64, f64)>], points: &[Option<f64>]) -> Summary {
    let n = intervals.len();
    let avail: Vec<(f64, f64)> = intervals.iter().flatten().copied().collect();
    let pts: Vec<f64> = points.iter().flatten().copied().collect();
    let mean_point = if pts.is_empty() {
        f64::NAN
    } else {
        pts.iter().sum::<f64>() / pts.len() as f64
    };
    let frac = |count: usize| count as f64 / n as f64;
    let covers = avail
        .iter()
        .filter(|(l, h)| *l <= truth && truth <= *h)
        .count();
    Summary {
        n,
        available: frac(avail.len()),
        coverage: frac(covers),
        lower_miss: frac(avail.iter().filter(|(_, h)| truth > *h).count()),
        upper_miss: frac(avail.iter().filter(|(l, _)| truth < *l).count()),
        conditional: if avail.is_empty() {
            f64::NAN
        } else {
            covers as f64 / avail.len() as f64
        },
        bias_eliminated: frac(
            avail
                .iter()
                .filter(|(l, h)| *l <= mean_point && mean_point <= *h)
                .count(),
        ),
        width: if avail.is_empty() {
            f64::NAN
        } else {
            avail.iter().map(|(l, h)| h - l).sum::<f64>() / avail.len() as f64
        },
    }
}

fn passes(s: &Summary) -> bool {
    (0.94..=0.96).contains(&s.coverage)
        && (0.015..=0.035).contains(&s.lower_miss)
        && (0.015..=0.035).contains(&s.upper_miss)
}

fn row(
    out: &mut String,
    cell: &Cell,
    estimand: &str,
    method: &str,
    truth: f64,
    s: &Summary,
    extra: &str,
) {
    let mcse = (s.coverage * (1.0 - s.coverage) / s.n as f64).sqrt();
    let data = if cell.data == Data::Normal {
        "normal"
    } else {
        "binary"
    };
    let _ = writeln!(
        out,
        "| {}×{}×{} | {} | {data} | {estimand} | {method} | {truth:.4} | {:.3} | {:.4} ({:.4}) | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {extra} | {} |",
        cell.sizes[0], cell.sizes[1], cell.sizes[2], cell.pattern, s.available, s.coverage, mcse,
        s.lower_miss, s.upper_miss, s.conditional, s.bias_eliminated, s.width, if passes(s) { "pass" } else { "fail" },
    );
}

fn report_cell(out: &mut String, cell: &Cell, reps: &[Rep]) {
    let t = truth(cell);
    let (gt, phit) = coefficients(&t, cell.sizes[1] as f64, cell.sizes[2] as f64);
    let fitted: Vec<&Rep> = reps.iter().filter(|r| r.fitted).collect();
    let unfit = reps.iter().filter(|r| !r.fitted && !r.error).count();
    let failures: usize = reps.iter().map(|r| r.failures).sum();
    for k in 0..7 {
        let est: Vec<f64> = fitted.iter().map(|r| r.est[k]).collect();
        let mean = est.iter().sum::<f64>() / est.len().max(1) as f64;
        let sd = (est.iter().map(|e| (e - mean).powi(2)).sum::<f64>()
            / (est.len().max(2) - 1) as f64)
            .sqrt();
        let ses: Vec<f64> = fitted.iter().filter_map(|r| r.se[k]).collect();
        // Morris et al. (2019): sqrt(mean SE²) over the empirical SD.
        let se_ratio =
            (ses.iter().map(|v| v * v).sum::<f64>() / ses.len().max(1) as f64).sqrt() / sd;
        let points: Vec<Option<f64>> = reps.iter().map(|r| r.fitted.then_some(r.est[k])).collect();
        let pick = |f: &dyn Fn(&Rep) -> Option<(f64, f64)>| reps.iter().map(f).collect::<Vec<_>>();
        let extra = format!("facet-SE ratio {se_ratio:.3}");
        row(
            out,
            cell,
            NAMES[k],
            "parametric",
            t[k],
            &summarize(t[k], &pick(&|r| r.ci[k]), &points),
            &extra,
        );
        row(
            out,
            cell,
            NAMES[k],
            "Satterthwaite",
            t[k],
            &summarize(t[k], &pick(&|r| r.satt[k]), &points),
            "",
        );
        if cell.baseline {
            row(
                out,
                cell,
                NAMES[k],
                "0.3.0 boot-p,i,r",
                t[k],
                &summarize(
                    t[k],
                    &pick(&|r| r.base.and_then(|b| b.components[k])),
                    &points,
                ),
                "",
            );
        }
    }
    let g_points: Vec<Option<f64>> = reps.iter().map(|r| r.g_clipped).collect();
    let phi_points: Vec<Option<f64>> = reps.iter().map(|r| r.phi_clipped).collect();
    let pick = |f: &dyn Fn(&Rep) -> Option<(f64, f64)>| reps.iter().map(f).collect::<Vec<_>>();
    let errors = reps.iter().filter(|r| r.error).count();
    let extra = format!("unfit {unfit}, errors {errors}, failed fits {failures}");
    row(
        out,
        cell,
        "G",
        "F interval",
        gt,
        &summarize(gt, &pick(&|r| r.g_ci), &g_points),
        &extra,
    );
    row(
        out,
        cell,
        "Phi",
        "parametric",
        phit,
        &summarize(phit, &pick(&|r| r.phi_ci), &phi_points),
        "",
    );
    row(
        out,
        cell,
        "Phi",
        "parametric, attached",
        phit,
        &summarize(phit, &pick(&|r| r.phi_ci_attached), &phi_points),
        "",
    );
    if cell.baseline {
        row(
            out,
            cell,
            "G",
            "0.3.0 boot-p,i,r",
            gt,
            &summarize(gt, &pick(&|r| r.base.and_then(|b| b.g)), &g_points),
            "",
        );
        row(
            out,
            cell,
            "Phi",
            "0.3.0 boot-p,i,r",
            phit,
            &summarize(phit, &pick(&|r| r.base.and_then(|b| b.phi)), &phi_points),
            "",
        );
    }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    let usage = "usage: salib-gtheory-coverage <smoke|full> <n_sim> <B> <out.md> [threads] [first_cell] [end_cell]";
    if args.len() < 5 {
        return Err(usage.into());
    }
    let full = match args[1].as_str() {
        "full" => true,
        "smoke" => false,
        _ => return Err(usage.into()),
    };
    let n_sim: usize = args[2].parse()?;
    let b: usize = args[3].parse()?;
    let threads: usize = match args.get(5) {
        Some(t) => t.parse()?,
        None => thread::available_parallelism().map_or(1, usize::from),
    };
    let all = cells(full);
    let first: usize = args.get(6).map_or(Ok(0), |v| v.parse())?;
    let end: usize = args
        .get(7)
        .map_or(Ok(all.len()), |v| v.parse())?
        .min(all.len());
    let mut file = std::fs::File::create(&args[4])?;
    let mut header = String::new();
    let _ = writeln!(
        header,
        "# G-theory coverage study ({}), cells {first}..{end} of {}\n",
        if full {
            "full"
        } else {
            "smoke: harness check only, not evidence"
        },
        all.len()
    );
    let _ = writeln!(header, "n_sim = {n_sim}, B = {b}, nominal {:.2}. Pass: coverage in [.94, .96] and each tail miss in [.015, .035]; a None interval counts as a miss. Lower miss: the interval lies entirely below the truth; upper miss: entirely above. Conditional coverage counts available intervals only; bias-eliminated coverage targets the mean point estimate.\n", 1.0 - ALPHA);
    let _ = writeln!(header, "| cell | pattern | data | estimand | method | truth | avail | coverage (MCSE) | lower miss | upper miss | conditional | bias-elim | mean width | notes | verdict |");
    let _ = writeln!(
        header,
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
    );
    file.write_all(header.as_bytes())?;
    for (id, cell) in all.iter().enumerate().take(end).skip(first) {
        let started = std::time::Instant::now();
        let chunks: Vec<Vec<Rep>> = thread::scope(|scope| {
            let handles: Vec<_> = (0..threads)
                .map(|t| {
                    scope.spawn(move || {
                        (t..n_sim)
                            .step_by(threads)
                            .map(|rep| {
                                replicate(cell, rep, b).unwrap_or_else(|e| {
                                    eprintln!("cell {id} rep {rep}: {e}");
                                    Rep {
                                        error: true,
                                        ..Rep::default()
                                    }
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
        let reps: Vec<Rep> = chunks.into_iter().flatten().collect();
        let mut rows = String::new();
        report_cell(&mut rows, cell, &reps);
        file.write_all(rows.as_bytes())?;
        file.flush()?;
        eprintln!(
            "cell {id} {:?} {} done in {:.1}s",
            cell.sizes,
            cell.pattern,
            started.elapsed().as_secs_f64()
        );
    }
    Ok(())
}
