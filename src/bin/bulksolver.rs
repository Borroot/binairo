use binairo::{puzzle, solver};
use std::fs;

fn main() {
    for size in [6, 8, 10, 14, 20] {
        for level in [1, 2] {

            let filename = &format!("../../sets/sets2/size{}level{}.txt", size, level);

            let contents = fs::read_to_string(filename).unwrap();
            let codices = contents.split("\n").map(|codex| codex.trim()).collect::<Vec<_>>();

            let mut text = String::new();

            for codex in &codices {
                let binairo = puzzle::Puzzle::from_codex(&codex, size, size).unwrap();
                let solution = &solver::solve(&binairo).unwrap();

                println!("{} {}", codex, solution.codex());
                text.push_str(&format!("{} {}\n", codex, solution.codex()));
            }

            fs::write(filename, text);
        }
    }
}
