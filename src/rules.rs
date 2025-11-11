//! Collapse constraints.

use crate::{
    state::{State, StateSet},
    Space,
};
use rand::{seq::IteratorRandom, Rng};
use std::marker::PhantomData;

/// For collapsing superpositions.
pub trait SetCollapseObserver {
    /// Arbitrarily collapse a superposition into a single state.
    fn observe(&self, cell: &mut StateSet, neighbors: &[Option<StateSet>], rng: &mut impl Rng);
}

/// Collapse a superposition into a uniformly-random one of its states.
#[derive(Clone)]
pub struct UniformSetCollapseObserver;

impl SetCollapseObserver for UniformSetCollapseObserver {
    fn observe(&self, cell: &mut StateSet, _: &[Option<StateSet>], rng: &mut impl Rng) {
        *cell = StateSet::with_states(&[cell.iter().choose(rng).unwrap()]);
    }
}

/// Adjacency rules.
pub struct SetCollapseRules<O: SetCollapseObserver> {
    state_rules: Box<[(State, Box<[Option<StateSet>]>)]>,
    observer: O,
}

struct StateRule {
    state: State,
    allowed_neighbors: Vec<Option<StateSet>>,
}

impl StateRule {
    fn add_allowed(&mut self, neighbor_index: usize, allowed: State) {
        while self.allowed_neighbors.len() <= neighbor_index {
            self.allowed_neighbors.push(None);
        }
        if let Some(allowed_neighbors) = &mut self.allowed_neighbors[neighbor_index] {
            allowed_neighbors.add(allowed);
        } else {
            self.allowed_neighbors[neighbor_index] = Some(StateSet::with_states(&[allowed]));
        }
    }
}

/// Builder for [SetCollapseRules]
///
/// Automatically collects used coordinate deltas and manages creating symmetric rules from asymmetric definitions
pub struct SetCollapseRulesBuilder<Sp: Space<StateSet>, O: SetCollapseObserver + Clone> {
    state_rules: Vec<StateRule>,
    observer: O,
    _spooky: PhantomData<Sp>,
}

impl<Sp: Space<StateSet>, O: SetCollapseObserver + Clone> SetCollapseRulesBuilder<Sp, O>
where
    Sp::Direction: Eq + Clone,
{
    pub fn new(observer: O) -> Self {
        Self {
            state_rules: Vec::new(),
            observer,
            _spooky: PhantomData,
        }
    }

    /// Set the allowed neighbors for a cell based on their coordinate deltas
    ///
    /// This will create symmetric rules. For example, if you set state A to be
    /// allowed to the left of B, then state B will be allowed to the right of
    /// A - you don't have to explicitly set both rules.
    ///
    /// States which do not have any allowed neighbors for a given coordinate
    /// delta will require that those coordinates are outside of world-space.
    pub fn allow(mut self, state: State, neighbors: &[(Sp::Direction, StateSet)]) -> Self {
        for (delta, neighbor) in neighbors {
            for n_state in neighbor.iter() {
                self.allow_symmetric(state, n_state, delta);
            }
        }
        self
    }

    fn allow_symmetric(&mut self, a: State, b: State, offset: &Sp::Direction) {
        let offset_index = self.get_offset_index(*offset);
        self.get_rule(a).add_allowed(offset_index, b);
        let offset_index = self.get_offset_index(-*offset);
        self.get_rule(b).add_allowed(offset_index, a);
    }

    fn get_offset_index(&mut self, offset: Sp::Direction) -> usize {
        for i in 0..Sp::DIRECTIONS.len() {
            if Sp::DIRECTIONS[i] == offset {
                return i;
            }
        }
        panic!("invalid neighbor at {offset:?}");
    }

    fn get_rule(&mut self, state: State) -> &mut StateRule {
        for i in 0..self.state_rules.len() {
            if self.state_rules[i].state == state {
                return &mut self.state_rules[i];
            }
        }
        self.state_rules.push(StateRule {
            state,
            allowed_neighbors: Vec::new(),
        });
        let index = self.state_rules.len() - 1;
        &mut self.state_rules[index]
    }

    pub fn build(self) -> SetCollapseRules<O> {
        let mut state_rules = Vec::new();
        let mut remaining_state = StateSet::all();
        for mut proto_rule in self.state_rules {
            while proto_rule.allowed_neighbors.len() < Sp::DIRECTIONS.len() {
                proto_rule.allowed_neighbors.push(None);
            }
            remaining_state.remove(proto_rule.state);
            state_rules.push((
                proto_rule.state,
                proto_rule.allowed_neighbors.into_boxed_slice(),
            ));
        }
        for remaining_state in remaining_state.iter() {
            state_rules.push((
                remaining_state,
                vec![None; Sp::DIRECTIONS.len()].into_boxed_slice(),
            ));
        }
        SetCollapseRules {
            state_rules: state_rules.into_boxed_slice(),
            observer: self.observer,
        }
    }
}

impl<O: SetCollapseObserver> SetCollapseRules<O> {
    pub fn state_count(&self) -> u32 {
        self.state_rules.len() as u32
    }

    pub fn collapse(&self, cell: &mut StateSet, neighbors: &[Option<StateSet>]) {
        for (state, allowed_neighbors) in &self.state_rules[..] {
            if cell.has(*state) {
                for i in 0..neighbors.len() {
                    if let Some(neighbor_state) = &neighbors[i] {
                        let allow = if let Some(allowed_state) = &allowed_neighbors[i] {
                            neighbor_state.has_any(allowed_state)
                        } else {
                            false
                        };
                        if !allow {
                            cell.remove(*state)
                        }
                    } else if allowed_neighbors[i].is_some() {
                        //cell.remove(*state);
                    }
                }
            }
        }
    }

    pub fn observe(&self, cell: &mut StateSet, neighbors: &[Option<StateSet>], rng: &mut impl Rng) {
        self.observer.observe(cell, neighbors, rng);
    }

    pub fn observer(&self) -> &O {
        &self.observer
    }
}
