//! Value-based equal-frequency conditioning shared by given-data estimators.

/// Assign each tied value block using its midpoint in sorted rank order.
/// Untied observations follow the existing nearly equal-frequency bounds.
/// Empty classes are omitted; equal input values are never split.
/// The caller validates finite values and a positive requested class count.
pub(crate) fn classes(data: &[f64], requested: usize) -> Vec<Vec<usize>> {
    debug_assert!(!data.is_empty() && requested > 0);
    let mut order: Vec<_> = (0..data.len()).collect();
    order.sort_by(|&a, &b| data[a].total_cmp(&data[b]));
    let mut result = vec![Vec::new(); requested];
    let mut start = 0;
    while start < order.len() {
        let mut end = start + 1;
        while end < order.len() && data[order[start]] == data[order[end]] {
            end += 1;
        }
        // Rank midpoint is one-based. ceil(midpoint * M / N) - 1
        // preserves the historical (floor(jN/M), floor((j+1)N/M)]
        // bins for untied observations, without cutting a tie block.
        let numerator = (start as u128 + end as u128 + 1) * requested as u128;
        let denominator = 2 * data.len() as u128;
        let class = ((numerator - 1) / denominator) as usize;
        result[class.min(requested - 1)].extend_from_slice(&order[start..end]);
        start = end;
    }
    result.retain(|class| !class.is_empty());
    result
}
