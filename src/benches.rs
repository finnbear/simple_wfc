extern crate test;
use crate::{
    collapse,
    grid_2d::{Axis2d, Coordinate2d, Grid2d},
    overlapping::{codify_patterns, Tile},
    Space, StateSet,
};
use rand::thread_rng;
use test::{black_box, Bencher};

// Nov 10 2025
// test benches::wfc_3x3_2d ... bench: 142,352,941.90 ns/iter (+/- 42,112,330.60)

// Nov 11 2025
// test benches::wfc_3x3_2d ... bench:  20,555,707.70 ns/iter (+/- 5,157,622.76)
// test benches::wfc_3x3_2d ... bench:  21,666,191.80 ns/iter (+/- 6,012,055.63)
// test benches::wfc_3x3_2d ... bench:  17,395,783.80 ns/iter (+/- 5,049,007.16)
// test benches::wfc_3x3_2d ... bench:  16,151,594.20 ns/iter (+/- 4,025,016.22)
// test benches::wfc_3x3_2d ... bench:  17,343,374.10 ns/iter (+/- 4,266,848.80) - fn
// test benches::wfc_3x3_2d ... bench:  12,905,514.40 ns/iter (+/- 3,673,940.29) - dup propagate

#[bench]
fn wfc_3x3_2d(b: &mut Bencher) {
    let input = parse_grid(
        r#"
____________________
__+---+__+---+______
__|+++|__|***|______
__|+++|__|***+---+__
__|+++|__|*******|__
__|+++|__|*******|__
__|+++|__|*******|__
__+---+__+-------+__
____________________
 "#,
    );

    let rule = codify_patterns::<_, _, Grid2d<StateSet>>(
        &input,
        Coordinate2d { x: 3, y: 3 },
        &[Axis2d::X, Axis2d::Y],
        Some(()),
    );

    StateSet::scope(rule.state_count(), || {
        b.iter(move || {
            let mut space = Grid2d::new(Coordinate2d { x: 20, y: 20 }, |_| StateSet::all());

            collapse(
                black_box(&mut space),
                black_box(&rule),
                &mut thread_rng(),
                |_| {},
            );
            black_box(rule.observer().decode_superposition::<Grid2d<_>, _>(&space));
        })
    });
}

fn parse_grid(s: &str) -> Grid2d<Option<CharTile>> {
    let lines = s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>();

    Grid2d::new(
        Coordinate2d {
            x: lines.iter().map(|x| x.len()).max().unwrap_or(0) as u32,
            y: lines.len() as u32,
        },
        |coordinate| {
            Some(CharTile {
                b: lines[coordinate.y as usize]
                    .as_bytes()
                    .get(coordinate.x as usize)
                    .copied()
                    .map(|b| if matches!(b, b' ' | b'_') { 0 } else { b })
                    .unwrap_or(0),
            })
            .filter(|c| c.b != 0)
        },
    )
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct CharTile {
    b: u8,
}

impl Tile<Axis2d, ()> for CharTile {
    fn flip(self, _axis: Axis2d) -> Self {
        self
    }

    fn perp(self, _axis: ()) -> Self {
        Self {
            b: match self.b {
                b'-' => b'|',
                b'|' => b'-',
                o => o,
            },
        }
    }
}
