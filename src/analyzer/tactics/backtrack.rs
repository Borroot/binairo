use crate::analyzer::tactics::{hint, Tactic};
use crate::puzzle;
use crate::solver;

/// When all other tactics fail plain backtracking can be applied. This tactic
/// will always return a hint for every empty cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Backtrack;

impl Tactic for Backtrack {
    fn hints(&self, puzzle: &puzzle::Puzzle) -> Vec<hint::Hint> {
        let solution = solver::solve(puzzle)
            .expect("All puzzles given to the backtrack tactic should be solvable.");

        let mut hints = Vec::new();
        for y in 0..puzzle.height() {
            for x in 0..puzzle.width() {
                if puzzle[y][x].is_none() {
                    hints.push(hint::Hint::new(x, y, solution[y][x].unwrap()));
                }
            }
        }
        return hints;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let puzzle = puzzle::Puzzle::from_codex("e11d0a1b", 4, 4).unwrap();
        println!("{:?}", Backtrack.hints(&puzzle));
        assert!(
            Backtrack.hints(&puzzle)
                == vec![
                    hint::Hint::new(0, 0, 1),
                    hint::Hint::new(1, 0, 0),
                    hint::Hint::new(2, 0, 0),
                    hint::Hint::new(3, 0, 1),
                    hint::Hint::new(0, 1, 0),
                    hint::Hint::new(3, 1, 0),
                    hint::Hint::new(0, 2, 1),
                    hint::Hint::new(1, 2, 0),
                    hint::Hint::new(2, 2, 1),
                    hint::Hint::new(0, 3, 0),
                    hint::Hint::new(2, 3, 0),
                    hint::Hint::new(3, 3, 1),
                ]
        );
    }
}
