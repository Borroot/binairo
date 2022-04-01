use std::{fmt, ops, result};

#[derive(Debug)]
pub struct Puzzle(Vec<Vec<Option<bool>>>);

impl Puzzle {
    pub fn new(width: usize, height: usize) -> Puzzle {
        Puzzle(vec![vec![None; width]; height])
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

        let tochar = |num: u32| {
            let base: u32 = 'a'.into();
            let c: char = (base + num - 1).try_into().unwrap();
            return c;
        };

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self[y][x].is_none() {
                    count += 1;
                } else {
                    while count > 0 {
                        if count < 26 {
                            codex.push(tochar(count));
                            count = 0;
                        } else {
                            count -= 26;
                            codex.push('z');
                        }
                    }

                    codex.push(match self[y][x].unwrap() {
                        true => '1',
                        false => '0',
                    })
                }
            }
        }
        if count > 0 {
            codex.push(tochar(count));
        }
        return codex;
    }

    /// Create a new puzzle from a codex string
    pub fn from_codex(codex: &str, width: usize, height: usize) -> result::Result<Puzzle, String> {
        if width % 2 != 0 || height % 2 != 0 {
            return Err("Width and height have to be a multiple of two.".to_string());
        }
        if width == 0 || height == 0 {
            return Err("Width and height have to be bigger than zero.".to_string());
        }

        let mut puzzle = Puzzle(vec![vec![None; width]; height]);
        let mut count = 0usize;

        for mut c in codex.chars() {
            if c == '0' || c == '1' {
                puzzle[count / width][count % width] = Some(c == '1');
                count += 1;
            } else {
                if !c.is_ascii_lowercase() {
                    return Err("Make sure all characters are ascii lowercase.".to_string());
                }
                while c >= 'a' {
                    puzzle[count / width][count % width] = None;
                    count += 1;

                    let tmp: u32 = c.into();
                    c = (tmp - 1).try_into().unwrap();
                }
            }
        }

        if count != width * height {
            return Err("The size of the codex is invalid.".to_string());
        }

        return Ok(puzzle);
    }
}

impl ops::Index<usize> for Puzzle {
    type Output = Vec<Option<bool>>;

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
                let symbol = match self[y][x] {
                    Some(false) => "0",
                    Some(true) => "1",
                    None => ".",
                };
                if let Err(e) = write!(f, "{} ", symbol) {
                    return Err(e);
                }
            }
            if let Err(e) = write!(f, "\n") {
                return Err(e);
            }
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
            [None, Some(true), None, None, None, None],
            [Some(true), Some(true), None, None, None, None],
            [Some(true), None, None, None, None, Some(false)],
            [None, None, None, None, None, None],
            [Some(false), None, Some(false), None, None, Some(false)],
            [None, None, None, Some(true), None, None],
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
            "1i1f1c0a1d01b0a11e0d1d1a0b0a1a0b1a0b0a0h0b0d0b1b00z1c1b11c0b1b0b10c1c0a10b00a0c11b0p1d11c0c1i1a1b1b1c01f1d1e1c0a10h0b1b0b1a0f11a1d0q1c1a1a00a1e1b0a0e0c0d1p0c1a0d1e0e1d00b01b10a1a1c0",
            "b0c11b0a0b1c1a1f1a1k0b00b0b0c1b0a1b1b1c1a11b0za11i0a110d1d00b0b0b1a1a1g0a11j00b0e0a1100f1b1a0c0e1c1f0c0e0f1a1f01c0d1g00d1b01b11b0b1b0c1b00e00c0b1a1g0e0b11e11e1c0a0e1n1b1b0b0b11e01d1g0a00",
            "10b00d0c1c0a10f0d1a0a10c0b10zb0b11b0b1a11b00b0h0b1d0f1i1f1c10a0a0g1b0a11e0a1b00a1g00n11b0c0a0a1b0c1c0c1b0b0a0b0b1c1j1b1c0f0b1b1d1b1c1a0c0c1b0c0b1c1g1n11a1b0f00a0j11b1f1c0c01a11c11c1c0a1b",
            "1b00a1d00c00d0a11b0i01k0b01b0d11d1a00za0a00d0b0c11a1b1b0a0a0a0c1b0a1c0b0b1c0b1c1a00c1c0e1g1e00c1j0j0a1b0b1a10c0e1d0d01c1c0c1d1d1b01d01b0d1b1d11a0b00i1n00b0f00a0c0e0b00e0d11d1b0g0f0f",
            "b1g1a0c0b0b1c11h0a00h0a0b1c0b0d01n11h0c0a0e0a0b1a10b10b0e1e1c1d11b1d1j1g1b0a0a11a0b0b00ze1a0a0b1a1b0a1b1d0a1f00a1c0a1h0e0j1e0d11c1c00c1b1d1g1g0a1c11c0a1c0b0f1g1e1a11c1a0a0d0k",
        ];
        for codex in codices {
            assert!(Puzzle::from_codex(codex, 20, 20).unwrap().codex() == codex);
        }
    }

    #[test]
    fn size() {
        let puzzle = Puzzle::from_codex("a1d11d1d0f", 6, 4).unwrap();
        assert!(puzzle.width() == 6);
        assert!(puzzle.height() == 4);
    }
}
