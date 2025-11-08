use wfc::square_grid::SquareGrid;
use wfc::state_set::StateSet;
use wfc::{collapse, set_rule::*, AllState};

type S = StateSet<3>;

const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);
const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);

const A: S = S::state(0);
const B: S = S::state(1);
const C: S = S::state(2);

fn to_char(s: &S) -> char {
    match *s {
        A => 'A',
        B => 'B',
        C => 'C',
        _ => '?',
    }
}

fn main() {
    let rule = SetCollapseRuleBuilder::new(UniformSetCollapseObserver)
        .allow(&A, &[(LEFT, B | C), (RIGHT, B | C), (UP, A)])
        .allow(&B, &[(UP, C), (DOWN, C)])
        .build();

    let mut space = SquareGrid::new(20, 10, |_, _| S::all());
    collapse(&mut space, &rule);
    for y in 0..10 {
        for x in 0..20 {
            print!("{} ", to_char(&space[(x, y)]));
        }
        println!();
    }
    println!();
}
