use crate::matrix::SudokuMatrix;
use crate::solver::{SudokuSolverState, solve_sudoku};
use rand::prelude::*;
use rand::seq::SliceRandom;

/**
 * Generate a valid, solable Sudoku matrix with a specified number of filled cells.
 *
 * # Arguments
 * * `filled_cnt` - The number of cells to remain filled in the returned matrix.
 *
 * # Returns
 * A SudokuMatrix with exactly `filled_cnt` cells filled with valid values.
 * There must be at least one solution for this matrix.
 */
pub fn create_matrix(filled_cnt: usize) -> SudokuMatrix {
    loop {
        let mut mat = create_seed_matrix(15);
        if solve_sudoku(&mut mat, false) {
            let mut rng = rand::rng();
            let mut idx: Vec<usize> = (0..81).collect();
            idx.shuffle(&mut rng);
            for to_remove in idx.iter().take(81).skip(filled_cnt) {
                let x = to_remove / 9;
                let y = to_remove % 9;
                mat.set_value(x, y, 0);
            }
            return mat;
        }
    }
}

fn create_seed_matrix(filled_cnt: usize) -> SudokuMatrix {
    let mut mat = SudokuMatrix::new();
    let mut state = SudokuSolverState::init_state_from_matrix(&mat);
    let mut rng = rand::rng();
    for fill_idx in 0..filled_cnt {
        let pid = (rng.random::<u32>() as usize) % (81 - fill_idx);
        if let Some((r, c)) = get_empty_cell_coordinate_by_count(&mat, pid) {
            let avail_vals: Vec<u8> = state.avail_vals[r][c].iter().copied().collect();
            if avail_vals.is_empty() {
                continue;
            }
            let v = avail_vals.choose(&mut rng).unwrap();
            mat.set_value(r, c, *v);
            state.update_with_new_value(r, c, *v);
        }
    }
    mat
}

fn get_empty_cell_coordinate_by_count(mat: &SudokuMatrix, count: usize) -> Option<(usize, usize)> {
    let mut count = count;
    for i in 0..9 {
        for j in 0..9 {
            if mat.get_value(i, j) == 0 {
                if count == 0 {
                    return Some((i, j));
                } else {
                    count -= 1
                }
            }
        }
    }
    None
}
