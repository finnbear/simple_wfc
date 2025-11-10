use crate::{InvertDelta, Space};
use std::ops::{Index, IndexMut};

/// Basic square grid implementing [`crate::Space`]
///
/// Coordinates are specified as [`Coordinate2d`].
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Grid3d<T> {
    cells: Box<[T]>,
    dimensions: Coordinate3d,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl<T> Index<Coordinate3d> for Grid3d<T> {
    type Output = T;

    fn index(&self, index: Coordinate3d) -> &Self::Output {
        let Coordinate3d { x, y, z } = index;
        &self.cells
            [(x + y * self.dimensions.x + z * self.dimensions.x * self.dimensions.y) as usize]
    }
}

impl<T> IndexMut<Coordinate3d> for Grid3d<T> {
    fn index_mut(&mut self, index: Coordinate3d) -> &mut Self::Output {
        let Coordinate3d { x, y, z } = index;
        &mut self.cells
            [(x + y * self.dimensions.x + z * self.dimensions.x * self.dimensions.y) as usize]
    }
}

impl<T: 'static> Space<T> for Grid3d<T> {
    type Coordinate = Coordinate3d;
    type Direction = Direction3d;

    const DIRECTIONS: &'static [Self::Direction] = &[
        Direction3d::PosX,
        Direction3d::NegX,
        Direction3d::PosY,
        Direction3d::NegY,
        Direction3d::PosZ,
        Direction3d::NegZ,
    ];

    /// Create a new `Grid3d`
    fn new(dimensions: Coordinate3d, init_fn: impl Fn(Coordinate3d) -> T) -> Self {
        let mut cells = Vec::with_capacity((dimensions.x * dimensions.y * dimensions.y) as usize);
        for z in 0..dimensions.z {
            for y in 0..dimensions.y {
                for x in 0..dimensions.x {
                    cells.push(init_fn(Coordinate3d { x, y, z }));
                }
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

    fn map(coordinate: Self::Coordinate, map_fn: impl Fn(u32) -> u32) -> Self::Coordinate {
        Coordinate3d {
            x: map_fn(coordinate.x),
            y: map_fn(coordinate.y),
            z: map_fn(coordinate.z),
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
        let z = start.z.checked_add_signed(add.z as i32 - sub.z as i32)?;
        if x >= self.dimensions.x - 1 || y >= self.dimensions.y - 1 || z >= self.dimensions.z - 1 {
            return None;
        }
        Some(Coordinate3d { x, y, z })
    }

    fn visit_coordinates(&self, mut visitor: impl FnMut(Self::Coordinate)) {
        for z in 0..self.dimensions.z {
            for y in 0..self.dimensions.y {
                for x in 0..self.dimensions.x {
                    visitor(Coordinate3d { x, y, z });
                }
            }
        }
    }

    fn neighbor(
        &self,
        coord: Self::Coordinate,
        direction: Self::Direction,
    ) -> Option<Self::Coordinate> {
        let Coordinate3d { x, y, z } = coord;
        let (dx, dy, dz) = direction.offset();
        if (x == 0 && dx == -1) || (y == 0 && dy == -1) || (z == 0 && dz == -1) {
            None
        } else if (x == self.dimensions.x - 1 && dx == 1)
            || (y == self.dimensions.y - 1 && dy == 1)
            || (z == self.dimensions.z - 1 && dz == 1)
        {
            None
        } else {
            Some(Coordinate3d {
                x: x.wrapping_add_signed(dx),
                y: y.wrapping_add_signed(dy),
                z: z.wrapping_add_signed(dz),
            })
        }
    }
}
