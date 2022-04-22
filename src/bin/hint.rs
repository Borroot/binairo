use binairo::{
    analyzer::tactics::{self, Tactic},
    puzzle,
};

fn main() {
    let puzzle = puzzle::Puzzle::from_codex("a1b01a1b10a0b01101010", 6, 4).unwrap();
    let tactics = vec![
        tactics::Tactics::Row2,
        tactics::Tactics::Row3,
        tactics::Tactics::CountFixed,
        tactics::Tactics::CountGuess,
        tactics::Tactics::Uniqueness,
    ];

    for tactic in tactics {
        let hints = tactic.hints(&puzzle);
        if hints.len() > 0 {
            println!("{:?} {:?}", tactic, hints);
            break;
        }
    }
}
