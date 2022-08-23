use binairo::{analyzer::level, generator, solver};

fn main() {
    for _ in 0..45 {
        let size = 12;
        let gen = generator::gen(size, size, level::Level::Easy, None).unwrap();
        println!("{} {} {}", size, gen.codex(), solver::solve(&gen).unwrap().codex());
    }
    println!();
    for _ in 0..14 {
        let size = 8;
        let gen = generator::gen(size, size, level::Level::Medium, None).unwrap();
        println!("{} {} {}", size, gen.codex(), solver::solve(&gen).unwrap().codex());
    }
    println!();
    for _ in 0..1 {
        let size = 8;
        let gen = generator::gen(size, size, level::Level::Inhuman, None).unwrap();
        println!("{} {} {}", size, gen.codex(), solver::solve(&gen).unwrap().codex());
    }
}
