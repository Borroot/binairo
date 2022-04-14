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
