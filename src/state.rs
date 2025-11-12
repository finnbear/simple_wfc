use bit_vec::BitVec;
use std::ops::{BitAnd, BitOr, BitXor};
use std::sync::atomic::{AtomicU32, Ordering};

type B = u64;

/// A superposition of multiple [State]'s.
///
/// You must use [Self::scope] to set the total number of states.
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct StateSet(BitVec<B>);

/// One possible state at a location.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct State(pub(crate) u32);

impl State {
    /// Creates the `n`th unique state (0-indexed).
    pub fn nth(n: u32) -> Self {
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

    /// Creates a superposition of `states`.
    pub fn with_states(states: &[State]) -> Self {
        let mut ret = BitVec::<B>::default();
        ret.grow(state_count() as usize, false);
        for state in states {
            ret.set(state.0 as usize, true);
        }
        Self(ret)
    }

    /// The total number of states, as defined by [Self::scope].
    #[inline(always)]
    pub fn len() -> u32 {
        state_count()
    }

    /// Superposition of all states.
    pub fn all() -> Self {
        let mut ret = BitVec::<B>::default();
        ret.grow(state_count() as usize, true);
        Self(ret)
    }

    /// Total number of possible states, minus 1.
    #[inline(always)]
    pub fn entropy(&self) -> u32 {
        (self.0.count_ones() as u32).saturating_sub(1)
    }

    /// Is `state` within the superposition?
    #[inline(always)]
    pub fn has(&self, state: State) -> bool {
        self.0.get(state.0 as usize).unwrap()
    }

    /// Are any of `states` within the superposition?
    #[inline(always)]
    pub fn has_any(&self, states: &Self) -> bool {
        self.0
            .blocks()
            .zip(states.0.blocks())
            .any(|(a, b)| a & b != 0)
    }

    /// Remove `state` from the superposition.
    #[inline(always)]
    pub fn remove(&mut self, state: State) {
        self.0.set(state.0 as usize, false);
    }

    /// Remove all `states` from the superposition.
    pub fn remove_all(&mut self, states: &Self) {
        for (state, present) in states.0.iter().enumerate() {
            if present {
                self.0.set(state, false);
            }
        }
    }

    /// Add `state` to the superposition.
    #[inline(always)]
    pub fn add(&mut self, state: State) {
        self.0.set(state.0 as usize, true);
    }

    /// Add all `states` to the superposition.
    pub fn add_all(&mut self, states: &Self) {
        self.0.or(&states.0);
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = State> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, p)| *p)
            .map(|(i, _)| State::nth(i as u32))
    }

    /// Filter states in place.
    pub fn retain(&mut self, mut filter: impl FnMut(State) -> bool) {
        for s in 0..StateSet::len() {
            let s = State::nth(s);
            if self.has(s) && !filter(s) {
                self.remove(s);
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
