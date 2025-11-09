use crate::{
    state::{State, StateSet},
    InvertDelta, Space,
};
use rand::{thread_rng, Rng};

pub trait SetCollapseObserver {
    fn observe(&self, cell: &mut StateSet, neighbors: &[Option<StateSet>]);
}

#[derive(Clone)]
pub struct UniformSetCollapseObserver;

impl SetCollapseObserver for UniformSetCollapseObserver {
    fn observe(&self, cell: &mut StateSet, _: &[Option<StateSet>]) {
        let mut final_states = Vec::new();
        cell.collect_final_states(&mut final_states);
        *cell =
            StateSet::with_states(&[final_states[thread_rng().gen_range(0..final_states.len())]]);
    }
}

pub struct SetCollapseRule<Sp: Space, O: SetCollapseObserver> {
    neighbor_offsets: Box<[Sp::CoordinateDelta]>,
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

/// builder for [SetCollapseRule]
///
/// Automatically collects used coordinate deltas and manages creating symmetric rules from asymmetric definitions
pub struct SetCollapseRuleBuilder<Sp: Space, O: SetCollapseObserver + Clone> {
    neighbor_offsets: Vec<Sp::CoordinateDelta>,
    state_rules: Vec<StateRule>,
    observer: O,
}

impl<Sp: Space, O: SetCollapseObserver + Clone> SetCollapseRuleBuilder<Sp, O>
where
    Sp::CoordinateDelta: Eq + Clone + InvertDelta,
{
    pub fn new(observer: O) -> Self {
        Self {
            neighbor_offsets: Vec::new(),
            state_rules: Vec::new(),
            observer,
        }
    }

    /// Set the allowed neighbors for a cell based on their coordinate deltas
    ///
    /// This will create symmetric rules. For example, if you set state A to be
    /// allowed to the left of B, then state B will be allowed to the right of
    /// A - you don't have to explicitly set both rules.
    ///
    /// States which do not have any allowed neighbors for a given coordinate
    /// delta will equire that those coordinates are outside of world-space.
    pub fn allow(mut self, state: State, neighbors: &[(Sp::CoordinateDelta, StateSet)]) -> Self {
        for (delta, neighbor) in neighbors {
            let mut neighbor_states = Vec::new();
            neighbor.collect_final_states(&mut neighbor_states);
            for n_state in neighbor_states {
                self.allow_symmetric(state, n_state, delta);
            }
        }
        self
    }

    fn allow_symmetric(&mut self, a: State, b: State, offset: &Sp::CoordinateDelta) {
        let offset_index = self.get_offset_index(offset.clone());
        self.get_rule(a).add_allowed(offset_index, b);
        let offset_index = self.get_offset_index(offset.invert_delta());
        self.get_rule(b).add_allowed(offset_index, a);
    }

    fn get_offset_index(&mut self, offset: Sp::CoordinateDelta) -> usize {
        for i in 0..self.neighbor_offsets.len() {
            if self.neighbor_offsets[i] == offset {
                return i;
            }
        }
        let i = self.neighbor_offsets.len();
        self.neighbor_offsets.push(offset);
        i
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

    pub fn build(self) -> SetCollapseRule<Sp, O> {
        let mut state_rules = Vec::new();
        let mut remaining_state = StateSet::all();
        for mut proto_rule in self.state_rules {
            while proto_rule.allowed_neighbors.len() < self.neighbor_offsets.len() {
                proto_rule.allowed_neighbors.push(None);
            }
            remaining_state.remove(proto_rule.state);
            state_rules.push((
                proto_rule.state,
                proto_rule.allowed_neighbors.into_boxed_slice(),
            ));
        }
        let mut remaining_states = Vec::new();
        remaining_state.collect_final_states(&mut remaining_states);
        for remaining_state in remaining_states {
            state_rules.push((
                remaining_state,
                vec![None; self.neighbor_offsets.len()].into_boxed_slice(),
            ));
        }
        SetCollapseRule {
            neighbor_offsets: self.neighbor_offsets.into_boxed_slice(),
            state_rules: state_rules.into_boxed_slice(),
            observer: self.observer,
        }
    }
}

impl<Sp: Space, O: SetCollapseObserver> SetCollapseRule<Sp, O>
where
    Sp::CoordinateDelta: Clone,
{
    pub fn neighbor_offsets(&self) -> Box<[<Sp as Space>::CoordinateDelta]> {
        self.neighbor_offsets.clone()
    }

    pub fn collapse(&self, cell: &mut StateSet, neighbors: &[Option<StateSet>]) {
        for (state, allowed_neighbors) in &self.state_rules[..] {
            if cell.has(*state) {
                for i in 0..neighbors.len() {
                    if let Some(neighbor_state) = &neighbors[i] {
                        let allow = if let Some(allowed_state) = &allowed_neighbors[i] {
                            neighbor_state.has_any_of(allowed_state)
                        } else {
                            false
                        };
                        if !allow {
                            cell.remove(*state)
                        }
                    }
                }
            }
        }
    }

    pub fn observe(&self, cell: &mut StateSet, neighbors: &[Option<StateSet>]) {
        self.observer.observe(cell, neighbors);
    }
}
