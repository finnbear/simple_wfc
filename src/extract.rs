use crate::{
    rules::{SetCollapseObserver, SetCollapseRule, SetCollapseRuleBuilder},
    state::{State, StateSet},
    InvertDelta, Space,
};
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};
use std::{collections::HashMap, hash::Hash, num::NonZeroU32};

#[derive(Clone)]
pub struct Pattern {
    center: Option<NonZeroU32>,
    frequency: u32,
}

#[derive(Clone)]
pub struct ExtractedPatterns {
    patterns: Vec<Pattern>,
}

impl ExtractedPatterns {
    pub fn center(&self, state: State) -> Option<NonZeroU32> {
        self.patterns[state.0 as usize].center
    }

    pub fn unextract<
        Osp: Space<Option<NonZeroU32>>,
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
                    return self.patterns[s as usize].center;
                }
            }
            overconstrained += 1;
            None
        });
        (ret, overconstrained)
    }
}

impl SetCollapseObserver for ExtractedPatterns {
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

pub fn extract_patterns<
    Sp: Space<Option<NonZeroU32>> + Hash + Eq + Clone,
    Ssp: Space<StateSet, Coordinate = Sp::Coordinate, Direction = Sp::Direction, Axis = Sp::Axis>,
>(
    input: &Sp,
    size: Sp::Coordinate,
    symmetries: &[Sp::Axis],
) -> SetCollapseRule<ExtractedPatterns> {
    let neg_radius = Sp::map(size, |_, c| c / 2);
    struct PatternInfo {
        index: u32,
        frequency: u32,
    }
    let mut patterns = HashMap::<Sp, PatternInfo>::new();
    input.visit_coordinates(|input_coordinate| {
        let mut grid = Sp::new(size, |pattern_coordinate| {
            let sample_coordinate =
                input.add_sub(input_coordinate, pattern_coordinate, neg_radius)?;
            input[sample_coordinate]
        });

        for axis in std::iter::once(None).chain(
            symmetries
                .iter()
                .chain(&symmetries[..symmetries.len().saturating_sub(1)])
                .map(Some),
        ) {
            if let Some(axis) = axis {
                let new_grid = Sp::new(size, |c| {
                    let c = Sp::map(c, |a, c| if a == *axis { size[*axis] - 1 - c } else { c });

                    grid[c]
                });
                grid = new_grid;
            }

            let next_index = patterns.len() as u32;
            // TODO.
            let entry = patterns.entry(grid.clone()).or_insert(PatternInfo {
                index: next_index,
                frequency: 0,
            });
            entry.frequency += 1;
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
                center: grid[neg_radius],
            };
        }
        let mut builder = SetCollapseRuleBuilder::<Ssp, _>::new(ExtractedPatterns {
            patterns: extracted_patterns,
        });

        for (pattern, info) in patterns.iter() {
            let mut neighbors = Vec::new();
            for &direction in Sp::DIRECTIONS {
                let mut allowed = StateSet::all();

                for (pattern2, info2) in patterns.iter() {
                    let mut compatible = true;
                    pattern.visit_coordinates(|coordinate| {
                        let coordinate2 = pattern.neighbor(coordinate, direction.invert_delta());
                        if let Some(coordinate2) = coordinate2 {
                            let value = pattern[coordinate];
                            let value2 = pattern2[coordinate2];
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
