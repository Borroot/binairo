use crate::analyzer::tactics::{hint, Tactic};
use crate::puzzle;

/// Tactic applied when a line has all the 0's or 1's already, fill the rest
/// with the other symbol.
struct CountFixed;

impl Tactic for CountFixed {
    fn hints(puzzle: &puzzle::Puzzle) -> Vec<hint::Hint> {
        let mut hints = Vec::new();

        // check horizontal lines
        for y in 0..puzzle.height() {
            let mut count: [usize; 2] = [0, 0];

            // count the 0's and 1's
            for x in 0..puzzle.width() {
                if let Some(symbol) = puzzle[y][x] {
                    count[symbol as usize] += 1;
                }
            }

            // fill
            for symbol in [0, 1] {
                if count[symbol] == puzzle.width() / 2 && count[symbol ^ 1] != puzzle.width() / 2 {
                    for x in 0..puzzle.width() {
                        if puzzle[y][x].is_none() {
                            hints.push(hint::Hint::new(x, y, (symbol ^ 1).try_into().unwrap()));
                        }
                    }
                }
            }
        }

        // check vertical lines
        for x in 0..puzzle.width() {
            let mut count: [usize; 2] = [0, 0];

            // count the 0's and 1's
            for y in 0..puzzle.height() {
                if let Some(symbol) = puzzle[y][x] {
                    count[symbol as usize] += 1;
                }
            }

            // fill
            for symbol in [0, 1] {
                if count[symbol] == puzzle.height() / 2 && count[symbol ^ 1] != puzzle.height() / 2
                {
                    for y in 0..puzzle.height() {
                        if puzzle[y][x].is_none() {
                            let hint = hint::Hint::new(x, y, (symbol ^ 1).try_into().unwrap());
                            if !hints.contains(&hint) {
                                hints.push(hint);
                            }
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
    fn count_fixed_horizontal() {
        let puzzle1 = puzzle::Puzzle::from_codex("11c11a1b1a1a1a0111a10h", 4, 8).unwrap();
        assert!(
            CountFixed::hints(&puzzle1)
                == vec![
                    hint::Hint::new(2, 0, 0),
                    hint::Hint::new(3, 0, 0),
                    hint::Hint::new(0, 1, 0),
                    hint::Hint::new(3, 1, 0),
                    hint::Hint::new(1, 2, 0),
                    hint::Hint::new(2, 2, 0),
                    hint::Hint::new(0, 3, 0),
                    hint::Hint::new(2, 3, 0),
                    hint::Hint::new(0, 4, 0),
                    hint::Hint::new(1, 5, 0),
                ]
        );

        let puzzle2 = puzzle::Puzzle::from_codex("00c00a0b0a0a0a1000a01h", 4, 8).unwrap();
        assert!(
            CountFixed::hints(&puzzle2)
                == vec![
                    hint::Hint::new(2, 0, 1),
                    hint::Hint::new(3, 0, 1),
                    hint::Hint::new(0, 1, 1),
                    hint::Hint::new(3, 1, 1),
                    hint::Hint::new(1, 2, 1),
                    hint::Hint::new(2, 2, 1),
                    hint::Hint::new(0, 3, 1),
                    hint::Hint::new(2, 3, 1),
                    hint::Hint::new(0, 4, 1),
                    hint::Hint::new(1, 5, 1),
                ]
        );
    }

    #[test]
    fn count_fixed_vertical() {
        let puzzle1 = puzzle::Puzzle::from_codex("b1110c1b11b11a10c1a1b1b", 8, 4).unwrap();
        assert!(
            CountFixed::hints(&puzzle1)
                == vec![
                    hint::Hint::new(0, 0, 0),
                    hint::Hint::new(0, 1, 0),
                    hint::Hint::new(1, 0, 0),
                    hint::Hint::new(1, 3, 0),
                    hint::Hint::new(2, 1, 0),
                    hint::Hint::new(2, 2, 0),
                    hint::Hint::new(3, 1, 0),
                    hint::Hint::new(3, 3, 0),
                    hint::Hint::new(4, 3, 0),
                    hint::Hint::new(5, 2, 0),
                ]
        );

        let puzzle2 = puzzle::Puzzle::from_codex("b0001c0b00b00a01c0a0b0b", 8, 4).unwrap();
        assert!(
            CountFixed::hints(&puzzle2)
                == vec![
                    hint::Hint::new(0, 0, 1),
                    hint::Hint::new(0, 1, 1),
                    hint::Hint::new(1, 0, 1),
                    hint::Hint::new(1, 3, 1),
                    hint::Hint::new(2, 1, 1),
                    hint::Hint::new(2, 2, 1),
                    hint::Hint::new(3, 1, 1),
                    hint::Hint::new(3, 3, 1),
                    hint::Hint::new(4, 3, 1),
                    hint::Hint::new(5, 2, 1),
                ]
        );
    }
}
