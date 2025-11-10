use crate::{
    rules::{SetCollapseObserver, SetCollapseRules, SetCollapseRulesBuilder},
    state::{State, StateSet},
    InvertDelta, Space,
};
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};
use std::{collections::HashMap, hash::Hash, num::NonZeroU32};

#[derive(Clone)]
pub struct Pattern<T> {
    center: Option<T>,
    frequency: u32,
}

pub trait Tile<F, R> {
    fn flip(self, axis: F) -> Self;
    fn perp(self, axis: R) -> Self;
}

impl<F, R> Tile<F, R> for NonZeroU32 {
    fn flip(self, _axis: F) -> Self {
        self
    }

    fn perp(self, _axis: R) -> Self {
        self
    }
}

#[derive(Clone)]
pub struct ExtractedPatterns<T> {
    patterns: Vec<Pattern<T>>,
}

impl<T: Clone> ExtractedPatterns<T> {
    pub fn center(&self, state: State) -> Option<&T> {
        self.patterns[state.0 as usize].center.as_ref()
    }

    pub fn decode_superposition<
        Osp: Space<Option<T>>,
        Sp: Space<StateSet, Coordinate = Osp::Coordinate>,
    >(
        &self,
        space: &Sp,
    ) -> (Osp, usize) {
        let mut overconstrained = 0;
        let ret = Osp::new(space.dimensions(), |coord| {
            let states = &space[coord];
            for s in 0..StateSet::len() {
                if states.has(State::state(s)) {
                    return self.patterns[s as usize].center.clone();
                }
            }
            overconstrained += 1;
            None
        });
        (ret, overconstrained)
    }
}

impl<T> SetCollapseObserver for ExtractedPatterns<T> {
    fn observe(&self, cell: &mut StateSet, _neighbors: &[Option<StateSet>]) {
        let dist = WeightedIndex::new((0..StateSet::len()).map(|s| {
            if cell.has(State::state(s)) {
                self.patterns[s as usize].frequency
            } else {
                0
            }
        }))
        .unwrap();

        *cell = StateSet::with_states(&[State::state(dist.sample(&mut thread_rng()) as u32)]);
    }
}

pub fn codify_patterns<
    T: Clone + PartialEq,
    Sp: Space<Option<T>> + Hash + Eq + Clone,
    Ssp: Space<StateSet, Coordinate = Sp::Coordinate, Direction = Sp::Direction, Axis = Sp::Axis>,
>(
    input: &Sp,
    size: Sp::Coordinate,
    flip_symmetries: &[Sp::Axis],
    rotational_symmetry: Option<Sp::RotationAxis>,
) -> SetCollapseRules<ExtractedPatterns<T>>
where
    T: Tile<Sp::Axis, Sp::RotationAxis>,
{
    let neg_radius = Sp::map(size, |_, c| c / 2);
    struct PatternInfo {
        index: u32,
        frequency: u32,
    }
    let mut patterns = HashMap::<Sp, PatternInfo>::new();
    Sp::visit_coordinates(input.dimensions(), |input_coordinate| {
        let mut grid = Sp::new(size, |pattern_coordinate| {
            let sample_coordinate =
                input.add_sub(input_coordinate, pattern_coordinate, neg_radius)?;
            input[sample_coordinate].clone()
        });

        for axis in std::iter::once(None).chain(
            flip_symmetries
                .iter()
                .chain(&flip_symmetries[..flip_symmetries.len().saturating_sub(1)])
                .map(Some),
        ) {
            if let Some(axis) = axis {
                let new_grid = Sp::new(size, |c| {
                    let c = Sp::map(c, |a, c| if a == *axis { size[*axis] - 1 - c } else { c });

                    grid[c].clone().map(|t| t.flip(*axis))
                });
                grid = new_grid;
            }

            for rotational_symmetry in std::iter::once(Option::<Sp::RotationAxis>::None)
                .chain(rotational_symmetry.into_iter().map(Some))
            {
                let rotated_grid = if let Some(rotation_axis) = rotational_symmetry {
                    Sp::new(size, |c| {
                        let c = grid.perp(c, rotation_axis);

                        grid[c].clone().map(|t| t.perp(rotation_axis))
                    })
                } else {
                    grid.clone()
                };

                let next_index = patterns.len() as u32;
                // TODO.
                let entry = patterns.entry(rotated_grid).or_insert(PatternInfo {
                    index: next_index,
                    frequency: 0,
                });
                entry.frequency += 1;
            }
        }
    });

    StateSet::scope(patterns.len() as u32, || {
        let mut extracted_patterns = vec![
            Pattern {
                center: None,
                frequency: 0
            };
            patterns.len()
        ];
        for (grid, info) in patterns.iter() {
            extracted_patterns[info.index as usize] = Pattern {
                frequency: info.frequency,
                center: grid[neg_radius].clone(),
            };
        }
        let mut builder = SetCollapseRulesBuilder::<Ssp, _>::new(ExtractedPatterns {
            patterns: extracted_patterns,
        });

        for (pattern, info) in patterns.iter() {
            let mut neighbors = Vec::new();
            for &direction in Sp::DIRECTIONS {
                let mut allowed = StateSet::all();

                for (pattern2, info2) in patterns.iter() {
                    let mut compatible = true;
                    Sp::visit_coordinates(pattern.dimensions(), |coordinate| {
                        let coordinate2 = pattern.neighbor(coordinate, direction.invert_delta());
                        if let Some(coordinate2) = coordinate2 {
                            let value = pattern[coordinate].clone();
                            let value2 = pattern2[coordinate2].clone();
                            // TODO: early exit.
                            compatible &= value == value2;
                        }
                    });
                    if !compatible {
                        allowed.remove(State::state(info2.index));
                    }
                }

                neighbors.push((direction, allowed));
            }
            builder = builder.allow(State::state(info.index), &neighbors);
        }

        builder.build()
    })
}
