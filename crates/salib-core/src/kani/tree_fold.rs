//! Short reduction inputs, including empty arrays and odd tails.

use crate::{tree_dot, tree_sum};

#[kani::proof]
#[kani::unwind(9)]
#[kani::solver(kissat)]
fn tree_sum_indexing() {
    let len = usize::from(kani::any::<u8>());
    kani::assume(len <= 8);
    let mut values = [0.0; 8];
    let mut reference = 0_i16;
    // Small integer-valued floats make all additions exact. A left-fold
    // equality would be an invalid oracle for arbitrary floating inputs.
    for value in &mut values[..len] {
        let integer: i8 = kani::any();
        kani::assume((-1..=1).contains(&integer));
        *value = match integer {
            -1 => -1.0,
            0 => 0.0,
            _ => 1.0,
        };
        reference += i16::from(integer);
    }
    // Dispatch literal slice lengths so symbolic execution does not explore
    // impossible Vec lengths through nine rounds of the halving loop.
    let result = match len {
        0 => tree_sum(&values[..0]),
        1 => tree_sum(&values[..1]),
        2 => tree_sum(&values[..2]),
        3 => tree_sum(&values[..3]),
        4 => tree_sum(&values[..4]),
        5 => tree_sum(&values[..5]),
        6 => tree_sum(&values[..6]),
        7 => tree_sum(&values[..7]),
        _ => tree_sum(&values[..8]),
    };
    assert_eq!(result, f64::from(reference));
}

#[kani::proof]
fn tree_sum_empty() {
    assert_eq!(tree_sum(&[]), 0.0);
}

#[kani::proof]
fn tree_sum_single() {
    let value: f64 = kani::any();
    kani::assume(value.is_finite());
    assert_eq!(tree_sum(&[value]).to_bits(), value.to_bits());
}

#[kani::proof]
#[kani::unwind(9)]
#[kani::solver(kissat)]
fn tree_dot_no_panic() {
    let len = usize::from(kani::any::<u8>());
    kani::assume(len <= 8);
    let mut a = [0.0; 8];
    let mut b = [0.0; 8];
    for index in 0..len {
        a[index] = kani::any::<f64>();
        b[index] = kani::any::<f64>();
        kani::assume(a[index].is_finite() && (-16.0..=16.0).contains(&a[index]));
        kani::assume(b[index].is_finite() && (-16.0..=16.0).contains(&b[index]));
    }
    // Literal lengths keep the internal allocation and halving bounds known.
    let _ = match len {
        0 => tree_dot(&a[..0], &b[..0]),
        1 => tree_dot(&a[..1], &b[..1]),
        2 => tree_dot(&a[..2], &b[..2]),
        3 => tree_dot(&a[..3], &b[..3]),
        4 => tree_dot(&a[..4], &b[..4]),
        5 => tree_dot(&a[..5], &b[..5]),
        6 => tree_dot(&a[..6], &b[..6]),
        7 => tree_dot(&a[..7], &b[..7]),
        _ => tree_dot(&a[..8], &b[..8]),
    };
}
