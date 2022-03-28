use binairo::binairo;

fn main() {
    let binairo = binairo::Binairo::from_codex("a1d11d1d0f0a0b0c1b", 6, 6).unwrap();
    println!("{}", binairo);
}
