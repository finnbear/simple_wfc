#![cfg_attr(test, feature(test))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Wave Function Collapse
//!
//! Provides a simple implementation of the wave function collapse algorithm.
//!
//! Wave function collapse works by iteratively "collapsing" a collecion of
//! cells (such as a square grid) from all possible states to only the states
//! possible with a given ruleset, selecting randomly where ambiguous.

#[cfg(all(test, not(miri)))]
mod benches;
mod collapse;
pub mod grid_2d;
pub mod grid_3d;
pub mod overlapping;
pub mod rules;
mod space;
mod state;

pub use collapse::*;
pub use space::*;
pub use state::*;
