//! Phi interval experiment (journal/2026-10-23-g-theory-bootstrap-plan.md,
//! "Phi experiment").
//!
//! For each cell and replication: draw a dataset, take the clipped point Phi,
//! then run a parametric bootstrap twice. Once from the zeroed point
//! components (salib's method) and once from the true components (an oracle
//! that isolates the plug-in effect). Each bootstrap yields three intervals:
//! percentile, basic and bias-corrected percentile.
//!
//! Usage: `salib-gtheory-coverage phi <n_sim> <B> <out.md> [threads]`.

use std::error::Error;
use std::fmt::Write as _;
use std::io::Write as _;
use std::thread;

use salib_core::RngState;
use salib_estimators::{estimate_g_theory_pir, GTheoryDesign};
use statrs::distribution::{ContinuousCDF, Normal};

use super::{coefficients, generate, percentile, row, summarize, Cell, Data};

const ALPHA: f64 = 0.05;
/// Jiang et al. (2022), Table 1: normal p×i×r components in salib's order
/// (p, i, r, pi, pr, ir, pir), simulated at 100×20×2.
const JIANG: [f64; 7] = [16.0, 4.0, 1.0, 64.0, 2.0, 3.0, 144.0];
const ALL_ONE: [f64; 7] = [1.0; 7];
const HIGH_G: [f64; 7] = [4.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5];

fn cells() -> Vec<Cell> {
    let cell = |sizes, pattern, theta| Cell {
        sizes,
        pattern,
        theta,
        data: Data::Normal,
        baseline: false,
    };
    vec![
        cell([100, 20, 2], "jiang-2022", JIANG),
        cell([200, 20, 2], "high-g", HIGH_G),
        cell([200, 20, 2], "all-one", ALL_ONE),
        cell([200, 20, 3], "high-g", HIGH_G),
        cell([200, 8, 2], "high-g", HIGH_G),
        cell([50, 20, 2], "high-g", HIGH_G),
        cell([20, 20, 3], "all-one", ALL_ONE),
        cell([50, 8, 5], "all-one", ALL_ONE),
    ]
}

const GENERATORS: [&str; 2] = ["zeroed estimate", "true components"];
const CONSTRUCTIONS: [&str; 3] = ["percentile", "basic", "bias-corrected"];

/// Intervals in GENERATORS × CONSTRUCTIONS order, plus the clipped point Phi.
#[derive(Default)]
struct PhiRep {
    error: bool,
    point: Option<f64>,
    intervals: [Option<(f64, f64)>; 6],
}

fn intervals(point: f64, mut draws: Vec<f64>) -> [Option<(f64, f64)>; 3] {
    if draws.len() < 2 {
        return [None; 3];
    }
    draws.sort_by(f64::total_cmp);
    let (lo, hi) = (
        percentile(&draws, ALPHA / 2.0),
        percentile(&draws, 1.0 - ALPHA / 2.0),
    );
    let basic = ((2.0 * point - hi).max(0.0), (2.0 * point - lo).min(1.0));
    // Bias-corrected percentile (Efron): shift the percentile levels by z0.
    let below = draws.iter().filter(|&&d| d < point).count() as f64;
    let ties = draws.iter().filter(|&&d| d == point).count() as f64;
    let frac = (below + 0.5 * ties) / draws.len() as f64;
    let bc = Normal::new(0.0, 1.0).ok().and_then(|n| {
        if frac <= 0.0 || frac >= 1.0 {
            return None;
        }
        let z0 = n.inverse_cdf(frac);
        let a = n.cdf(2.0 * z0 + n.inverse_cdf(ALPHA / 2.0));
        let b = n.cdf(2.0 * z0 + n.inverse_cdf(1.0 - ALPHA / 2.0));
        Some((percentile(&draws, a), percentile(&draws, b)))
    });
    [Some((lo, hi)), Some(basic), bc]
}

fn replicate(cell: &Cell, rep: usize, b: usize) -> Result<PhiRep, Box<dyn Error + Send + Sync>> {
    let root = RngState::from_seed(
        *b"salib-gtheory-coverage/2026-10-25"[..32]
            .first_chunk::<32>()
            .ok_or("seed")?,
    );
    let [np, ni, nr] = cell.sizes;
    let salt = |what: &str| format!("phi/{np}x{ni}x{nr}/{}/rep{rep}/{what}", cell.pattern);
    let y = generate(cell, &mut root.fork(salt("data").as_bytes()).into_chacha());
    let Ok(point) = estimate_g_theory_pir(y.view(), GTheoryDesign::Crossed) else {
        return Ok(PhiRep::default());
    };
    let zeroed = point.with_negative_components_zeroed();
    let Ok(phi_hat) = zeroed.phi_coefficient else {
        return Ok(PhiRep::default());
    };
    let mut out = PhiRep {
        point: Some(phi_hat),
        ..PhiRep::default()
    };
    for (g, theta) in [zeroed.components(), cell.theta].into_iter().enumerate() {
        let generating = Cell { theta, ..*cell };
        let mut rng = root.fork(salt(GENERATORS[g]).as_bytes()).into_chacha();
        let draws: Vec<f64> = (0..b)
            .filter_map(|_| {
                let yb = generate(&generating, &mut rng);
                let fit = estimate_g_theory_pir(yb.view(), GTheoryDesign::Crossed).ok()?;
                fit.with_negative_components_zeroed().phi_coefficient.ok()
            })
            .collect();
        let ci = intervals(phi_hat, draws);
        out.intervals[3 * g..3 * g + 3].copy_from_slice(&ci);
    }
    Ok(out)
}

pub fn run(args: &[String]) -> Result<(), Box<dyn Error + Send + Sync>> {
    let usage = "usage: salib-gtheory-coverage phi <n_sim> <B> <out.md> [threads]";
    if args.len() < 5 {
        return Err(usage.into());
    }
    let n_sim: usize = args[2].parse()?;
    let b: usize = args[3].parse()?;
    let threads: usize = match args.get(5) {
        Some(t) => t.parse()?,
        None => thread::available_parallelism().map_or(1, usize::from),
    };
    let mut file = std::fs::File::create(&args[4])?;
    let mut header = String::new();
    let _ = writeln!(header, "# Phi interval experiment\n");
    let _ = writeln!(header, "n_sim = {n_sim}, B = {b}, nominal {:.2}. Method = generating values / interval construction. \"true components\" is an oracle diagnostic, not a method. Lower miss: interval entirely below the truth; upper miss: entirely above.\n", 1.0 - ALPHA);
    let _ = writeln!(header, "| cell | pattern | data | estimand | method | truth | avail | coverage (MCSE) | lower miss | upper miss | conditional | bias-elim | mean width | notes | verdict |");
    let _ = writeln!(
        header,
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
    );
    file.write_all(header.as_bytes())?;
    for cell in cells() {
        let started = std::time::Instant::now();
        let cell_ref = &cell;
        let chunks: Vec<Vec<PhiRep>> = thread::scope(|scope| {
            let handles: Vec<_> = (0..threads)
                .map(|t| {
                    scope.spawn(move || {
                        (t..n_sim)
                            .step_by(threads)
                            .map(|rep| {
                                match std::panic::catch_unwind(|| replicate(cell_ref, rep, b)) {
                                    Ok(Ok(r)) => r,
                                    _ => PhiRep {
                                        error: true,
                                        ..PhiRep::default()
                                    },
                                }
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
        let reps: Vec<PhiRep> = chunks.into_iter().flatten().collect();
        let (_, truth) = coefficients(&cell.theta, cell.sizes[1] as f64, cell.sizes[2] as f64);
        let points: Vec<Option<f64>> = reps.iter().map(|r| r.point).collect();
        let errors = reps.iter().filter(|r| r.error).count();
        let mut rows = String::new();
        for (k, label) in GENERATORS
            .iter()
            .flat_map(|g| CONSTRUCTIONS.iter().map(move |c| format!("{g} / {c}")))
            .enumerate()
        {
            let ivs: Vec<Option<(f64, f64)>> = reps.iter().map(|r| r.intervals[k]).collect();
            row(
                &mut rows,
                &cell,
                "Phi",
                &label,
                truth,
                &summarize(truth, &ivs, &points),
                &format!("errors {errors}"),
            );
        }
        file.write_all(rows.as_bytes())?;
        file.flush()?;
        eprintln!(
            "phi cell {:?} {} done in {:.1}s",
            cell.sizes,
            cell.pattern,
            started.elapsed().as_secs_f64()
        );
    }
    Ok(())
}
