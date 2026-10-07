#!/usr/bin/env python3
"""Check HDMR against an additive-model variance oracle; no Python dependencies.

Run from any directory. Requires the project's Rust dependencies in Cargo's cache.
Exit 1 means the literature identity fails, 0 means all checks pass.

For independent inputs and Y=X0+X1, S_i=Var(X_i)/sum(Var(X_j)).
Source: Sudret (2008), sections 2 and 5, doi:10.1016/j.ress.2007.04.002.
The triangular case checks explicit rejection of unsupported input measures.
Legendre weights require a uniform measure (Xiu/Karniadakis 2002 Table 4.1).
"""

import json
import os
from pathlib import Path
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[1]
RUST = r'''
use ndarray::Array2;
use salib_core::{Distribution, ProblemBuilder};
use salib_estimators::{estimate_hdmr, HdmrError};

fn check(name: &str, second: Distribution, variances: [f64; 2]) -> bool {
    let first = Distribution::Uniform { lo: -1.0, hi: 1.0 };
    let problem = ProblemBuilder::new()
        .factor("uniform", first.clone())
        .factor("second", second.clone())
        .build().unwrap();
    // Tensor grid of marginal quantiles. The fitted model is exactly linear,
    // so this check has neither Monte Carlo nor truncation uncertainty.
    let side = 16;
    let mut x = Array2::<f64>::zeros((side * side, 2));
    let mut y = vec![0.0; side * side];
    for i in 0..side {
        for j in 0..side {
            let row = i * side + j;
            x[[row, 0]] = first.quantile((i as f64 + 0.5) / side as f64);
            x[[row, 1]] = second.quantile((j as f64 + 0.5) / side as f64);
            y[row] = x[[row, 0]] + x[[row, 1]];
        }
    }
    let estimate = estimate_hdmr(x.view(), &y, &problem, 2, 1);
    if matches!(second, Distribution::Triangular { .. }) {
        let passed = matches!(estimate, Err(HdmrError::UnsupportedDistribution { index: 1, .. }));
        println!("{} {name}: unsupported measure must be rejected", if passed { "PASS" } else { "FAIL" });
        return passed;
    }
    let result = match estimate {
        Ok(result) => result,
        Err(error) => { println!("FAIL {name}: {error}"); return false; }
    };
    let variance = variances.iter().sum::<f64>();
    let expected = variances.map(|v| v / variance);
    let error = result.first_order.iter().chain(result.total_order.iter())
        .zip(expected.iter().cycle()).map(|(got, want)| (got-want).abs())
        .fold(0.0_f64, f64::max);
    let passed = error < 1e-10 && (result.total_variance-variance).abs() < 1e-10;
    println!("{} {name}: S={:?}, expected={:?}; variance={}, expected={}",
        if passed { "PASS" } else { "FAIL" }, result.first_order,
        expected, result.total_variance, variance);
    passed
}

fn main() {
    let uniform = check("Uniform + Uniform",
        Distribution::Uniform { lo: -1.0, hi: 1.0 }, [1.0/3.0, 1.0/3.0]);
    let normal = check("Uniform + Normal",
        Distribution::Normal { mu: 10.0, sigma: 2.0 }, [1.0/3.0, 4.0]);
    let triangular = check("Uniform + Triangular",
        Distribution::Triangular { lo: -1.0, hi: 1.0, mode: 0.0 }, [1.0/3.0, 1.0/6.0]);
    if !(uniform && normal && triangular) { std::process::exit(1); }
}
'''


def main():
    with tempfile.TemporaryDirectory(prefix="salib-hdmr-audit-") as temporary:
        directory = Path(temporary)
        (directory / "src").mkdir()
        (directory / "src/main.rs").write_text(RUST)
        (directory / "Cargo.toml").write_text(
            '[package]\nname = "salib-hdmr-literature-check"\n'
            'version = "0.0.0"\nedition = "2024"\n'
            '[dependencies]\nndarray = "0.16"\n'
            'salib-core = { path = '
            + json.dumps(str(ROOT / "crates/salib-core"))
            + ' }\nsalib-estimators = { path = '
            + json.dumps(str(ROOT / "crates/salib-estimators"))
            + ', features = ["surrogate"] }\n'
        )
        environment = os.environ.copy()
        environment.setdefault("CARGO_TARGET_DIR", str(ROOT / "target"))
        return subprocess.run(
            ["cargo", "run", "--quiet", "--offline", "--manifest-path",
             str(directory / "Cargo.toml")], env=environment, check=False
        ).returncode


if __name__ == "__main__":
    raise SystemExit(main())
