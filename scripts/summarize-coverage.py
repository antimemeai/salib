#!/usr/bin/env python3
"""Summarize salib-gtheory-coverage output (one or more Markdown row files).

Usage: scripts/summarize-coverage.py coverage-*.md > summary.md

Reads the per-cell rows the harness writes and prints, as Markdown:
- pass counts and coverage quantiles by estimand and method;
- mean lower and upper misses by component and method;
- facet-SE ratios by component;
- G and Phi coverage by pattern, data distribution and rater count.

A row passes when coverage is in [.94, .96] and each tail miss is in
[.015, .035] (the plan's criterion); a missing interval counts as a miss.
"""
import collections
import statistics
import sys

COMPONENTS = ["p", "i", "r", "pi", "pr", "ir", "pir"]


def rows(paths):
    for path in paths:
        for line in open(path, encoding="utf-8"):
            if not line.startswith("| ") or "---" in line or "| estimand |" in line:
                continue
            c = [x.strip() for x in line.strip().strip("|").split("|")]
            yield {
                "cell": c[0], "pattern": c[1], "data": c[2], "estimand": c[3], "method": c[4],
                "avail": float(c[6]), "coverage": float(c[7].split()[0]),
                "lower": float(c[8]), "upper": float(c[9]), "notes": c[13], "pass": c[14] == "pass",
                "n_r": int(c[0].split("×")[2]),
            }


def q(values):
    v = sorted(values)
    return f"{v[0]:.3f} / {statistics.median(v):.3f} / {v[-1]:.3f}"


def main(paths):
    data = list(rows(paths))
    cells = {r["cell"] + r["pattern"] + r["data"] for r in data}
    print(f"{len(cells)} cells, {len(data)} rows.\n")

    print("## By estimand and method\n")
    print("| estimand | method | pass | coverage min / median / max | mean lower miss | mean upper miss | median avail |")
    print("|---|---|---|---|---|---|---|")
    groups = collections.defaultdict(list)
    for r in data:
        groups[(r["estimand"], r["method"])].append(r)
    order = {e: k for k, e in enumerate(COMPONENTS + ["G", "Phi"])}
    for (e, m), rs in sorted(groups.items(), key=lambda kv: (order.get(kv[0][0], 99), kv[0][1])):
        print(f"| {e} | {m} | {sum(r['pass'] for r in rs)}/{len(rs)} | {q(r['coverage'] for r in rs)} | "
              f"{statistics.mean(r['lower'] for r in rs):.3f} | {statistics.mean(r['upper'] for r in rs):.3f} | "
              f"{statistics.median(r['avail'] for r in rs):.3f} |")

    print("\n## Facet standard-error ratios (√mean SE² / empirical SD)\n")
    print("| component | min / median / max |")
    print("|---|---|")
    for e in COMPONENTS:
        v = [float(r["notes"].split()[-1]) for r in data
             if r["estimand"] == e and "ratio" in r["notes"]]
        if v:
            print(f"| {e} | {q(v)} |")

    print("\n## G and Phi by pattern, data and rater count\n")
    print("| estimand | method | slice | pass | coverage min / median / max |")
    print("|---|---|---|---|---|")
    methods = sorted({(r["estimand"], r["method"]) for r in data if r["estimand"] in ("G", "Phi")})
    for e, m in methods:
        for key in ["pattern", "data", "n_r"]:
            sl = collections.defaultdict(list)
            for r in data:
                if r["estimand"] == e and r["method"] == m:
                    sl[r[key]].append(r)
            for val, rs in sorted(sl.items(), key=lambda kv: str(kv[0])):
                print(f"| {e} | {m} | {key}={val} | {sum(r['pass'] for r in rs)}/{len(rs)} | {q(r['coverage'] for r in rs)} |")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    main(sys.argv[1:])
