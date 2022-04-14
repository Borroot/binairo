use crate::puzzle;

mod count_fixed;
mod count_guess;
mod row2;
mod row3;

pub mod hint;

/// Different tactics that humans apply to binairo puzzles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tactics {
    // TODO let these point to the structs implementing the Tactic trait
    Row2,
    Row3,
    CountFixed,
    CountGuess,
    Uniqueness, // TODO
    Backtrack,  // TODO
}

pub trait Tactic {
    fn hints(puzzle: &puzzle::Puzzle) -> Vec<hint::Hint>;
}