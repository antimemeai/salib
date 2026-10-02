//! Two seeds, two root streams, two salt-addressed children, and 0..=3 words
//! per live handle. Fork depth is one. A recorded snapshot is separate from the
//! live parent, so drawing and checkpoint/resume can be interleaved arbitrarily.
//! Positions here are canonical and far below ChaCha's 68-bit position limit.

use rand::RngCore;
use salib_core::RngState;
use stateright::{Model, Property};

const SALTS: [&[u8]; 2] = [b"task-0", b"task-1"];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Checkpoint {
    seed: u8,
    stream: u64,
    position: u8,
}

impl Checkpoint {
    fn concrete(self) -> RngState {
        RngState::from_parts([self.seed; 32], self.stream, u128::from(self.position))
    }

    fn from_actual(actual: &RngState) -> Self {
        assert_eq!(actual.seed, [actual.seed[0]; 32]);
        assert!(actual.word_pos <= 3);
        Self {
            seed: actual.seed[0],
            stream: actual.stream,
            position: u8::try_from(actual.word_pos)
                .unwrap_or_else(|_| panic!("position outside domain")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Handle {
    live: Checkpoint,
    /// Independent count of words consumed since this stream's origin.
    reference_position: u8,
}

impl Handle {
    fn draw(self, count: u8) -> Self {
        let parent = self.live.concrete();
        let mut live = parent.clone().into_chacha();
        // Restore the reference at position zero and advance with actual draws,
        // independently of the live snapshot's recorded word position.
        let mut reference = RngState::from_parts(parent.seed, parent.stream, 0).into_chacha();
        for _ in 0..self.reference_position {
            reference.next_u32();
        }
        for _ in 0..count {
            assert_eq!(
                live.next_u32(),
                reference.next_u32(),
                "resume/draw equivalence"
            );
        }
        let actual = RngState::snapshot(&live, &parent);
        assert_eq!(actual.seed, parent.seed, "checkpoint seed provenance");
        assert_eq!(actual.algorithm, parent.algorithm);
        assert_eq!(actual.stream, parent.stream);
        assert_eq!(actual.word_pos, u128::from(self.reference_position + count));
        Self {
            live: Checkpoint::from_actual(&actual),
            reference_position: self.reference_position + count,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    parent: Handle,
    forks: [Option<Handle>; 2],
    snapshot: Option<Handle>,
    resumed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Fork(usize),
    Draw { handle: usize, count: u8 },
    Snapshot,
    Resume,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RngProtocolModel;

impl RngProtocolModel {
    pub fn new() -> Self {
        Self
    }
}

impl Model for RngProtocolModel {
    type State = State;
    type Action = Action;

    fn init_states(&self) -> Vec<State> {
        let mut states = Vec::new();
        for seed in [0, 0x42] {
            for stream in [0, 7] {
                states.push(State {
                    parent: Handle {
                        live: Checkpoint {
                            seed,
                            stream,
                            position: 0,
                        },
                        reference_position: 0,
                    },
                    forks: [None; 2],
                    snapshot: None,
                    resumed: false,
                });
            }
        }
        states
    }

    fn actions(&self, state: &State, actions: &mut Vec<Action>) {
        actions.extend([Action::Fork(0), Action::Fork(1), Action::Snapshot]);
        if state.snapshot.is_some() {
            actions.push(Action::Resume);
        }
        for (handle, live) in std::iter::once(Some(state.parent))
            .chain(state.forks)
            .enumerate()
        {
            if let Some(live) = live {
                for count in 0..=3 - live.reference_position {
                    actions.push(Action::Draw { handle, count });
                }
            }
        }
    }

    fn next_state(&self, state: &State, action: Action) -> Option<State> {
        let mut next = state.clone();
        match action {
            Action::Fork(salt) => {
                let parent = state.parent.live.concrete();
                let before = parent.clone();
                let child = parent.fork(SALTS[salt]);
                assert_eq!(parent, before, "fork preserves parent");
                assert_eq!(child, before.fork(SALTS[salt]), "fork determinism");
                let origin = RngState::from_parts(before.seed, before.stream, 0);
                assert_eq!(
                    child,
                    origin.fork(SALTS[salt]),
                    "fork position independence"
                );
                assert_eq!(child.word_pos, 0, "child starts at zero");
                assert_eq!(child.seed, before.seed);
                assert_eq!(child.algorithm, before.algorithm);
                next.forks[salt] = Some(Handle {
                    live: Checkpoint::from_actual(&child),
                    reference_position: 0,
                });
                assert_eq!(next.parent, state.parent);
            }
            Action::Draw { handle: 0, count } => next.parent = state.parent.draw(count),
            Action::Draw { handle, count } => {
                let child = state.forks[handle - 1]?;
                next.forks[handle - 1] = Some(child.draw(count));
            }
            Action::Snapshot => {
                let parent = state.parent.live.concrete();
                let live = parent.clone().into_chacha();
                let snapshot = RngState::snapshot(&live, &parent);
                assert_eq!(snapshot, parent);
                next.snapshot = Some(Handle {
                    live: Checkpoint::from_actual(&snapshot),
                    reference_position: state.parent.reference_position,
                });
            }
            Action::Resume => {
                let saved = state.snapshot?;
                // Compare every allowed lookahead count with the uninterrupted
                // stream, even if this resumed state was already visited.
                for count in 0..=3 - saved.reference_position {
                    saved.draw(count);
                }
                next.parent = saved;
                next.resumed = true;
            }
        }
        Some(next)
    }

    fn properties(&self) -> Vec<Property<Self>> {
        vec![
            Property::<Self>::always("live and reference positions agree", |_, s| {
                std::iter::once(Some(s.parent))
                    .chain(s.forks)
                    .chain([s.snapshot])
                    .flatten()
                    .all(|h| h.live.position == h.reference_position)
            }),
            Property::<Self>::always("fork identity is independent of parent position", |_, s| {
                s.forks.iter().enumerate().all(|(salt, child)| {
                    child.is_none_or(|h| {
                        let expected = s.parent.live.concrete().fork(SALTS[salt]);
                        h.live.seed == expected.seed[0] && h.live.stream == expected.stream
                    })
                })
            }),
            Property::<Self>::always("snapshot retains parent provenance", |_, s| {
                s.snapshot.is_none_or(|h| {
                    h.live.seed == s.parent.live.seed && h.live.stream == s.parent.live.stream
                })
            }),
            Property::<Self>::sometimes("both logical forks created", |_, s| {
                s.forks.iter().all(Option::is_some)
            }),
            Property::<Self>::sometimes("checkpoint resumed after draws", |_, s| {
                s.resumed && s.parent.reference_position > 0
            }),
            Property::<Self>::sometimes("checkpoint lags live parent", |_, s| {
                s.snapshot
                    .is_some_and(|h| h.reference_position < s.parent.reference_position)
            }),
            Property::<Self>::sometimes("child drew independently", |_, s| {
                s.forks.iter().flatten().any(|h| h.reference_position > 0)
            }),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_rng_protocol() {
        let result = crate::check(RngProtocolModel::new());
        assert!(result.explored > 0);
    }
}
