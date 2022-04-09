use binairo::{generator};

fn main() {
    let g = generator::gen(8, 8, None);
    println!("{}", g);
}
