//! salib-validation's closed forms against exact Sobol' decompositions of each
//! function's definition (verify/sage/validation_analytic.sage).
#![allow(clippy::unwrap_used)]

// Generated source is included, not declared as a module, so rustfmt leaves
// it byte-identical to what the Sage script writes.
mod oracle {
    include!("oracles/analytic_indices.rs");
}

use salib_validation::{ishigami, sobol_g};

/// A few-flop closed form should land within a handful of ulps of the exact value.
fn close(name: &str, got: f64, exact: f64) {
    close_within(name, got, exact, 8.0);
}

/// Within `bound` ulps of the exact value; an exact zero must be exactly zero.
fn close_within(name: &str, got: f64, exact: f64, bound: f64) {
    if exact == 0.0 {
        assert!(got == 0.0, "{name}: got {got:e}, exact 0");
        return;
    }
    let ulps = (got - exact).abs() / (exact.abs() * f64::EPSILON);
    assert!(
        ulps <= bound,
        "{name}: got {got:e}, exact {exact:e} ({ulps:.1} ulp)"
    );
}

#[test]
fn ishigami_matches_exact_decomposition() {
    for case in oracle::ISHIGAMI_CASES {
        let s = ishigami::analytic_indices(case.a, case.b);
        let tag = format!("a={} b={}", case.a, case.b);
        close(&format!("{tag} D"), s.total_variance, case.total_variance);
        for i in 0..3 {
            close(
                &format!("{tag} S{i}"),
                s.first_order[i],
                case.first_order[i],
            );
            close(
                &format!("{tag} ST{i}"),
                s.total_order[i],
                case.total_order[i],
            );
        }
        let s2 = s.second_order.unwrap();
        for (k, got) in [s2[0][0], s2[0][1], s2[1][0]].into_iter().enumerate() {
            close(&format!("{tag} S2[{k}]"), got, case.second_order[k]);
        }
    }
}

#[test]
fn sobol_g_matches_exact_decomposition() {
    for case in oracle::SOBOL_G_CASES {
        let s = sobol_g::analytic_indices(case.a);
        let tag = format!("a={:?}", case.a);
        assert_eq!(s.first_order.len(), case.a.len(), "{tag}");
        assert_eq!(s.total_order.len(), case.a.len(), "{tag}");
        // D and each S_Ti come from about three rounded operations per factor,
        // each with relative error at most ε/2: about 3d ulps, plus slack.
        let bound = 8.0 + 3.0 * case.a.len() as f64;
        close_within(
            &format!("{tag} D"),
            s.total_variance,
            case.total_variance,
            bound,
        );
        for i in 0..case.a.len() {
            close_within(
                &format!("{tag} S{i}"),
                s.first_order[i],
                case.first_order[i],
                bound,
            );
            close_within(
                &format!("{tag} ST{i}"),
                s.total_order[i],
                case.total_order[i],
                bound,
            );
        }
    }
}

/// Ishigami value and gradient against Sage's symbolic derivation at fixed
/// points. Each output is a few operations on libm sin/cos (each within an
/// ulp or so), so it lands within 16ε times the sum of its absolute terms.
#[test]
fn ishigami_value_and_gradient_match_symbolic_derivation() {
    for p in oracle::ISHIGAMI_POINTS {
        let tag = format!("a={} b={} x={:?}", p.a, p.b, p.x);
        let within = |name: &str, got: f64, exact: f64, scale: f64| {
            assert!(
                (got - exact).abs() <= 16.0 * f64::EPSILON * scale,
                "{tag} {name}: got {got:e}, exact {exact:e}"
            );
        };
        within(
            "f",
            ishigami::ishigami_with_params(&p.x, p.a, p.b),
            p.value,
            p.value_scale,
        );
        let g = ishigami::ishigami_gradient_with_params(&p.x, p.a, p.b);
        for (k, (got, (exact, scale))) in g
            .iter()
            .zip(p.gradient.iter().zip(&p.gradient_scale))
            .enumerate()
        {
            within(&format!("df/dx{}", k + 1), *got, *exact, *scale);
        }
        if (p.a, p.b) == (ishigami::ISHIGAMI_DEFAULT_A, ishigami::ISHIGAMI_DEFAULT_B) {
            within(
                "f (default)",
                ishigami::ishigami(&p.x),
                p.value,
                p.value_scale,
            );
            let g = ishigami::ishigami_gradient(&p.x);
            for (k, (got, (exact, scale))) in g
                .iter()
                .zip(p.gradient.iter().zip(&p.gradient_scale))
                .enumerate()
            {
                within(&format!("df/dx{} (default)", k + 1), *got, *exact, *scale);
            }
        }
    }
}
