//! Population oracles (11, 12, 15), using coupled designs within independent LHS runs.
#![allow(clippy::unwrap_used, clippy::expect_used)]

mod metamorphic_statistical {
    use salib_core::{Group, RngState};
    use salib_estimators::*;
    use salib_samplers::*;

    const N: usize = 16_384;
    const TOL: f64 = 0.05;
    const RUNS: usize = 4;

    fn rng(run: usize) -> RngState {
        // Independent ChaCha streams; changing seeds of unscrambled Sobol would
        // reproduce the same design and would not supply independent evidence.
        RngState::from_seed([0u8; 32]).fork(&run.to_le_bytes())
    }
    fn close(got: f64, want: f64) {
        assert!(got.is_finite() && want.is_finite());
        assert!(
            (got - want).abs() < TOL,
            "got {got}, population value {want}"
        );
    }
    fn population_bounds(s: &[f64], st: &[f64]) {
        for (&s, &st) in s.iter().zip(st) {
            assert!(s.is_finite() && st.is_finite());
            assert!(s >= -TOL && s <= st + TOL && st <= 1.0 + TOL);
        }
        assert!(s.iter().sum::<f64>() <= 1.0 + TOL);
        assert!(st.iter().sum::<f64>() >= 1.0 - TOL);
    }

    /// Relation 11: coefficient magnitude, sign reversal, and additive shares.
    #[test]
    fn additive_coefficient_monotonically_changes_population_shares() {
        let ts = [0.0, 1.0, 2.0, 4.0];
        // Store estimates for every method to assess paired mean differences.
        let mut means = vec![vec![vec![0.0; 3]; ts.len()]; 8];
        for run in 0..RUNS {
            let mut state = rng(run);
            let m = build_saltelli_matrix(&LhsSampler::classic(6), N, true, &mut state).unwrap();
            let o = build_owen_matrix(&LhsSampler::classic(9), N, &mut state).unwrap();
            let fast = build_fast_design(3, 4097, 4, &mut state).unwrap();
            let x = LhsSampler::classic(3).unit_sample(N, &mut state);
            for (k, &t) in ts.iter().enumerate() {
                let f = |x: &[f64]| t * (x[0] - 0.5) + 2.0 * (x[1] - 0.5) + 3.0 * (x[2] - 0.5);
                let a = estimate_saltelli2010(&m, f);
                population_bounds(&a.first_order, &a.total_order);
                for v in a.second_order.as_ref().unwrap().iter().flatten() {
                    close(*v, 0.0);
                }
                let ft = estimate_fast(&fast, f).unwrap();
                let y: Vec<_> = x
                    .rows()
                    .into_iter()
                    .map(|r| f(r.as_slice().unwrap()))
                    .collect();
                // A sorted linear response is a sawtooth: 32 harmonics
                // keep spectral truncation below the statistical tolerance.
                let fields = [a.first_order,
                    a.total_order,
                    estimate_jansen(&m, f).first_order,
                    estimate_janon(&m, f).first_order,
                    estimate_owen(&o, f).first_order,
                    ft.s,
                    estimate_given_data_sobol(x.view(), &y).unwrap().s1,
                    estimate_rbd_fast(x.view(), &y, 32).unwrap().s];
                let want = [
                    t * t / (t * t + 13.0),
                    4.0 / (t * t + 13.0),
                    9.0 / (t * t + 13.0),
                ];
                for (method, field) in fields.iter().enumerate() {
                    for i in 0..3 {
                        assert!(field[i].is_finite());
                        assert!(
                            (field[i] - want[i]).abs() < TOL,
                            "method {method}, run {run}, t={t}, factor {i}: got {}, want {}",
                            field[i],
                            want[i]
                        );
                        means[method][k][i] += field[i] / RUNS as f64;
                    }
                }
                for (i, &w) in want.iter().enumerate().take(3) {
                    close(ft.st[i], w);
                }
                let negative =
                    |x: &[f64]| -t * (x[0] - 0.5) + 2.0 * (x[1] - 0.5) + 3.0 * (x[2] - 0.5);
                let neg = estimate_saltelli2010(&m, negative);
                for i in 0..3 {
                    close(neg.first_order[i], want[i]);
                    close(neg.total_order[i], want[i]);
                    close(neg.first_order[i], fields[0][i]);
                    close(neg.total_order[i], fields[1][i]);
                }
            }
        }
        for method in means {
            for pair in method.windows(2) {
                // Require separated paired means for these coefficient steps
                // across paired runs, instead of ordering near-equal noise.
                assert!(pair[1][0] - pair[0][0] > 0.04);
                assert!(pair[0][1] - pair[1][1] > 0.005);
                assert!(pair[0][2] - pair[1][2] > 0.02);
            }
        }
    }

    /// Relation 12: known pure-interaction decomposition and paired directions.
    #[test]
    fn pure_interaction_decreases_first_and_increases_total_effects() {
        let mut means = [[0.0; 5]; 3];
        for run in 0..RUNS {
            let mut state = rng(run);
            let m = build_saltelli_matrix(&LhsSampler::classic(4), N, true, &mut state).unwrap();
            let fast = build_fast_design(2, 4097, 4, &mut state).unwrap();
            for (k, t) in [0.0, 1.0, 2.0].into_iter().enumerate() {
                let f = |x: &[f64]| {
                    let u = 2.0 * x[0] - 1.0;
                    let v = 2.0 * x[1] - 1.0;
                    u + v + t * u * v
                };
                let d = 2.0 / 3.0 + t * t / 9.0;
                let s = (1.0 / 3.0) / d;
                let interaction = (t * t / 9.0) / d;
                let st = s + interaction;
                let a = estimate_saltelli2010(&m, f);
                let j = estimate_jansen(&m, f);
                let n = estimate_janon(&m, f);
                let ft = estimate_fast(&fast, f).unwrap();
                population_bounds(&a.first_order, &a.total_order);
                for field in [&a.first_order, &j.first_order, &n.first_order, &ft.s] {
                    for &got in field {
                        close(got, s);
                    }
                }
                for field in [&a.total_order, &ft.st] {
                    for &got in field {
                        close(got, st);
                    }
                }
                for second in [
                    a.second_order.as_ref().unwrap(),
                    j.second_order.as_ref().unwrap(),
                    n.second_order.as_ref().unwrap(),
                ] {
                    close(second[0][0], interaction);
                }
                for i in 0..2 {
                    close(
                        a.total_order[i],
                        a.first_order[i] + a.second_order.as_ref().unwrap()[0][0],
                    );
                    means[k][i] += a.first_order[i] / RUNS as f64;
                    means[k][i + 2] += a.total_order[i] / RUNS as f64;
                }
                means[k][4] += a.second_order.as_ref().unwrap()[0][0] / RUNS as f64;
            }
        }
        for pair in means.windows(2) {
            for i in 0..2 {
                assert!(pair[0][i] - pair[1][i] > 0.03);
                assert!(pair[1][i + 2] - pair[0][i + 2] > 0.03);
            }
            assert!(pair[1][4] - pair[0][4] > 0.08);
        }
    }

    /// Relation 15: nested groups, with interaction included in the larger set.
    #[test]
    fn larger_factor_groups_capture_more_first_and_total_variance() {
        let mut differences = [0.0; 2];
        for run in 0..RUNS {
            let sampler = LhsSampler::classic(6);
            let initial = rng(run);
            let small = [
                Group {
                    name: "zero".into(),
                    factor_indices: vec![0],
                },
                Group {
                    name: "rest".into(),
                    factor_indices: vec![1, 2],
                },
            ];
            let large = [
                Group {
                    name: "zero_one".into(),
                    factor_indices: vec![0, 1],
                },
                Group {
                    name: "rest".into(),
                    factor_indices: vec![2],
                },
            ];
            let a = build_grouped_saltelli_matrix(&sampler, &small, N, false, &mut initial.clone())
                .unwrap();
            let b = build_grouped_saltelli_matrix(&sampler, &large, N, false, &mut initial.clone())
                .unwrap();
            assert_eq!(a.a, b.a);
            assert_eq!(a.b, b.b);
            let f = |x: &[f64]| {
                let u = 2.0 * x[0] - 1.0;
                let v = 2.0 * x[1] - 1.0;
                u + v + 2.0 * u * v + (2.0 * x[2] - 1.0)
            };
            let aa = estimate_saltelli2010(&a, f);
            let bb = estimate_saltelli2010(&b, f);
            let d = 1.0 + 4.0 / 9.0;
            close(aa.first_order[0], (1.0 / 3.0) / d);
            close(aa.total_order[0], (1.0 / 3.0 + 4.0 / 9.0) / d);
            close(bb.first_order[0], (2.0 / 3.0 + 4.0 / 9.0) / d);
            close(bb.total_order[0], (2.0 / 3.0 + 4.0 / 9.0) / d);
            assert!(aa.first_order[0] <= bb.first_order[0] + TOL);
            assert!(aa.total_order[0] <= bb.total_order[0] + TOL);
            differences[0] += (bb.first_order[0] - aa.first_order[0]) / RUNS as f64;
            differences[1] += (bb.total_order[0] - aa.total_order[0]) / RUNS as f64;
        }
        close(differences[0], 7.0 / 13.0);
        close(differences[1], 3.0 / 13.0);
    }
}
