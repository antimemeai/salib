#!/usr/bin/env python3
"""
Generate docs/benchmarks.md from Criterion results.

Reads: target/criterion/*/new/estimates.json
Writes: docs/benchmarks.md

Timing tables are generated from the recorded Criterion medians.
"""

import argparse
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRITERION_BASE = os.path.join(REPO, "target", "criterion")
OUTPUT = os.path.join(REPO, "docs", "benchmarks.md")


def load_rust_median(group, param):
    path = os.path.join(CRITERION_BASE, group, str(param), "new", "estimates.json")
    if not os.path.isfile(path):
        return None
    with open(path) as f:
        d = json.load(f)
    return d["median"]["point_estimate"]  # nanoseconds


def fmt_time(ns):
    us = ns / 1000
    if us < 1000:
        return f"{us:.0f} µs" if us >= 10 else f"{us:.1f} µs"
    ms = us / 1000
    return f"{ms:.2f} ms" if ms < 10 else f"{ms:.0f} ms"


# (doc_name, criterion_group, N_values, includes_fn_eval)
METHODS = [
    ("Sobol (Saltelli 2010)", "saltelli2010_ishigami", [1024, 4096, 8192, 16384], True),
    ("Jansen",                "jansen_ishigami",        [1024, 4096, 8192],        True),
    ("Janon",                 "janon_ishigami",          [1024, 4096, 8192],        True),
    ("Sobol on Sobol' G (8D)", "saltelli2010_sobol_g",  [1024, 4096, 8192],        True),
    ("FAST / eFAST",          "fast_ishigami",           [1025, 4097, 8193],        True),
    ("Morris",                "morris_ishigami",         [10, 20, 50],              True),
    ("RBD-FAST",              "rbd_fast_ishigami",       [1024, 4096, 8192],        False),
    ("Borgonovo $\\delta$",   "borgonovo_ishigami",      [1024, 4096, 8192],        False),
    ("PAWN",                  "pawn_ishigami",           [1024, 4096, 8192],        False),
    ("DGSM",                  "dgsm_ishigami",           [1024, 4096, 8192],        False),
    ("Regression (SRC/SRRC/PCC/PRCC)", "regression_ishigami", [1024, 4096, 8192],  False),
]

SAMPLING = [
    ("Saltelli matrix",  "sampling_saltelli", [1024, 4096, 16384]),
    ("Morris trajectories", "sampling_morris", [10, 50, 100]),
]


def main():
    global CRITERION_BASE
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--criterion-dir", default=CRITERION_BASE)
    parser.add_argument("--run-date", default="not recorded")
    parser.add_argument("--revision", default="not recorded")
    parser.add_argument("--platform", default="not recorded")
    parser.add_argument("--rustc", default="not recorded")
    parser.add_argument("--sample-size", type=int, default=100)
    parser.add_argument("--warm-up-time", type=float, default=3.0)
    parser.add_argument("--measurement-time", type=float, default=5.0)
    args = parser.parse_args()
    CRITERION_BASE = args.criterion_dir
    # Missing results must not silently replace the document with partial tables.
    missing = [f"{group}/{n}" for _, group, ns, *_ in METHODS + SAMPLING
               for n in ns if load_rust_median(group, n) is None]
    if missing:
        raise SystemExit("Missing Criterion results: " + ", ".join(missing))
    lines = []
    w = lines.append

    w("# Benchmarks")
    w("")
    w("Timings for the methods and samplers in [the benchmark harness](../benches/sensitivity.rs). Accuracy must be checked separately.")
    w("")
    w("## Methodology")
    w("")
    w(f"Measured on **{args.run_date}**, source revision `{args.revision}` (documentation changes do not alter benchmark algorithms). Platform: {args.platform}; compiler: `{args.rustc}`.")
    w("")
    w(f"[Criterion](https://github.com/bheisler/criterion.rs) 0.5; {args.sample_size} samples per benchmark; {args.warm_up_time:g} s warmup and {args.measurement_time:g} s requested measurement time. Statistic: **median** of Criterion's per-iteration timing estimates.")
    w("")
    w("These release-build measurements used short timing windows. Criterion extends collection when the requested sample count needs longer and reports outliers. For performance decisions, repeat with longer windows on the target hardware; other processes and machine temperature affect timings.")
    w("")
    w("**Workload:** the three-input Ishigami formula evaluated on unit-cube samples. This differs from its canonical `[-pi,pi]` input range. Sobol\' G uses eight unit-cube inputs. These timings do not establish convergence to canonical analytic indices.")
    w("")
    w("**Timed operations:** [fn] methods include model evaluation and estimation on a prebuilt design; sampling is outside their timed loops. Given-data methods time analysis of precomputed inputs/outputs. DGSM excludes gradient acquisition and output-variance computation. Sampling rows time design construction separately.")
    w("")

    # ── Analysis benchmarks ──
    w("## Analysis")
    w("")
    w("| Method | $N$ | Time |")
    w("|---|---|---|")
    for doc_name, group, ns, has_fn in METHODS:
        tag = " [fn]" if has_fn else ""
        for n in ns:
            rust_ns = load_rust_median(group, n)
            if rust_ns is None:
                continue
            w(f"| {doc_name}{tag} | {n} | {fmt_time(rust_ns)} |")
    w("")

    w("**[fn]** includes model calls. Ishigami is a small arithmetic model; simulator or service latency can dominate in an application. No Python SALib speedup is inferred from these measurements.")
    w("")

    # ── Morris note ──
    w("Morris $N$ is trajectory count $r$; total evaluations are $r \\times (d + 1)$.")
    w("")
    w("The benchmark uses odd FAST sample counts. These recorded timings predate the 0.3.0 correctness fixes; rerun the harness to measure the corrected algorithms.")
    w("")

    # ── Sampling benchmarks ──
    w("## Sampling")
    w("")
    w("| Method | $N$ | Time |")
    w("|---|---|---|")
    for doc_name, group, ns in SAMPLING:
        for n in ns:
            rust_ns = load_rust_median(group, n)
            if rust_ns is None:
                continue
            w(f"| {doc_name} | {n} | {fmt_time(rust_ns)} |")
    w("")

    w("Saltelli $N$ is rows per base matrix: the bundle requires $N(d+2)$ evaluations and contains separate base/hybrid arrays, not a single stacked matrix. Morris $N$ is trajectory count.")
    w("")

    # ── Reproducing ──
    w("## Reproducing")
    w("")
    w("```bash")
    w(f'CRITERION_HOME="$PWD/target/criterion" cargo bench -p salib --bench sensitivity -- --sample-size {args.sample_size} --warm-up-time {args.warm_up_time:g} --measurement-time {args.measurement_time:g}')
    w("")
    w("# CRITERION_HOME keeps generated artifacts in the ignored workspace target directory.")
    w("# Regenerate from those results; use --criterion-dir for a custom output location.")
    w("python3 benches/generate_benchmark_doc.py "
      f"--run-date '{args.run_date}' --revision '{args.revision}' "
      f"--platform '{args.platform}' --rustc '{args.rustc}' "
      f"--sample-size {args.sample_size} --warm-up-time {args.warm_up_time:g} "
      f"--measurement-time {args.measurement_time:g}")
    w("```")
    w("")

    doc = "\n".join(lines)
    with open(OUTPUT, "w") as f:
        f.write(doc.rstrip() + "\n")
    print(f"Wrote {OUTPUT}")
    print(f"  {len(lines)} lines")


if __name__ == "__main__":
    main()
