use crate::analyzer::tactics::{hint, Tactic};
use crate::puzzle;

/// Tactic applied when seen two consecutive same numbers, put the other number
/// at both sides of these consecutives. E.g. _00_ becomes 1001.
struct Row2;

impl Tactic for Row2 {
    fn hints(puzzle: &puzzle::Puzzle) -> Vec<hint::Hint> {
        let mut hints = Vec::new();

        // check horizontal lines
        for y in 0..puzzle.height() {
            for x in 0..puzzle.width() - 1 {
                if puzzle[y][x].is_some() && puzzle[y][x] == puzzle[y][x + 1] {
                    // west
                    if x > 0 && puzzle[y][x - 1].is_none() {
                        hints.push(hint::Hint::new(x - 1, y, puzzle[y][x].unwrap() ^ 1));
                    }
                    // east
                    if x + 2 < puzzle.width() && puzzle[y][x + 2].is_none() {
                        hints.push(hint::Hint::new(x + 2, y, puzzle[y][x].unwrap() ^ 1));
                    }
                }
            }
        }

        // check vertical lines
        for x in 0..puzzle.width() {
            for y in 0..puzzle.height() - 1 {
                if puzzle[y][x].is_some() && puzzle[y][x] == puzzle[y + 1][x] {
                    // north
                    if y > 0 && puzzle[y - 1][x].is_none() {
                        let hint = hint::Hint::new(x, y - 1, puzzle[y][x].unwrap() ^ 1);
                        if !hints.contains(&hint) {
                            hints.push(hint);
                        }
                    }
                    // south
                    if y + 2 < puzzle.height() && puzzle[y + 2][x].is_none() {
                        let hint = hint::Hint::new(x, y + 2, puzzle[y][x].unwrap() ^ 1);
                        if !hints.contains(&hint) {
                            hints.push(hint);
                        }
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
    fn row2_horizontal() {
        let puzzle = puzzle::Puzzle::from_codex("11c00cd11", 4, 4).unwrap();
        assert!(
            Row2::hints(&puzzle)
                == vec![
                    hint::Hint::new(2, 0, 0),
                    hint::Hint::new(0, 1, 1),
                    hint::Hint::new(3, 1, 1),
                    hint::Hint::new(1, 3, 0),
                ]
        );
    }

    #[test]
    fn row2_vertical() {
        let puzzle = puzzle::Puzzle::from_codex("c1a0a110b1c", 4, 4).unwrap();
        assert!(
            Row2::hints(&puzzle)
                == vec![
                    hint::Hint::new(0, 1, 0),
                    hint::Hint::new(1, 0, 1),
                    hint::Hint::new(1, 3, 1),
                    hint::Hint::new(3, 2, 0),
                ]
        );
    }
}
