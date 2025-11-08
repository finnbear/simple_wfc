use bit_vec::BitVec;
use std::ops::{BitAnd, BitOr, BitXor};

/// A state type which uses bits of a u64 to describe up to 64 separate possible final states.
///
/// * `FINAL_STATE_COUNT` - the total number of final (fully collapsed) states
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct StateSet(BitVec);

const FINAL_STATE_COUNT: u32 = 11;

impl StateSet {
    /// Creates the `n`th unique state
    pub fn state(n: u32) -> Self {
        let mut ret = BitVec::from_elem(FINAL_STATE_COUNT as usize, false);
        ret.set(n as usize, true);
        Self(ret)
    }

    /// Creates a state representing the states numbered by members of `states`
    pub fn with_states(states: &[u32]) -> Self {
        let mut ret = BitVec::from_elem(FINAL_STATE_COUNT as usize, false);
        for i in states {
            ret.set(*i as usize, true);
        }
        Self(ret)
    }

    pub fn all() -> Self {
        Self(BitVec::from_elem(FINAL_STATE_COUNT as usize, true))
    }

    pub fn entropy(&self) -> u32 {
        self.0.count_ones() as u32 - 1
    }

    pub fn has_any_of(&self, states: &Self) -> bool {
        for (state, present) in states.0.iter().enumerate() {
            if present && self.0.get(state).unwrap() {
                return true;
            }
        }
        false
    }

    pub fn clear_states(&mut self, states: &Self) {
        for (state, present) in states.0.iter().enumerate() {
            if present {
                self.0.set(state, false);
            }
        }
    }

    pub fn set_states(&mut self, states: &Self) {
        self.0.or(&states.0);
    }

    pub fn collect_final_states(&self, states: &mut Vec<Self>) {
        for (state, present) in self.0.iter().enumerate() {
            if present {
                states.push(Self::state(state as u32));
            }
        }
    }
}

impl BitOr for StateSet {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.0.or(&rhs.0);
        self
    }
}

impl BitAnd for StateSet {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.0.and(&rhs.0);
        self
    }
}

impl BitXor for StateSet {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self.0.xor(&rhs.0);
        self
    }
}
