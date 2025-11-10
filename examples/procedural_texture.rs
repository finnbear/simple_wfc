use image::{ColorType, ImageFormat, RgbImage};
use wfc::grid_2d::{Coordinate2d, Direction2d, Grid2d};
use wfc::state::{State, StateSet};
use wfc::{collapse, rules::*, Space};

type S = StateSet;

type Grid = Grid2d<S>;

const WIDTH_TILES: u32 = 40;
const HEIGHT_TILES: u32 = 40;

fn main() {
    StateSet::scope(11, || {
        test();
    });
}

#[allow(non_snake_case)]
fn test() {
    let A = State::state(0);
    let B = State::state(1);
    let C = State::state(2);
    let D = State::state(3);
    let E = State::state(4);
    let F = State::state(5);
    let G = State::state(6);
    let H = State::state(7);
    let I = State::state(8);
    let J = State::state(9);
    let K = State::state(10);

    //
    //  A B C J
    //  D E F K
    //  G H I
    //
    //
    // A-I form a 9-quadrant for rectangles, J is open space around them, and K can touch only J

    let rule = SetCollapseRuleBuilder::<Grid2d<StateSet>, _>::new(UniformSetCollapseObserver)
        .allow(
            E,
            &[
                (Direction2d::Down, E.clone() | B.clone()),
                (Direction2d::Left, E.clone() | D.clone()),
                (Direction2d::Right, E.clone() | F.clone()),
                (Direction2d::Up, E.clone() | H.clone()),
            ],
        )
        .allow(
            A,
            &[
                (Direction2d::Left, C.clone() | F.clone() | I.clone()),
                (Direction2d::Down, G.clone() | H.clone() | I.clone()),
            ],
        )
        .allow(
            B,
            &[
                (Direction2d::Left, A.clone() | B.clone()),
                (Direction2d::Right, C.clone() | B.clone()),
                (Direction2d::Down, G.clone() | H.clone() | I.clone()),
            ],
        )
        .allow(
            C,
            &[
                (Direction2d::Down, G.clone() | H.clone() | I.clone()),
                (Direction2d::Right, A.clone() | D.clone() | G.clone()),
            ],
        )
        .allow(
            G,
            &[
                (Direction2d::Up, A.clone() | B.clone() | C.clone()),
                (Direction2d::Left, C.clone() | F.clone() | I.clone()),
            ],
        )
        .allow(
            I,
            &[
                (Direction2d::Right, A.clone() | D.clone() | G.clone()),
                (Direction2d::Up, A.clone() | B.clone() | C.clone()),
            ],
        )
        .allow(
            H,
            &[
                (Direction2d::Left, G.clone() | H.clone()),
                (Direction2d::Right, I.clone() | H.clone()),
                (Direction2d::Up, A.clone() | B.clone() | C.clone()),
            ],
        )
        .allow(
            F,
            &[
                (Direction2d::Down, C.clone() | F.clone()),
                (Direction2d::Up, I.clone() | F.clone()),
                (Direction2d::Right, A.clone() | D.clone() | C.clone()),
            ],
        )
        .allow(
            D,
            &[
                (Direction2d::Down, A.clone() | D.clone()),
                (Direction2d::Up, G.clone() | D.clone()),
                (Direction2d::Left, C.clone() | F.clone() | I.clone()),
            ],
        )
        .allow(
            J,
            &[
                (
                    Direction2d::Down,
                    J.clone() | G.clone() | H.clone() | I.clone() | K.clone(),
                ),
                (
                    Direction2d::Up,
                    J.clone() | A.clone() | B.clone() | C.clone() | K.clone(),
                ),
                (
                    Direction2d::Left,
                    J.clone() | C.clone() | F.clone() | I.clone() | K.clone(),
                ),
                (
                    Direction2d::Right,
                    J.clone() | A.clone() | D.clone() | G.clone() | K.clone(),
                ),
            ],
        )
        .build();
    let mut grid = Grid::new(
        Coordinate2d {
            x: WIDTH_TILES,
            y: HEIGHT_TILES,
        },
        |_| S::all(),
    );
    collapse(&mut grid, &rule);

    let image_bytes = include_bytes!("pattern.png");
    let input_image = image::load_from_memory_with_format(&image_bytes[..], ImageFormat::Png)
        .unwrap()
        .into_rgb8();
    let mut output_image = RgbImage::new(8 * WIDTH_TILES, 8 * HEIGHT_TILES);

    for y in 0..HEIGHT_TILES {
        for x in 0..WIDTH_TILES {
            let image_start_x = x * 8;
            let image_start_y = y * 8;
            let tile_start_x;
            let tile_start_y;
            match grid[Coordinate2d { x, y }].clone() {
                a if a.has(A) => {
                    tile_start_x = 0;
                    tile_start_y = 0;
                }
                a if a.has(B) => {
                    tile_start_x = 8;
                    tile_start_y = 0;
                }
                a if a.has(C) => {
                    tile_start_x = 16;
                    tile_start_y = 0;
                }
                a if a.has(D) => {
                    tile_start_x = 0;
                    tile_start_y = 8;
                }
                a if a.has(E) => {
                    tile_start_x = 8;
                    tile_start_y = 8;
                }
                a if a.has(F) => {
                    tile_start_x = 16;
                    tile_start_y = 8;
                }
                a if a.has(G) => {
                    tile_start_x = 0;
                    tile_start_y = 16;
                }
                a if a.has(H) => {
                    tile_start_x = 8;
                    tile_start_y = 16;
                }
                a if a.has(I) => {
                    tile_start_x = 16;
                    tile_start_y = 16;
                }
                a if a.has(J) => {
                    tile_start_x = 24;
                    tile_start_y = 0;
                }
                a if a.has(K) => {
                    tile_start_x = 24;
                    tile_start_y = 8;
                }
                a => panic!("unknown state {a:?}!"),
            }
            for j in 0..8 {
                for i in 0..8 {
                    let pixel = input_image.get_pixel(tile_start_x + i, tile_start_y + j);
                    output_image.put_pixel(image_start_x + i, image_start_y + j, *pixel);
                }
            }
        }
    }
    image::save_buffer(
        "procedural_texture.png",
        &output_image,
        output_image.width(),
        output_image.height(),
        ColorType::Rgb8,
    )
    .unwrap();
}
