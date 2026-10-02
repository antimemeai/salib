//! Problem content addressing invariants.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use salib_core::{Distribution, FactorKind, Problem, ProblemBuilder};

fn uniform_problem(name: &str, hi: f64) -> Problem {
    ProblemBuilder::new()
        .factor(name, Distribution::Uniform { lo: 0.0, hi })
        .build()
        .expect("problem")
}

#[test]
fn repeated_hashes_are_identical() {
    let p = uniform_problem("x", 1.0);
    assert_eq!(p.content_hash(), p.content_hash());
}
#[test]
fn independently_built_equal_problems_hash_identically() {
    let make = || {
        ProblemBuilder::new()
            .factor("x", Distribution::Uniform { lo: 0.0, hi: 1.0 })
            .factor(
                "y",
                Distribution::Normal {
                    mu: 0.0,
                    sigma: 2.0,
                },
            )
            .build()
            .expect("problem")
    };
    assert_eq!(make().content_hash(), make().content_hash());
}
#[test]
fn distribution_parameters_change_hash() {
    assert_ne!(
        uniform_problem("x", 1.0).content_hash(),
        uniform_problem("x", 2.0).content_hash()
    );
}
#[test]
fn factor_names_change_hash() {
    assert_ne!(
        uniform_problem("x", 1.0).content_hash(),
        uniform_problem("y", 1.0).content_hash()
    );
}
#[test]
fn factor_order_changes_hash() {
    let a = Distribution::Uniform { lo: 0.0, hi: 1.0 };
    let b = Distribution::Uniform { lo: 2.0, hi: 3.0 };
    let ab = ProblemBuilder::new()
        .factor("a", a.clone())
        .factor("b", b.clone())
        .build()
        .unwrap();
    let ba = ProblemBuilder::new()
        .factor("b", b)
        .factor("a", a)
        .build()
        .unwrap();
    assert_ne!(ab.content_hash(), ba.content_hash());
}
#[test]
fn factor_kind_changes_hash() {
    let make = |kind| {
        ProblemBuilder::new()
            .factor_with_kind("x", Distribution::Uniform { lo: 0.0, hi: 1.0 }, kind)
            .build()
            .unwrap()
    };
    assert_ne!(
        make(FactorKind::Continuous).content_hash(),
        make(FactorKind::Discrete).content_hash()
    );
}
#[test]
fn hash_is_exactly_32_bytes() {
    assert_eq!(uniform_problem("x", 1.0).content_hash().len(), 32);
}
#[test]
fn serde_round_trip_preserves_hash() {
    let original = uniform_problem("x", 1.0);
    let json = serde_json::to_string(&original).unwrap();
    let restored: Problem = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.content_hash(), original.content_hash());
}
