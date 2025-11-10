Kahuna
======

[![Crates.io](https://img.shields.io/crates/v/simple_wfc.svg?label=simple_wfc)](https://crates.io/crates/simple_wfc) [![docs.rs](https://docs.rs/simple_wfc/badge.svg)](https://docs.rs/simple_wfc/)

This crate is a simple implementation of [Wave Function Collapse](https://github.com/mxgmn/WaveFunctionCollapse) a.k.a. [Model Synthesis](https://paulmerrell.org/model-synthesis/).

## Features

- [x] 2D
- [x] 3D
- [ ] Periodic coordinate wrapping
- [x] Simple tiles
- [x] Overlapping tiles
  - [x] Custom pattern size
  - [x] Custom symmetry
- [ ] Error reporting

## Example (3x3 overlapping patterns)

```
____________________
__+---+__+---+______
__|+++|__|***|______
__|+++|__|***+---+__
__|+++|__|*******|__
__|+++|__|*******|__
__|+++|__|*******|__
__+---+__+-------+__
____________________
```

```
__________________________________________________
__________________________________________________
_+---+_________________________________+--+_______
_|***|______+-----+______+---------+___|**|_______
_|***|______|+++++|______|*********|___|**+----+__
_|***|______|+++++|______|*********|___|*******|__
_|***|______+-----+______+--+******|___|*******|__
_|***|______________________|******|___+-------+__
_+---+______________________+------+______________
_________+-----------+____________________________
_________|***********|_____________________+---+__
_+-------+***********|_____________+---+___|***|__
_|**************+----+_____+--+____|***|___|***|__
_|**************|__________|**|____|***|___|***|__
_+--------------+__________|**|____|***|___|***|__
____________________+---+__|**|____+---+___+---+__
____________________|***|__|**|___________________
____________________|***|__|**|___________________
____________________+---+__+--+___________________
__________________________________________________
```

## Acknowledgements

This is a simplified and extended fork of [`kahuna`](https://crates.io/crates/kahuna).

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
