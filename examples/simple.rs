use std::time::Instant;

use rand::thread_rng;
use simple_wfc::{
    collapse,
    grid_2d::{Axis2d, Coordinate2d, Grid2d},
    overlapping::{codify_patterns, Tile},
    Space, StateSet,
};

fn main() {
    let profiler = pprof::ProfilerGuard::new(1000).unwrap();

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

    println!("input:");
    print_grid(&input);

    let rule = codify_patterns::<_, _, Grid2d<StateSet>>(
        &input,
        Coordinate2d { x: 3, y: 3 },
        &[Axis2d::X, Axis2d::Y],
        Some(()),
    );

    println!("rules: {}", rule.state_count());

    StateSet::scope(rule.state_count(), || {
        let start_collapse = Instant::now();

        let dimensions = Coordinate2d { x: 100, y: 100 };
        let mut space = Grid2d::new(dimensions, |coord| {
            let mut state = StateSet::all();

            if coord.x == 0
                || coord.y == 0
                || coord.x == dimensions.x - 1
                || coord.y == dimensions.y - 1
            {
                state.retain(|s| rule.observer().center(s).is_none());
            }

            state
        });

        collapse(&mut space, &rule, &mut thread_rng());

        let collapse_time = start_collapse.elapsed().as_secs_f32();

        let start_decode = Instant::now();

        let (unextracted, overconstrained) =
            rule.observer().decode_superposition::<Grid2d<_>, _>(&space);

        let decode_time = start_decode.elapsed().as_secs_f32();

        println!("overconstrained: {overconstrained}");
        println!("collapse: {collapse_time:.3}s decode: {decode_time:.3}s");
        println!("output:");
        print_grid(&unextracted);

        //println!("{unextracted:?}");
    });

    if let Ok(report) = profiler.report().build() {
        let mut buf = Vec::new();
        report.flamegraph(&mut buf).unwrap();
        std::fs::write("./flamegraph.svg", buf).unwrap();
    }
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

fn print_grid(grid: &Grid2d<Option<CharTile>>) {
    for y in 0..grid.dimensions().y {
        for x in 0..grid.dimensions().x {
            let v = grid[Coordinate2d { x, y }];
            print!(
                "{}",
                if let Some(v) = v {
                    format!("{}", v.b as char)
                } else {
                    "_".to_owned()
                }
            );
        }
        println!();
    }
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
