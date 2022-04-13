use crate::puzzle;

// TODO add stats struct which shows how often tactics are applied
// TODO add a difficulty measure function based on the stats

/// Different tactics that humans apply to binairo puzzles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tactics {
    Row2,
    Row3,
    CountFixed,
    CountGuess, // TODO
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

type Tactic = fn(&puzzle::Puzzle) -> Vec<Hint>;

// TODO add all of the tactic functions
const TACTICS_FN: [Tactic; 3] = [row2, row3, count_fixed];

fn row2(puzzle: &puzzle::Puzzle) -> Vec<Hint> {
    let mut hints = Vec::new();

    // check horizontal lines
    for y in 0..puzzle.height() {
        for x in 0..puzzle.width() - 1 {
            if puzzle[y][x].is_some() && puzzle[y][x] == puzzle[y][x + 1] {
                // west
                if x > 0 && puzzle[y][x - 1].is_none() {
                    hints.push(Hint::new(x - 1, y, puzzle[y][x].unwrap() ^ 1));
                }
                // east
                if x + 2 < puzzle.width() && puzzle[y][x + 2].is_none() {
                    hints.push(Hint::new(x + 2, y, puzzle[y][x].unwrap() ^ 1));
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
                    let hint = Hint::new(x, y - 1, puzzle[y][x].unwrap() ^ 1);
                    if !hints.contains(&hint) {
                        hints.push(hint);
                    }
                }
                // south
                if y + 2 < puzzle.height() && puzzle[y + 2][x].is_none() {
                    let hint = Hint::new(x, y + 2, puzzle[y][x].unwrap() ^ 1);
                    if !hints.contains(&hint) {
                        hints.push(hint);
                    }
                }
            }
        }
    }

    return hints;
}

fn row3(puzzle: &puzzle::Puzzle) -> Vec<Hint> {
    let mut hints = Vec::new();

    // check horizontal lines
    for y in 0..puzzle.height() {
        for x in 0..puzzle.width() - 2 {
            if puzzle[y][x].is_some()
                && puzzle[y][x + 1].is_none()
                && puzzle[y][x] == puzzle[y][x + 2]
            {
                hints.push(Hint::new(x + 1, y, puzzle[y][x].unwrap() ^ 1));
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
                let hint = Hint::new(x, y + 1, puzzle[y][x].unwrap() ^ 1);
                if !hints.contains(&hint) {
                    hints.push(hint);
                }
            }
        }
    }

    return hints;
}

fn count_fixed(puzzle: &puzzle::Puzzle) -> Vec<Hint> {
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
                        hints.push(Hint::new(x, y, (symbol ^ 1).try_into().unwrap()));
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
            if count[symbol] == puzzle.height() / 2 && count[symbol ^ 1] != puzzle.height() / 2 {
                for y in 0..puzzle.height() {
                    if puzzle[y][x].is_none() {
                        let hint = Hint::new(x, y, (symbol ^ 1).try_into().unwrap());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row2_horizontal() {
        let puzzle = puzzle::Puzzle::from_codex("11c00cd11", 4, 4).unwrap();
        assert!(
            row2(&puzzle)
                == vec![
                    Hint::new(2, 0, 0),
                    Hint::new(0, 1, 1),
                    Hint::new(3, 1, 1),
                    Hint::new(1, 3, 0),
                ]
        );
    }

    #[test]
    fn row2_vertical() {
        let puzzle = puzzle::Puzzle::from_codex("c1a0a110b1c", 4, 4).unwrap();
        assert!(
            row2(&puzzle)
                == vec![
                    Hint::new(0, 1, 0),
                    Hint::new(1, 0, 1),
                    Hint::new(1, 3, 1),
                    Hint::new(3, 2, 0),
                ]
        );
    }

    #[test]
    fn row3_horizontal() {
        let puzzle = puzzle::Puzzle::from_codex("1a1j0a0", 4, 4).unwrap();
        assert!(row3(&puzzle) == vec![Hint::new(1, 0, 0), Hint::new(2, 3, 1),]);
    }

    #[test]
    fn row3_vertical() {
        let puzzle = puzzle::Puzzle::from_codex("1f01f0", 4, 4).unwrap();
        assert!(row3(&puzzle) == vec![Hint::new(0, 1, 0), Hint::new(3, 2, 1),]);
    }

    #[test]
    fn count_fixed_horizontal() {
        let puzzle1 = puzzle::Puzzle::from_codex("11c11a1b1a1a1a0111a10h", 4, 8).unwrap();
        assert!(
            count_fixed(&puzzle1)
                == vec![
                    Hint::new(2, 0, 0),
                    Hint::new(3, 0, 0),
                    Hint::new(0, 1, 0),
                    Hint::new(3, 1, 0),
                    Hint::new(1, 2, 0),
                    Hint::new(2, 2, 0),
                    Hint::new(0, 3, 0),
                    Hint::new(2, 3, 0),
                    Hint::new(0, 4, 0),
                    Hint::new(1, 5, 0),
                ]
        );

        let puzzle2 = puzzle::Puzzle::from_codex("00c00a0b0a0a0a1000a01h", 4, 8).unwrap();
        assert!(
            count_fixed(&puzzle2)
                == vec![
                    Hint::new(2, 0, 1),
                    Hint::new(3, 0, 1),
                    Hint::new(0, 1, 1),
                    Hint::new(3, 1, 1),
                    Hint::new(1, 2, 1),
                    Hint::new(2, 2, 1),
                    Hint::new(0, 3, 1),
                    Hint::new(2, 3, 1),
                    Hint::new(0, 4, 1),
                    Hint::new(1, 5, 1),
                ]
        );
    }

    #[test]
    fn count_fixed_vertical() {
        let puzzle1 = puzzle::Puzzle::from_codex("b1110c1b11b11a10c1a1b1b", 8, 4).unwrap();
        println!("{:?}", count_fixed(&puzzle1));
        assert!(count_fixed(&puzzle1) == vec![
            Hint::new(0, 0, 0),
            Hint::new(0, 1, 0),
            Hint::new(1, 0, 0),
            Hint::new(1, 3, 0),
            Hint::new(2, 1, 0),
            Hint::new(2, 2, 0),
            Hint::new(3, 1, 0),
            Hint::new(3, 3, 0),
            Hint::new(4, 3, 0),
            Hint::new(5, 2, 0),
        ]);

        let puzzle2 = puzzle::Puzzle::from_codex("b0001c0b00b00a01c0a0b0b", 8, 4).unwrap();
        assert!(count_fixed(&puzzle2) == vec![
            Hint::new(0, 0, 1),
            Hint::new(0, 1, 1),
            Hint::new(1, 0, 1),
            Hint::new(1, 3, 1),
            Hint::new(2, 1, 1),
            Hint::new(2, 2, 1),
            Hint::new(3, 1, 1),
            Hint::new(3, 3, 1),
            Hint::new(4, 3, 1),
            Hint::new(5, 2, 1),
        ]);
    }
}
