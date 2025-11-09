use rand::{thread_rng, Rng};
use wfc::collapse;
use wfc::grid_2d::Grid2d;
use wfc::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PossibleStates {
    AB,
    A,
    B,
}

impl State for PossibleStates {
    fn entropy(&self) -> u32 {
        match self {
            &PossibleStates::AB => 1,
            _ => 0,
        }
    }
}

type TestGrid = Grid2d<PossibleStates>;

struct Rule;

impl CollapseRule<PossibleStates, TestGrid> for Rule {
    fn neighbor_offsets(&self) -> Box<[<TestGrid as Space<PossibleStates>>::CoordinateDelta]> {
        vec![(-1, 0), (1, 0)].into_boxed_slice()
    }

    fn collapse(&self, cell: &mut PossibleStates, neighbors: &[Option<PossibleStates>]) {
        if *cell == PossibleStates::AB {
            match (neighbors[0], neighbors[1]) {
                (Some(PossibleStates::A), _) => *cell = PossibleStates::B,
                (_, Some(PossibleStates::A)) => *cell = PossibleStates::B,
                (_, Some(PossibleStates::B)) => *cell = PossibleStates::A,
                (Some(PossibleStates::B), _) => *cell = PossibleStates::A,
                _ => {}
            }
        }
    }

    fn observe(&self, cell: &mut PossibleStates, _neighbors: &[Option<PossibleStates>]) {
        if let PossibleStates::AB = *cell {
            *cell = if thread_rng().gen::<bool>() {
                PossibleStates::A
            } else {
                PossibleStates::B
            };
        }
    }
}

#[test]
fn test_basic() {
    let mut grid = Grid2d::new(10, 10, |_, _| PossibleStates::AB);
    collapse(&mut grid, &Rule);
    for y in 0..10 {
        for x in 0..9 {
            assert_ne!(grid[(x, y)], PossibleStates::AB);
        }
        for x in 0..9 {
            assert_ne!(grid[(x, y)], grid[(x + 1, y)]);
        }
    }
}
