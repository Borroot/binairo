use std::{fmt, ops, result};

#[derive(Debug)]
pub struct Binairo(Vec<Vec<Option<bool>>>);

impl Binairo {
    pub fn from_codex(codex: &str, width: usize, height: usize) -> result::Result<Binairo, String> {
        if width % 2 != 0 || height % 2 != 0 {
            return Err("Width and height have to be a multiple of two.".to_string());
        }
        if width == 0 || height == 0 {
            return Err("Width and height have to be bigger than zero.".to_string());
        }

        let mut binairo = Binairo(vec![vec![None; width]; height]);
        let mut count = 0usize;

        for mut c in codex.chars() {
            if c == '0' || c == '1' {
                binairo[count / width][count % width] = Some(c == '1');
                count += 1;
            } else {
                if !c.is_ascii_lowercase() {
                    return Err("Make sure all characters are ascii lowercase.".to_string());
                }
                while c >= 'a' {
                    binairo[count / width][count % width] = None;
                    count += 1;

                    let tmp: u32 = c.into();
                    c = (tmp - 1).try_into().unwrap();
                }
            }
        }

        if count != width * height {
            return Err("The size of the codex is invalid.".to_string());
        }

        return Ok(binairo);
    }

    pub fn width(&self) -> usize {
        self[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }
}

impl ops::Index<usize> for Binairo {
    type Output = Vec<Option<bool>>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl ops::IndexMut<usize> for Binairo {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl fmt::Display for Binairo {
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
        assert!(Binairo::from_codex("0110", 2, 4).is_err());
        assert!(Binairo::from_codex("!@()", 2, 2).is_err());
        assert!(Binairo::from_codex("A0A1", 2, 2).is_err());

        assert!(Binairo::from_codex("", 5, 4).is_err());
        assert!(Binairo::from_codex("", 4, 5).is_err());
        assert!(Binairo::from_codex("", 4, 0).is_err());
        assert!(Binairo::from_codex("", 0, 4).is_err());
    }

    #[test]
    fn from_codex_valid() {
        let binairo = Binairo::from_codex("a1d11d1d0f0a0b0c1b", 6, 6).unwrap();
        let puzzle = [
            [None, Some(true), None, None, None, None],
            [Some(true), Some(true), None, None, None, None],
            [Some(true), None, None, None, None, Some(false)],
            [None, None, None, None, None, None],
            [Some(false), None, Some(false), None, None, Some(false)],
            [None, None, None, Some(true), None, None],
        ];
        for y in 0..puzzle.len() {
            for x in 0..puzzle[y].len() {
                assert!(binairo[y][x] == puzzle[y][x]);
            }
        }
    }

    #[test]
    fn size() {
        let binairo = Binairo::from_codex("a1d11d1d0f", 6, 4).unwrap();
        assert!(binairo.width() == 6);
        assert!(binairo.height() == 4);
    }
}
