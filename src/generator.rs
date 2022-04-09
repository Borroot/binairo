use rand::{self, Rng, SeedableRng};
use crate::{puzzle, solver};

pub fn gen(width: usize, height: usize, seed: Option<u64>) -> puzzle::Puzzle {
    let mut rng = {
        if seed.is_none() {
            rand::rngs::SmallRng::from_rng(rand::thread_rng()).unwrap()
        } else {
            rand::rngs::SmallRng::seed_from_u64(seed.unwrap())
        }
    };

    let mut gen = puzzle::Puzzle::new(width, height);

    for symbol in [false, false, true, true] {
        let y = rng.gen_range(0..height);
        let x = rng.gen_range(0..width);
        gen[y][x] = Some(symbol);
    }

    let mut gen = solver::solve(&gen).unwrap();

    // todo sample this order
    for y in 0..height {
        for x in 0..width {
            let symbol = gen[y][x];
            gen[y][x] = None;

            if solver::unique(&gen) != Some(true) {
                gen[y][x] = symbol;
            }
        }
    }
    return gen;
}