use rand::{distributions::OpenClosed01, prelude::*};

use crate::deviation::std_deviation;
use crate::sudoku::Sudoku;

pub trait SudokuSolver {
    fn solve(&self, initial: &Sudoku) -> Sudoku;
}

pub struct SudokuState {
    pub sudoku: Sudoku,
    pub difference: i32,
}

pub struct SASolver {
    cooling_rate: f64,
}

impl SASolver {
    pub fn calculate_number_of_iterations(fixed_sudoku: &Sudoku) -> u32 {
        let mut res = 0;
        for i in 0..9 {
            for j in 0..9 {
                if fixed_sudoku[i][j] != 0 {
                    res += 1;
                }
            }
        }
        res
    }

    pub fn calculate_initial_temp(sudoku: &Sudoku) -> Option<f32> {
        let mut list_of_differences = [0; 9];

        for i in 0..9 {
            let mut tmp_sudoku = sudoku.clone();
            tmp_sudoku.randomly_fill();
            list_of_differences[i] = tmp_sudoku.calculate_errors() as i32;
        }
        std_deviation(&list_of_differences)
    }

    pub fn choose_new_state(
        current_sudoku: &Sudoku,
        fixed_sudoku: &Sudoku,
        temp: f64,
    ) -> SudokuState {
        let random_block = Sudoku::get_random_block_positions();
        let swap_positions = Sudoku::get_two_random_positions_in_block(fixed_sudoku, &random_block);
        let mut new_sudoku = current_sudoku.clone();
        new_sudoku.flip_elements(swap_positions);

        let current_cost = current_sudoku
            .calculate_number_of_errors_for_position(swap_positions.0 .0, swap_positions.0 .1)
            + current_sudoku
                .calculate_number_of_errors_for_position(swap_positions.1 .0, swap_positions.1 .1);
        let new_cost = new_sudoku
            .calculate_number_of_errors_for_position(swap_positions.0 .0, swap_positions.0 .1)
            + new_sudoku
                .calculate_number_of_errors_for_position(swap_positions.1 .0, swap_positions.1 .1);
        let diff = new_cost - current_cost;
        let accept_prob = (-(diff as f64) / temp).exp();
        if thread_rng().sample::<f64, _>(OpenClosed01) < accept_prob {
            SudokuState {
                sudoku: new_sudoku,
                difference: diff,
            }
        } else {
            SudokuState {
                sudoku: current_sudoku.clone(),
                difference: 0,
            }
        }
    }
}

impl Default for SASolver {
    fn default() -> SASolver {
        Self { cooling_rate: 0.99 }
    }
}

impl SudokuSolver for SASolver {
    fn solve(&self, initial: &Sudoku) -> Sudoku {
        let mut tmp_sudoku = initial.clone();

        let mut stuck_count = 0;
        let fixed_sudoku = Sudoku::get_fixed_values(&initial);
        tmp_sudoku.randomly_fill();

        let mut temp = Self::calculate_initial_temp(&initial)
            .expect("Failed to calculate initial temperature") as f64;
        let mut score: i32 = tmp_sudoku.calculate_errors();
        let iterations = SASolver::calculate_number_of_iterations(&fixed_sudoku);

        if score <= 0 {
            return tmp_sudoku;
        }

        loop {
            let previous_score = score;

            for _ in 0..iterations {
                let new_state = SASolver::choose_new_state(&tmp_sudoku, &fixed_sudoku, temp);
                tmp_sudoku = new_state.sudoku;
                let score_diff = new_state.difference;
                score += score_diff;

                if score <= 0 {
                    return tmp_sudoku;
                }
            }

            temp *= self.cooling_rate;

            if score >= previous_score {
                stuck_count += 1;
            } else {
                stuck_count = 0;
            }

            if stuck_count > 80 {
                temp += 2.0;
            }
        }
    }
}
