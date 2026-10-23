//! Conformance with Brennan (2001) Synthetic Data Set No. 3 (#6).
//!
//! Printed values are four decimals for components and two for coefficients,
//! so tolerances are half a unit in the last printed place. salib 0.3.0
//! already matched; this replaces the self-derived "textbook" oracle.

#![allow(clippy::unwrap_used)]

mod g_theory_fixtures;

use g_theory_fixtures::synthetic3;
use salib_estimators::{estimate_g_theory_pir, GTheoryDesign};

#[test]
fn synthetic_data_set_3_components_and_coefficients() {
    let r = estimate_g_theory_pir(synthetic3().view(), GTheoryDesign::Crossed).unwrap();
    let printed = [0.5528, 0.4417, 0.0074, 0.5750, 0.1009, 0.1565, 0.9352];
    for (actual, printed) in r.components().iter().zip(printed) {
        assert!(
            (actual - printed).abs() <= 0.000_05,
            "component {actual} vs printed {printed}"
        );
    }
    assert!((r.g_coefficient.unwrap() - 0.64).abs() <= 0.005);
    assert!((r.phi_coefficient.unwrap() - 0.55).abs() <= 0.005);
}
