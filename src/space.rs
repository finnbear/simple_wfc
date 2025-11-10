use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Index, IndexMut},
};

/// Represents coordinate deltas which have an inverse - the delta which undoes
/// the change represented by this delta.
///
/// On a 2d grid, the inverse of `(2, -1)` would be `(-2, 1)`
pub trait InvertDelta {
    fn invert_delta(&self) -> Self;
}

/// Defines the space or "world" to run WFC on.
///
/// This is the primary data structure behind WFC, and is modified in-place by
/// the algorithm. The only expectation placed on Space is that it's size and
/// shape do not change during calls to `crate::collapse`.
///
/// In order to support arbitrary dimension and shape, two associated types are
/// defined:
/// - `Coordinate` is the index type for this space. Cells in the space are
/// uniquely identified by coordinates.
/// - `CoordinateDelta` represents adjacency relations between cells. In
/// general, a collapse rule supplies a list of coordinate deltas to get
/// neighbor cell coordinates.
pub trait Space<T>: IndexMut<Self::Coordinate, Output = T> + 'static {
    /// Coordinates for cells in the space
    type Coordinate: Default + Copy + Hash + Ord + Index<Self::Axis, Output = u32>;
    /// Spatial relationship between cells for accessing neighbors
    type Direction: Copy + Debug + Eq + InvertDelta + 'static;
    type Axis: Copy + Debug + Eq + 'static;

    const DIRECTIONS: &'static [Self::Direction];

    /// The grid will occupy `(0,0,0)..dimensions`.
    fn new(dimensions: Self::Coordinate, init_fn: impl FnMut(Self::Coordinate) -> T) -> Self;

    fn dimensions(&self) -> Self::Coordinate;

    fn map(
        coordinate: Self::Coordinate,
        map_fn: impl Fn(Self::Axis, u32) -> u32,
    ) -> Self::Coordinate;

    /// Computes `start + add - sub`, returning `Some` if the result is in the space.
    fn add_sub(
        &self,
        start: Self::Coordinate,
        add: Self::Coordinate,
        sub: Self::Coordinate,
    ) -> Option<Self::Coordinate>;

    /// Get every valid coordinate in the space.
    fn visit_coordinates(&self, visitor: impl FnMut(Self::Coordinate));
    /// Get the neighbor coordinates of a given cell based on a direction.
    fn neighbor(
        &self,
        coord: Self::Coordinate,
        direction: Self::Direction,
    ) -> Option<Self::Coordinate>;
}
