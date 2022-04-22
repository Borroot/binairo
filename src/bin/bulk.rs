use binairo::{analyzer::level, generator};

fn main() {
    for _ in 0..100 {
        let gen = generator::gen(10, 10, level::Level::Medium, None).unwrap();
        println!("{}", gen.codex());
    }
}
