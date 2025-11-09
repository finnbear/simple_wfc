use crate::{state::StateSet, InvertDelta, Space};
use std::ops::{Index, IndexMut};

/// Basic square grid implementing [`crate::Space`]
///
/// Coordinates are specified as [`Coordinate2d`].
pub struct Grid3d<T> {
    cells: Box<[T]>,
    width: u32,
    height: u32,
    depth: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coordinate3d {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction3d {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl Direction3d {
    fn offset(self) -> (i32, i32, i32) {
        match self {
            Self::PosX => (1, 0, 0),
            Self::NegX => (-1, 0, 0),
            Self::PosY => (0, 1, 0),
            Self::NegY => (0, -1, 0),
            Self::PosZ => (0, 0, 1),
            Self::NegZ => (0, 0, -1),
        }
    }
}

impl InvertDelta for Direction3d {
    fn invert_delta(&self) -> Self {
        match self {
            Self::PosX => Self::NegX,
            Self::NegX => Self::PosX,
            Self::PosY => Self::NegY,
            Self::NegY => Self::PosY,
            Self::PosZ => Self::NegZ,
            Self::NegZ => Self::PosZ,
        }
    }
}

impl<T> Grid3d<T> {
    /// Create a new `Grid3d`
    pub fn new(width: u32, height: u32, depth: u32, init_fn: impl Fn(Coordinate3d) -> T) -> Self {
        let mut cells = Vec::with_capacity((width * height) as usize);
        for z in 0..depth {
            for y in 0..height {
                for x in 0..width {
                    cells.push(init_fn(Coordinate3d { x, y, z }));
                }
            }
        }
        Self {
            cells: cells.into_boxed_slice(),
            width,
            height,
            depth,
        }
    }
}

impl Index<<Self as Space>::Coordinate> for Grid3d<StateSet> {
    type Output = StateSet;

    fn index(&self, index: <Self as Space>::Coordinate) -> &Self::Output {
        let Coordinate3d { x, y, z } = index;
        &self.cells[(x + y * self.width + z * self.width * self.width) as usize]
    }
}

impl IndexMut<<Self as Space>::Coordinate> for Grid3d<StateSet> {
    fn index_mut(&mut self, index: <Self as Space>::Coordinate) -> &mut Self::Output {
        let Coordinate3d { x, y, z } = index;
        &mut self.cells[(x + y * self.width + z * self.width * self.width) as usize]
    }
}

impl Space for Grid3d<StateSet> {
    type Coordinate = Coordinate3d;
    type CoordinateDelta = Direction3d;

    const DIRECTIONS: &'static [Self::CoordinateDelta] = &[
        Direction3d::PosX,
        Direction3d::NegX,
        Direction3d::PosY,
        Direction3d::NegY,
        Direction3d::PosZ,
        Direction3d::NegZ,
    ];

    fn visit_coordinates(&self, mut visitor: impl FnMut(Self::Coordinate)) {
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    visitor(Coordinate3d { x, y, z });
                }
            }
        }
    }

    fn neighbors(&self, coord: Self::Coordinate, neighbors: &mut [Option<Self::Coordinate>]) {
        assert!(Self::DIRECTIONS.len() <= neighbors.len());

        let Coordinate3d { x, y, z } = coord;
        for i in 0..Self::DIRECTIONS.len() {
            let (dx, dy, dz) = Self::DIRECTIONS[i].offset();
            neighbors[i] = if (x == 0 && dx == -1) || (y == 0 && dy == -1) || (z == 0 && dz == -1) {
                None
            } else if (x == self.width - 1 && dx == 1)
                || (y == self.height - 1 && dy == 1)
                || (z == self.depth - 1 && dz == 1)
            {
                None
            } else {
                Some(Coordinate3d {
                    x: x.wrapping_add_signed(dx),
                    y: y.wrapping_add_signed(dy),
                    z: z.wrapping_add_signed(dz),
                })
            };
        }
    }
}
