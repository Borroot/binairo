use binairo::{generator, solver};

fn main() {
    let g = generator::gen(6, 4, None).unwrap();
    println!("{}", g);
    println!("{}", solver::solve(&g).unwrap());
}
