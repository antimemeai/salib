//! A prospective asynchronous output collector around the real cached-output
//! Saltelli estimators. First/total designs: d=1..=2, n=2..=3. Second-order:
//! d=2, n=2 (twelve output slots). Each slot has a campaign/role/index/row
//! identity. Values are either small exact integers or uniformly constant.
//! Malformed series taint a campaign; no repair transition is modeled.

use salib_estimators::{
    estimate_saltelli2010_from_outputs, estimate_saltelli2010_from_outputs_with_second_order,
    SobolIndices,
};
use stateright::{Model, Property};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Role {
    A,
    B,
    Ab(u8),
    Ba(u8),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OutputIdentity {
    campaign: u8,
    role: Role,
    row: u8,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Campaign {
    factors: u8,
    rows: u8,
    second_order: bool,
    constant: bool,
}

impl Campaign {
    fn roles(self) -> Vec<Role> {
        let mut roles = vec![Role::A, Role::B];
        roles.extend((0..self.factors).map(Role::Ab));
        if self.second_order {
            roles.extend((0..self.factors).map(Role::Ba));
        }
        roles
    }

    fn required(self) -> u32 {
        (1 << (self.roles().len() * usize::from(self.rows))) - 1
    }

    fn slot(self, identity: OutputIdentity) -> Option<u32> {
        if identity.campaign != 0 || identity.row >= self.rows {
            return None;
        }
        let role = self
            .roles()
            .iter()
            .position(|role| *role == identity.role)?;
        Some(1 << (role * usize::from(self.rows) + usize::from(identity.row)))
    }

    fn series(self, role: Role) -> Vec<f64> {
        (0..self.rows)
            .map(|row| {
                if self.constant {
                    return 1.0;
                }
                let offset = match role {
                    Role::A => 1,
                    Role::B => 2,
                    Role::Ab(i) => 3 + i,
                    Role::Ba(i) => 5 + i,
                };
                f64::from(row + offset)
            })
            .collect()
    }

    fn estimate(self) -> SobolIndices {
        let fa = self.series(Role::A);
        let fb = self.series(Role::B);
        let fab: Vec<_> = (0..self.factors)
            .map(|i| self.series(Role::Ab(i)))
            .collect();
        if self.second_order {
            let fba: Vec<_> = (0..self.factors)
                .map(|i| self.series(Role::Ba(i)))
                .collect();
            estimate_saltelli2010_from_outputs_with_second_order(&fa, &fb, &fab, &fba)
        } else {
            estimate_saltelli2010_from_outputs(&fa, &fb, &fab)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Status {
    Collecting,
    Duplicate,
    RejectedIdentity,
    RejectedIncomplete,
    RejectedLength,
    Estimated,
    Published,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ResultSummary {
    finite: bool,
    all_zero: bool,
    correct_shape: bool,
}

fn summarize(result: &SobolIndices, campaign: Campaign) -> ResultSummary {
    let values = std::iter::once(&result.total_variance)
        .chain(&result.first_order)
        .chain(&result.total_order)
        .chain(result.second_order.iter().flatten().flatten());
    let finite = values.clone().all(|v| v.is_finite());
    let all_zero = values.clone().all(|v| *v == 0.0);
    let dim = usize::from(campaign.factors);
    let second_shape = if campaign.second_order {
        result.second_order.as_ref().is_some_and(|rows| {
            rows.len() == dim
                && rows
                    .iter()
                    .enumerate()
                    .all(|(i, row)| row.len() == dim - i - 1)
        })
    } else {
        result.second_order.is_none()
    };
    ResultSummary {
        finite,
        all_zero,
        correct_shape: result.n == usize::from(campaign.rows)
            && result.dim == dim
            && result.first_order.len() == dim
            && result.total_order.len() == dim
            && second_shape,
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    campaign: Campaign,
    received: u32,
    malformed: bool,
    status: Status,
    result: Option<ResultSummary>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Receive(OutputIdentity),
    ReceiveSeries { role: Role, length: u8 },
    AttemptEstimation,
    Publish,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EstimatorCompletenessModel;

impl EstimatorCompletenessModel {
    pub fn new() -> Self {
        Self
    }
}

impl Model for EstimatorCompletenessModel {
    type State = State;
    type Action = Action;

    fn init_states(&self) -> Vec<State> {
        let mut states = Vec::new();
        for constant in [false, true] {
            for factors in 1..=2 {
                for rows in 2..=3 {
                    states.push(State {
                        campaign: Campaign {
                            factors,
                            rows,
                            second_order: false,
                            constant,
                        },
                        received: 0,
                        malformed: false,
                        status: Status::Collecting,
                        result: None,
                    });
                }
            }
            states.push(State {
                campaign: Campaign {
                    factors: 2,
                    rows: 2,
                    second_order: true,
                    constant,
                },
                received: 0,
                malformed: false,
                status: Status::Collecting,
                result: None,
            });
        }
        states
    }

    fn actions(&self, state: &State, actions: &mut Vec<Action>) {
        match state.status {
            Status::Published => return,
            Status::Estimated => {
                actions.push(Action::Publish);
                return;
            }
            _ => {}
        }
        for role in state.campaign.roles() {
            for row in 0..state.campaign.rows {
                // Including already received slots explores duplicate retries.
                actions.push(Action::Receive(OutputIdentity {
                    campaign: 0,
                    role,
                    row,
                }));
            }
            for length in [state.campaign.rows - 1, state.campaign.rows + 1] {
                actions.push(Action::ReceiveSeries { role, length });
            }
        }
        // Stale campaign, out-of-range row, and invalid hybrid index.
        actions.push(Action::Receive(OutputIdentity {
            campaign: 1,
            role: Role::A,
            row: 0,
        }));
        actions.push(Action::Receive(OutputIdentity {
            campaign: 0,
            role: Role::B,
            row: state.campaign.rows,
        }));
        actions.push(Action::Receive(OutputIdentity {
            campaign: 0,
            role: Role::Ab(state.campaign.factors),
            row: 0,
        }));
        actions.push(Action::AttemptEstimation);
    }

    fn next_state(&self, state: &State, action: Action) -> Option<State> {
        let mut next = state.clone();
        match action {
            Action::Receive(identity) => {
                if let Some(slot) = state.campaign.slot(identity) {
                    next.status = if state.received & slot != 0 {
                        Status::Duplicate
                    } else {
                        Status::Collecting
                    };
                    next.received |= slot;
                } else {
                    next.status = Status::RejectedIdentity;
                    assert_eq!(
                        next.received, state.received,
                        "foreign outputs never fill slots"
                    );
                }
            }
            Action::ReceiveSeries { role, length } => {
                assert!(state.campaign.roles().contains(&role));
                assert_ne!(length, state.campaign.rows);
                next.malformed = true;
                next.status = Status::RejectedLength;
            }
            Action::AttemptEstimation => {
                next.status = if state.malformed {
                    Status::RejectedLength
                } else if state.received != state.campaign.required() {
                    Status::RejectedIncomplete
                } else {
                    // Only here may the real estimator consume assembled series.
                    next.result = Some(summarize(&state.campaign.estimate(), state.campaign));
                    Status::Estimated
                };
            }
            Action::Publish => {
                if state.status != Status::Estimated
                    || state.malformed
                    || state.received != state.campaign.required()
                {
                    return None;
                }
                next.status = Status::Published;
            }
        }
        Some(next)
    }

    fn properties(&self) -> Vec<Property<Self>> {
        vec![
            Property::<Self>::always("estimation requires every output identity", |_, s| {
                s.result.is_none() || s.received == s.campaign.required()
            }),
            Property::<Self>::always("length mismatches never reach estimation", |_, s| {
                !s.malformed || s.result.is_none()
            }),
            Property::<Self>::always("results have finite values and correct shape", |_, s| {
                s.result.is_none_or(|r| r.finite && r.correct_shape)
            }),
            Property::<Self>::always("constant outputs produce zeros", |_, s| {
                !s.campaign.constant || s.result.is_none_or(|r| r.all_zero)
            }),
            Property::<Self>::always("publication requires validated complete outputs", |_, s| {
                s.status != Status::Published
                    || (!s.malformed
                        && s.received == s.campaign.required()
                        && s.result.is_some_and(|r| r.finite && r.correct_shape))
            }),
            Property::<Self>::always("received slots belong to this campaign", |_, s| {
                s.received & !s.campaign.required() == 0
            }),
            Property::<Self>::sometimes("ordinary outputs published", |_, s| {
                !s.campaign.constant && s.status == Status::Published
            }),
            Property::<Self>::sometimes("constant outputs published as zeros", |_, s| {
                s.campaign.constant && s.status == Status::Published
            }),
            Property::<Self>::sometimes("second order outputs published", |_, s| {
                s.campaign.second_order && s.status == Status::Published
            }),
            Property::<Self>::sometimes("incomplete outputs rejected", |_, s| {
                s.status == Status::RejectedIncomplete
            }),
            Property::<Self>::sometimes("complete but mismatched outputs rejected", |_, s| {
                s.received == s.campaign.required() && s.status == Status::RejectedLength
            }),
            Property::<Self>::sometimes("duplicate delivery explored", |_, s| {
                s.status == Status::Duplicate
            }),
            Property::<Self>::sometimes("foreign identity rejected", |_, s| {
                s.status == Status::RejectedIdentity
            }),
            Property::<Self>::sometimes("missing BA blocks second order estimation", |_, s| {
                s.campaign.second_order && s.status == Status::RejectedIncomplete && {
                    let first_slots = (1
                        << (usize::from(s.campaign.rows) * usize::from(s.campaign.factors + 2)))
                        - 1;
                    s.received & first_slots == first_slots && s.received != s.campaign.required()
                }
            }),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_estimator_completeness() {
        let result = crate::check(EstimatorCompletenessModel::new());
        assert!(result.explored > 0);
    }

    #[test]
    fn real_estimators_reject_mismatched_lengths_and_hybrid_counts() {
        // Directly exercise the synchronous API's documented panic boundary.
        // Model collection safety alone does not establish production rejection.
        let fa = [1.0, 2.0];
        let fb = [2.0, 3.0];
        let fab = vec![vec![3.0, 4.0], vec![4.0, 5.0]];
        let fba = vec![vec![5.0, 6.0], vec![6.0, 7.0]];
        for length in [0, 1, 3] {
            let wrong = vec![1.0; length];
            assert!(
                std::panic::catch_unwind(|| estimate_saltelli2010_from_outputs(&fa, &wrong, &fab))
                    .is_err()
            );
            assert!(std::panic::catch_unwind(|| {
                estimate_saltelli2010_from_outputs_with_second_order(&fa, &wrong, &fab, &fba)
            })
            .is_err());
            for index in 0..2 {
                let mut bad_ab = fab.clone();
                bad_ab[index] = wrong.clone();
                assert!(
                    std::panic::catch_unwind(|| estimate_saltelli2010_from_outputs(
                        &fa, &fb, &bad_ab
                    ))
                    .is_err()
                );
                assert!(std::panic::catch_unwind(|| {
                    estimate_saltelli2010_from_outputs_with_second_order(&fa, &fb, &bad_ab, &fba)
                })
                .is_err());
                let mut bad_ba = fba.clone();
                bad_ba[index] = wrong.clone();
                assert!(std::panic::catch_unwind(|| {
                    estimate_saltelli2010_from_outputs_with_second_order(&fa, &fb, &fab, &bad_ba)
                })
                .is_err());
            }
        }
        assert!(
            std::panic::catch_unwind(|| estimate_saltelli2010_from_outputs(&fa, &fb, &[])).is_err()
        );
        assert!(std::panic::catch_unwind(|| {
            estimate_saltelli2010_from_outputs_with_second_order(&fa, &fb, &[], &[])
        })
        .is_err());
        for count in [0, 1, 3] {
            let bad_ba = vec![vec![1.0, 2.0]; count];
            assert!(std::panic::catch_unwind(|| {
                estimate_saltelli2010_from_outputs_with_second_order(&fa, &fb, &fab, &bad_ba)
            })
            .is_err());
        }
    }
}
