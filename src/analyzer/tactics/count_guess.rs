use crate::analyzer::tactics::{hint, Tactic};
use crate::puzzle;

/// Advanced tactic which uses backtracking on one line. Guess a value, if no
/// valid line can follow we know the guessed value should be the opposite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountGuess;

impl Tactic for CountGuess {
    fn hints(&self, puzzle: &puzzle::Puzzle) -> Vec<hint::Hint> {
        let mut hints = Vec::new();

        for y in 0..puzzle.height() {
            'next_cell: for x in 0..puzzle.width() {
                if puzzle[y][x].is_none() {
                    for guess in [0, 1] {
                        // create the horizontal line
                        let mut line = puzzle[y].clone();
                        line[x] = Some(guess);

                        // check horizontal line
                        if !backtrack(&mut line, None) {
                            hints.push(hint::Hint::new(x, y, guess ^ 1));
                            continue 'next_cell;
                        }

                        // create the vertical line
                        let mut line: Vec<_> = (0..puzzle.height()).map(|y| puzzle[y][x]).collect();
                        line[y] = Some(guess);

                        // check vertical line
                        if !backtrack(&mut line, None) {
                            hints.push(hint::Hint::new(x, y, guess ^ 1));
                            continue 'next_cell;
                        }
                    }
                }
            }
        }
        return hints;
    }
}

/// Backtrack on the given line and return whether a valid state exists or not.
fn backtrack(line: &mut Vec<Option<u8>>, last: Option<usize>) -> bool {
    if last.is_some() && !valid(line, last.unwrap()) {
        return false; // the line is invalid
    }

    if line.iter().all(|v| v.is_some()) {
        return true; // the line is valid and completely filled
    }

    for i in 0..line.len() {
        if line[i].is_none() {
            for guess in [0, 1] {
                line[i] = Some(guess);

                if backtrack(line, Some(i)) {
                    return true;
                }

                line[i] = None;
            }
        }
    }

    return false; // no valid state was found
}

/// Check whether a given row is still valid given the last changed index.
fn valid(line: &Vec<Option<u8>>, last: usize) -> bool {
    // check [. . last]
    if last > 1 && line[last] == line[last - 1] && line[last] == line[last - 2] {
        return false;
    }

    // check [last . .]
    if last < line.len() - 2 && line[last] == line[last + 1] && line[last] == line[last + 2] {
        return false;
    }

    // check [. last .]
    if last > 0
        && last < line.len() - 1
        && line[last] == line[last - 1]
        && line[last] == line[last + 1]
    {
        return false;
    }

    // check counts
    let mut count: [usize; 2] = [0, 0];
    for i in 0..line.len() {
        if let Some(symbol) = line[i] {
            count[symbol as usize] += 1;
        }
    }
    if count[0] > line.len() / 2 || count[1] > line.len() / 2 {
        return false;
    }

    // everything is valid
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal() {
        let puzzle = puzzle::Puzzle::from_codex("11e100a0dhhhh", 8, 6).unwrap();
        assert!(
            CountGuess.hints(&puzzle)
                == vec![
                    hint::Hint::new(2, 0, 0),
                    hint::Hint::new(3, 0, 0),
                    hint::Hint::new(4, 0, 1),
                    hint::Hint::new(5, 0, 0),
                    hint::Hint::new(6, 0, 0),
                    hint::Hint::new(2, 1, 1),
                    hint::Hint::new(4, 1, 1),
                    hint::Hint::new(7, 1, 1),
                ]
        )
    }

    #[test]
    fn vertical() {
        let puzzle = puzzle::Puzzle::from_codex("d01d01fd0afffe1", 6, 8).unwrap();
        assert!(
            CountGuess.hints(&puzzle)
                == vec![
                    hint::Hint::new(4, 2, 1),
                    hint::Hint::new(5, 2, 0),
                    hint::Hint::new(5, 3, 0),
                    hint::Hint::new(4, 4, 1),
                    hint::Hint::new(5, 4, 1),
                    hint::Hint::new(5, 5, 0),
                    hint::Hint::new(5, 6, 0),
                    hint::Hint::new(4, 7, 1),
                ]
        )
    }

    #[test]
    fn noduplicates() {
        let puzzle = puzzle::Puzzle::from_codex("d0a0b0c0b", 4, 4).unwrap();
        assert!(
            CountGuess.hints(&puzzle)
                == vec![
                    hint::Hint::new(1, 0, 1),
                    hint::Hint::new(1, 1, 1),
                    hint::Hint::new(3, 1, 1),
                ]
        )
    }
}
