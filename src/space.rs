use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Index, IndexMut, Neg},
};

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
    type Direction: Copy + Debug + Eq + Neg<Output = Self::Direction> + 'static;
    /// Axes of translation/flipping.
    type Axis: Copy + Debug + Eq + 'static;
    /// Axes of rotation.
    type RotationAxis: Copy + Debug + Eq + 'static;

    const DIRECTIONS: &'static [Self::Direction];

    /// The grid will occupy `(0,0,0)..dimensions`.
    fn new(dimensions: Self::Coordinate, init_fn: impl FnMut(Self::Coordinate) -> T) -> Self;

    /// The dimensions passed to [Space::new] during construction.
    fn dimensions(&self) -> Self::Coordinate;

    /// Apply `map_fn` to each component of `coordinate`.
    fn map(
        coordinate: Self::Coordinate,
        map_fn: impl Fn(Self::Axis, u32) -> u32,
    ) -> Self::Coordinate;

    /// 90 degree rotation of `coordinate` around `axis` as if by looking down that axis
    /// in a left-handed coordinate system and rotating counter-clockwise.
    fn perp(&self, coordinate: Self::Coordinate, axis: Self::RotationAxis) -> Self::Coordinate;

    /// Computes `start + add - sub`, returning `Some` if the result is in the space.
    fn add_sub(
        &self,
        start: Self::Coordinate,
        add: Self::Coordinate,
        sub: Self::Coordinate,
    ) -> Option<Self::Coordinate>;

    /// Get every valid coordinate in the space.
    fn visit_coordinates(dimensions: Self::Coordinate, visitor: impl FnMut(Self::Coordinate));
    /// Get the neighbor coordinates of a given cell based on a direction.
    fn neighbor(
        &self,
        coord: Self::Coordinate,
        direction: Self::Direction,
    ) -> Option<Self::Coordinate>;
}
