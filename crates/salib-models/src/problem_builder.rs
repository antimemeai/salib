//! Up to three ordered factors, names A/B/C, and four distribution classes.

use salib_core::{BuildError, Distribution, ProblemBuilder};
use stateright::{Model, Property};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DistributionClass {
    Uniform,
    ReversedUniform,
    NanNormal,
    Normal,
}

impl DistributionClass {
    fn concrete(self) -> Distribution {
        match self {
            Self::Uniform => Distribution::Uniform { lo: 0.0, hi: 1.0 },
            Self::ReversedUniform => Distribution::Uniform { lo: 1.0, hi: 0.0 },
            Self::NanNormal => Distribution::Normal {
                mu: f64::NAN,
                sigma: 1.0,
            },
            Self::Normal => Distribution::Normal {
                mu: 0.0,
                sigma: 1.0,
            },
        }
    }

    fn valid(self) -> bool {
        matches!(self, Self::Uniform | Self::Normal)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SymbolicFactor {
    name: u8,
    distribution: DistributionClass,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BuildStatus {
    Draft,
    Built,
    Empty,
    Duplicate(u8),
    InvalidDistribution(u8),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    factors: Vec<SymbolicFactor>,
    status: BuildStatus,
}

impl State {
    fn expected(&self) -> BuildStatus {
        if self.factors.is_empty() {
            return BuildStatus::Empty;
        }
        for (index, factor) in self.factors.iter().enumerate() {
            if self.factors[..index].iter().any(|f| f.name == factor.name) {
                return BuildStatus::Duplicate(factor.name);
            }
        }
        for factor in &self.factors {
            if !factor.distribution.valid() {
                return BuildStatus::InvalidDistribution(factor.name);
            }
        }
        BuildStatus::Built
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action {
    Add {
        name: u8,
        distribution: DistributionClass,
    },
    Build,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ProblemBuilderModel;

impl ProblemBuilderModel {
    pub fn new() -> Self {
        Self
    }
}

const NAMES: [&str; 3] = ["A", "B", "C"];

impl Model for ProblemBuilderModel {
    type State = State;
    type Action = Action;

    fn init_states(&self) -> Vec<State> {
        vec![State {
            factors: Vec::new(),
            status: BuildStatus::Draft,
        }]
    }

    fn actions(&self, state: &State, actions: &mut Vec<Action>) {
        if state.status != BuildStatus::Draft {
            return;
        }
        if state.factors.len() < 3 {
            for name in 0..3 {
                for distribution in [
                    DistributionClass::Uniform,
                    DistributionClass::ReversedUniform,
                    DistributionClass::NanNormal,
                    DistributionClass::Normal,
                ] {
                    actions.push(Action::Add { name, distribution });
                }
            }
        }
        actions.push(Action::Build);
    }

    fn next_state(&self, state: &State, action: Action) -> Option<State> {
        let mut next = state.clone();
        match action {
            Action::Add { name, distribution } => {
                next.factors.push(SymbolicFactor { name, distribution });
            }
            Action::Build => {
                let mut builder = ProblemBuilder::new();
                for factor in &state.factors {
                    builder = builder.factor(
                        NAMES[usize::from(factor.name)],
                        factor.distribution.concrete(),
                    );
                }
                next.status = match builder.build() {
                    Ok(problem) => {
                        assert_eq!(problem.dim(), state.factors.len());
                        // Re-enter the private validate_distribution through its public
                        // builder boundary, also checking insertion order and exact bits.
                        for (actual, symbolic) in problem.factors().iter().zip(&state.factors) {
                            assert_eq!(actual.name, NAMES[usize::from(symbolic.name)]);
                            assert_eq!(actual.distribution, symbolic.distribution.concrete());
                            assert!(ProblemBuilder::new()
                                .factor(&actual.name, actual.distribution.clone())
                                .build()
                                .is_ok());
                        }
                        BuildStatus::Built
                    }
                    Err(BuildError::Empty) => BuildStatus::Empty,
                    Err(BuildError::DuplicateName { name }) => {
                        BuildStatus::Duplicate(name_id(&name))
                    }
                    Err(BuildError::InvalidDistribution { name, .. }) => {
                        BuildStatus::InvalidDistribution(name_id(&name))
                    }
                    Err(error) => panic!("unexpected builder error: {error}"),
                };
            }
        }
        Some(next)
    }

    fn properties(&self) -> Vec<Property<Self>> {
        vec![
            Property::<Self>::always("successful problems are nonempty", |_, s| {
                s.status != BuildStatus::Built || !s.factors.is_empty()
            }),
            Property::<Self>::always("successful problems have unique names", |_, s| {
                s.status != BuildStatus::Built
                    || s.factors
                        .iter()
                        .enumerate()
                        .all(|(i, f)| !s.factors[..i].iter().any(|other| other.name == f.name))
            }),
            Property::<Self>::always("successful distributions are valid", |_, s| {
                s.status != BuildStatus::Built || s.factors.iter().all(|f| f.distribution.valid())
            }),
            Property::<Self>::always(
                "empty then duplicate then distribution error precedence",
                |_, s| s.status == BuildStatus::Draft || s.status == s.expected(),
            ),
            Property::<Self>::sometimes("build succeeds", |_, s| s.status == BuildStatus::Built),
            Property::<Self>::sometimes("empty rejected", |_, s| s.status == BuildStatus::Empty),
            Property::<Self>::sometimes("duplicate precedes invalid distribution", |_, s| {
                matches!(s.status, BuildStatus::Duplicate(_))
                    && s.factors.iter().any(|f| !f.distribution.valid())
            }),
            Property::<Self>::sometimes("NaN mean rejected", |_, s| {
                matches!(s.status, BuildStatus::InvalidDistribution(_))
                    && s.factors
                        .iter()
                        .any(|f| f.distribution == DistributionClass::NanNormal)
            }),
        ]
    }
}

fn name_id(name: &str) -> u8 {
    match name {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => panic!("unknown factor name {name}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_problem_builder() {
        let result = crate::check(ProblemBuilderModel::new());
        assert!(result.explored > 0);
    }
}
