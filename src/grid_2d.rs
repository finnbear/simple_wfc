use crate::{InvertDelta, Space};
use std::ops::{Index, IndexMut};

/// Basic square grid implementing [`crate::Space`]
///
/// Coordinates are specified as [`Coordinate2d`].
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Grid2d<T> {
    cells: Box<[T]>,
    dimensions: Coordinate2d,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Axis2d {
    X,
    Y,
}

impl Index<Axis2d> for Coordinate2d {
    type Output = u32;

    fn index(&self, index: Axis2d) -> &Self::Output {
        match index {
            Axis2d::X => &self.x,
            Axis2d::Y => &self.y,
        }
    }
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

impl<T> Index<Coordinate2d> for Grid2d<T> {
    type Output = T;

    fn index(&self, index: Coordinate2d) -> &Self::Output {
        let Coordinate2d { x, y } = index;
        &self.cells[(x + y * self.dimensions.x) as usize]
    }
}

impl<T> IndexMut<Coordinate2d> for Grid2d<T> {
    fn index_mut(&mut self, index: Coordinate2d) -> &mut Self::Output {
        let Coordinate2d { x, y } = index;
        &mut self.cells[(x + y * self.dimensions.x) as usize]
    }
}

impl<T: 'static> Space<T> for Grid2d<T> {
    type Coordinate = Coordinate2d;
    type Direction = Direction2d;
    type Axis = Axis2d;
    type RotationAxis = ();

    const DIRECTIONS: &'static [Self::Direction] = &[
        Direction2d::Right,
        Direction2d::Up,
        Direction2d::Left,
        Direction2d::Down,
    ];

    /// Create a new `Grid2d`
    fn new(dimensions: Coordinate2d, mut init_fn: impl FnMut(Coordinate2d) -> T) -> Self {
        let mut cells = Vec::with_capacity((dimensions.x * dimensions.y) as usize);
        for y in 0..dimensions.y {
            for x in 0..dimensions.x {
                cells.push(init_fn(Coordinate2d { x, y }));
            }
        }
        Self {
            cells: cells.into_boxed_slice(),
            dimensions,
        }
    }

    fn dimensions(&self) -> Self::Coordinate {
        self.dimensions
    }

    fn map(
        coordinate: Self::Coordinate,
        map_fn: impl Fn(Self::Axis, u32) -> u32,
    ) -> Self::Coordinate {
        Coordinate2d {
            x: map_fn(Axis2d::X, coordinate.x),
            y: map_fn(Axis2d::Y, coordinate.y),
        }
    }

    fn perp(&self, coordinate: Self::Coordinate, _: Self::RotationAxis) -> Self::Coordinate {
        assert_eq!(self.dimensions.x, self.dimensions.y);
        Coordinate2d {
            x: self.dimensions.y - 1 - coordinate.y,
            y: coordinate.x,
        }
    }

    fn add_sub(
        &self,
        start: Self::Coordinate,
        add: Self::Coordinate,
        sub: Self::Coordinate,
    ) -> Option<Self::Coordinate> {
        let x = start.x.checked_add_signed(add.x as i32 - sub.x as i32)?;
        let y = start.y.checked_add_signed(add.y as i32 - sub.y as i32)?;
        if x >= self.dimensions.x || y >= self.dimensions.y {
            return None;
        }
        Some(Coordinate2d { x, y })
    }

    fn visit_coordinates(dimensions: Self::Coordinate, mut visitor: impl FnMut(Self::Coordinate)) {
        for y in 0..dimensions.y {
            for x in 0..dimensions.x {
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
        } else if (x == self.dimensions.x - 1 && dx == 1) || (y == self.dimensions.y - 1 && dy == 1)
        {
            None
        } else {
            Some(Coordinate2d {
                x: x.wrapping_add_signed(dx),
                y: y.wrapping_add_signed(dy),
            })
        }
    }
}
