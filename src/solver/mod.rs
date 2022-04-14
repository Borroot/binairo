use crate::puzzle;
use z3;

mod constraints;

/// Give the number of solutions asked for
pub fn solves(puzzle: &puzzle::Puzzle, number: Option<usize>) -> Option<Vec<puzzle::Puzzle>> {
    let ctx = &z3::Context::new(&z3::Config::default());
    let solver = &z3::Solver::new(&ctx);
    let shadow = &init(ctx, puzzle);

    constraints::all(ctx, solver, puzzle, shadow);
    let mut solutions = Vec::new();

    while (number == None || solutions.len() < number.unwrap())
        && solver.check() == z3::SatResult::Sat
    {
        // Extract the solution and add new rules to z3 to ensure solution uniqeness
        let solution = extract(solver, puzzle, shadow);
        solutions.push(solution);
        constraints::solution(ctx, solver, &solutions.last().unwrap(), shadow);
    }

    return match solutions.len() {
        0 => None,
        _ => Some(solutions),
    };
}

/// Give the first solution found
pub fn solve(puzzle: &puzzle::Puzzle) -> Option<puzzle::Puzzle> {
    match solves(puzzle, Some(1)) {
        None => None,
        Some(mut solutions) => Some(solutions.remove(0)),
    }
}

/// Checks whether the given puzzle has just one unique solution
pub fn unique(puzzle: &puzzle::Puzzle) -> Option<bool> {
    match solves(puzzle, Some(2)) {
        None => None,
        Some(solutions) => Some(solutions.len() == 1),
    }
}

/// Initialize a 2d array with z3 objects.
fn init<'a>(ctx: &'a z3::Context, puzzle: &puzzle::Puzzle) -> Vec<Vec<z3::ast::Int<'a>>> {
    (0..puzzle.height())
        .map(|y| {
            (0..puzzle.width())
                .map(|x| z3::ast::Int::new_const(ctx, format!("x{}y{}", x, y)))
                .collect()
        })
        .collect()
}

/// Extract the found solution from the solver and return it.
fn extract(
    solver: &z3::Solver,
    puzzle: &puzzle::Puzzle,
    shadow: &Vec<Vec<z3::ast::Int>>,
) -> puzzle::Puzzle {
    let mut solution = puzzle::Puzzle::new(puzzle.width(), puzzle.height()).unwrap();
    let model = solver.get_model().unwrap();

    for y in 0..puzzle.height() {
        for x in 0..puzzle.width() {
            let value = model.eval(&shadow[y][x], true).unwrap().as_u64().unwrap();
            solution[y][x] = Some(value.try_into().unwrap());
        }
    }
    return solution;
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
