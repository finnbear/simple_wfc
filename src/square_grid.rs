use crate::{state::StateSet, InvertDelta, Space};
use std::ops::{Index, IndexMut};

/// Basic square grid implementing `crate::Space`
///
/// coordinates and coordinate directions are specified as `(isize, isize)`.
pub struct SquareGrid<T> {
    cells: Box<[T]>,
    width: u32,
    height: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Delta2d {
    Right,
    Up,
    Left,
    Down,
}

impl Delta2d {
    pub fn offset(self) -> (i32, i32) {
        match self {
            Self::Right => (1, 0),
            Self::Up => (0, 1),
            Self::Left => (-1, 0),
            Self::Down => (0, -1),
        }
    }
}

impl InvertDelta for Delta2d {
    fn invert_delta(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
        }
    }
}

impl<T> SquareGrid<T> {
    /// Create a new `SquareGrid`
    ///
    /// * `width` - width of the grid
    /// * `height` - height of the grid
    /// * `init_fn` - callback to set the initial state of each cell based on
    /// coordinate
    pub fn new(width: u32, height: u32, init_fn: impl Fn(u32, u32) -> T) -> Self {
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
    type Coordinate = (u32, u32);
    type CoordinateDelta = Delta2d;

    const NEIGHBORS: &'static [Self::CoordinateDelta] =
        &[Delta2d::Right, Delta2d::Up, Delta2d::Left, Delta2d::Down];

    fn visit_coordinates(&self, mut visitor: impl FnMut(Self::Coordinate)) {
        for y in 0..self.height {
            for x in 0..self.width {
                visitor((x, y));
            }
        }
    }

    fn neighbors(&self, coord: Self::Coordinate, neighbors: &mut [Option<Self::Coordinate>]) {
        assert!(Self::NEIGHBORS.len() <= neighbors.len());

        let (x, y) = coord;
        for i in 0..Self::NEIGHBORS.len() {
            let (dx, dy) = Self::NEIGHBORS[i].offset();
            neighbors[i] = if (x == 0 && dx == -1) || (y == 0 && dy == -1) {
                None
            } else if (x == self.width - 1 && dx == 1) || (y == self.height - 1 && dy == 1) {
                None
            } else {
                Some((x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
            };
        }
    }
}
