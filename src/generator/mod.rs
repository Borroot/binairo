use crate::{
    analyzer::{self, level},
    puzzle, solver,
};
use itertools::Itertools;
use rand::{self, seq::SliceRandom, Rng, SeedableRng};
use std::result;

/// Generate a puzzle with the given sizes and from the given seed, if any.
pub fn gen(
    width: usize,
    height: usize,
    level: level::Level,
    seed: Option<u64>,
) -> result::Result<puzzle::Puzzle, String> {
    // use the seed if given
    let mut rng = {
        if seed.is_none() {
            rand::rngs::SmallRng::from_rng(rand::thread_rng()).unwrap()
        } else {
            rand::rngs::SmallRng::seed_from_u64(seed.unwrap())
        }
    };

    // generate a puzzle
    let mut gen = init(width, height, &mut rng)?;
    eliminate(&mut gen, level, &mut rng);

    return Ok(gen);
}

/// Generate a random solved puzzle of the given size.
fn init(
    width: usize,
    height: usize,
    rng: &mut rand::rngs::SmallRng,
) -> result::Result<puzzle::Puzzle, String> {
    let mut gen = puzzle::Puzzle::new(width, height)?;

    // fill the puzzle with a few numbers
    for symbol in [0, 0, 1, 1] {
        let y = rng.gen_range(0..height);
        let x = rng.gen_range(0..width);
        gen[y][x] = Some(symbol);
    }

    // solve the puzzle (it is always solvable)
    gen = solver::solve(&gen).expect(&format!(
        "It is not possible to generate a puzzle with width {} and height {}.",
        width, height
    ));

    return Ok(gen);
}

/// Eliminate all the values which are not required for a unique solution.
fn eliminate(gen: &mut puzzle::Puzzle, level: level::Level, mut rng: &mut rand::rngs::SmallRng) {
    // shuffle the order in which all the cells are visited
    let mut cells: Vec<_> = (0..gen.height())
        .cartesian_product(0..gen.width())
        .collect();
    cells.shuffle(&mut rng);

    // keep a value only if the solution is not unique upon removal
    for (y, x) in cells {
        let symbol = gen[y][x];
        gen[y][x] = None;

        // check if the solution is still unique and solvable given the level
        if solver::unique(&gen) != Some(true)
            || !analyzer::Stats::from(&gen, Some(level.clone()))
                .solved
                .isfull()
        {
            gen[y][x] = symbol;
        }
    }
}
