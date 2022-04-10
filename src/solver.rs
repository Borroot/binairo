use crate::puzzle;
use z3::{self, ast::Ast};

/// Give the number of solutions asked for
pub fn solves(puzzle: &puzzle::Puzzle, number: Option<usize>) -> Option<Vec<puzzle::Puzzle>> {
    // TODO split up this function

    let ctx = &z3::Context::new(&z3::Config::default());
    let solver = z3::Solver::new(ctx);

    // Create a reference list with z3 objects corresponding to the puzzle cells
    let shadow = &init(ctx, puzzle.width(), puzzle.height());
    let shadow = &(0..puzzle.height()) // TODO beautify
        .map(|y| (0..puzzle.width()).map(|x| &shadow[y][x]).collect())
        .collect();

    // Add all the constraints to the solver
    constraint_numbers(ctx, &solver, puzzle, shadow);
    constraint_consecutive(ctx, &solver, puzzle, shadow);
    constraint_balance(ctx, &solver, puzzle, shadow);
    constraint_uniqueness(ctx, &solver, puzzle, shadow);

    let mut solutions = Vec::new();

    while (number == None || solutions.len() < number.unwrap())
        && solver.check() == z3::SatResult::Sat
    {
        // Extract the solution and add new rules to z3 to ensure solution uniqeness
        let mut solution = puzzle::Puzzle::new(puzzle.width(), puzzle.height()).unwrap();
        let mut compare = Vec::new();

        let model = solver.get_model().unwrap();

        for y in 0..puzzle.height() {
            for x in 0..puzzle.width() {
                let value = model.eval(shadow[y][x], true).unwrap().as_u64().unwrap();
                solution[y][x] = Some(value.try_into().unwrap());
                compare.push(shadow[y][x]._eq(&z3::ast::Int::from_u64(ctx, value)));
            }
        }
        solutions.push(solution);

        let compare = (0..compare.len()).map(|i| &compare[i]).collect::<Vec<_>>(); // TODO beautify
        solver.assert(&z3::ast::Bool::and(ctx, &compare).not());
    }

    if solutions.len() == 0 {
        return None;
    }
    return Some(solutions);
}

/// Checks whether the given puzzle has just one unique solution
pub fn unique(puzzle: &puzzle::Puzzle) -> Option<bool> {
    match solves(puzzle, Some(2)) {
        None => None,
        Some(solutions) => Some(solutions.len() == 1),
    }
}

/// Give the first solution found
pub fn solve(puzzle: &puzzle::Puzzle) -> Option<puzzle::Puzzle> {
    match solves(puzzle, Some(1)) {
        None => None,
        Some(mut solutions) => Some(solutions.remove(0)),
    }
}

/// Initialize a 2d array with z3 objects.
fn init(ctx: &z3::Context, width: usize, height: usize) -> Vec<Vec<z3::ast::Int>> {
    // TODO beautify
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| z3::ast::Int::new_const(ctx, format!("x{}y{}", x, y)))
                .collect()
        })
        .collect()
}

/// Add the given puzzle constraints and make sure every cell is a 0 or 1.
fn constraint_numbers(
    ctx: &z3::Context,
    solver: &z3::Solver,
    puzzle: &puzzle::Puzzle,
    shadow: &Vec<Vec<&z3::ast::Int>>,
) {
    for y in 0..puzzle.height() {
        for x in 0..puzzle.width() {
            // Some cells are according to the given puzzle a 0 or 1
            if let Some(value) = puzzle[y][x] {
                solver.assert(&shadow[y][x]._eq(&z3::ast::Int::from_u64(ctx, value as u64)))
            } else {
                // Every other cell is either a 0 or a 1
                solver.assert(&z3::ast::Bool::or(
                    ctx,
                    &[
                        &shadow[y][x]._eq(&z3::ast::Int::from_u64(ctx, 0)),
                        &shadow[y][x]._eq(&z3::ast::Int::from_u64(ctx, 1)),
                    ],
                ))
            }
        }
    }
}

/// Make sure there are no more than two consecutive 0's or 1's anywhere.
fn constraint_consecutive(
    ctx: &z3::Context,
    solver: &z3::Solver,
    puzzle: &puzzle::Puzzle,
    shadow: &Vec<Vec<&z3::ast::Int>>,
) {
    // No more than 2 consecutive 0's or 1's per row
    for y in 0..puzzle.height() {
        for x in 0..puzzle.width() - 2 {
            let sum = &z3::ast::Int::add(ctx, &[shadow[y][x], shadow[y][x + 1], shadow[y][x + 2]]);
            solver.assert(&z3::ast::Bool::or(
                ctx,
                &[
                    &sum._eq(&z3::ast::Int::from_i64(ctx, 1)),
                    &sum._eq(&z3::ast::Int::from_i64(ctx, 2)),
                ],
            ));
        }
    }

    // No more than 2 consecutive 0's or 1's per column
    for x in 0..puzzle.width() {
        for y in 0..puzzle.height() - 2 {
            let sum = &z3::ast::Int::add(ctx, &[shadow[y][x], shadow[y + 1][x], shadow[y + 2][x]]);
            solver.assert(&z3::ast::Bool::or(
                ctx,
                &[
                    &sum._eq(&z3::ast::Int::from_i64(ctx, 1)),
                    &sum._eq(&z3::ast::Int::from_i64(ctx, 2)),
                ],
            ));
        }
    }
}

/// Make sure every row and column has the same number of 0's as 1's.
fn constraint_balance(
    ctx: &z3::Context,
    solver: &z3::Solver,
    puzzle: &puzzle::Puzzle,
    shadow: &Vec<Vec<&z3::ast::Int>>,
) {
    // Same number of 0's and 1's per row
    for y in 0..puzzle.height() {
        solver.assert(
            &z3::ast::Int::add(ctx, shadow[y].as_slice())
                ._eq(&z3::ast::Int::from_u64(ctx, puzzle.width() as u64 / 2)),
        );
    }

    // Same number of 0's and 1's per column
    for x in 0..puzzle.width() {
        let column = (0..puzzle.height())
            .map(|y| shadow[y][x])
            .collect::<Vec<_>>();
        solver.assert(
            &z3::ast::Int::add(ctx, column.as_slice())
                ._eq(&z3::ast::Int::from_u64(ctx, puzzle.height() as u64 / 2)),
        );
    }
}

/// Make sure that every row is unique, and every column is unique.
fn constraint_uniqueness(
    ctx: &z3::Context,
    solver: &z3::Solver,
    puzzle: &puzzle::Puzzle,
    shadow: &Vec<Vec<&z3::ast::Int>>,
) {
    // Unique rows
    for y1 in 0..puzzle.height() {
        for y2 in y1 + 1..puzzle.height() {
            let cmp = (0..puzzle.width())
                .map(|x| shadow[y1][x]._eq(&shadow[y2][x]).not())
                .collect::<Vec<_>>();
            let cmp = (0..puzzle.width()).map(|i| &cmp[i]).collect::<Vec<_>>(); // TODO beautify
            solver.assert(&z3::ast::Bool::or(ctx, cmp.as_slice()));
        }
    }

    // Unique columns
    for x1 in 0..puzzle.width() {
        for x2 in x1 + 1..puzzle.width() {
            let cmp = (0..puzzle.height())
                .map(|y| shadow[y][x1]._eq(&shadow[y][x2]).not())
                .collect::<Vec<_>>();
            let cmp = (0..puzzle.height()).map(|i| &cmp[i]).collect::<Vec<_>>(); // TODO beautify
            solver.assert(&z3::ast::Bool::or(ctx, cmp.as_slice()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test whether the uniqueness test works properly
    #[test]
    fn solve_uniqueness() {
        assert!(!unique(&puzzle::Puzzle::from_codex("11c00i", 4, 4).unwrap()).unwrap());
        assert!(!unique(&puzzle::Puzzle::from_codex("11d11h", 4, 4).unwrap()).unwrap());

        assert!(unique(&puzzle::Puzzle::from_codex("11d111010d", 4, 4).unwrap()).unwrap());
    }

    /// Test whether the uniqeness constraint is correctly applied
    #[test]
    fn solve_constraint_unique() {
        // rows
        assert!(solve(&puzzle::Puzzle::from_codex("11d00h", 4, 4).unwrap()).is_none());
        assert!(solve(&puzzle::Puzzle::from_codex("h11d00", 4, 4).unwrap()).is_none());
        assert!(solve(&puzzle::Puzzle::from_codex("11l00", 4, 4).unwrap()).is_none());

        // columns
        assert!(solve(&puzzle::Puzzle::from_codex("1c1d0c0b", 4, 4).unwrap()).is_none());
        assert!(solve(&puzzle::Puzzle::from_codex("b1c1d0c0", 4, 4).unwrap()).is_none());
        assert!(solve(&puzzle::Puzzle::from_codex("1c1f0c0", 4, 4).unwrap()).is_none());
        assert!(solve(&puzzle::Puzzle::from_codex("a1c1e0c0", 4, 4).unwrap()).is_none());
    }

    /// Test whether multiple solutions are found correctly
    #[test]
    fn solve_more() {
        let solutions = |codex: &str, number: Option<usize>| -> usize {
            let p = &puzzle::Puzzle::from_codex(codex, 4, 4).unwrap();
            solves(p, number).unwrap().len()
        };

        assert!(solves(
            &puzzle::Puzzle::from_codex("11d11h", 4, 4).unwrap(),
            Some(0)
        )
        .is_none());

        assert!(solutions("11c00i", None) == 2);
        assert!(solutions("11d11h", None) == 4);

        assert!(solutions("11d11h", Some(1)) == 1);
        assert!(solutions("11d11h", Some(2)) == 2);
        assert!(solutions("11d11h", Some(3)) == 3);
        assert!(solutions("11d11h", Some(4)) == 4);
    }
}
