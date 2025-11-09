use crate::{state::StateSet, InvertDelta, Space};
use std::ops::{Index, IndexMut};

/// Basic square grid implementing [`crate::Space`]
///
/// Coordinates are specified as [`Coordinate2d`].
pub struct Grid2d<T> {
    cells: Box<[T]>,
    width: u32,
    height: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coordinate2d {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction2d {
    Right,
    Up,
    Left,
    Down,
}

impl Direction2d {
    fn offset(self) -> (i32, i32) {
        match self {
            Self::Right => (1, 0),
            Self::Up => (0, 1),
            Self::Left => (-1, 0),
            Self::Down => (0, -1),
        }
    }
}

impl InvertDelta for Direction2d {
    fn invert_delta(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
        }
    }
}

impl<T> Grid2d<T> {
    /// Create a new `Grid2d`
    ///
    /// * `width` - width of the grid
    /// * `height` - height of the grid
    /// * `init_fn` - callback to set the initial state of each cell based on
    /// coordinate
    pub fn new(width: u32, height: u32, init_fn: impl Fn(Coordinate2d) -> T) -> Self {
        let mut cells = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                cells.push(init_fn(Coordinate2d { x, y }));
            }
        }
        Self {
            cells: cells.into_boxed_slice(),
            width,
            height,
        }
    }
}

impl Index<<Self as Space>::Coordinate> for Grid2d<StateSet> {
    type Output = StateSet;

    fn index(&self, index: <Self as Space>::Coordinate) -> &Self::Output {
        let Coordinate2d { x, y } = index;
        &self.cells[(x + y * self.width) as usize]
    }
}

impl IndexMut<<Self as Space>::Coordinate> for Grid2d<StateSet> {
    fn index_mut(&mut self, index: <Self as Space>::Coordinate) -> &mut Self::Output {
        let Coordinate2d { x, y } = index;
        &mut self.cells[(x + y * self.width) as usize]
    }
}

impl Space for Grid2d<StateSet> {
    type Coordinate = Coordinate2d;
    type Direction = Direction2d;

    const DIRECTIONS: &'static [Self::Direction] = &[
        Direction2d::Right,
        Direction2d::Up,
        Direction2d::Left,
        Direction2d::Down,
    ];

    fn visit_coordinates(&self, mut visitor: impl FnMut(Self::Coordinate)) {
        for y in 0..self.height {
            for x in 0..self.width {
                visitor(Coordinate2d { x, y });
            }
        }
    }

    fn neighbor(
        &self,
        coord: Self::Coordinate,
        direction: Self::Direction,
    ) -> Option<Self::Coordinate> {
        let Coordinate2d { x, y } = coord;
        let (dx, dy) = direction.offset();
        if (x == 0 && dx == -1) || (y == 0 && dy == -1) {
            None
        } else if (x == self.width - 1 && dx == 1) || (y == self.height - 1 && dy == 1) {
            None
        } else {
            Some(Coordinate2d {
                x: x.wrapping_add_signed(dx),
                y: y.wrapping_add_signed(dy),
            })
        }
    }
}
