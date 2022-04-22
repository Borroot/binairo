use crate::{
    analyzer::{self, tactics},
    puzzle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Easy,
    Medium,
    Hard,
}

impl Level {
    pub fn from(puzzle: &puzzle::Puzzle) -> Self {
        // assume the level is hard so all tactics will be used
        let stats = analyzer::Stats::from(puzzle, Some(Level::Hard));

        if stats.count(tactics::Tactics::Backtrack).unwrap() > 0 {
            return Level::Hard;
        }
        if stats.count(tactics::Tactics::Uniqueness).unwrap() > 0
            || stats.count(tactics::Tactics::CountGuess).unwrap() > 0
        {
            return Level::Medium;
        }
        return Level::Easy;
    }

    pub fn tactics(&self) -> Vec<tactics::Tactics> {
        let mut tactics = vec![
            tactics::Tactics::Row2,
            tactics::Tactics::Row3,
            tactics::Tactics::CountFixed,
        ];
        if *self >= Level::Medium {
            tactics.push(tactics::Tactics::CountGuess);
            tactics.push(tactics::Tactics::Uniqueness);
        }
        if *self == Level::Hard {
            tactics.push(tactics::Tactics::Backtrack);
        }
        return tactics;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_order() {
        assert!(Level::Easy < Level::Medium);
        assert!(Level::Easy < Level::Hard);
        assert!(Level::Medium < Level::Hard);
    }

    #[test]
    fn tactics() {
        assert!(
            Level::Easy.tactics()
                == vec![
                    tactics::Tactics::Row2,
                    tactics::Tactics::Row3,
                    tactics::Tactics::CountFixed,
                ]
        );

        assert!(
            Level::Medium.tactics()
                == vec![
                    tactics::Tactics::Row2,
                    tactics::Tactics::Row3,
                    tactics::Tactics::CountFixed,
                    tactics::Tactics::CountGuess,
                    tactics::Tactics::Uniqueness,
                ]
        );

        assert!(
            Level::Hard.tactics()
                == vec![
                    tactics::Tactics::Row2,
                    tactics::Tactics::Row3,
                    tactics::Tactics::CountFixed,
                    tactics::Tactics::CountGuess,
                    tactics::Tactics::Uniqueness,
                    tactics::Tactics::Backtrack,
                ]
        );
    }
}
