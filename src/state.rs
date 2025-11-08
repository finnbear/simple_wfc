/// Cell state - represents all possible states a cell can take on
pub trait State: Clone + PartialEq {
    fn all() -> Self;
    /// Adds the states represented in `states` to `self`
    fn set_states(&mut self, states: &Self);
    /// Checks if there are any overlapping states between `self` and `states`
    fn has_any_of(&self, states: &Self) -> bool;
    /// Removes states from `self` that are present in `states`
    fn clear_states(&mut self, states: &Self);
    /// Separates out all the final (0-entropy) states from this state into a Vec
    fn collect_final_states(&self, states: &mut Vec<Self>);

    /// Gets the entropy value of this state. Zero means that the state is
    /// final, and cannot be collapsed further, while higher values mean there
    /// are more possible values this state could collapse to.
    fn entropy(&self) -> u32;
}
