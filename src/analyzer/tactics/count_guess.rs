use crate::analyzer::tactics::{self, Tactic};
use crate::puzzle;

/// Advanced tactic which uses backtracking on one line. Guess a value, if no
/// valid line can follow we know the guessed value should be the opposite.
struct CountGuess;

impl Tactic for CountGuess {
    fn hints(puzzle: &puzzle::Puzzle) -> Vec<tactics::Hint> {
        let hints = Vec::new();

        // TODO

        return hints;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_guess_horizontal() {}

    #[test]
    fn count_guess_vertical() {}
}
