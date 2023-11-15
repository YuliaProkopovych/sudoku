use std::{str::FromStr, time::Instant};

mod solver;
mod sudoku;
use solver::{SASolver, SudokuSolver};
use sudoku::Sudoku;

const SUDOKU: &str = "
                    024007000
                    600000000
                    003680415
                    431005000
                    500000032
                    790000060
                    209710800
                    040093000
                    310004750
                    ";
const SUDOKU1: &str = "
                    008002000
                    006000095
                    030050000
                    200080700
                    740000003
                    000000400
                    000800630
                    000300010
                    000724000
                    ";
const SUDOKU3: &str = "
                250000004
                000050009
                080300250
                000000002
                030007000
                800040160
                100060580
                000000090
                006400000
                ";
fn main() {
    let sudoku = Sudoku::from_str(SUDOKU.trim()).unwrap();

    let solver = SASolver::default();
    let now = Instant::now();
    let s = solver.solve(&sudoku);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("{}", s);
}
