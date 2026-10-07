#!/usr/bin/env python3
"""Check frequency estimators against EASI and discrete Parseval oracles.

Runs the repository's Rust APIs without Python packages or new dependencies.
Exit 1 indicates a failed oracle; original failures are documented in
papers/2026-10-07-frequency-audit.md. Requires cached Cargo dependencies.
"""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
RUST = r'''
use std::f64::consts::PI;
use ndarray::Array2;
use salib_core::RngState;
use salib_estimators::{estimate_fast, estimate_rbd_fast};
use salib_samplers::build_fast_design;

fn check(label: &str, got: f64, expected: f64, tolerance: f64) -> bool {
    let ok = (got - expected).abs() < tolerance;
    println!("{} {label}: got={got:.12}, expected={expected:.12}",
        if ok { "PASS" } else { "FAIL" });
    ok
}

fn main() {
    // Plischke 2010, section 3: odd sorted ranks ascending, then even
    // ranks descending, produces the triangular trace. For cos(pi*x),
    // this trace has only the fundamental frequency in the continuum.
    let n = 4097;
    let x = Array2::from_shape_fn((n, 1), |(j, _)| (j as f64 + 0.5)/n as f64);
    let y: Vec<_> = (0..n).map(|j| (PI*x[[j,0]]).cos()).collect();
    let raw = estimate_rbd_fast(x.view(), &y, 4).unwrap().s[0];
    let shape_ok = check("RBD API vs EASI single-frequency oracle", raw, 1.0, 1e-5);

    // Finite-vector Parseval oracle: y_j=cos(2*pi*j/N)+(-1)^j.
    // Low-frequency variance is 1/2, Nyquist variance is 1, so
    // total variance is 3/2. Apply the API's stated bias correction.
    let n = 64;
    let x = Array2::from_shape_fn((n, 1), |(j, _)| j as f64/n as f64);
    let trace: Vec<_> = (0..n).map(|j| (2.0*PI*j as f64/n as f64).cos()
        + if j%2 == 0 { 1.0 } else { -1.0 }).collect();
    // Invert EASI traversal to feed the desired exact trace through the API.
    let mut y = vec![0.0; n];
    for (j, rank) in (0..n).step_by(2).chain((1..n).step_by(2).rev()).enumerate() {
        y[rank] = trace[j];
    }
    let lambda = 2.0/n as f64;
    let expected = (1.0/3.0-lambda)/(1.0-lambda);
    let actual = estimate_rbd_fast(x.view(), &y, 1).unwrap().s[0];
    let rbd_ok = check("RBD even-N Nyquist weighting", actual, expected, 1e-12);

    // Feed an exact spectral fixture through a real FastDesign. The
    // lookup makes the callback deterministic for every sampled input.
    let mut rng = RngState::from_seed([19;32]);
    let design = build_fast_design(1,74,4,&mut rng).unwrap();
    let omega = design.omegas[[0,0]] as f64;
    let n = design.n_per_factor;
    let coordinates: Vec<_> = (0..n).map(|j| design.samples[[j,0]]).collect();
    assert!(coordinates.iter().enumerate().all(|(j,x)|
        coordinates[..j].iter().all(|prior| prior.to_bits()!=x.to_bits())));
    let result = estimate_fast(&design, |u| {
        let j=coordinates.iter().position(|x| x.to_bits()==u[0].to_bits()).unwrap();
        (2.0*PI*omega*j as f64/n as f64).cos()
            + if j%2 == 0 { 1.0 } else { -1.0 }
    }).unwrap();
    let fast_ok = check("FAST even-N Nyquist weighting", result.s[0], 1.0/3.0, 1e-12);
    if !(shape_ok && rbd_ok && fast_ok) { std::process::exit(1); }
}
'''

def main():
    with tempfile.TemporaryDirectory(prefix="salib-frequency-audit-") as temporary:
        directory = Path(temporary)
        (directory / "src").mkdir()
        (directory / "src/main.rs").write_text(RUST)
        manifest = ('[package]\nname="salib-frequency-literature-check"\n'
                    'version="0.0.0"\nedition="2024"\n'
                    '[dependencies]\nndarray="0.16"\n')
        for crate in ("salib-core", "salib-samplers", "salib-estimators"):
            manifest += crate + '={path=' + json.dumps(str(ROOT/"crates"/crate)) + '}\n'
        (directory / "Cargo.toml").write_text(manifest)
        environment=os.environ.copy()
        environment.setdefault("CARGO_TARGET_DIR",str(ROOT/"target"))
        return subprocess.run(["cargo","run","--quiet","--offline","--manifest-path",
                               str(directory/"Cargo.toml")],env=environment,check=False).returncode

if __name__ == "__main__":
    raise SystemExit(main())
