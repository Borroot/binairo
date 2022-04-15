use crate::puzzle;
use std::{boxed, convert};

mod count_fixed;
mod count_guess;
mod row2;
mod row3;

pub mod hint;

/// Different tactics that humans apply to binairo puzzles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tactics {
    Row2,
    Row3,
    CountFixed,
    CountGuess,
    // Uniqueness, // TODO
    // Backtrack,  // TODO
}

pub trait Tactic {
    fn hints(&self, puzzle: &puzzle::Puzzle) -> Vec<hint::Hint>;
}

impl Tactic for Tactics {
    fn hints(&self, puzzle: &puzzle::Puzzle) -> Vec<hint::Hint> {
        match self {
            Tactics::Row2 => row2::Row2.hints(puzzle),
            Tactics::Row3 => row3::Row3.hints(puzzle),
            Tactics::CountFixed => count_fixed::CountFixed.hints(puzzle),
            Tactics::CountGuess => count_guess::CountGuess.hints(puzzle),
            // Tactics::Uniqueness => uniqueness::Uniqueness.hints(puzzle), // TODO
            // Tactics::Backtrack => backtrack::Backtrack.hints(puzzle), // TODO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tactic_order() {
        assert!(Tactics::Row2 < Tactics::Row3);
        assert!(Tactics::Row3 < Tactics::CountFixed);
        assert!(Tactics::CountFixed < Tactics::CountGuess);
    }

    #[test]
    fn tactic_row2() {
        let puzzle = puzzle::Puzzle::from_codex("11c00cd11", 4, 4).unwrap();
        assert!(
            Tactics::Row2.hints(&puzzle)
                == vec![
                    hint::Hint::new(2, 0, 0),
                    hint::Hint::new(0, 1, 1),
                    hint::Hint::new(3, 1, 1),
                    hint::Hint::new(1, 3, 0),
                ]
        );
    }

    #[test]
    fn tactic_row3() {
        let puzzle = puzzle::Puzzle::from_codex("1f01f0", 4, 4).unwrap();
        assert!(
            Tactics::Row3.hints(&puzzle)
                == vec![hint::Hint::new(0, 1, 0), hint::Hint::new(3, 2, 1)]
        );
    }

    #[test]
    fn tactic_instantiate() {
        fn tactic_run(tactic: Tactics) {
            let puzzle = puzzle::Puzzle::from_codex("1f01f0", 4, 4).unwrap();
            assert!(
                tactic.hints(&puzzle)
                    == vec![hint::Hint::new(0, 1, 0), hint::Hint::new(3, 2, 1)]
            );
        }
        let tactic = Tactics::Row3;
        tactic_run(tactic);
    }
}
