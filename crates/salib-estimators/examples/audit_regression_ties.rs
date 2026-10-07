//! Check row-order invariance of rank indices on a balanced binary design.
//! Run: cargo run -p salib-estimators --example audit_regression_ties
//! The assertions check zero association and joint row-permutation invariance.
use ndarray::array;
use salib_estimators::estimate_regression_indices;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Every (X,Y) pair occurs once, so X and Y are independent in this sample.
    let x = array![[0.0], [0.0], [1.0], [1.0]];
    let y = [0.0, 1.0, 0.0, 1.0];
    let first = estimate_regression_indices(x.view(), &y)?;
    // Permute rows jointly: [1,0,3,2]. X is unchanged because rows are tied.
    let permuted_y = [1.0, 0.0, 1.0, 0.0];
    let second = estimate_regression_indices(x.view(), &permuted_y)?;
    println!("Independent binary X,Y; expected SRRC=PRCC=0 in both row orders.");
    println!("Original: SRRC={:?}, PRCC={:?}", first.srrc, first.prcc);
    println!("Permuted: SRRC={:?}, PRCC={:?}", second.srrc, second.prcc);
    assert!(first.src[0].abs() < 1e-12 && first.pcc[0].abs() < 1e-12);
    assert!(
        first.srrc[0].abs() < 1e-12 && first.prcc[0].abs() < 1e-12,
        "tied ranks introduce an association absent from the data"
    );
    assert!(
        (first.srrc[0] - second.srrc[0]).abs() < 1e-12,
        "joint row permutation changes rank sensitivity"
    );
    Ok(())
}
