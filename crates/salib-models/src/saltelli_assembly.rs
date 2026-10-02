//! Production Saltelli assembly with unique, exactly representable cell tags.
//! Two factors, two rows, both first/total and second-order modes. Base halves
//! and hybrids can be committed in any legal order; completion is explicit.

use ndarray::Array2;
use salib_core::RngState;
use salib_samplers::{build_saltelli_matrix, SaltelliMatrix, Sampler};
use stateright::{Model, Property};

const DIM: usize = 2;
const ROWS: usize = 2;
type Cells = [[u64; DIM]; ROWS];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Half {
    A,
    B,
}

fn tag(half: Half, row: usize, column: usize) -> f64 {
    let half = usize::from(half == Half::B);
    // Tags 1..=8 scaled into the unit cube; every value is exact in f64.
    let code = u32::try_from(1 + half * ROWS * DIM + row * DIM + column)
        .unwrap_or_else(|_| panic!("tag outside domain"));
    f64::from(code) / 16.0
}

struct SymbolicSampler;

impl Sampler for SymbolicSampler {
    fn dim(&self) -> usize {
        2 * DIM
    }
    fn unit_sample(&self, n: usize, _: &mut RngState) -> Array2<f64> {
        assert_eq!(n, ROWS);
        Array2::from_shape_fn((ROWS, 2 * DIM), |(row, column)| {
            tag(
                if column < DIM { Half::A } else { Half::B },
                row,
                column % DIM,
            )
        })
    }
    fn config_hash(&self) -> [u8; 32] {
        [0; 32]
    }
}

fn cells(matrix: &Array2<f64>) -> Cells {
    assert_eq!(matrix.dim(), (ROWS, DIM));
    std::array::from_fn(|row| std::array::from_fn(|column| matrix[[row, column]].to_bits()))
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    second_order: bool,
    a: Option<Cells>,
    b: Option<Cells>,
    ab: [Option<Cells>; DIM],
    ba: [Option<Cells>; DIM],
    complete: bool,
}

impl State {
    fn ready(&self) -> bool {
        self.a.is_some()
            && self.b.is_some()
            && self.ab.iter().all(Option::is_some)
            && (!self.second_order || self.ba.iter().all(Option::is_some))
    }

    fn provenance(&self) -> bool {
        let matches = |matrix: Option<Cells>, default: Half, swapped: Option<usize>| {
            matrix.is_none_or(|matrix| {
                matrix.iter().enumerate().all(|(row, values)| {
                    values.iter().enumerate().all(|(column, value)| {
                        let half = if swapped == Some(column) {
                            if default == Half::A {
                                Half::B
                            } else {
                                Half::A
                            }
                        } else {
                            default
                        };
                        *value == tag(half, row, column).to_bits()
                    })
                })
            })
        };
        matches(self.a, Half::A, None)
            && matches(self.b, Half::B, None)
            && (0..DIM).all(|i| {
                matches(self.ab[i], Half::A, Some(i)) && matches(self.ba[i], Half::B, Some(i))
            })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    ConstructA,
    ConstructB,
    ConstructAb(usize),
    ConstructBa(usize),
    Complete,
}

pub struct SaltelliAssemblyModel {
    first: SaltelliMatrix,
    second: SaltelliMatrix,
}

impl Default for SaltelliAssemblyModel {
    fn default() -> Self {
        Self::new()
    }
}

impl SaltelliAssemblyModel {
    pub fn new() -> Self {
        let build = |second_order| {
            build_saltelli_matrix(
                &SymbolicSampler,
                ROWS,
                second_order,
                &mut RngState::from_seed([0; 32]),
            )
            .unwrap_or_else(|error| panic!("symbolic design failed: {error}"))
        };
        Self {
            first: build(false),
            second: build(true),
        }
    }

    fn matrix(&self, state: &State) -> &SaltelliMatrix {
        if state.second_order {
            &self.second
        } else {
            &self.first
        }
    }
}

impl Model for SaltelliAssemblyModel {
    type State = State;
    type Action = Action;

    fn init_states(&self) -> Vec<State> {
        [false, true]
            .into_iter()
            .map(|second_order| State {
                second_order,
                a: None,
                b: None,
                ab: [None; DIM],
                ba: [None; DIM],
                complete: false,
            })
            .collect()
    }

    fn actions(&self, state: &State, actions: &mut Vec<Action>) {
        if state.complete {
            return;
        }
        if state.a.is_none() {
            actions.push(Action::ConstructA);
        }
        if state.b.is_none() {
            actions.push(Action::ConstructB);
        }
        if state.a.is_some() && state.b.is_some() {
            for i in 0..DIM {
                if state.ab[i].is_none() {
                    actions.push(Action::ConstructAb(i));
                }
                if state.second_order && state.ba[i].is_none() {
                    actions.push(Action::ConstructBa(i));
                }
            }
        }
        if state.ready() {
            actions.push(Action::Complete);
        }
    }

    fn next_state(&self, state: &State, action: Action) -> Option<State> {
        let matrix = self.matrix(state);
        let mut next = state.clone();
        match action {
            Action::ConstructA => next.a = Some(cells(&matrix.a)),
            Action::ConstructB => next.b = Some(cells(&matrix.b)),
            Action::ConstructAb(i) => next.ab[i] = Some(cells(&matrix.a_b[i])),
            Action::ConstructBa(i) => {
                let Some(ba) = &matrix.b_a else {
                    return None;
                };
                next.ba[i] = Some(cells(&ba[i]));
            }
            Action::Complete => {
                assert!(state.ready());
                assert_eq!(matrix.n, ROWS);
                assert_eq!(matrix.dim, DIM);
                assert_eq!(matrix.a_b.len(), DIM);
                assert_eq!(
                    matrix.b_a.as_ref().map(Vec::len),
                    state.second_order.then_some(DIM)
                );
                next.complete = true;
            }
        }
        Some(next)
    }

    fn properties(&self) -> Vec<Property<Self>> {
        vec![
            Property::<Self>::always("all cells have correct base/hybrid provenance", |_, s| {
                s.provenance()
            }),
            Property::<Self>::always("completion requires all requested hybrids", |_, s| {
                !s.complete || s.ready()
            }),
            Property::<Self>::always("first order contains no BA hybrids", |_, s| {
                s.second_order || s.ba.iter().all(Option::is_none)
            }),
            Property::<Self>::always("evaluation count matches design mode", |m, s| {
                m.matrix(s).total_evaluations()
                    == ROWS * (if s.second_order { 2 * DIM + 2 } else { DIM + 2 })
            }),
            Property::<Self>::sometimes("first order completed", |_, s| {
                !s.second_order && s.complete
            }),
            Property::<Self>::sometimes("second order completed", |_, s| {
                s.second_order && s.complete
            }),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_saltelli_assembly() {
        let result = crate::check(SaltelliAssemblyModel::new());
        assert!(result.explored > 0);
    }
}
