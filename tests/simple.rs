use std::num::NonZeroU32;
use wfc::{
    collapse,
    extract::extract_patterns,
    grid_2d::{Axis2d, Coordinate2d, Grid2d},
    state::StateSet,
    Space,
};

#[test]
fn simple() {
    /*
    let input_dim = Coordinate2d { x: 22, y: 10 };
    let input = Grid2d::new(input_dim, |c| {
        if c.x < 2 || c.y < 2 || c.x > input_dim.x - 3 || c.y > input_dim.y - 3 {
            return None;
        }
        Some(NonZeroU32::new(c.x % 3 + 1).unwrap())
    });
    */

    /*
        let input = parse_grid(r#"
    xxxx
    x  x
    x  x
    xxxxxxx
       x  x
       x  x
       xxxx
        "#);
        */

    let input = parse_grid(
        r#"
__________
__________
__________
___xxxx___
___x++x___
___x++x___
___xxxx___
__________
__________
__________
 "#,
    );

    println!("input:");
    print_grid(&input);

    let rule = extract_patterns::<_, Grid2d<StateSet>>(
        &input,
        Coordinate2d { x: 3, y: 3 },
        &[Axis2d::X, Axis2d::Y],
    );

    println!("rules: {}", rule.state_count());

    StateSet::scope(rule.state_count(), || {
        let mut space = Grid2d::new(Coordinate2d { x: 10, y: 10 }, |_| StateSet::all());

        collapse(&mut space, &rule);

        let (unextracted, overconstrained) = rule.observer().unextract::<Grid2d<_>, _>(&space);

        println!("overconstrained: {overconstrained}");
        println!("output:");
        print_grid(&unextracted);

        //println!("{unextracted:?}");
    });
}

fn parse_grid(s: &str) -> Grid2d<Option<NonZeroU32>> {
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
            NonZeroU32::new(
                lines[coordinate.y as usize]
                    .as_bytes()
                    .get(coordinate.x as usize)
                    .copied()
                    .map(|b| if matches!(b, b' ' | b'_') { 0 } else { b })
                    .unwrap_or(0) as u32,
            )
        },
    )
}

fn print_grid(grid: &Grid2d<Option<NonZeroU32>>) {
    for y in 0..grid.dimensions().y {
        for x in 0..grid.dimensions().x {
            let v = grid[Coordinate2d { x, y }];
            print!(
                "{}",
                if let Some(v) = v {
                    format!("{}", v.get() as u8 as char)
                } else {
                    "_".to_owned()
                }
            );
        }
        println!();
    }
}
