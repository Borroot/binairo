use crate::{
    analyzer::tactics::{hint::Hint, Tactic},
    puzzle,
};
use std::result;

pub mod level;
pub mod tactics;

#[derive(Debug, PartialEq, Eq)]
pub struct Counter {
    pub tactic: tactics::Tactics,
    pub count: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Stats {
    pub counters: Vec<Counter>,
    pub unsolved: puzzle::Puzzle,
    pub solved: puzzle::Puzzle,
}

impl Stats {
    pub fn from_level(puzzle: &puzzle::Puzzle, level: level::Level) -> Stats {
        Self::solve(puzzle, &level.tactics())
    }

    pub fn from(puzzle: &puzzle::Puzzle) -> Stats {
        Self::solve(puzzle, &level::Level::Inhuman.tactics())
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
                if tactics[tactic_index] <= tactics::Tactics::CountFixed {
                    // apply all hints if it is a basic tactic
                    for Hint { x, y, v } in hints {
                        solved[y][x] = Some(v);
                        counters[tactic_index].count += 1;
                    }
                } else {
                    // apply only one hint if it is an advanced tactic
                    let Hint { x, y, v } = hints[0];
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
        Stats::from(&puzzle::Puzzle::new(4, 4).unwrap());
    }

    #[test]
    fn stats_small_easy() {
        let puzzle = puzzle::Puzzle::from_codex("e11f1b", 4, 4).unwrap();
        let tactics = vec![tactics::Tactics::Row2, tactics::Tactics::Row3];
        println!("{:?}", Stats::solve(&puzzle, &tactics));
        assert!(
            Stats::solve(&puzzle, &tactics)
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
            Stats::solve(&puzzle, &tactics)
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

    #[test]
    fn stats_middle_hard() {
        let puzzle = puzzle::Puzzle::from_codex("11dfa1b00fff", 6, 6).unwrap();
        let tactics = vec![
            tactics::Tactics::Row2,
            tactics::Tactics::Row3,
            tactics::Tactics::CountFixed,
            tactics::Tactics::CountGuess,
            tactics::Tactics::Uniqueness,
        ];
        assert!(
            Stats::solve(&puzzle, &tactics)
                == Stats {
                    counters: vec![
                        Counter {
                            tactic: tactics::Tactics::Row2,
                            count: 3,
                        },
                        Counter {
                            tactic: tactics::Tactics::Row3,
                            count: 5,
                        },
                        Counter {
                            tactic: tactics::Tactics::CountFixed,
                            count: 1,
                        },
                        Counter {
                            tactic: tactics::Tactics::CountGuess,
                            count: 1,
                        },
                        Counter {
                            tactic: tactics::Tactics::Uniqueness,
                            count: 1,
                        },
                    ],
                    unsolved: puzzle.clone(),
                    solved: puzzle::Puzzle::from_codex("110010001b1110100fff", 6, 6).unwrap(),
                }
        );
    }
}
