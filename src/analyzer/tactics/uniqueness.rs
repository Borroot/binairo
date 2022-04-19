use crate::analyzer::tactics::{hint, Tactic};
use crate::puzzle;

/// Advanced tactic which uses backtracking on one line, but takes into account
/// the uniqueness compared to other lines. Guess a value, if no valid unique
/// line can follow we know the guessed value should be the opposite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uniqueness;

impl Tactic for Uniqueness {
    fn hints(&self, puzzle: &puzzle::Puzzle) -> Vec<hint::Hint> {
        let mut hints = Vec::new();

        for y in 0..puzzle.height() {
            'next_cell: for x in 0..puzzle.width() {
                if puzzle[y][x].is_none() {
                    for guess in [0, 1] {
                        // create the horizontal line
                        let mut line = puzzle[y].clone();
                        line[x] = Some(guess);
                        let lines = lines_cmp(&puzzle, &line, y, true);

                        // check horizontal line
                        if !backtrack(&mut line, None, &lines) {
                            hints.push(hint::Hint::new(x, y, guess ^ 1));
                            continue 'next_cell;
                        }

                        // create the vertical line
                        let mut line: Vec<_> = (0..puzzle.height()).map(|y| puzzle[y][x]).collect();
                        line[y] = Some(guess);
                        let lines = lines_cmp(&puzzle, &line, x, false);

                        // check vertical line
                        if !backtrack(&mut line, None, &lines) {
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

/// Return true if the base line could possibly collide with the extra line.
/// Note that when the base line is filled this function returns whether the
/// base line collides with the extra line.
fn collision_exists(line_base: &Vec<Option<u8>>, line_extra: &Vec<Option<u8>>) -> bool {
    for x in 0..line_base.len() {
        if line_base[x].is_some() && line_base[x] != line_extra[x] {
            return false;
        }
    }
    return true;
}

/// Get all the lines which are full and possibly collide with the given line.
fn lines_cmp(
    puzzle: &puzzle::Puzzle,
    line: &Vec<Option<u8>>,
    line_i: usize,
    horizontal: bool,
) -> Vec<Vec<Option<u8>>> {
    // first get all the lines not equal to the given line
    let lines: Vec<Vec<Option<u8>>> = if horizontal {
        (0..puzzle.height())
            .filter(|&y| y != line_i)
            .map(|y| puzzle[y].clone())
            .collect()
    } else {
        (0..puzzle.width())
            .filter(|&x| x != line_i)
            .map(|x| (0..puzzle.height()).map(|y| puzzle[y][x]).collect())
            .collect()
    };

    // filter out the lines which are filled and possible collides with the given line
    return lines
        .into_iter()
        .filter(|l| l.iter().all(|v| v.is_some())) // check if the line is full
        .filter(|l| collision_exists(&line, &l)) // check if the line possibly collides
        .collect();
}

/// Backtrack on the given line and return whether a valid state exists or not.
fn backtrack(
    line: &mut Vec<Option<u8>>,
    last: Option<usize>,
    lines: &Vec<Vec<Option<u8>>>,
) -> bool {
    if last.is_some() && !valid(line, last.unwrap()) {
        return false; // the line is invalid
    }

    if line.iter().all(|v| v.is_some()) {
        // the line is valid, completely filled and does not collide
        return lines.iter().all(|l| !collision_exists(&line, &l));
    }

    for i in 0..line.len() {
        if line[i].is_none() {
            for guess in [0, 1] {
                line[i] = Some(guess);

                if backtrack(line, Some(i), lines) {
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
    fn collisions() {
        assert!(collision_exists(
            &vec![None; 4],
            &vec![Some(0), Some(1), Some(1), Some(0)]
        ));
        assert!(collision_exists(
            &vec![None, Some(1), None, Some(0)],
            &vec![Some(0), Some(1), Some(1), Some(0)]
        ));
        assert!(collision_exists(
            &vec![Some(0), Some(1), Some(1), Some(0)],
            &vec![Some(0), Some(1), Some(1), Some(0)]
        ));
        assert!(!collision_exists(
            &vec![None, Some(0), None, Some(0)],
            &vec![Some(0), Some(1), Some(1), Some(0)]
        ));
    }

    #[test]
    fn lines_horizontal() {
        assert!(
            lines_cmp(
                &puzzle::Puzzle::from_codex("001a011000110c", 4, 4).unwrap(),
                &vec![Some(0), None, None, None],
                3,
                true,
            ) == vec![
                vec![Some(0), Some(1), Some(1), Some(0)],
                vec![Some(0), Some(0), Some(1), Some(1)],
            ]
        );
    }

    #[test]
    fn lines_vertical() {
        assert!(
            lines_cmp(
                &puzzle::Puzzle::from_codex("0000011a1a1a1a01", 4, 4).unwrap(),
                &vec![Some(0), Some(1), None, None],
                1,
                false,
            ) == vec![vec![Some(0), Some(1), Some(1), Some(0)],]
        );
    }

    #[test]
    fn horizontal() {
        assert!(
            Uniqueness.hints(&puzzle::Puzzle::from_codex("10100b10b11b0", 4, 4).unwrap())
                == vec![hint::Hint::new(1, 3, 1), hint::Hint::new(2, 3, 0),]
        );
    }

    #[test]
    fn vertical() {
        assert!(
            Uniqueness.hints(&puzzle::Puzzle::from_codex("10010c1c0110", 4, 4).unwrap())
                == vec![hint::Hint::new(3, 1, 1), hint::Hint::new(3, 2, 0),]
        );
    }
}
