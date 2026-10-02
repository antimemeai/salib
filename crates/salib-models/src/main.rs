use salib_models::{
    check, estimator_completeness::EstimatorCompletenessModel,
    problem_builder::ProblemBuilderModel, rng_protocol::RngProtocolModel,
    saltelli_assembly::SaltelliAssemblyModel, ExplorationStats,
};

fn report(name: &str, stats: ExplorationStats) {
    println!(
        "{name}: PASS; states={}, transitions={}, depth={}, safety={}, reachability={}; violations=0",
        stats.explored, stats.transitions, stats.depth,
        stats.safety_properties, stats.reachability_properties,
    );
}

fn main() {
    report("problem_builder", check(ProblemBuilderModel::new()));
    report("rng_protocol", check(RngProtocolModel::new()));
    report("saltelli_assembly", check(SaltelliAssemblyModel::new()));
    report(
        "estimator_completeness",
        check(EstimatorCompletenessModel::new()),
    );
}
