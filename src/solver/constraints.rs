use crate::puzzle;
use z3::{self, ast::Ast};

/// Add constraints so that the given solution will not be found again.
pub fn solution(
    ctx: &z3::Context,
    solver: &z3::Solver,
    solution: &puzzle::Puzzle,
    shadow: &Vec<Vec<z3::ast::Int>>,
) {
    let mut z3solution = Vec::new();

    for y in 0..solution.height() {
        for x in 0..solution.width() {
            z3solution.push(shadow[y][x]._eq(&z3::ast::Int::from_u64(
                &ctx,
                solution[y][x].unwrap() as u64,
            )));
        }
    }

    let z3solution = (0..z3solution.len())
        .map(|index| &z3solution[index])
        .collect::<Vec<_>>();

    solver.assert(&z3::ast::Bool::and(&ctx, &z3solution).not());
}

/// Add all the binairo puzzle constraints to the provided solver.
pub fn all(
    ctx: &z3::Context,
    solver: &z3::Solver,
    puzzle: &puzzle::Puzzle,
    shadow: &Vec<Vec<z3::ast::Int>>,
) {
    constraint_numbers(ctx, solver, puzzle, shadow);
    constraint_consecutive(ctx, solver, puzzle, shadow);
    constraint_balance(ctx, solver, puzzle, shadow);
    constraint_uniqueness(ctx, solver, puzzle, shadow);
}

/// Add the given puzzle constraints and make sure every cell is a 0 or 1.
fn constraint_numbers(
    ctx: &z3::Context,
    solver: &z3::Solver,
    puzzle: &puzzle::Puzzle,
    shadow: &Vec<Vec<z3::ast::Int>>,
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
    shadow: &Vec<Vec<z3::ast::Int>>,
) {
    // No more than 2 consecutive 0's or 1's per row
    for y in 0..puzzle.height() {
        for x in 0..puzzle.width() - 2 {
            let sum =
                &z3::ast::Int::add(ctx, &[&shadow[y][x], &shadow[y][x + 1], &shadow[y][x + 2]]);
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
            let sum =
                &z3::ast::Int::add(ctx, &[&shadow[y][x], &shadow[y + 1][x], &shadow[y + 2][x]]);
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
    shadow: &Vec<Vec<z3::ast::Int>>,
) {
    // Same number of 0's and 1's per row
    for y in 0..puzzle.height() {
        let row = (0..puzzle.width())
            .map(|x| &shadow[y][x])
            .collect::<Vec<_>>();

        solver.assert(
            &z3::ast::Int::add(ctx, row.as_slice())
                ._eq(&z3::ast::Int::from_u64(ctx, puzzle.width() as u64 / 2)),
        );
    }

    // Same number of 0's and 1's per column
    for x in 0..puzzle.width() {
        let column = (0..puzzle.height())
            .map(|y| &shadow[y][x])
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
    shadow: &Vec<Vec<z3::ast::Int>>,
) {
    // Unique rows
    for y1 in 0..puzzle.height() {
        for y2 in y1 + 1..puzzle.height() {
            let compare = (0..puzzle.width())
                .map(|x| shadow[y1][x]._eq(&shadow[y2][x]).not())
                .collect::<Vec<_>>();

            let compare = (0..compare.len())
                .map(|index| &compare[index])
                .collect::<Vec<_>>();

            solver.assert(&z3::ast::Bool::or(ctx, compare.as_slice()));
        }
    }

    // Unique columns
    for x1 in 0..puzzle.width() {
        for x2 in x1 + 1..puzzle.width() {
            let compare = (0..puzzle.height())
                .map(|y| shadow[y][x1]._eq(&shadow[y][x2]).not())
                .collect::<Vec<_>>();

            let compare = (0..compare.len())
                .map(|index| &compare[index])
                .collect::<Vec<_>>();

            solver.assert(&z3::ast::Bool::or(ctx, compare.as_slice()));
        }
    }
}
