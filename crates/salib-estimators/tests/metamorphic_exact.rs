//! Finite-sample oracles from docs/analysis/metamorphic-oracles.md.
#![allow(clippy::unwrap_used, clippy::expect_used)]

// Module names make `cargo test -p salib-estimators metamorphic_exact` run this suite.
mod metamorphic_exact {
    use ndarray::{Array2, Array3, Axis};
    use rand::seq::SliceRandom;
    use salib_core::{Distribution, Group, Problem, ProblemBuilder, RngState};
    use salib_estimators::*;
    use salib_samplers::*;
    use salib_validation::ishigami;

    const TOL: f64 = 1e-10;
    fn close(got: f64, want: f64) {
        assert!(
            got.is_finite() && want.is_finite(),
            "nonfinite: {got}, {want}"
        );
        assert!((got - want).abs() <= TOL, "got {got}, want {want}");
    }
    fn same(a: &[f64], b: &[f64]) {
        assert_eq!(a.len(), b.len());
        for (&a, &b) in a.iter().zip(b) {
            close(a, b);
        }
    }
    fn scaled(a: &[f64], b: &[f64], scale: f64) {
        same(&a.iter().map(|v| scale * v).collect::<Vec<_>>(), b);
    }
    fn second(a: &Option<Vec<Vec<f64>>>, b: &Option<Vec<Vec<f64>>>) {
        assert_eq!(a.is_some(), b.is_some());
        if let (Some(a), Some(b)) = (a, b) {
            assert_eq!(a.len(), b.len());
            for (a, b) in a.iter().zip(b) {
                same(a, b);
            }
        }
    }
    fn rng() -> RngState {
        RngState::from_seed([0u8; 32])
    }
    fn design() -> SaltelliMatrix {
        build_saltelli_matrix(&LhsSampler::classic(6), 256, true, &mut rng()).unwrap()
    }
    fn model(x: &[f64]) -> f64 {
        let p = std::f64::consts::PI;
        ishigami::ishigami(&[
            p * (2.0 * x[0] - 1.0),
            p * (2.0 * x[1] - 1.0),
            p * (2.0 * x[2] - 1.0),
        ]) / 8.0
    }
    fn evaluate(x: &Array2<f64>, f: impl Fn(&[f64]) -> f64) -> Vec<f64> {
        x.rows()
            .into_iter()
            .map(|r| f(r.as_slice().unwrap()))
            .collect()
    }
    fn data() -> (Array2<f64>, Vec<f64>) {
        let x = LhsSampler::classic(3).unit_sample(256, &mut rng());
        let y = evaluate(&x, model);
        (x, y)
    }
    fn problem() -> Problem {
        (0..3)
            .fold(ProblemBuilder::new(), |b, i| {
                b.factor(&format!("x{i}"), Distribution::Uniform { lo: 0.0, hi: 1.0 })
            })
            .build()
            .unwrap()
    }
    fn groups(indices: &[&[usize]]) -> Vec<Group> {
        indices
            .iter()
            .enumerate()
            .map(|(i, x)| Group {
                name: format!("g{i}"),
                factor_indices: x.to_vec(),
            })
            .collect()
    }
    fn pawn_fields(p: PawnIndices) -> Vec<Vec<f64>> {
        vec![p.minimum, p.mean, p.median, p.maximum, p.cv]
    }
    // Each vector is indexed by factor. Rank fields exclude raw regression fields.
    fn rank_fields(x: &Array2<f64>, y: &[f64]) -> Vec<Vec<f64>> {
        let mut fields = pawn_fields(estimate_pawn(x.view(), y, 8).unwrap());
        fields.push(estimate_given_data_sobol(x.view(), y).unwrap().s1);
        fields.push(estimate_borgonovo_delta(x.view(), y).unwrap().delta);
        fields.push(estimate_qosa(x.view(), y, 0.5).unwrap().s);
        fields.push(estimate_rbd_fast(x.view(), y, 4).unwrap().s);
        let r = estimate_regression_indices(x.view(), y).unwrap();
        fields.extend([r.srrc, r.prcc]);
        fields
    }
    fn permute(m: &SaltelliMatrix, p: &[usize]) -> SaltelliMatrix {
        let mut out = m.clone();
        out.a = m.a.select(Axis(1), p).as_standard_layout().to_owned();
        out.b = m.b.select(Axis(1), p).as_standard_layout().to_owned();
        out.a_b = p
            .iter()
            .map(|&i| m.a_b[i].select(Axis(1), p).as_standard_layout().to_owned())
            .collect();
        out.b_a = m.b_a.as_ref().map(|v| {
            p.iter()
                .map(|&i| v[i].select(Axis(1), p).as_standard_layout().to_owned())
                .collect()
        });
        out
    }
    fn undo(x: &[f64], p: &[usize]) -> Vec<f64> {
        let mut old = vec![0.0; x.len()];
        for (j, &i) in p.iter().enumerate() {
            old[i] = x[j];
        }
        old
    }
    fn grid(t: f64) -> Array2<f64> {
        Array2::from_shape_fn((3, 3), |(i, j)| {
            let u = i as f64 - 1.0;
            let v = j as f64 - 1.0;
            u + 2.0 * v + t * u * v
        })
    }
    fn grid3() -> Array3<f64> {
        Array3::from_shape_fn((3, 3, 3), |(i, j, k)| {
            let (u, v, w) = (i as f64 - 1.0, j as f64 - 1.0, k as f64 - 1.0);
            4.0 * u + 2.0 * v + w + u * v + 0.5 * u * w + 0.25 * v * w + 0.125 * u * v * w
        })
    }
    fn fractions(a: &AnovaThreeWayResult) -> Vec<f64> {
        vec![
            a.v_data,
            a.v_brittleness,
            a.v_inference,
            a.v_data_brittleness,
            a.v_data_inference,
            a.v_brittleness_inference,
            a.v_data_brittleness_inference,
            a.v_residual,
        ]
    }
    fn mean_squares(a: &AnovaThreeWayResult) -> Vec<f64> {
        vec![
            a.ms_data,
            a.ms_brittleness,
            a.ms_inference,
            a.ms_data_brittleness,
            a.ms_data_inference,
            a.ms_brittleness_inference,
            a.ms_data_brittleness_inference,
            a.ms_residual,
        ]
    }
    fn components(g: &GTheoryResult) -> Vec<f64> {
        vec![
            g.sigma_p,
            g.sigma_i,
            g.sigma_r,
            g.sigma_pi,
            g.sigma_pr,
            g.sigma_ir,
            g.sigma_pir,
        ]
    }

    /// Relation 1: all pick-freeze entry points, spectral and given-data estimators.
    #[test]
    fn scaling_preserves_indices() {
        let m = design();
        let o = build_owen_matrix(&LhsSampler::classic(9), 256, &mut rng()).unwrap();
        let fast = build_fast_design(3, 257, 4, &mut rng()).unwrap();
        let (x, y) = data();
        let a = estimate_saltelli2010(&m, model);
        let j = estimate_jansen(&m, model);
        let n = estimate_janon(&m, model);
        let ow = estimate_owen(&o, model);
        let ft = estimate_fast(&fast, model).unwrap();
        let fa = evaluate(&m.a, model);
        let fb = evaluate(&m.b, model);
        let fab: Vec<_> = m.a_b.iter().map(|x| evaluate(x, model)).collect();
        let fba: Vec<_> = m
            .b_a
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| evaluate(x, model))
            .collect();
        for scale in [2.0, -1.0, -4.0] {
            let f = |x: &[f64]| scale * model(x);
            let b = estimate_saltelli2010(&m, f);
            same(&a.first_order, &b.first_order);
            same(&a.total_order, &b.total_order);
            second(&a.second_order, &b.second_order);
            close(b.total_variance, scale * scale * a.total_variance);
            let map = |v: &[f64]| v.iter().map(|y| scale * y).collect::<Vec<_>>();
            let ab: Vec<_> = fab.iter().map(|v| map(v)).collect();
            let ba: Vec<_> = fba.iter().map(|v| map(v)).collect();
            let cached = estimate_saltelli2010_from_outputs(&map(&fa), &map(&fb), &ab);
            same(&a.first_order, &cached.first_order);
            same(&a.total_order, &cached.total_order);
            close(cached.total_variance, b.total_variance);
            let cached2 = estimate_saltelli2010_from_outputs_with_second_order(
                &map(&fa),
                &map(&fb),
                &ab,
                &ba,
            );
            second(&a.second_order, &cached2.second_order);
            let jj = estimate_jansen(&m, f);
            let nn = estimate_janon(&m, f);
            let oo = estimate_owen(&o, f);
            same(&j.first_order, &jj.first_order);
            same(&n.first_order, &nn.first_order);
            same(&ow.first_order, &oo.first_order);
            second(&j.second_order, &jj.second_order);
            second(&n.second_order, &nn.second_order);
            close(jj.total_variance, scale * scale * j.total_variance);
            close(nn.total_variance, scale * scale * n.total_variance);
            close(oo.total_variance, scale * scale * ow.total_variance);
            let ff = estimate_fast(&fast, f).unwrap();
            same(&ft.s, &ff.s);
            same(&ft.st, &ff.st);
            same(
                &estimate_given_data_sobol(x.view(), &y).unwrap().s1,
                &estimate_given_data_sobol(x.view(), &map(&y)).unwrap().s1,
            );
            same(
                &estimate_rbd_fast(x.view(), &y, 4).unwrap().s,
                &estimate_rbd_fast(x.view(), &map(&y), 4).unwrap().s,
            );
        }
    }

    /// Relation 2: deliberately exclude Saltelli S1 and raw S2.
    #[test]
    fn affine_changes_preserve_shift_invariant_fields() {
        let m = design();
        let f = |x: &[f64]| 2.0 * model(x) + 3.0;
        same(
            &estimate_saltelli2010(&m, model).total_order,
            &estimate_saltelli2010(&m, f).total_order,
        );
        same(
            &estimate_jansen(&m, model).first_order,
            &estimate_jansen(&m, f).first_order,
        );
        same(
            &estimate_janon(&m, model).first_order,
            &estimate_janon(&m, f).first_order,
        );
        let o = build_owen_matrix(&LhsSampler::classic(9), 256, &mut rng()).unwrap();
        same(
            &estimate_owen(&o, model).first_order,
            &estimate_owen(&o, f).first_order,
        );
        let fast = build_fast_design(3, 257, 4, &mut rng()).unwrap();
        let a = estimate_fast(&fast, model).unwrap();
        let b = estimate_fast(&fast, f).unwrap();
        same(&a.s, &b.s);
        same(&a.st, &b.st);
        let (x, y) = data();
        let yy: Vec<_> = y.iter().map(|y| 2.0 * y + 3.0).collect();
        same(
            &estimate_given_data_sobol(x.view(), &y).unwrap().s1,
            &estimate_given_data_sobol(x.view(), &yy).unwrap().s1,
        );
        same(
            &estimate_rbd_fast(x.view(), &y, 4).unwrap().s,
            &estimate_rbd_fast(x.view(), &yy, 4).unwrap().s,
        );
    }

    /// Relation 3: all three Saltelli surfaces, including the non-invariant witness.
    #[test]
    fn saltelli_shift_obeys_mean_difference_correction() {
        let a = vec![0.0, 1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 4.0, 5.0];
        let mut m = build_saltelli_matrix(&LhsSampler::classic(2), 4, true, &mut rng()).unwrap();
        m.a = Array2::from_shape_vec((4, 1), a.clone()).unwrap();
        m.b = Array2::from_shape_vec((4, 1), b.clone()).unwrap();
        m.a_b = vec![m.b.clone()];
        m.b_a = Some(vec![m.a.clone()]);
        for (scale, shift) in [(1.0, 10.0), (2.0, 3.0), (-2.0, 3.0)] {
            let aa: Vec<_> = a.iter().map(|v| scale * v + shift).collect();
            let bb: Vec<_> = b.iter().map(|v| scale * v + shift).collect();
            let base =
                estimate_saltelli2010_from_outputs(&a, &b, std::slice::from_ref(&b));
            close(base.first_order[0], 4.2);
            let expected = base.first_order[0] + shift / (scale * base.total_variance) * 1.5;
            for got in [
                estimate_saltelli2010(&m, |x| scale * x[0] + shift),
                estimate_saltelli2010_from_outputs(
                    &aa,
                    &bb,
                    std::slice::from_ref(&bb),
                ),
                estimate_saltelli2010_from_outputs_with_second_order(
                    &aa,
                    &bb,
                    std::slice::from_ref(&bb),
                    std::slice::from_ref(&aa),
                ),
            ] {
                close(got.first_order[0], expected);
            }
        }
    }

    /// Relation 4: KS ordering survives exp; KDE supports both affine orientations.
    #[test]
    fn moment_independent_indices_preserve_output_transformations() {
        let (x, y) = data();
        let exp: Vec<_> = y.iter().map(|y| y.exp()).collect();
        for (a, b) in pawn_fields(estimate_pawn(x.view(), &y, 8).unwrap())
            .iter()
            .zip(pawn_fields(estimate_pawn(x.view(), &exp, 8).unwrap()))
        {
            same(a, &b);
        }
        for scale in [2.0, -2.0] {
            let yy: Vec<_> = y.iter().map(|y| scale * y + 3.0).collect();
            same(
                &estimate_borgonovo_delta(x.view(), &y).unwrap().delta,
                &estimate_borgonovo_delta(x.view(), &yy).unwrap().delta,
            );
        }
    }

    /// Relation 5: fixed tail counts 32/31; shifts must use the pre-clamp formula.
    #[test]
    fn qosa_scaling_and_shift_follow_tail_counts() {
        let y: Vec<_> = (1..=64).map(f64::from).collect();
        let x = Array2::from_shape_vec((64, 1), y.clone()).unwrap();
        let base = estimate_qosa(x.view(), &y, 0.5).unwrap();
        close(base.s[0], 0.708984375);
        let mean = 32.5;
        let rg = 32.0 / 32.0;
        let ri = 31.0 / 32.0;
        let ti = base.global_cte - base.s[0] * (base.global_cte - mean);
        for (a, b) in [(2.0, 0.0), (1.0, 10.0), (2.0, 10.0)] {
            let yy: Vec<_> = y.iter().map(|y| a * y + b).collect();
            let got = estimate_qosa(x.view(), &yy, 0.5).unwrap();
            close(got.global_quantile, a * base.global_quantile + b);
            close(got.global_cte, a * base.global_cte + b * rg);
            close(
                got.s[0],
                ((a * (base.global_cte - ti) + b * (rg - ri))
                    / (a * (base.global_cte - mean) + b * (rg - 1.0)))
                    .clamp(0.0, 1.0),
            );
            if a == 1.0 {
                close(got.s[0], 0.728515625);
            }
        }
    }

    /// Relation 6: signed versus absolute effects, including grouped Morris.
    #[test]
    fn elementary_and_factorial_effects_scale_with_output_units() {
        let f = |x: &[f64]| -2.0 * model(x) + 7.0;
        let t = build_morris_trajectories(3, 32, 4, &mut rng()).unwrap();
        let a = estimate_morris_effects(&t, model).unwrap();
        let b = estimate_morris_effects(&t, f).unwrap();
        scaled(&a.mu, &b.mu, -2.0);
        scaled(&a.mu_star, &b.mu_star, 2.0);
        scaled(&a.sigma, &b.sigma, 2.0);
        let g = groups(&[&[0, 2], &[1]]);
        let t = build_grouped_morris_trajectories(&g, 3, 32, 4, &mut rng()).unwrap();
        let a = estimate_grouped_morris_effects(&t, &g, model).unwrap();
        let b = estimate_grouped_morris_effects(&t, &g, f).unwrap();
        scaled(
            a.grouped_mu.as_ref().unwrap(),
            b.grouped_mu.as_ref().unwrap(),
            -2.0,
        );
        scaled(
            a.grouped_mu_star.as_ref().unwrap(),
            b.grouped_mu_star.as_ref().unwrap(),
            2.0,
        );
        scaled(
            a.grouped_sigma.as_ref().unwrap(),
            b.grouped_sigma.as_ref().unwrap(),
            2.0,
        );
        let pb = build_plackett_burman(3).unwrap();
        let a = estimate_fractional_factorial(&pb, &problem(), model);
        let b = estimate_fractional_factorial(&pb, &problem(), f);
        scaled(&a.main_effects, &b.main_effects, -2.0);
        scaled(&a.main_effects_abs, &b.main_effects_abs, 2.0);
    }

    /// Relation 7: reorder an existing design and its metadata, never resample.
    #[test]
    fn factor_permutation_reorders_indices() {
        let p = [2, 0, 1];
        let m = design();
        let mm = permute(&m, &p);
        let f = |x: &[f64]| model(&undo(x, &p));
        let o = build_owen_matrix(&LhsSampler::classic(9), 256, &mut rng()).unwrap();
        let mut oo = o.clone();
        let columns = |x: &Array2<f64>| x.select(Axis(1), &p).as_standard_layout().to_owned();
        oo.a = columns(&o.a);
        oo.b = columns(&o.b);
        oo.c = columns(&o.c);
        oo.a_c = p.iter().map(|&i| columns(&o.a_c[i])).collect();
        oo.b_a = p.iter().map(|&i| columns(&o.b_a[i])).collect();
        let a = estimate_owen(&o, model);
        let b = estimate_owen(&oo, f);
        same(&p.map(|i| a.first_order[i]), &b.first_order);
        close(a.total_variance, b.total_variance);
        let a = estimate_saltelli2010(&m, model);
        let b = estimate_saltelli2010(&mm, f);
        same(&p.map(|i| a.first_order[i]), &b.first_order);
        same(&p.map(|i| a.total_order[i]), &b.total_order);
        for (a, b) in [
            (
                estimate_jansen(&m, model).first_order,
                estimate_jansen(&mm, f).first_order,
            ),
            (
                estimate_janon(&m, model).first_order,
                estimate_janon(&mm, f).first_order,
            ),
        ] {
            same(&p.map(|i| a[i]), &b);
        }
        let (x, y) = data();
        let xx = x.select(Axis(1), &p).as_standard_layout().to_owned();
        for (a, b) in rank_fields(&x, &y).iter().zip(rank_fields(&xx, &y)) {
            same(&p.map(|i| a[i]), &b);
        }
        let a = estimate_regression_indices(x.view(), &y).unwrap();
        let b = estimate_regression_indices(xx.view(), &y).unwrap();
        same(&p.map(|i| a.src[i]), &b.src);
        same(&p.map(|i| a.pcc[i]), &b.pcc);
        close(a.r2_linear, b.r2_linear);
        close(a.r2_rank, b.r2_rank);
        let t = build_morris_trajectories(3, 32, 4, &mut rng()).unwrap();
        let mut tt = t.clone();
        tt.trajectories = t
            .trajectories
            .select(Axis(2), &p)
            .as_standard_layout()
            .to_owned();
        tt.deltas = t.deltas.select(Axis(1), &p).as_standard_layout().to_owned();
        tt.factor_order = t
            .factor_order
            .mapv(|i| p.iter().position(|&j| j == i).unwrap());
        let a = estimate_morris_effects(&t, model).unwrap();
        let b = estimate_morris_effects(&tt, f).unwrap();
        same(&p.map(|i| a.mu[i]), &b.mu);
        same(&p.map(|i| a.mu_star[i]), &b.mu_star);
        same(&p.map(|i| a.sigma[i]), &b.sigma);
        let t = build_fast_design(3, 257, 4, &mut rng()).unwrap();
        let mut tt = t.clone();
        tt.samples = Array2::from_shape_fn(t.samples.dim(), |(r, j)| {
            t.samples[[
                p[r / t.n_per_factor] * t.n_per_factor + r % t.n_per_factor,
                p[j],
            ]]
        });
        tt.omegas = t
            .omegas
            .select(Axis(0), &p)
            .select(Axis(1), &p)
            .as_standard_layout()
            .to_owned();
        tt.phases = t
            .phases
            .select(Axis(0), &p)
            .select(Axis(1), &p)
            .as_standard_layout()
            .to_owned();
        let a = estimate_fast(&t, model).unwrap();
        let b = estimate_fast(&tt, f).unwrap();
        same(&p.map(|i| a.s[i]), &b.s);
        same(&p.map(|i| a.st[i]), &b.st);
        let pb = build_plackett_burman(3).unwrap();
        let mut pp = pb.clone();
        pp.matrix = pb
            .matrix
            .select(Axis(1), &p)
            .as_standard_layout()
            .to_owned();
        let a = estimate_fractional_factorial(&pb, &problem(), model);
        let b = estimate_fractional_factorial(&pp, &problem(), f);
        same(&p.map(|i| a.main_effects[i]), &b.main_effects);
        same(&p.map(|i| a.main_effects_abs[i]), &b.main_effects_abs);
    }

    /// Relation 8: the exact cross-term difference, rather than false exact symmetry.
    #[test]
    fn second_order_permutation_obeys_cross_product_correction() {
        let m = build_saltelli_matrix(&LhsSampler::classic(4), 256, true, &mut rng()).unwrap();
        let p = [1, 0];
        let mm = permute(&m, &p);
        let f = |x: &[f64]| x[0] + 2.0 * x[1] + x[0] * x[1];
        let ff = |x: &[f64]| f(&undo(x, &p));
        let ab: Vec<_> = m.a_b.iter().map(|x| evaluate(x, f)).collect();
        let ba: Vec<_> = m
            .b_a
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| evaluate(x, f))
            .collect();
        let delta = (0..m.n)
            .map(|k| ba[0][k] * ab[1][k] - ba[1][k] * ab[0][k])
            .sum::<f64>()
            / m.n as f64;
        let fa = evaluate(&m.a, f);
        let fb = evaluate(&m.b, f);
        let a = estimate_saltelli2010_from_outputs_with_second_order(&fa, &fb, &ab, &ba);
        let b = estimate_saltelli2010_from_outputs_with_second_order(
            &fa,
            &fb,
            &[ab[1].clone(), ab[0].clone()],
            &[ba[1].clone(), ba[0].clone()],
        );
        close(
            b.second_order.unwrap()[0][0] - a.second_order.unwrap()[0][0],
            delta / a.total_variance,
        );
        let a = estimate_saltelli2010(&m, f);
        let b = estimate_saltelli2010(&mm, ff);
        close(
            b.second_order.unwrap()[0][0] - a.second_order.unwrap()[0][0],
            delta / a.total_variance,
        );
        let a = estimate_jansen(&m, f);
        let b = estimate_jansen(&mm, ff);
        close(
            b.second_order.unwrap()[0][0] - a.second_order.unwrap()[0][0],
            delta / a.total_variance,
        );
        let a = estimate_janon(&m, f);
        let b = estimate_janon(&mm, ff);
        close(
            b.second_order.unwrap()[0][0] - a.second_order.unwrap()[0][0],
            delta / a.total_variance,
        );
    }

    /// Relation 9: distinct LHS inputs avoid ordinal tie-breaking changes.
    #[test]
    fn joint_row_permutations_preserve_paired_estimates() {
        let m = design();
        let (x, y) = data();
        let mut shuffled: Vec<_> = (0..m.n).collect();
        shuffled.shuffle(&mut rng().into_chacha());
        let o = build_owen_matrix(&LhsSampler::classic(9), 256, &mut rng()).unwrap();
        for rows in [(0..m.n).rev().collect::<Vec<_>>(), shuffled] {
            let mut oo = o.clone();
            oo.a = o.a.select(Axis(0), &rows);
            oo.b = o.b.select(Axis(0), &rows);
            oo.c = o.c.select(Axis(0), &rows);
            oo.a_c = o.a_c.iter().map(|x| x.select(Axis(0), &rows)).collect();
            oo.b_a = o.b_a.iter().map(|x| x.select(Axis(0), &rows)).collect();
            let a = estimate_owen(&o, model);
            let b = estimate_owen(&oo, model);
            same(&a.first_order, &b.first_order);
            close(a.total_variance, b.total_variance);
            let mut mm = m.clone();
            mm.a = m.a.select(Axis(0), &rows);
            mm.b = m.b.select(Axis(0), &rows);
            mm.a_b = m.a_b.iter().map(|x| x.select(Axis(0), &rows)).collect();
            mm.b_a = m
                .b_a
                .as_ref()
                .map(|v| v.iter().map(|x| x.select(Axis(0), &rows)).collect());
            let a = estimate_saltelli2010(&m, model);
            let b = estimate_saltelli2010(&mm, model);
            same(&a.first_order, &b.first_order);
            same(&a.total_order, &b.total_order);
            second(&a.second_order, &b.second_order);
            let a = estimate_jansen(&m, model);
            let b = estimate_jansen(&mm, model);
            same(&a.first_order, &b.first_order);
            second(&a.second_order, &b.second_order);
            let a = estimate_janon(&m, model);
            let b = estimate_janon(&mm, model);
            same(&a.first_order, &b.first_order);
            second(&a.second_order, &b.second_order);
            let xx = x.select(Axis(0), &rows);
            let yy: Vec<_> = rows.iter().map(|&i| y[i]).collect();
            for (a, b) in rank_fields(&x, &y).iter().zip(rank_fields(&xx, &yy)) {
                same(a, &b);
            }
            let a = estimate_regression_indices(x.view(), &y).unwrap();
            let b = estimate_regression_indices(xx.view(), &yy).unwrap();
            same(&a.src, &b.src);
            same(&a.pcc, &b.pcc);
            close(a.r2_linear, b.r2_linear);
            close(a.r2_rank, b.r2_rank);
            let a = estimate_dgsm(x.view(), &[1.0, 2.0, 3.0], 2.0).unwrap();
            let b = estimate_dgsm(xx.view(), &[1.0, 2.0, 3.0], 2.0).unwrap();
            same(&a.vi, &b.vi);
            same(&a.st_upper, &b.st_upper);
        }
    }

    /// Relation 10: exact balanced-grid counterpart of the population decomposition.
    #[test]
    fn additive_decomposition_sums_to_one() {
        let a = estimate_anova_two_way(grid(0.0).view()).unwrap();
        close(a.v_row + a.v_column, 1.0);
        close(a.v_interaction, 0.0);
        close(a.v_residual, 0.0);
        let y = Array3::from_shape_fn((3, 3, 3), |(i, j, k)| {
            i as f64 + 2.0 * j as f64 + 3.0 * k as f64
        });
        let a = estimate_anova_three_way(y.view()).unwrap();
        same(
            &fractions(&a),
            &[1.0 / 14.0, 4.0 / 14.0, 9.0 / 14.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        );
    }

    /// Relation 11: exact coefficient oracle on a balanced grid.
    #[test]
    fn additive_coefficient_changes_follow_grid_variances() {
        for t in [0.0, 1.0, 2.0, 4.0, -4.0] {
            let y = Array2::from_shape_fn((3, 3), |(i, j)| {
                t * (i as f64 - 1.0) + 2.0 * (j as f64 - 1.0)
            });
            let a = estimate_anova_two_way(y.view()).unwrap();
            close(a.v_row, t * t / (t * t + 4.0));
            close(a.v_column, 4.0 / (t * t + 4.0));
            close(a.v_interaction, 0.0);
        }
    }

    /// Relation 12: centered-grid interaction variance is t² Var(U) Var(V).
    #[test]
    fn pure_interaction_has_exact_grid_decomposition() {
        for t in [0.0, 1.0, 2.0] {
            let a = estimate_anova_two_way(grid(t).view()).unwrap();
            let variance = 2.0 / 3.0;
            let d = 5.0 * variance + t * t * variance * variance;
            close(a.v_row, variance / d);
            close(a.v_column, 4.0 * variance / d);
            close(a.v_interaction, t * t * variance * variance / d);
        }
    }

    /// Relation 13: only estimators with finite-sample dummy zeros are asserted.
    #[test]
    fn inactive_factors_have_exact_zeros_and_active_factor_ones() {
        let m = design();
        let f = |x: &[f64]| x[0] * x[0];
        let a = estimate_saltelli2010(&m, f);
        same(&a.first_order[1..], &[0.0, 0.0]);
        same(&a.total_order[1..], &[0.0, 0.0]);
        close(estimate_jansen(&m, f).first_order[0], 1.0);
        close(estimate_janon(&m, f).first_order[0], 1.0);
        let mut mm = m.clone();
        for x in std::iter::once(&mut mm.a)
            .chain(std::iter::once(&mut mm.b))
            .chain(mm.a_b.iter_mut())
            .chain(mm.b_a.as_mut().unwrap().iter_mut())
        {
            for mut row in x.rows_mut() {
                row[1] = 99.0;
                row[2] = -20.0;
            }
        }
        let b = estimate_saltelli2010(&mm, f);
        same(&a.first_order, &b.first_order);
        same(&a.total_order, &b.total_order);
        let o = build_owen_matrix(&LhsSampler::classic(9), 256, &mut rng()).unwrap();
        same(&estimate_owen(&o, f).first_order[1..], &[0.0, 0.0]);
        let t = build_morris_trajectories(3, 32, 4, &mut rng()).unwrap();
        let e = estimate_morris_effects(&t, f).unwrap();
        same(&e.mu[1..], &[0.0, 0.0]);
        same(&e.mu_star[1..], &[0.0, 0.0]);
        same(&e.sigma[1..], &[0.0, 0.0]);
    }

    /// Relation 14: grouping sums differences exactly; ST is deliberately excluded.
    #[test]
    fn grouping_additive_factors_sums_saltelli_first_order() {
        let sampler = LhsSampler::classic(6);
        let m = design();
        let g = groups(&[&[0, 2], &[1]]);
        let grouped = build_grouped_saltelli_matrix(&sampler, &g, 256, true, &mut rng()).unwrap();
        assert_eq!(m.a, grouped.a);
        assert_eq!(m.b, grouped.b);
        let f = |x: &[f64]| x[0] + 2.0 * x[1] + 3.0 * x[2];
        let a = estimate_saltelli2010(&m, f);
        let b = estimate_saltelli2010(&grouped, f);
        close(b.first_order[0], a.first_order[0] + a.first_order[2]);
        close(b.first_order[1], a.first_order[1]);
        let singleton = build_grouped_saltelli_matrix(
            &sampler,
            &groups(&[&[0], &[1], &[2]]),
            256,
            true,
            &mut rng(),
        )
        .unwrap();
        assert_eq!(m.a_b, singleton.a_b);
        assert_eq!(m.b_a, singleton.b_a);
    }

    /// Relation 15: exact decomposition counterpart for nested factor sets.
    #[test]
    fn nested_groups_capture_more_grid_variance() {
        let a = estimate_anova_three_way(grid3().view()).unwrap();
        let s_g = a.v_data;
        let s_h = a.v_data + a.v_brittleness + a.v_data_brittleness;
        let st_g =
            a.v_data + a.v_data_brittleness + a.v_data_inference + a.v_data_brittleness_inference;
        let st_h = 1.0 - a.v_inference;
        assert!(s_g.is_finite() && s_h.is_finite() && st_g.is_finite() && st_h.is_finite());
        assert!(s_g < s_h);
        assert!(st_g < st_h);
    }

    /// Relation 16, and HDMR portions of 1/2/7/9/10 (requires its public feature).
    #[cfg(feature = "surrogate")]
    #[test]
    fn hdmr_conserves_fitted_variance_and_reporting_order() {
        let (x, _) = data();
        let y = evaluate(&x, |x| x[0] + 2.0 * x[1] + x[0] * x[2] + x[0] * x[1] * x[2]);
        let problem = problem();
        let a = estimate_hdmr(x.view(), &y, &problem, 1, 3).unwrap();
        let b = estimate_hdmr(x.view(), &y, &problem, 3, 3).unwrap();
        same(&a.first_order, &b.first_order);
        same(&a.total_order, &b.total_order);
        close(a.total_variance, b.total_variance);
        for (a, b) in a.second_order.iter().zip(&b.second_order) {
            same(a, b);
        }
        same(&a.pce.coefficients, &b.pce.coefficients);
        assert_eq!(a.pce.multi_indices, b.pce.multi_indices);
        assert_eq!(a.order_variance.len(), 1);
        assert_eq!(b.order_variance.len(), 3);
        close(a.order_variance[0], b.order_variance[0]);
        close(b.first_order.iter().sum(), b.order_variance[0]);
        close(b.order_variance.iter().sum(), 1.0);
        close(
            b.total_order.iter().sum(),
            b.order_variance
                .iter()
                .enumerate()
                .map(|(i, v)| (i + 1) as f64 * v)
                .sum(),
        );
        close(b.second_order.iter().flatten().sum(), b.order_variance[1]);
        for (&s, &st) in b.first_order.iter().zip(&b.total_order) {
            bounded(s, 0.0, st);
            bounded(st, 0.0, 1.0);
        }
        for (scale, shift) in [(2.0, 0.0), (-1.0, 0.0), (-4.0, 0.0), (2.0, 3.0)] {
            let yy: Vec<_> = y.iter().map(|v| scale * v + shift).collect();
            let c = estimate_hdmr(x.view(), &yy, &problem, 3, 3).unwrap();
            same(&b.first_order, &c.first_order);
            same(&b.total_order, &c.total_order);
            close(c.total_variance, scale * scale * b.total_variance);
            for (a, c) in b.second_order.iter().zip(&c.second_order) {
                same(a, c);
            }
        }
        let p = [2, 0, 1];
        let xx = x.select(Axis(1), &p).as_standard_layout().to_owned();
        let permuted_problem = p
            .iter()
            .fold(ProblemBuilder::new(), |builder, &i| {
                let factor = &problem.factors()[i];
                builder.factor(&factor.name, factor.distribution.clone())
            })
            .build()
            .unwrap();
        let c = estimate_hdmr(xx.view(), &y, &permuted_problem, 3, 3).unwrap();
        same(&p.map(|i| b.first_order[i]), &c.first_order);
        same(&p.map(|i| b.total_order[i]), &c.total_order);
        // Relation 21: HDMR canonical mapping commutes with changes of units.
        let scales = [2.0, 0.5, 3.0];
        let shifts = [3.0, -1.0, 4.0];
        let units_problem = (0..3)
            .fold(ProblemBuilder::new(), |builder, i| {
                builder.factor(
                    &format!("x{i}"),
                    Distribution::Uniform {
                        lo: shifts[i],
                        hi: shifts[i] + scales[i],
                    },
                )
            })
            .build()
            .unwrap();
        let xx = Array2::from_shape_fn(x.dim(), |(r, j)| scales[j] * x[[r, j]] + shifts[j]);
        let c = estimate_hdmr(xx.view(), &y, &units_problem, 3, 3).unwrap();
        same(&b.first_order, &c.first_order);
        same(&b.total_order, &c.total_order);
        close(b.total_variance, c.total_variance);
        let rows: Vec<_> = (0..x.nrows()).rev().collect();
        let xx = x.select(Axis(0), &rows);
        let yy: Vec<_> = rows.iter().map(|&i| y[i]).collect();
        let c = estimate_hdmr(xx.view(), &yy, &problem, 3, 3).unwrap();
        same(&b.first_order, &c.first_order);
        same(&b.total_order, &c.total_order);
        let yy = evaluate(&x, |x| x[0] + 2.0 * x[1] + 3.0 * x[2]);
        let c = estimate_hdmr(x.view(), &yy, &problem, 3, 3).unwrap();
        same(&c.first_order, &[1.0 / 14.0, 4.0 / 14.0, 9.0 / 14.0]);
        same(&c.first_order, &c.total_order);
        close(c.order_variance[0], 1.0);
    }
    fn bounded(v: f64, lo: f64, hi: f64) {
        assert!(v.is_finite());
        assert!(v >= lo - TOL && v <= hi + TOL, "{v} outside [{lo}, {hi}]");
    }

    /// Relation 17: estimator-specific bounds, plus a Saltelli out-of-range witness.
    #[test]
    fn finite_sample_bounds_are_estimator_specific() {
        let m = design();
        let (x, y) = data();
        let fast = build_fast_design(3, 257, 4, &mut rng()).unwrap();
        for (a, b) in [(1.0, 0.0), (-2.0, 3.0)] {
            let f = |x: &[f64]| a * model(x) + b;
            let yy: Vec<_> = y.iter().map(|y| a * y + b).collect();
            for v in estimate_saltelli2010(&m, f).total_order {
                assert!(v.is_finite() && v >= 0.0);
            }
            for v in estimate_jansen(&m, f).first_order {
                assert!(v.is_finite() && v <= 1.0 + TOL);
            }
            for v in estimate_janon(&m, f).first_order {
                bounded(v, -1.0, 1.0);
            }
            let ft = estimate_fast(&fast, f).unwrap();
            for (&s, &st) in ft.s.iter().zip(&ft.st) {
                bounded(s, 0.0, st);
                bounded(st, 0.0, 1.0);
            }
            let lambda = 8.0 / x.nrows() as f64;
            for v in estimate_rbd_fast(x.view(), &yy, 4).unwrap().s {
                bounded(v, -lambda / (1.0 - lambda), 1.0);
            }
            for v in estimate_given_data_sobol(x.view(), &yy)
                .unwrap()
                .s1
                .into_iter()
                .chain(estimate_qosa(x.view(), &yy, 0.5).unwrap().s)
            {
                bounded(v, 0.0, 1.0);
            }
            let pawn = estimate_pawn(x.view(), &yy, 8).unwrap();
            for v in pawn
                .minimum
                .into_iter()
                .chain(pawn.mean)
                .chain(pawn.median)
                .chain(pawn.maximum)
            {
                bounded(v, 0.0, 1.0);
            }
        }
        let witness = estimate_saltelli2010_from_outputs(
            &[0.0, 1.0, 2.0, 3.0],
            &[1.0, 2.0, 4.0, 5.0],
            &[vec![1.0, 2.0, 4.0, 5.0]],
        );
        close(witness.first_order[0], 4.2);
        assert!(witness.first_order[0] > 1.0);
    }

    /// Relation 18: independently computed denominators and squared differences.
    #[test]
    fn janon_and_jansen_share_squared_difference_numerator() {
        let m = design();
        let y = evaluate(&m.b, model);
        let n = m.n as f64;
        let mean = y.iter().sum::<f64>() / n;
        let var = y.iter().map(|v| v * v).sum::<f64>() / n - mean * mean;
        let a = estimate_janon(&m, model);
        let b = estimate_jansen(&m, model);
        for i in 0..m.dim {
            let z = evaluate(&m.a_b[i], model);
            let joint_mean = (mean + z.iter().sum::<f64>() / n) / 2.0;
            let d = y
                .iter()
                .zip(&z)
                .map(|(y, z)| (y * y + z * z) / 2.0)
                .sum::<f64>()
                / n
                - joint_mean * joint_mean;
            let q = y
                .iter()
                .zip(&z)
                .map(|(y, z)| (y - z).powi(2) / 2.0)
                .sum::<f64>()
                / n;
            close((1.0 - a.first_order[i]) * d, q);
            close((1.0 - b.first_order[i]) * var, q);
        }
    }

    /// Relation 19: supplied gradients, with no finite-difference approximation.
    #[test]
    fn dgsm_scaling_constants_variance_and_gradients_obey_oracles() {
        let (g, _) = data();
        let c = [1.0, 2.0, 3.0];
        let a = estimate_dgsm(g.view(), &c, 2.0).unwrap();
        let gg = g.mapv(|v| 2.0 * v);
        let b = estimate_dgsm(gg.view(), &c, 8.0).unwrap();
        scaled(&a.vi, &b.vi, 4.0);
        same(&a.st_upper, &b.st_upper);
        let b = estimate_dgsm(g.view(), &[2.0, 4.0, 6.0], 2.0).unwrap();
        scaled(&a.st_upper, &b.st_upper, 2.0);
        let b = estimate_dgsm(g.view(), &c, 4.0).unwrap();
        scaled(&a.st_upper, &b.st_upper, 0.5);
        let gg = g.mapv(|v| v + 1.0);
        let b = estimate_dgsm(gg.view(), &c, 2.0).unwrap();
        for (a, b) in a.st_upper.iter().zip(b.st_upper) {
            assert!(b.is_finite() && b > *a);
        }
    }

    /// Relation 20: nonlinear input transforms preserve rank fields only.
    #[test]
    fn increasing_input_transforms_preserve_rank_based_indices() {
        let (x, y) = data();
        let xx = x.mapv(f64::exp);
        for (a, b) in rank_fields(&x, &y).iter().zip(rank_fields(&xx, &y)) {
            same(a, &b);
        }
        let a = estimate_regression_indices(x.view(), &y).unwrap();
        let yy: Vec<_> = y.iter().map(|v| v.exp()).collect();
        let b = estimate_regression_indices(xx.view(), &yy).unwrap();
        same(&a.srrc, &b.srrc);
        same(&a.prcc, &b.prcc);
        close(a.r2_rank, b.r2_rank);
    }

    /// Relation 25: second-order sampling consumes no additional random draws.
    #[test]
    fn second_order_sampling_preserves_existing_evaluations() {
        for grouped in [false, true] {
            let sampler = LhsSampler::classic(6);
            let mut ra = rng();
            let mut rb = ra.clone();
            let g = groups(&[&[0, 2], &[1]]);
            let build = |second, r: &mut RngState| {
                if grouped {
                    build_grouped_saltelli_matrix(&sampler, &g, 256, second, r)
                } else {
                    build_saltelli_matrix(&sampler, 256, second, r)
                }
            };
            let a = build(false, &mut ra).unwrap();
            let b = build(true, &mut rb).unwrap();
            assert_eq!(a.a, b.a);
            assert_eq!(a.b, b.b);
            assert_eq!(a.a_b, b.a_b);
            assert_eq!(ra, rb);
            assert!(a.b_a.is_none() && b.b_a.is_some());
            let aa = estimate_saltelli2010(&a, model);
            let bb = estimate_saltelli2010(&b, model);
            assert_eq!(aa.first_order, bb.first_order);
            assert_eq!(aa.total_order, bb.total_order);
            assert_eq!(aa.total_variance, bb.total_variance);
            assert_eq!(
                estimate_janon(&a, model).first_order,
                estimate_janon(&b, model).first_order
            );
            assert_eq!(
                estimate_jansen(&a, model).first_order,
                estimate_jansen(&b, model).first_order
            );
        }
    }

    /// Relation 26: coupled bootstrap samples preserve transformations and CI nesting.
    #[test]
    fn bootstrap_scaling_preserves_intervals_and_confidence_nests_them() {
        let m = design();
        let fa = evaluate(&m.a, model);
        let fb = evaluate(&m.b, model);
        let fab: Vec<_> = m.a_b.iter().map(|x| evaluate(x, model)).collect();
        let mut previous: Option<SobolIndicesWithCi> = None;
        for alpha in [0.05, 0.10, 0.20] {
            let a = estimate_saltelli2010_with_bootstrap(&m, model, 96, alpha, &mut rng());
            let b = estimate_saltelli2010_with_bootstrap(
                &m,
                |x| -2.0 * model(x),
                96,
                alpha,
                &mut rng(),
            );
            let c = estimate_saltelli2010_from_outputs_with_bootstrap(
                &fa,
                &fb,
                &fab,
                96,
                alpha,
                &mut rng(),
            );
            let a = a.unwrap();
            let b = b.unwrap();
            let c = c.unwrap();
            assert_eq!(a.bootstrap_resamples, b.bootstrap_resamples);
            for ((a, b), c) in a
                .first_order_ci
                .iter()
                .chain(&a.total_order_ci)
                .zip(b.first_order_ci.iter().chain(&b.total_order_ci))
                .zip(c.first_order_ci.iter().chain(&c.total_order_ci))
            {
                close(a.0, b.0);
                close(a.1, b.1);
                close(a.0, c.0);
                close(a.1, c.1);
            }
            if let Some(wide) = previous {
                for (w, n) in wide
                    .first_order_ci
                    .iter()
                    .chain(&wide.total_order_ci)
                    .zip(a.first_order_ci.iter().chain(&a.total_order_ci))
                {
                    assert!(n.0.is_finite() && n.1.is_finite());
                    assert!(w.0 <= n.0 + TOL && w.1 >= n.1 - TOL);
                }
            }
            previous = Some(a);
        }
        let (x, y) = data();
        let yy: Vec<_> = y.iter().map(|v| -2.0 * v + 3.0).collect();
        let mut previous: Option<BootstrapCi> = None;
        for alpha in [0.05, 0.10, 0.20] {
            let estimator = |x: ndarray::ArrayView2<'_, f64>, y: &[f64]| {
                estimate_given_data_sobol(x, y)
                    .map(|v| v.s1)
                    .map_err(|e| Box::new(e) as BoxedEstimatorError)
            };
            let a = bootstrap_given_data(x.view(), &y, 64, alpha, &mut rng(), estimator).unwrap();
            let b = bootstrap_given_data(x.view(), &yy, 64, alpha, &mut rng(), estimator).unwrap();
            same(&a.ci_low, &b.ci_low);
            same(&a.ci_high, &b.ci_high);
            assert_eq!(a.n_skipped, b.n_skipped);
            if let Some(w) = previous {
                for i in 0..a.ci_low.len() {
                    assert!(w.ci_low[i] <= a.ci_low[i] + TOL && w.ci_high[i] >= a.ci_high[i] - TOL);
                }
            }
            previous = Some(a);
        }
    }

    /// Relation 27: affine and level permutations; reliability needs positive components.
    #[test]
    fn anova_and_g_theory_preserve_affine_and_level_transformations() {
        let y = grid3();
        let a = estimate_anova_three_way(y.view()).unwrap();
        let g = estimate_g_theory_pir(y.view(), GTheoryDesign::Crossed).unwrap();
        let yy = y.mapv(|v| -2.0 * v + 3.0);
        let b = estimate_anova_three_way(yy.view()).unwrap();
        let gg = estimate_g_theory_pir(yy.view(), GTheoryDesign::Crossed).unwrap();
        same(&fractions(&a), &fractions(&b));
        scaled(&mean_squares(&a), &mean_squares(&b), 4.0);
        scaled(&components(&g), &components(&gg), 4.0);
        close(g.g_coefficient, gg.g_coefficient);
        close(g.phi_coefficient, gg.phi_coefficient);
        let yy = y
            .select(Axis(0), &[2, 0, 1])
            .select(Axis(1), &[1, 2, 0])
            .select(Axis(2), &[2, 1, 0]);
        let b = estimate_anova_three_way(yy.view()).unwrap();
        let gg = estimate_g_theory_pir(yy.view(), GTheoryDesign::Crossed).unwrap();
        same(&fractions(&a), &fractions(&b));
        same(&mean_squares(&a), &mean_squares(&b));
        same(&components(&g), &components(&gg));
        close(g.g_coefficient, gg.g_coefficient);
        close(g.phi_coefficient, gg.phi_coefficient);
        let yy = y.clone().permuted_axes([0, 2, 1]);
        let gg = estimate_g_theory_pir(yy.view(), GTheoryDesign::Crossed).unwrap();
        same(
            &components(&gg),
            &[
                g.sigma_p,
                g.sigma_r,
                g.sigma_i,
                g.sigma_pr,
                g.sigma_pi,
                g.sigma_ir,
                g.sigma_pir,
            ],
        );
        let p = project_g_theory_d_study(&g, 4, 6).unwrap();
        let q = project_g_theory_d_study(&gg, 6, 4).unwrap();
        close(p.g_coefficient, q.g_coefficient);
        close(p.phi_coefficient, q.phi_coefficient);
        let positive = GTheoryResult::from_components(10.0, 2.0, 1.0, 3.0, 2.0, 0.5, 1.0, 0.0, 0.0);
        let mut prev = None;
        for (ni, nr) in [(2, 2), (4, 2), (4, 4), (8, 8)] {
            let p = project_g_theory_d_study(&positive, ni, nr).unwrap();
            bounded(p.phi_coefficient, 0.0, p.g_coefficient);
            bounded(p.g_coefficient, 0.0, 1.0);
            if let Some((g, phi)) = prev {
                assert!(p.g_coefficient >= g && p.phi_coefficient >= phi);
            }
            prev = Some((p.g_coefficient, p.phi_coefficient));
        }
        let y = grid(1.0);
        let a = estimate_anova_two_way(y.view()).unwrap();
        for yy in [
            y.mapv(|v| -2.0 * v + 3.0),
            y.select(Axis(0), &[2, 0, 1]).select(Axis(1), &[1, 2, 0]),
        ] {
            let b = estimate_anova_two_way(yy.view()).unwrap();
            same(
                &[a.v_row, a.v_column, a.v_interaction, a.v_residual],
                &[b.v_row, b.v_column, b.v_interaction, b.v_residual],
            );
        }
    }

    /// Relation 28: anchored L2-star is intentionally excluded from reflections.
    #[test]
    fn discrepancy_preserves_permutations_replication_reflection_and_torus_shifts() {
        let x = Array2::from_shape_vec(
            (5, 3),
            vec![
                0.1, 0.2, 0.7, 0.3, 0.8, 0.4, 0.6, 0.1, 0.9, 0.8, 0.5, 0.2, 0.4, 0.7, 0.6,
            ],
        )
        .unwrap();
        let a = compute_discrepancy(x.view()).unwrap();
        for xx in [
            x.select(Axis(0), &[4, 2, 0, 3, 1])
                .select(Axis(1), &[2, 0, 1]),
            x.select(Axis(0), &[0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4]),
        ] {
            let b = compute_discrepancy(xx.view()).unwrap();
            same(
                &[a.centered, a.wrap_around, a.modified, a.l2_star],
                &[b.centered, b.wrap_around, b.modified, b.l2_star],
            );
        }
        for j in 0..3 {
            let mut xx = x.clone();
            xx.column_mut(j).mapv_inplace(|v| 1.0 - v);
            let b = compute_discrepancy(xx.view()).unwrap();
            same(
                &[a.centered, a.wrap_around, a.modified],
                &[b.centered, b.wrap_around, b.modified],
            );
        }
        let xx = Array2::from_shape_fn(x.dim(), |(i, j)| (x[[i, j]] + [0.23, 0.41, 0.17][j]) % 1.0);
        close(
            a.wrap_around,
            compute_discrepancy(xx.view()).unwrap().wrap_around,
        );
    }
}
