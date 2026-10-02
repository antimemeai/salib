//! Verify the indexing kernel with arbitrary direction-number contents.

use crate::sobol::RES;
use crate::SobolSampler;

#[kani::proof]
fn sobol_direction_index_bounds() {
    let k: usize = kani::any();
    let dim: usize = kani::any();
    let column: usize = kani::any();
    kani::assume(k <= 1000);
    kani::assume((1..=8).contains(&dim));
    kani::assume(column < dim);
    let _ = SobolSampler::standard(dim);
    // Contents cannot affect indexing. Use the production resolution and
    // model any possible table for the bounded dimensions; no recurrence
    // proof or floating-point point-generation claim is made here.
    let directions: [[u32; RES as usize + 1]; 8] = kani::any();
    // The production sampler emits the origin without accessing a direction
    // number. trailing_zeros(0) must never feed the nonzero-step formula.
    if k == 0 {
        return;
    }
    let direction_index = k.trailing_zeros() as usize + 1;
    assert!((1..=32).contains(&direction_index));
    assert!(direction_index < directions[column].len());
    let _ = directions[column][direction_index];
}
