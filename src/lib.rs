//! Wave Function Collapse
//!
//! Provides an implementation of the wave function collapse algorithm.
//!
//! Wave function collapse works by iteratively "collapsing" a collecion of
//! cells (such as a square grid) from all possible states to only the states
//! possible with a given ruleset, selecting randomly where ambiguous.

mod collapse;
pub mod extract;
pub mod grid_2d;
pub mod grid_3d;
pub mod rules;
mod space;
pub mod state;

pub use collapse::*;
pub use space::*;
