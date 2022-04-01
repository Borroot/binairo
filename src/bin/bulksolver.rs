use binairo::{puzzle, solver};

fn main() {
    let binairo = puzzle::Puzzle::from_codex(
        "b1k0b0a0b1c0a0a1f0e0d10b11g0f1c0d0h00g1c1b1a1a10b1c0c1b0f0a1b0g0b1o1b0c1g0b1d0a1e0a0a0",
        14,
        14,
    )
    .unwrap();
    println!("{}", binairo);

    let solution = &solver::solve(&binairo).unwrap();
    println!("{}", solution)
}
