//! Wave Function Collapse
//!
//! Provides an implementation of the wave function collapse algorithm.
//!
//! Wave function collapse works by iteratively "collapsing" a collecion of
//! cells (such as a square grid) from all possible states to only the states
//! possible with a given ruleset, selecting randomly where ambiguous.

pub mod set_rule;
mod space;
pub mod square_grid;
pub mod state;

use rand::{thread_rng, Rng};
use set_rule::{SetCollapseObserver, SetCollapseRule};
pub use space::*;
use state::StateSet;
use std::collections::{HashSet, VecDeque};

fn find_next_to_collapse<Sp: Space>(
    unresoved_set: &mut HashSet<Sp::Coordinate>,
    lowest_entropy_set: &mut Vec<Sp::Coordinate>,
    resolved_set: &mut HashSet<Sp::Coordinate>,
    space: &Sp,
) -> Option<Sp::Coordinate> {
    let mut lowest_entropy = u32::MAX;
    lowest_entropy_set.clear();
    resolved_set.clear();
    for unresolved in unresoved_set.iter() {
        let entropy = space[*unresolved].entropy();
        if entropy == 0 {
            resolved_set.insert(*unresolved);
        } else if entropy < lowest_entropy {
            lowest_entropy = entropy;
            lowest_entropy_set.clear();
            lowest_entropy_set.push(*unresolved);
        } else if entropy == lowest_entropy {
            lowest_entropy_set.push(*unresolved);
        }
    }
    unresoved_set.retain(|x| !resolved_set.contains(x));
    if lowest_entropy_set.is_empty() {
        None
    } else {
        Some(lowest_entropy_set[thread_rng().gen_range(0..lowest_entropy_set.len())])
    }
}

/// Perform the wave function collapse algorithm on a given state-space with
/// the provided collapse rule.
pub fn collapse<Sp: Space, O: SetCollapseObserver>(space: &mut Sp, rule: &SetCollapseRule<Sp, O>) {
    let mut unresolved_set = HashSet::new();
    let mut resolved_set = HashSet::new();
    let mut lowest_entropy_set = Vec::new();
    let neighbor_directions = rule.neighbor_offsets();
    space.visit_coordinates(|coord| {
        if space[coord].entropy() > 0 {
            unresolved_set.insert(coord);
        }
    });
    let mut neighbors = vec![None; neighbor_directions.len()].into_boxed_slice();
    let mut neighbor_states =
        vec![Option::<StateSet>::None; neighbor_directions.len()].into_boxed_slice();
    let mut to_propogate = VecDeque::new();

    for coordinate in unresolved_set.iter() {
        to_propogate.push_back(*coordinate);
    }
    run_propogation(
        space,
        rule,
        &mut to_propogate,
        &neighbor_directions,
        &mut neighbors,
        &mut neighbor_states,
    );

    while let Some(to_collapse) = find_next_to_collapse(
        &mut unresolved_set,
        &mut lowest_entropy_set,
        &mut resolved_set,
        space,
    ) {
        to_propogate.clear();
        space.neighbors(to_collapse, &neighbor_directions, &mut neighbors);
        for i in 0..neighbor_directions.len() {
            neighbor_states[i] = neighbors[i].map(|coord| space[coord].clone());
        }
        rule.observe(&mut space[to_collapse], &neighbor_states[..]);
        for i in 0..neighbor_directions.len() {
            if let Some(neighbor_coord) = neighbors[i] {
                to_propogate.push_back(neighbor_coord);
            }
        }
        run_propogation(
            space,
            rule,
            &mut to_propogate,
            &neighbor_directions,
            &mut neighbors,
            &mut neighbor_states,
        );
    }
}

fn run_propogation<Sp: Space, O: SetCollapseObserver>(
    space: &mut Sp,
    rule: &SetCollapseRule<Sp, O>,
    to_propogate: &mut VecDeque<Sp::Coordinate>,
    neighbor_directions: &[Sp::CoordinateDelta],
    neighbors: &mut [Option<Sp::Coordinate>],
    neighbor_states: &mut [Option<StateSet>],
) {
    while let Some(propogating) = to_propogate.pop_front() {
        let entropy_before = space[propogating].entropy();

        if entropy_before != 0 {
            space.neighbors(propogating, neighbor_directions, neighbors);
            for i in 0..neighbor_directions.len() {
                neighbor_states[i] = neighbors[i].map(|coord| space[coord].clone());
            }
            rule.collapse(&mut space[propogating], neighbor_states);
            let entropy_after = space[propogating].entropy();

            if entropy_after < entropy_before {
                for i in 0..neighbor_directions.len() {
                    if let Some(neighbor) = neighbors[i] {
                        if space[neighbor].entropy() != 0 {
                            to_propogate.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }
}
