//! Exhaustive, bounded protocol models backed by salib's production APIs.
//!
//! These establish safety within the documented finite domains, not unbounded
//! correctness. The estimator collector is a proposed asynchronous protocol;
//! salib's current estimators consume synchronous, already collected arrays.

#![forbid(unsafe_code)]

pub mod estimator_completeness;
pub mod problem_builder;
pub mod rng_protocol;
pub mod saltelli_assembly;

use std::fmt::Debug;
use std::hash::Hash;

use stateright::{Checker, Model};

/// Statistics from a completed Stateright breadth-first exploration.
#[derive(Debug, Clone, Copy)]
pub struct ExplorationStats {
    /// Distinct reachable states (Stateright deduplicates by fingerprint).
    pub explored: usize,
    /// Generated, non-`None` transitions, including revisits and self loops.
    pub transitions: usize,
    /// Maximum shortest-path action count (initial states have depth zero).
    pub depth: usize,
    pub safety_properties: usize,
    pub reachability_properties: usize,
}

/// Exhaust all reachable states and fail on any assertion or property violation.
pub fn check<M>(model: M) -> ExplorationStats
where
    M: Model + Send + Sync + 'static,
    M::State: Clone + Debug + Eq + Hash + Send + Sync,
    M::Action: Clone + Debug + Eq,
{
    let initial_count = model.init_states().len();
    let properties = model.properties();
    let reachability_properties = properties
        .iter()
        .filter(|p| p.expectation == stateright::Expectation::Sometimes)
        .count();
    let safety_properties = properties.len() - reachability_properties;
    // No time/depth/state cutoff. An unfalsified `always` property keeps the
    // checker exploring even after every `sometimes` witness has been found.
    let checker = model.checker().threads(1).spawn_bfs().join();
    checker.assert_properties();
    assert!(checker.is_done(), "exploration must complete");
    assert!(checker.unique_state_count() > 0, "empty state space");
    ExplorationStats {
        explored: checker.unique_state_count(),
        transitions: checker.state_count() - initial_count,
        depth: checker.max_depth().saturating_sub(1),
        safety_properties,
        reachability_properties,
    }
}
