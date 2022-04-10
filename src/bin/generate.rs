use binairo::{generator, solver};

fn main() {
    let g = generator::gen(12, 6, None);
    println!("{}", g);
    println!("{}", solver::solve(&g).unwrap());
}
