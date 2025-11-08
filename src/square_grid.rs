use crate::{state::StateSet, InvertDelta, Space};
use std::ops::{Index, IndexMut};

/// Basic square grid implementing `crate::Space`
///
/// coordinates and coordinate directions are specified as `(isize, isize)`.
pub struct SquareGrid<T> {
    cells: Box<[T]>,
    width: isize,
    height: isize,
}

impl InvertDelta for (isize, isize) {
    fn invert_delta(&self) -> Self {
        let (dx, dy) = *self;
        (-dx, -dy)
    }
}

impl<T> SquareGrid<T> {
    /// Create a new `SquareGrid`
    ///
    /// * `width` - width of the grid
    /// * `height` - height of the grid
    /// * `init_fn` - callback to set the initial state of each cell based on
    /// coordinate
    pub fn new(width: isize, height: isize, init_fn: impl Fn(isize, isize) -> T) -> Self {
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(init_fn(x, y));
            }
        }
        Self {
            cells: cells.into_boxed_slice(),
            width,
            height,
        }
    }
}

impl Index<<Self as Space>::Coordinate> for SquareGrid<StateSet> {
    type Output = StateSet;

    fn index(&self, index: <Self as Space>::Coordinate) -> &Self::Output {
        let (x, y) = index;
        &self.cells[(x + y * self.width) as usize]
    }
}

impl IndexMut<<Self as Space>::Coordinate> for SquareGrid<StateSet> {
    fn index_mut(&mut self, index: <Self as Space>::Coordinate) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.cells[(x + y * self.width) as usize]
    }
}

impl Space for SquareGrid<StateSet> {
    type Coordinate = (isize, isize);
    type CoordinateDelta = (isize, isize);

    fn visit_coordinates(&self, mut visitor: impl FnMut(Self::Coordinate)) {
        for y in 0..self.height {
            for x in 0..self.width {
                visitor((x, y));
            }
        }
    }

    fn neighbors(
        &self,
        coord: Self::Coordinate,
        neighbor_directions: &[Self::CoordinateDelta],
        neighbors: &mut [Option<Self::Coordinate>],
    ) {
        assert!(neighbor_directions.len() <= neighbors.len());

        let (x, y) = coord;
        for i in 0..neighbor_directions.len() {
            let (dx, dy) = neighbor_directions[i];
            let (nx, ny) = (x + dx, y + dy);
            if nx.clamp(0, self.width - 1) == nx && ny.clamp(0, self.height - 1) == ny {
                neighbors[i] = Some((nx, ny));
            } else {
                neighbors[i] = None;
            }
        }
    }
}
