use bit_vec::BitVec;
use std::ops::{BitAnd, BitOr, BitXor};
use std::sync::atomic::{AtomicU32, Ordering};

/// A state type which uses bits of a u64 to describe up to 64 separate possible final states.
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct StateSet(BitVec);

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct State(pub(crate) u32);

impl State {
    /// Creates the `n`th unique state
    pub fn state(n: u32) -> Self {
        Self(n)
    }
}

thread_local! {
    static STATE_COUNT: AtomicU32 = const { AtomicU32::new(u32::MAX) };
}

fn state_count() -> u32 {
    STATE_COUNT.with(|count| {
        let loaded = count.load(Ordering::Relaxed);
        debug_assert_ne!(
            loaded,
            u32::MAX,
            "all StateSet's must be constructed within StateSet::scope"
        );
        loaded
    })
}

impl StateSet {
    /// All [`StateSet`]'s created in `scope` will have `state_count` states.
    pub fn scope<R>(state_count: u32, scope: impl FnOnce() -> R) -> R {
        STATE_COUNT.with(|count| {
            #[cfg(debug_assertions)]
            let old = count.load(Ordering::Relaxed);
            count.store(state_count, Ordering::Relaxed);
            let ret = scope();
            #[cfg(debug_assertions)]
            count.store(old, Ordering::Relaxed);
            ret
        })
    }

    /// Creates a state representing the states numbered by members of `states`
    pub fn with_states(states: &[State]) -> Self {
        let mut ret = BitVec::from_elem(state_count() as usize, false);
        for i in states {
            ret.set(i.0 as usize, true);
        }
        Self(ret)
    }

    pub fn len() -> u32 {
        state_count()
    }

    pub fn all() -> Self {
        Self(BitVec::from_elem(state_count() as usize, true))
    }

    pub fn entropy(&self) -> u32 {
        (self.0.count_ones() as u32).saturating_sub(1)
    }

    pub fn has(&self, state: State) -> bool {
        self.0.get(state.0 as usize).unwrap()
    }

    pub fn has_any_of(&self, states: &Self) -> bool {
        for (state, present) in states.0.iter().enumerate() {
            if present && self.0.get(state).unwrap() {
                return true;
            }
        }
        false
    }

    pub fn remove(&mut self, state: State) {
        self.0.set(state.0 as usize, false);
    }

    pub fn clear_states(&mut self, states: &Self) {
        for (state, present) in states.0.iter().enumerate() {
            if present {
                self.0.set(state, false);
            }
        }
    }

    pub fn add(&mut self, state: State) {
        self.0.set(state.0 as usize, true);
    }

    pub fn set_states(&mut self, states: &Self) {
        self.0.or(&states.0);
    }

    pub fn collect_final_states(&self, states: &mut Vec<State>) {
        for (state, present) in self.0.iter().enumerate() {
            if present {
                states.push(State::state(state as u32));
            }
        }
    }
}

impl BitOr<Self> for StateSet {
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

impl BitOr for State {
    type Output = StateSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        StateSet::with_states(&[self, rhs])
    }
}

impl BitOr<State> for StateSet {
    type Output = Self;

    fn bitor(mut self, rhs: State) -> Self::Output {
        self.add(rhs);
        self
    }
}
