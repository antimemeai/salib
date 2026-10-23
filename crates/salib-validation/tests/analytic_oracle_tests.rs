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
    if exact == 0.0 {
        assert!(got == 0.0, "{name}: got {got:e}, exact 0");
        return;
    }
    let ulps = (got - exact).abs() / (exact.abs() * f64::EPSILON);
    assert!(
        ulps <= 8.0,
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
        close(&format!("{tag} D"), s.total_variance, case.total_variance);
        for i in 0..case.a.len() {
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
    }
}
