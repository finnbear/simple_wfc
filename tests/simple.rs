use std::num::NonZeroU32;
use wfc::{
    collapse,
    extract::extract_patterns,
    grid_2d::{Coordinate2d, Grid2d},
    state::StateSet,
    Space,
};

#[test]
fn simple() {
    let input = Grid2d::new(Coordinate2d { x: 5, y: 5 }, |c| {
        Some(NonZeroU32::new(c.x % 2 + 1).unwrap())
    });

    let rule = extract_patterns::<_, Grid2d<StateSet>>(&input, Coordinate2d { x: 3, y: 3 });

    println!("rules: {}", rule.state_count());

    StateSet::scope(rule.state_count(), || {
        let mut space = Grid2d::new(Coordinate2d { x: 25, y: 5 }, |_| StateSet::all());

        collapse(&mut space, &rule);

        let unextracted = rule.observer().unextract::<Grid2d<_>, _>(&space);

        for y in 0..unextracted.dimensions().y {
            for x in 0..unextracted.dimensions().x {
                let v = unextracted[Coordinate2d { x, y }];
                print!(
                    "{}",
                    if let Some(v) = v {
                        v.to_string()
                    } else {
                        "_".to_owned()
                    }
                );
            }
            println!();
        }

        //println!("{unextracted:?}");
    });
}
