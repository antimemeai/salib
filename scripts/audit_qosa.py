#!/usr/bin/env python3
"""Check QOSA population identities; exit nonzero on an implementation defect.

Run: python3 scripts/audit_qosa.py --toolchain 1.95.0
Uses existing workspace dependencies and removes its temporary example afterward.
"""

import argparse
import os
from pathlib import Path
import subprocess
import sys
import tempfile

SOURCE = r'''
use ndarray::Array2;
use salib_estimators::estimate_qosa;
fn main() {
    for n in [122_880, 245_760] {
        let y: Vec<f64> = (0..n).map(|j| (j as f64 + 0.5) / n as f64).collect();
        let x = Array2::from_shape_vec((n,1), y.clone()).unwrap();
        let result = estimate_qosa(x.view(), &y, 0.5).unwrap();
        println!("cap: N={n}, true S=1, 48-bin target={}, actual={:.12}",47.0/48.0,result.s[0]);
        assert!((result.s[0] - 47.0/48.0).abs() < 1e-12, "incorrect empirical partition target");
    }
    let n=64;
    let x=Array2::from_shape_fn((n,1),|(j,_)| (j%4) as f64);
    let y: Vec<f64>=(0..n).map(|j| [0.0,1.0,1.0,2.0][j%4]).collect();
    let atoms_ok = estimate_qosa(x.view(),&y,0.5).map(|v| (v.s[0]-1.0).abs()<1e-12).unwrap_or(false);
    println!("atoms: Y determined by X, true contrast S=1, actual={:?}",estimate_qosa(x.view(),&y,0.5));
    let n=64;
    let x=Array2::from_shape_fn((n,1),|(j,_)| (j as f64+0.5)/n as f64);
    let y:Vec<f64>=x.column(0).to_vec();
    let shifted:Vec<f64>=y.iter().map(|v| v+100.0).collect();
    println!("translation: continuous model Y=X, partition target unchanged by shift; actual base={:?}; shifted={:?}",estimate_qosa(x.view(),&y,0.9).map(|v|v.s),estimate_qosa(x.view(),&shifted,0.9).map(|v|v.s));
    let base = estimate_qosa(x.view(), &y, 0.9).unwrap().s[0];
    let offset = estimate_qosa(x.view(), &shifted, 0.9).unwrap().s[0];
    assert!((base-offset).abs()<1e-12 && atoms_ok,
        "QOSA must preserve translation and handle this nondegenerate atomic contrast");
}
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--toolchain", help="optional installed rustup toolchain")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    examples = root / "crates/salib-estimators/examples"
    existed = examples.exists()
    examples.mkdir(exist_ok=True)
    example = None
    try:
        with tempfile.NamedTemporaryFile(mode="w", prefix="qosa_audit_", suffix=".rs",
                                         dir=examples, delete=False) as handle:
            handle.write(SOURCE)
            example = Path(handle.name)
        env = os.environ.copy()
        rustup_bin = Path.home() / ".cargo/bin"
        if (rustup_bin / "cargo").exists():
            env["PATH"] = str(rustup_bin) + os.pathsep + env.get("PATH", "")
        command = ["cargo"]
        if args.toolchain:
            command.append("+" + args.toolchain)
        command += ["run", "--offline", "--locked", "--quiet", "-p", "salib-estimators",
                    "--example", example.stem]
        return subprocess.run(command, cwd=root, env=env).returncode
    finally:
        if example is not None:
            example.unlink(missing_ok=True)
        if not existed:
            examples.rmdir()


if __name__ == "__main__":
    sys.exit(main())
