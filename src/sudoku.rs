use rand::prelude::*;
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Clone, Debug)]
pub struct Sudoku {
    elements: [[u32; 9]; 9],
}

impl Sudoku {
    pub fn create_blocks() -> [[(usize, usize); 9]; 9] {
        let mut sudoku_blocks = [[(0, 0); 9]; 9];
        let mut blocks_iter = sudoku_blocks.iter_mut();
        for block_index in 0..9 {
            let mut block_x = [0; 3];
            let mut block_y = [0; 3];
            for i in 0..3 {
                block_x[i] = i + 3 * (block_index % 3);
                block_y[i] = i + 3 * (block_index / 3);
            }
            let mut block_elements = [(0, 0); 9];
            let mut elems_iter = block_elements.iter_mut();
            for x in block_x {
                for y in block_y {
                    let elem = elems_iter.next().unwrap();
                    *elem = (x, y);
                }
            }
            *(blocks_iter.next().unwrap()) = block_elements;
        }
        sudoku_blocks
    }

    pub fn get_two_random_positions_in_block(
        fixed_sudoku: &Sudoku,
        block: &[(usize, usize); 9],
    ) -> ((usize, usize), (usize, usize)) {
        let mut first_box = block.choose(&mut rand::thread_rng()).unwrap();
        while fixed_sudoku[first_box.0][first_box.1] == 1 {
            first_box = block.choose(&mut rand::thread_rng()).unwrap();
        }

        let mut second_box = block.choose(&mut rand::thread_rng()).unwrap();
        while fixed_sudoku[second_box.0][second_box.1] == 1 || first_box == second_box {
            second_box = block.choose(&mut rand::thread_rng()).unwrap();
        }

        (*first_box, *second_box)
    }

    pub fn flip_elements(&mut self, positions: ((usize, usize), (usize, usize))) {
        let elem = self[positions.0 .0][positions.0 .1];
        self[positions.0 .0][positions.0 .1] = self[positions.1 .0][positions.1 .1];
        self[positions.1 .0][positions.1 .1] = elem;
    }

    pub fn get_random_block_positions() -> [(usize, usize); 9] {
        let index = (0..9).choose(&mut rand::thread_rng()).unwrap();
        let mut block_x = [0; 3];
        let mut block_y = [0; 3];
        for i in 0..3 {
            block_x[i] = i + 3 * (index % 3);
            block_y[i] = i + 3 * (index / 3);
        }
        let mut block = [(0, 0); 9];
        let mut block_iter = block.iter_mut();
        for x in block_x {
            for y in block_y {
                let elem = block_iter.next().unwrap();
                *elem = (x, y);
            }
        }

        block
    }

    pub fn randomly_fill(&mut self) {
        let blocks = Sudoku::create_blocks();
        for block in blocks {
            let mut current_block_missing_numbers: HashSet<u32> =
                [1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
            for elem in block {
                if self[elem.0][elem.1] != 0 {
                    current_block_missing_numbers.remove(&(self[elem.0][elem.1]));
                }
            }
            let mut current_block_missing_numbers: Vec<u32> =
                current_block_missing_numbers.into_iter().collect();
            current_block_missing_numbers.shuffle(&mut rand::thread_rng());
            for place in block {
                if self[place.0][place.1] == 0 {
                    self[place.0][place.1] = current_block_missing_numbers.pop().unwrap();
                }
            }
        }
    }

    pub fn calculate_errors(&self) -> i32 {
        let mut res = 0;
        for i in 0..9 {
            //row
            let unique_r: HashSet<_> = self[i].iter().collect();
            res += 9 - unique_r.len();

            //column
            let mut unique_c: HashSet<_> = HashSet::new();
            for j in 0..9 {
                unique_c.insert(self[j][i]);
            }
            res += 9 - unique_c.len();
        }
        res as i32
    }

    pub fn calculate_number_of_errors_for_position(&self, row: usize, column: usize) -> i32 {
        let unique_in_row: HashSet<_> = self[row].iter().collect();
        let mut unique_in_column: HashSet<_> = HashSet::new();

        for i in 0..9 {
            unique_in_column.insert(self[i][column]);
        }
        18 - unique_in_column.len() as i32 - unique_in_row.len() as i32
    }

    pub fn get_fixed_values(&self) -> Sudoku {
        let mut fixed = Sudoku::default();
        for i in 0..9 {
            for j in 0..9 {
                if self[i][j] != 0 {
                    fixed[i][j] = 1;
                }
            }
        }
        fixed
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self {
            elements: [[0; 9]; 9],
        }
    }
}

impl FromStr for Sudoku {
    type Err = String;
    fn from_str(s: &str) -> Result<Sudoku, Self::Err> {
        let mut elements = [[0; 9]; 9];
        let chars: Vec<_> = s
            .split_whitespace()
            .map(|str| str.chars().collect::<Vec<_>>())
            .collect();
        for i in 0..9 {
            for j in 0..9 {
                if chars[i][j] != '0' && chars[i][j].is_alphanumeric() {
                    elements[i][j] = chars[i][j].to_digit(10).ok_or("Wrong input")?;
                }
            }
        }
        Ok(Sudoku { elements })
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            if i != 0 && i % 3 == 0 {
                write!(f, "{}\n", ["-"; 21].concat())?;
            }
            for j in 0..9 {
                if j % 3 == 0 && j != 0 {
                    write!(f, "| ")?;
                }
                write!(f, "{} ", self.elements[i][j])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Index<usize> for Sudoku {
    type Output = [u32; 9];

    fn index(&self, index: usize) -> &Self::Output {
        self.elements.index(index)
    }
}

impl IndexMut<usize> for Sudoku {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.elements.index_mut(index)
    }
}
