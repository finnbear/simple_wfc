use crate::rules::{SetCollapseObserver, SetCollapseRules};
use crate::space::*;
use crate::state::StateSet;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;

fn find_next_to_collapse<Sp: Space<StateSet>>(
    unresoved_set: &mut Vec<Sp::Coordinate>,
    lowest_entropy_set: &mut Vec<Sp::Coordinate>,
    space: &Sp,
) -> Option<Sp::Coordinate> {
    let mut lowest_entropy = u32::MAX;
    lowest_entropy_set.clear();
    unresoved_set.retain(|unresolved| {
        let entropy = space[*unresolved].entropy();
        if entropy == 0 {
            return false;
        }
        if entropy < lowest_entropy {
            lowest_entropy = entropy;
            lowest_entropy_set.clear();
            lowest_entropy_set.push(*unresolved);
        } else if entropy == lowest_entropy {
            lowest_entropy_set.push(*unresolved);
        }
        true
    });
    if lowest_entropy_set.is_empty() {
        None
    } else {
        Some(lowest_entropy_set[thread_rng().gen_range(0..lowest_entropy_set.len())])
    }
}

/// Perform the wave function collapse algorithm on a given state-space with
/// the provided collapse rule.
pub fn collapse<Sp: Space<StateSet>, O: SetCollapseObserver>(
    space: &mut Sp,
    rule: &SetCollapseRules<O>,
    rng: &mut impl Rng,
) {
    let mut unresolved_set = Vec::new();
    let mut lowest_entropy_set = Vec::new();
    Sp::visit_coordinates(space.dimensions(), |coord| {
        if space[coord].entropy() > 0 {
            unresolved_set.push(coord);
        }
    });
    let mut neighbors = vec![None; Sp::DIRECTIONS.len()].into_boxed_slice();
    let mut neighbor_states =
        vec![Option::<StateSet>::None; Sp::DIRECTIONS.len()].into_boxed_slice();
    let mut to_propogate = VecDeque::new();

    for coordinate in unresolved_set.iter() {
        to_propogate.push_back(*coordinate);
    }
    run_propogation(
        space,
        rule,
        &mut to_propogate,
        &mut neighbors,
        &mut neighbor_states,
    );

    while let Some(to_collapse) =
        find_next_to_collapse(&mut unresolved_set, &mut lowest_entropy_set, space)
    {
        to_propogate.clear();
        fill_neighbors(&*space, to_collapse, &mut neighbors);
        for i in 0..Sp::DIRECTIONS.len() {
            neighbor_states[i] = neighbors[i].map(|coord| space[coord].clone());
        }
        rule.observe(&mut space[to_collapse], &neighbor_states[..], rng);
        for i in 0..Sp::DIRECTIONS.len() {
            if let Some(neighbor_coord) = neighbors[i] {
                to_propogate.push_back(neighbor_coord);
            }
        }
        run_propogation(
            space,
            rule,
            &mut to_propogate,
            &mut neighbors,
            &mut neighbor_states,
        );
    }
}

fn fill_neighbors<Sp: Space<StateSet>>(
    space: &Sp,
    coord: Sp::Coordinate,
    directions: &mut [Option<Sp::Coordinate>],
) {
    for (i, direction) in Sp::DIRECTIONS.iter().enumerate() {
        directions[i] = space.neighbor(coord, *direction);
    }
}

fn run_propogation<Sp: Space<StateSet>, O: SetCollapseObserver>(
    space: &mut Sp,
    rule: &SetCollapseRules<O>,
    to_propogate: &mut VecDeque<Sp::Coordinate>,
    neighbors: &mut [Option<Sp::Coordinate>],
    neighbor_states: &mut [Option<StateSet>],
) {
    while let Some(propogating) = to_propogate.pop_front() {
        let entropy_before = space[propogating].entropy();

        if entropy_before != 0 {
            fill_neighbors(&*space, propogating, neighbors);
            for i in 0..Sp::DIRECTIONS.len() {
                neighbor_states[i] = neighbors[i].map(|coord| space[coord].clone());
            }
            rule.collapse(&mut space[propogating], neighbor_states);
            let entropy_after = space[propogating].entropy();

            if entropy_after < entropy_before {
                for i in 0..Sp::DIRECTIONS.len() {
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
