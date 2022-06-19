use binairo::{analyzer::level, generator, solver};

fn main() {
    for _ in 0..45 {
        let gen = generator::gen(12, 12, level::Level::Easy, None).unwrap();
        println!("{} {}", gen.codex(), solver::solve(&gen).unwrap().codex());
    }
    println!();
    for _ in 0..14 {
        let gen = generator::gen(8, 8, level::Level::Medium, None).unwrap();
        println!("{} {}", gen.codex(), solver::solve(&gen).unwrap().codex());
    }
    println!();
    for _ in 0..1 {
        let gen = generator::gen(8, 8, level::Level::Inhuman, None).unwrap();
        println!("{} {}", gen.codex(), solver::solve(&gen).unwrap().codex());
    }
}
