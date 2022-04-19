use binairo::{generator, solver};

fn main() {
    let g = generator::gen(6, 4, Some(0)).unwrap();
    println!("{}", g);
    println!("{}", solver::solve(&g).unwrap());
}
