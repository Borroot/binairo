use crate::puzzle;

// TODO add stats struct which shows how often tactics are applied

pub struct Move {
    x: usize,
    y: usize,
    v: u8,  // value of 0 or 1
}

impl Move {
    pub fn new(x: usize, y: usize, v: u8) -> Self {
        Self {
            x,
            y,
            v: if v == 0 { 0 } else { 1 },
        }
    }
}

/// Different tactics that humans apply to binairo puzzles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tactics {
    Row2,
    Row3,
    CountFixed,
    CountGuess,
    Uniqueness,
    Backtrack,
}

// TODO add all of the tactic functions

const tactic_fn: [Tactic; 2] = [row2, row3];

type Tactic = fn(&puzzle::Puzzle) -> Vec<Move>;

fn row2(puzzle: &puzzle::Puzzle) -> Vec<Move> {
    vec!(Move::new(0, 0, 0))
}

fn row3(puzzle: &puzzle::Puzzle) -> Vec<Move> {
    vec!(Move::new(0, 0, 0))
}