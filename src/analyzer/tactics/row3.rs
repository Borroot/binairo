use crate::analyzer::tactics::{self, Tactic};
use crate::puzzle;

/// Tactic applied when seen two consecutive same numbers, with a gap in
/// between. E.g. 0_0 becomes 010.
struct Row3;

impl Tactic for Row3 {
    fn hints(puzzle: &puzzle::Puzzle) -> Vec<tactics::Hint> {
        let mut hints = Vec::new();

        // check horizontal lines
        for y in 0..puzzle.height() {
            for x in 0..puzzle.width() - 2 {
                if puzzle[y][x].is_some()
                    && puzzle[y][x + 1].is_none()
                    && puzzle[y][x] == puzzle[y][x + 2]
                {
                    hints.push(tactics::Hint::new(x + 1, y, puzzle[y][x].unwrap() ^ 1));
                }
            }
        }

        // check vertical lines
        for x in 0..puzzle.width() {
            for y in 0..puzzle.height() - 2 {
                if puzzle[y][x].is_some()
                    && puzzle[y + 1][x].is_none()
                    && puzzle[y][x] == puzzle[y + 2][x]
                {
                    let hint = tactics::Hint::new(x, y + 1, puzzle[y][x].unwrap() ^ 1);
                    if !hints.contains(&hint) {
                        hints.push(hint);
                    }
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
    fn row3_horizontal() {
        let puzzle = puzzle::Puzzle::from_codex("1a1j0a0", 4, 4).unwrap();
        assert!(
            Row3::hints(&puzzle) == vec![tactics::Hint::new(1, 0, 0), tactics::Hint::new(2, 3, 1),]
        );
    }

    #[test]
    fn row3_vertical() {
        let puzzle = puzzle::Puzzle::from_codex("1f01f0", 4, 4).unwrap();
        assert!(
            Row3::hints(&puzzle) == vec![tactics::Hint::new(0, 1, 0), tactics::Hint::new(3, 2, 1),]
        );
    }
}
