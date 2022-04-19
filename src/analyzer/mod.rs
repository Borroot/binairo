use crate::{
    analyzer::tactics::{hint::Hint, Tactic},
    puzzle,
};
use std::result;
use strum::{IntoEnumIterator};

mod tactics;

// TODO add a difficulty measure function based on the stats

#[derive(Debug, PartialEq, Eq)]
struct Counter {
    tactic: tactics::Tactics,
    count: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Stats {
    counters: Vec<Counter>,
    unsolved: puzzle::Puzzle,
    solved: puzzle::Puzzle,
}

impl Stats {
    pub fn from(puzzle: &puzzle::Puzzle, tactics: Option<Vec<tactics::Tactics>>) -> Stats {
        // initialize the list of tactics and sort
        let mut tactics = if tactics.is_none() || tactics.as_ref().unwrap().len() == 0 {
            tactics::Tactics::iter().collect()
        } else {
            tactics.unwrap()
        };
        tactics.sort();

        // solve the puzzle using the given tactics
        return Self::solve(puzzle, &tactics);
    }

    fn solve(puzzle: &puzzle::Puzzle, tactics: &Vec<tactics::Tactics>) -> Stats {
        // initialize all of the counters
        let mut counters = tactics
            .iter()
            .map(|tactic| Counter {
                tactic: *tactic,
                count: 0,
            })
            .collect();

        // apply the tactics in a loop
        let mut solved = puzzle.clone();
        let mut tactic_index = 0;
        loop {
            if tactic_index == tactics.len() {
                return Stats::new(counters, puzzle.clone(), solved);
            }

            let hints = tactics[tactic_index].hints(&solved);
            if hints.len() == 0 {
                tactic_index += 1;
            } else {
                for Hint { x, y, v } in hints {
                    solved[y][x] = Some(v);
                    counters[tactic_index].count += 1;
                }
                tactic_index = 0;
            }
        }
    }

    fn new(counters: Vec<Counter>, unsolved: puzzle::Puzzle, solved: puzzle::Puzzle) -> Self {
        Stats {
            counters,
            unsolved,
            solved,
        }
    }

    /// Retrieve how many times the provided tactic was applied.
    pub fn count(&self, tactic: tactics::Tactics) -> result::Result<usize, String> {
        for counter in &self.counters {
            if counter.tactic == tactic {
                return Ok(counter.count);
            }
        }
        return Err("Given tactic is not found in the counters list.".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_creation_list() {
        let counters = vec![
            Counter {
                tactic: tactics::Tactics::Row2,
                count: 3,
            },
            Counter {
                tactic: tactics::Tactics::CountFixed,
                count: 2,
            },
        ];
        let puzzle = puzzle::Puzzle::new(4, 4).unwrap();
        let stats = Stats::new(counters, puzzle.clone(), puzzle.clone());

        assert!(stats.count(tactics::Tactics::Row2) == Ok(3));
        assert!(stats.count(tactics::Tactics::CountFixed) == Ok(2));
        assert!(stats.count(tactics::Tactics::Row3).is_err());
        assert!(stats.count(tactics::Tactics::CountGuess).is_err());
    }

    #[test]
    fn stats_empty_list() {
        // should not panic due to index out of bounds error
        Stats::from(&puzzle::Puzzle::new(4, 4).unwrap(), Some(Vec::new()));
    }

    #[test]
    fn stats_small_easy() {
        let puzzle = puzzle::Puzzle::from_codex("e11f1b", 4, 4).unwrap();
        let tactics = vec![tactics::Tactics::Row3, tactics::Tactics::Row2];
        assert!(
            Stats::from(&puzzle, Some(tactics))
                == Stats {
                    counters: vec![
                        Counter {
                            tactic: tactics::Tactics::Row2,
                            count: 2,
                        },
                        Counter {
                            tactic: tactics::Tactics::Row3,
                            count: 1,
                        },
                    ],
                    unsolved: puzzle.clone(),
                    solved: puzzle::Puzzle::from_codex("d0110a0c1b", 4, 4).unwrap(),
                }
        );
    }

    #[test]
    fn stats_middle_easy() {
        let puzzle = puzzle::Puzzle::from_codex("a1d11d1d0f0a0b0c1b", 6, 6).unwrap();
        let tactics = vec![
            tactics::Tactics::Row2,
            tactics::Tactics::Row3,
            tactics::Tactics::CountFixed,
            tactics::Tactics::CountGuess,
        ];
        assert!(
            Stats::from(&puzzle, Some(tactics))
                == Stats {
                    counters: vec![
                        Counter {
                            tactic: tactics::Tactics::Row2,
                            count: 13,
                        },
                        Counter {
                            tactic: tactics::Tactics::Row3,
                            count: 2,
                        },
                        Counter {
                            tactic: tactics::Tactics::CountFixed,
                            count: 12,
                        },
                        Counter {
                            tactic: tactics::Tactics::CountGuess,
                            count: 0,
                        },
                    ],
                    unsolved: puzzle.clone(),
                    solved: puzzle::Puzzle::from_codex(
                        "011001110010101100001011010110100101",
                        6,
                        6
                    )
                    .unwrap(),
                }
        );
    }
}
