use binairo::{
    analyzer::{self, tactics},
    generator,
};

fn main() {
    let tactics = vec![
        tactics::Tactics::Row2,
        tactics::Tactics::Row3,
        tactics::Tactics::CountFixed,
        tactics::Tactics::CountGuess,
        tactics::Tactics::Uniqueness,
    ];
    let gen = generator::gen(6, 6, Some(tactics.clone()), None).unwrap();
    let stats = analyzer::Stats::from(&gen, Some(tactics.clone()));

    println!("{}", gen);
    println!("{}", stats.solved);
    println!("{:?}", stats.counters);
}
