
use crate::matrix::SudokuMatrix;
use crate::solver::SudokuSolverState;
use rand::prelude::*;

pub fn create_matrix(filled_cnt: usize) -> SudokuMatrix {
    let mut mat = SudokuMatrix::new();
    let mut state = SudokuSolverState::init_state_from_matrix(&mat);
    let mut rng = rand::rng();
    for fill_idx in 0..filled_cnt {
        let pid = (rng.random::<u32>() as usize) % (81 - fill_idx);
        if let Some((r, c)) = get_empty_cell_coordinate_by_count(&mat, pid) {
            let avail_vals: Vec<u8> = state.avail_vals[r][c].iter().copied().collect();
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
                    return Some((i, j))
                } else {
                    count -= 1
                }
            }
        }
    }
    None
}
