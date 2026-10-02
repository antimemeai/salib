//! Coupling and deterministic sampler oracles (21–24).
#![allow(clippy::unwrap_used, clippy::expect_used)]

mod metamorphic_sampler {
    use ndarray::{Array2, Axis};
    use salib_core::{Distribution, RngState};
    use salib_estimators::{estimate_owen, estimate_saltelli2010};
    use salib_samplers::*;

    fn rng() -> RngState {
        RngState::from_seed([0u8; 32])
    }
    fn close(got: f64, want: f64) {
        assert!(
            got.is_finite() && want.is_finite(),
            "nonfinite: {got}, {want}"
        );
        assert!((got - want).abs() <= 1e-10, "got {got}, want {want}");
    }
    fn same(a: &[f64], b: &[f64]) {
        assert_eq!(a.len(), b.len());
        for (&a, &b) in a.iter().zip(b) {
            close(a, b);
        }
    }
    fn map(x: &Array2<f64>, distributions: &[Distribution]) -> Array2<f64> {
        Array2::from_shape_fn(x.dim(), |(i, j)| distributions[j].quantile(x[[i, j]]))
    }
    fn model(x: &[f64]) -> f64 {
        x[0] + 0.5 * x[1] + x[2] + 0.25 * x[0] * x[2]
    }
    fn evaluate(x: &Array2<f64>, f: impl Fn(&[f64]) -> f64) -> Vec<f64> {
        x.rows()
            .into_iter()
            .map(|r| f(r.as_slice().unwrap()))
            .collect()
    }

    /// Relation 21: affine quantile coupling, then model and estimator coupling.
    #[test]
    fn affine_input_units_commute_with_quantiles_and_model_evaluation() {
        let base = [
            Distribution::Uniform { lo: -1.0, hi: 2.0 },
            Distribution::Normal {
                mu: 0.5,
                sigma: 0.75,
            },
            Distribution::Triangular {
                lo: -2.0,
                mode: 0.0,
                hi: 1.0,
            },
        ];
        let units = [
            Distribution::Uniform { lo: 1.0, hi: 7.0 },
            Distribution::Normal {
                mu: -0.75,
                sigma: 0.375,
            },
            Distribution::Triangular {
                lo: -2.0,
                mode: 4.0,
                hi: 7.0,
            },
        ];
        let scales = [2.0, 0.5, 3.0];
        let shifts = [3.0, -1.0, 4.0];
        for j in 0..3 {
            for k in 1..100 {
                let u = f64::from(k) / 100.0;
                close(
                    units[j].quantile(u),
                    scales[j] * base[j].quantile(u) + shifts[j],
                );
            }
        }
        let compensated = |x: &[f64]| {
            model(&[
                (x[0] - shifts[0]) / scales[0],
                (x[1] - shifts[1]) / scales[1],
                (x[2] - shifts[2]) / scales[2],
            ])
        };
        let unit = build_saltelli_matrix(&LhsSampler::classic(6), 256, true, &mut rng()).unwrap();
        let mut a = unit.clone();
        let mut b = unit.clone();
        a.a = map(&unit.a, &base);
        a.b = map(&unit.b, &base);
        a.a_b = unit.a_b.iter().map(|x| map(x, &base)).collect();
        a.b_a = unit
            .b_a
            .as_ref()
            .map(|v| v.iter().map(|x| map(x, &base)).collect());
        b.a = map(&unit.a, &units);
        b.b = map(&unit.b, &units);
        b.a_b = unit.a_b.iter().map(|x| map(x, &units)).collect();
        b.b_a = unit
            .b_a
            .as_ref()
            .map(|v| v.iter().map(|x| map(x, &units)).collect());
        for (a, b) in std::iter::once(&a.a)
            .chain(std::iter::once(&a.b))
            .chain(a.a_b.iter())
            .chain(a.b_a.as_ref().unwrap())
            .zip(
                std::iter::once(&b.a)
                    .chain(std::iter::once(&b.b))
                    .chain(b.a_b.iter())
                    .chain(b.b_a.as_ref().unwrap()),
            )
        {
            same(&evaluate(a, model), &evaluate(b, compensated));
        }
        let aa = estimate_saltelli2010(&a, model);
        let bb = estimate_saltelli2010(&b, compensated);
        same(&aa.first_order, &bb.first_order);
        same(&aa.total_order, &bb.total_order);
        close(aa.total_variance, bb.total_variance);
        for (a, b) in aa
            .second_order
            .unwrap()
            .iter()
            .zip(bb.second_order.unwrap())
        {
            same(a, &b);
        }
        let unit = build_owen_matrix(&LhsSampler::classic(9), 256, &mut rng()).unwrap();
        let mut a = unit.clone();
        let mut b = unit.clone();
        a.a = map(&unit.a, &base);
        a.b = map(&unit.b, &base);
        a.c = map(&unit.c, &base);
        a.a_c = unit.a_c.iter().map(|x| map(x, &base)).collect();
        a.b_a = unit.b_a.iter().map(|x| map(x, &base)).collect();
        b.a = map(&unit.a, &units);
        b.b = map(&unit.b, &units);
        b.c = map(&unit.c, &units);
        b.a_c = unit.a_c.iter().map(|x| map(x, &units)).collect();
        b.b_a = unit.b_a.iter().map(|x| map(x, &units)).collect();
        for (a, b) in std::iter::once(&a.a)
            .chain(std::iter::once(&a.b))
            .chain(std::iter::once(&a.c))
            .chain(a.a_c.iter())
            .chain(a.b_a.iter())
            .zip(
                std::iter::once(&b.a)
                    .chain(std::iter::once(&b.b))
                    .chain(std::iter::once(&b.c))
                    .chain(b.a_c.iter())
                    .chain(b.b_a.iter()),
            )
        {
            same(&evaluate(a, model), &evaluate(b, compensated));
        }
        let aa = estimate_owen(&a, model);
        let bb = estimate_owen(&b, compensated);
        same(&aa.first_order, &bb.first_order);
        close(aa.total_variance, bb.total_variance);
    }

    /// Relation 22: every supported distribution, including discrete jumps.
    #[test]
    fn quantiles_are_monotone_and_match_support_endpoints() {
        let distributions = [
            Distribution::Uniform { lo: -2.0, hi: 3.0 },
            Distribution::Normal {
                mu: 1.0,
                sigma: 2.0,
            },
            Distribution::LogNormal {
                mu_log: 0.5,
                sigma_log: 0.75,
            },
            Distribution::Triangular {
                lo: -2.0,
                mode: 0.0,
                hi: 3.0,
            },
            Distribution::Beta {
                alpha: 2.0,
                beta: 3.0,
                lo: -1.0,
                hi: 2.0,
            },
            Distribution::Gamma {
                shape: 2.0,
                scale: 0.5,
            },
            Distribution::Weibull {
                shape: 1.5,
                scale: 2.0,
            },
            Distribution::Exponential { lambda: 2.0 },
            Distribution::Bernoulli { p: 0.3 },
            Distribution::DiscreteUniform { lo: -2, hi: 4 },
        ];
        for d in distributions {
            let (lo, hi) = d.support();
            assert_eq!(d.quantile(0.0), lo);
            assert_eq!(d.quantile(1.0), hi);
            let mut previous = lo;
            for k in 1..100 {
                let q = d.quantile(f64::from(k) / 100.0);
                assert!(q.is_finite());
                assert!(q >= previous, "{d:?}: {q} < {previous}");
                previous = q;
            }
            assert!(previous <= hi);
        }
    }

    /// Relation 22: location, rate, scale, and Bernoulli parameter order.
    #[test]
    fn quantile_parameter_changes_follow_location_and_scale_oracles() {
        let pairs = [
            (
                Distribution::Normal {
                    mu: 0.5,
                    sigma: 0.75,
                },
                Distribution::Normal {
                    mu: 3.5,
                    sigma: 0.75,
                },
                1.0,
                3.0,
            ),
            (
                Distribution::Exponential { lambda: 0.75 },
                Distribution::Exponential { lambda: 2.25 },
                1.0 / 3.0,
                0.0,
            ),
            (
                Distribution::Gamma {
                    shape: 2.0,
                    scale: 0.75,
                },
                Distribution::Gamma {
                    shape: 2.0,
                    scale: 2.25,
                },
                3.0,
                0.0,
            ),
            (
                Distribution::Weibull {
                    shape: 1.5,
                    scale: 0.75,
                },
                Distribution::Weibull {
                    shape: 1.5,
                    scale: 2.25,
                },
                3.0,
                0.0,
            ),
        ];
        for (a, b, scale, shift) in pairs {
            for k in 1..100 {
                let u = f64::from(k) / 100.0;
                close(b.quantile(u), scale * a.quantile(u) + shift);
            }
        }
        for k in 0..=100 {
            let u = f64::from(k) / 100.0;
            let mut previous = 0.0;
            for p in [0.0, 0.2, 0.5, 0.8, 1.0] {
                let q = Distribution::Bernoulli { p }.quantile(u);
                assert!(q.is_finite() && q >= previous);
                previous = q;
            }
        }
    }

    /// Relation 23: IC only reorders values; target correlations need not be exact.
    #[test]
    fn iman_conover_preserves_marginals_and_commutes_with_increasing_transforms() {
        let mut x = LhsSampler::classic(3).unit_sample(128, &mut rng());
        // Include ties in one marginal: multiset preservation must retain them.
        for i in 0..128 {
            x[[i, 2]] = (i % 7) as f64 / 7.0;
        }
        let targets = [
            Array2::eye(3),
            Array2::from_shape_vec((3, 3), vec![1.0, 0.4, 0.2, 0.4, 1.0, -0.1, 0.2, -0.1, 1.0])
                .unwrap(),
        ];
        for target in targets {
            let mut a = rng();
            let mut b = a.clone();
            let transformed = x.mapv(f64::exp);
            let y = iman_conover_transform(&x, &target, &mut a).unwrap();
            let yy = iman_conover_transform(&transformed, &target, &mut b).unwrap();
            assert_eq!(a, b);
            assert_eq!(yy, y.mapv(f64::exp));
            for j in 0..3 {
                let mut before = x.column(j).to_vec();
                let mut after = y.column(j).to_vec();
                before.sort_by(f64::total_cmp);
                after.sort_by(f64::total_cmp);
                assert_eq!(before, after);
            }
        }
    }

    /// Relation 24: extending dimensions preserves existing columns (not LHS rows).
    #[test]
    fn lhs_and_sobol_dimension_extensions_preserve_column_prefixes() {
        for centered in [false, true] {
            let lhs = |d| {
                if centered {
                    LhsSampler::centered(d)
                } else {
                    LhsSampler::classic(d)
                }
            };
            let a = lhs(2).unit_sample(64, &mut rng());
            let b = lhs(5).unit_sample(64, &mut rng());
            assert_eq!(a, b.select(Axis(1), &[0, 1]));
        }
        for skip in [false, true] {
            let a = SobolSampler::standard(2)
                .with_skip_first(skip)
                .unit_sample(64, &mut rng());
            let b = SobolSampler::standard(5)
                .with_skip_first(skip)
                .unit_sample(64, &mut rng());
            assert_eq!(a, b.select(Axis(1), &[0, 1]));
        }
    }

    /// Relation 24: unscrambled Sobol neither consumes nor consults RNG state.
    #[test]
    fn sobol_size_extension_preserves_rows_and_ignores_rng_state() {
        for skip in [false, true] {
            let sampler = SobolSampler::standard(3).with_skip_first(skip);
            let initial = rng();
            let mut state = initial.clone();
            let a = sampler.unit_sample(64, &mut state);
            assert_eq!(state, initial);
            let b = sampler.unit_sample(128, &mut state);
            assert_eq!(a, b.select(Axis(0), &(0..64).collect::<Vec<_>>()));
            assert_eq!(state, initial);
            for initial in [
                RngState::from_seed([17u8; 32]),
                RngState::from_parts([0u8; 32], 19, 1234),
            ] {
                let mut state = initial.clone();
                assert_eq!(a, sampler.unit_sample(64, &mut state));
                assert_eq!(state, initial);
            }
            assert_eq!(a, sampler.unit_sample(64, &mut state));
        }
    }

    /// Relation 24: compare complete ordered trajectories and their metadata.
    #[test]
    fn morris_trajectory_extension_preserves_complete_prefix() {
        let a = build_morris_trajectories(3, 8, 4, &mut rng()).unwrap();
        let b = build_morris_trajectories(3, 20, 4, &mut rng()).unwrap();
        let prefix: Vec<_> = (0..8).collect();
        assert_eq!(a.trajectories, b.trajectories.select(Axis(0), &prefix));
        assert_eq!(a.deltas, b.deltas.select(Axis(0), &prefix));
        assert_eq!(a.factor_order, b.factor_order.select(Axis(0), &prefix));
        assert_eq!(a.group_order, b.group_order);
    }

    /// Relation 24: exact centers across seeds, and one classic point per stratum.
    #[test]
    fn lhs_seed_changes_preserve_centers_or_stratum_occupancy() {
        let n = 64;
        for seed in [[0u8; 32], [17u8; 32], [93u8; 32]] {
            let centered = LhsSampler::centered(3).unit_sample(n, &mut RngState::from_seed(seed));
            let classic = LhsSampler::classic(3).unit_sample(n, &mut RngState::from_seed(seed));
            for j in 0..3 {
                let mut centers = centered.column(j).to_vec();
                centers.sort_by(f64::total_cmp);
                let expected: Vec<_> = (0..n).map(|k| (k as f64 + 0.5) / n as f64).collect();
                assert_eq!(centers, expected);
                let mut strata: Vec<_> = classic
                    .column(j)
                    .iter()
                    .map(|&v| {
                        assert!(v.is_finite() && (0.0..1.0).contains(&v));
                        (v * n as f64).floor() as usize
                    })
                    .collect();
                strata.sort_unstable();
                assert_eq!(strata, (0..n).collect::<Vec<_>>());
            }
        }
    }
}
