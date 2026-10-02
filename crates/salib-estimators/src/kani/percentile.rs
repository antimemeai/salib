//! Percentile endpoints and order over short, sorted finite samples.

use crate::bootstrap::percentile_value;

fn sorted_sample() -> ([f64; 8], usize) {
    let len: usize = kani::any();
    kani::assume((1..=8).contains(&len));
    let mut values = [0.0; 8];
    for index in 0..len {
        values[index] = kani::any::<f64>();
        kani::assume(values[index].is_finite() && (0.0..=100.0).contains(&values[index]));
        if index > 0 {
            kani::assume(values[index - 1] <= values[index]);
        }
    }
    (values, len)
}

#[kani::proof]
#[kani::unwind(9)]
fn percentile_endpoints() {
    let (values, len) = sorted_sample();
    let sorted = &values[..len];
    let alpha: f64 = kani::any();
    kani::assume(alpha.is_finite() && (0.0..=1.0).contains(&alpha));
    assert_eq!(percentile_value(sorted, 0.0), sorted[0]);
    assert_eq!(percentile_value(sorted, 1.0), sorted[len - 1]);
    assert!(percentile_value(sorted, alpha).is_finite());
}

#[kani::proof]
#[kani::unwind(9)]
fn percentile_monotone() {
    let (values, len) = sorted_sample();
    let alpha1: f64 = kani::any();
    let alpha2: f64 = kani::any();
    kani::assume(alpha1.is_finite() && (0.0..=1.0).contains(&alpha1));
    kani::assume(alpha2.is_finite() && (alpha1..=1.0).contains(&alpha2));
    let sorted = &values[..len];
    assert!(percentile_value(sorted, alpha1) <= percentile_value(sorted, alpha2));
}
