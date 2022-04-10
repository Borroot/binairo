use std::{char, fmt, ops, result};

#[derive(Debug)]
pub struct Puzzle(Vec<Vec<Option<u8>>>);

impl Puzzle {
    pub fn new(width: usize, height: usize) -> result::Result<Puzzle, String> {
        Self::check(width, height)?;
        return Ok(Puzzle(vec![vec![None; width]; height]));
    }

    pub fn width(&self) -> usize {
        self[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    /// Return the codex representation of this puzzle
    pub fn codex(&self) -> String {
        let mut codex = String::new();
        let mut count = 0;

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self[y][x].is_none() {
                    count += 1;
                    if count == 26 {
                        codex.push('z');
                        count -= 26;
                    }
                } else {
                    if count > 0 {
                        codex.push(char::from_u32('a' as u32 + count - 1).unwrap());
                        count = 0;
                    }
                    codex.push(char::from_digit(self[y][x].unwrap().into(), 10).unwrap());
                }
            }
        }
        if count > 0 {
            codex.push(char::from_u32('a' as u32 + count - 1).unwrap());
        }
        return codex;
    }

    /// Create a new puzzle from a codex string
    pub fn from_codex(codex: &str, width: usize, height: usize) -> result::Result<Puzzle, String> {
        Self::check(width, height)?;

        let mut puzzle = Puzzle(vec![vec![None; width]; height]);
        let mut count: usize = 0;

        for mut c in codex.chars() {
            if c == '0' || c == '1' {
                puzzle[count / width][count % width] =
                    Some(c.to_digit(10).unwrap().try_into().unwrap());
                count += 1;
            } else {
                if !c.is_ascii_lowercase() {
                    return Err("Make sure all characters are ascii lowercase.".to_string());
                }
                while c >= 'a' {
                    puzzle[count / width][count % width] = None;
                    count += 1;
                    c = (c as u32 - 1).try_into().unwrap();
                }
            }
        }

        if count != width * height {
            return Err("The size of the codex is invalid.".to_string());
        }

        return Ok(puzzle);
    }

    fn check(width: usize, height: usize) -> result::Result<(), String> {
        if width % 2 != 0 || height % 2 != 0 {
            return Err("Width and height have to be a multiple of two.".to_string());
        }
        if width == 0 || height == 0 {
            return Err("Width and height have to be bigger than zero.".to_string());
        }
        return Ok(());
    }
}

impl ops::Index<usize> for Puzzle {
    type Output = Vec<Option<u8>>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl ops::IndexMut<usize> for Puzzle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(
                    f,
                    "{} ",
                    match self[y][x] {
                        Some(0) => "0",
                        Some(1) => "1",
                        _ => ".",
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_codex_error() {
        assert!(Puzzle::from_codex("0110", 2, 4).is_err());
        assert!(Puzzle::from_codex("!@()", 2, 2).is_err());
        assert!(Puzzle::from_codex("A0A1", 2, 2).is_err());

        assert!(Puzzle::from_codex("", 5, 4).is_err());
        assert!(Puzzle::from_codex("", 4, 5).is_err());
        assert!(Puzzle::from_codex("", 4, 0).is_err());
        assert!(Puzzle::from_codex("", 0, 4).is_err());
    }

    #[test]
    fn from_codex_valid() {
        let puzzle1 = Puzzle::from_codex("a1d11d1d0f0a0b0c1b", 6, 6).unwrap();
        let puzzle2 = [
            [None, Some(1), None, None, None, None],
            [Some(1), Some(1), None, None, None, None],
            [Some(1), None, None, None, None, Some(0)],
            [None, None, None, None, None, None],
            [Some(0), None, Some(0), None, None, Some(0)],
            [None, None, None, Some(1), None, None],
        ];
        for y in 0..puzzle1.height() {
            for x in 0..puzzle1.width() {
                assert!(puzzle1[y][x] == puzzle2[y][x]);
            }
        }
    }

    #[test]
    fn codex() {
        let codices = [
            "n0b1e1b0j0h0d1a00c1a0b1",
            "1010011010101100010100110100110110110010110011000101100100110011",
            "zzl",
            "1z0zi1",
        ];
        for codex in codices {
            assert!(Puzzle::from_codex(codex, 8, 8).unwrap().codex() == codex);
        }
    }

    #[test]
    fn size() {
        let puzzle = Puzzle::from_codex("a1d11d1d0f", 6, 4).unwrap();
        assert!(puzzle.width() == 6);
        assert!(puzzle.height() == 4);
    }

    #[test]
    fn small_size() {
        assert!(Puzzle::from_codex("b", 0, 2).is_err());
        assert!(Puzzle::from_codex("b", 1, 2).is_err());
        assert!(Puzzle::from_codex("b", 2, 0).is_err());
        assert!(Puzzle::from_codex("b", 2, 1).is_err());

        assert!(Puzzle::new(0, 2).is_err());
        assert!(Puzzle::new(1, 2).is_err());
        assert!(Puzzle::new(2, 0).is_err());
        assert!(Puzzle::new(2, 1).is_err());
    }

    #[test]
    fn odd_size() {
        assert!(Puzzle::new(4, 7).is_err());
        assert!(Puzzle::new(7, 4).is_err());

        assert!(Puzzle::from_codex("l", 3, 4).is_err());
        assert!(Puzzle::from_codex("l", 4, 3).is_err());
    }
}
