# Benchmarks

Current measurements for the subset of methods in [the harness](../benches/sensitivity.rs), plus sampling. This is a timing comparison, not an estimator accuracy test.

## Methodology

Measured on **2026-10-07**, source revision `bda959d` (documentation changes do not alter benchmark algorithms). Platform: macOS arm64 sandbox; compiler: `rustc 1.94.1`.

[Criterion](https://github.com/bheisler/criterion.rs) 0.5; 100 samples per benchmark; 0.5 s warmup and 1 s requested measurement time. Statistic: **median** of Criterion's per-iteration timing estimates.

These local release-build measurements are for orientation. The requested window is short; Criterion extends collection when 100 samples need longer and reports outliers. They are not precise cross-machine performance claims. Repeat with longer windows on your target hardware; concurrent processes and thermal state affect timing.

**Workload:** the Ishigami formula with three inputs. The current harness feeds unit-cube samples directly, so this is **not** canonical Ishigami on `[-pi,pi]`. Sobol' G uses eight unit-cube inputs. These timings do not establish convergence to canonical analytic indices.

**What is timed:** [fn] methods include model evaluation and estimation on a prebuilt design; sampling is outside their timed loops. Given-data methods time analysis of precomputed inputs/outputs. DGSM excludes gradient acquisition and output-variance computation. Sampling rows time design construction separately.

## Analysis

| Method | $N$ | Time |
|---|---|---|
| Sobol (Saltelli 2010) [fn] | 1024 | 98 µs |
| Sobol (Saltelli 2010) [fn] | 4096 | 361 µs |
| Sobol (Saltelli 2010) [fn] | 8192 | 782 µs |
| Sobol (Saltelli 2010) [fn] | 16384 | 1.54 ms |
| Jansen [fn] | 1024 | 79 µs |
| Jansen [fn] | 4096 | 309 µs |
| Jansen [fn] | 8192 | 657 µs |
| Janon [fn] | 1024 | 98 µs |
| Janon [fn] | 4096 | 364 µs |
| Janon [fn] | 8192 | 803 µs |
| Sobol on Sobol' G (8D) [fn] | 1024 | 180 µs |
| Sobol on Sobol' G (8D) [fn] | 4096 | 874 µs |
| Sobol on Sobol' G (8D) [fn] | 8192 | 5.25 ms |
| FAST / eFAST [fn] | 1025 | 224 µs |
| FAST / eFAST [fn] | 4097 | 727 µs |
| FAST / eFAST [fn] | 8193 | 2.43 ms |
| Morris [fn] | 10 | 14 µs |
| Morris [fn] | 20 | 4.4 µs |
| Morris [fn] | 50 | 9.2 µs |
| RBD-FAST | 1024 | 192 µs |
| RBD-FAST | 4096 | 986 µs |
| RBD-FAST | 8192 | 1.85 ms |
| Borgonovo $\delta$ | 1024 | 3.80 ms |
| Borgonovo $\delta$ | 4096 | 41 ms |
| Borgonovo $\delta$ | 8192 | 29 ms |
| PAWN | 1024 | 366 µs |
| PAWN | 4096 | 7.21 ms |
| PAWN | 8192 | 4.06 ms |
| DGSM | 1024 | 7.7 µs |
| DGSM | 4096 | 34 µs |
| DGSM | 8192 | 65 µs |
| Regression (SRC/SRRC/PCC/PRCC) | 1024 | 1.28 ms |
| Regression (SRC/SRRC/PCC/PRCC) | 4096 | 3.77 ms |
| Regression (SRC/SRRC/PCC/PRCC) | 8192 | 9.66 ms |

**[fn]** includes model calls. Ishigami is a small arithmetic model; simulator or service latency can dominate in an application. No Python SALib speedup is inferred from these measurements.

Morris $N$ is trajectory count $r$; total evaluations are $r \times (d + 1)$.

FAST $N$ values are odd (required by the algorithm).

## Sampling

| Method | $N$ | Time |
|---|---|---|
| Saltelli matrix | 1024 | 37 µs |
| Saltelli matrix | 4096 | 151 µs |
| Saltelli matrix | 16384 | 642 µs |
| Morris trajectories | 10 | 2.7 µs |
| Morris trajectories | 50 | 11 µs |
| Morris trajectories | 100 | 29 µs |

Saltelli $N$ is rows per base matrix: the bundle requires $N(d+2)$ evaluations and contains separate base/hybrid arrays, not a single stacked matrix. Morris $N$ is trajectory count.

## Reproducing

```bash
CRITERION_HOME="$PWD/target/criterion" cargo bench -p salib --bench sensitivity -- --sample-size 100 --warm-up-time 0.5 --measurement-time 1

# CRITERION_HOME keeps generated artifacts in the ignored workspace target directory.
# Regenerate from those results; use --criterion-dir for a custom output location.
python3 benches/generate_benchmark_doc.py --run-date '2026-10-07' --revision 'bda959d' --platform 'macOS arm64 sandbox' --rustc 'rustc 1.94.1' --sample-size 100 --warm-up-time 0.5 --measurement-time 1
```
