use crate::puzzle;

mod count_fixed;
mod count_guess;
mod row2;
mod row3;

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

#[derive(Debug, PartialEq, Eq)]
pub struct Hint {
    x: usize,
    y: usize,
    v: u8, // value of 0 or 1
}

impl Hint {
    pub fn new(x: usize, y: usize, v: u8) -> Self {
        Self {
            x,
            y,
            v: if v == 0 { 0 } else { 1 },
        }
    }
}

pub trait Tactic {
    fn hints(puzzle: &puzzle::Puzzle) -> Vec<Hint>;
}
