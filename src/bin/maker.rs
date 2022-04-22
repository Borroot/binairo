use binairo::{
    analyzer::{self, level},
    generator,
};

fn main() {
    let gen = generator::gen(10, 10, level::Level::Hard, None).unwrap();
    let stats = analyzer::Stats::from(&gen);

    println!("{}", gen);
    println!("{}", stats.solved);
    println!("{:?}", stats.counters);
    println!("{}", gen.codex());
}
