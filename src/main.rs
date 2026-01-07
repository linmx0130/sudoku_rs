mod matrix;

use std::collections::HashSet;
use matrix::SudokuMatrix;

#[derive(Debug, Clone)]
struct SudokuSolverState {
    avail_vals: Vec<Vec<HashSet<u8>>>
}

impl SudokuSolverState {
    fn new() -> Self {
        let mut avail_vals = vec![];
        for i in 0..9 {
            avail_vals.push(vec![]);
            for _ in 0..9 {
                let mut set = HashSet::new();
                for v in 1u8..10u8 {
                    set.insert(v);
                }
                avail_vals[i].push(set);
            }
        }
        SudokuSolverState {
            avail_vals
        }
    }
    
    fn init_state_from_matrix(mat: &SudokuMatrix) -> SudokuSolverState {
        // create state
        let mut state = SudokuSolverState::new();
        for r in 0..9 {
            for c in 0..9 {
                // if the value is given, clean the state.
                if mat.get_value(r, c) != 0 {
                    state.avail_vals[r][c].clear();
                } else {
                    let block_r = r / 3;
                    let block_c = c / 3;
                    
                    for idx in 0..9 {
                        state.avail_vals[r][c].remove(&mat.get_value(r, idx));
                        state.avail_vals[r][c].remove(&mat.get_value(idx, c));
                        state.avail_vals[r][c].remove(&mat.get_value(block_r * 3 + idx / 3, block_c * 3 + idx % 3));
                    }
                }
            }
        }
        state
    }

    fn update_with_new_value(&mut self, r: usize, c: usize, v: u8) {
        let block_r = r / 3;
        let block_c = c / 3;
        self.avail_vals[r][c].clear();
        for idx in 0..9 {
            self.avail_vals[r][idx].remove(&v);
            self.avail_vals[idx][c].remove(&v);
            self.avail_vals[block_r * 3 + idx / 3][block_c * 3 + idx % 3].remove(&v);
        }
    }
}

fn solve_sudoku_derive(mat: &mut SudokuMatrix, state: &mut SudokuSolverState) -> bool {
    let mut updated = false;
    for r in 0..9 {
        for c in 0..9 {
            if state.avail_vals[r][c].len() == 1 {
                let v = state.avail_vals[r][c].iter().next().unwrap();
                println!("Set pos ({}, {}) to {}", r, c, v);
                mat.set_value(r, c, *v);
                state.update_with_new_value(r, c, *v);
                updated = true;
            }
        }
    }
    updated
}

fn solve_sudoku_derive_until_no_change(mat: &mut SudokuMatrix, state: &mut SudokuSolverState) {
    while !mat.is_complete() {
        if solve_sudoku_derive(mat, state) {
            mat.print();
        } else {
            break;
        }
        if !mat.is_compatible() {
            println!("Failed!");
            break;
        }
    }
}

fn solve_sudoku(mat: &mut SudokuMatrix) -> bool {
    mat.print();
    if !mat.is_compatible() {
        return false;
    }
    let mut state = SudokuSolverState::init_state_from_matrix(mat);
    solve_sudoku_derive_until_no_change(mat, &mut state);
    if mat.is_complete() {
        return true;
    }
    let mut candidate: Option<(usize, usize)> = None;
    let mut candidate_options = 10;
    for i in 0..9 {
        for j in 0..9 {
            if mat.get_value(i, j) == 0 {
                let avail_cnt = state.avail_vals[i][j].len();
                if avail_cnt == 0 {
                    return false;
                }
                if avail_cnt < candidate_options {
                    candidate_options = avail_cnt;
                    candidate = Some((i, j));
                }
            }
        }
    }
    if let Some((cr, cc)) = candidate {
        for v in state.avail_vals[cr][cc].iter() {
            println!("Try set ({}, {}) to {}", cr, cc, v);
            let mut new_mat = mat.clone();
            new_mat.set_value(cr, cc, *v);
            if solve_sudoku(&mut new_mat) {
                *mat = new_mat;
                return true;
            } else {
                println!("Trial failed");
            }
        }
        false
    } else {
        false
    }
}

fn main() {
    let mut mat = SudokuMatrix::new();
    mat.set_value(0, 0, 2);
    mat.set_value(0, 4, 6);
    mat.set_value(0, 5, 5);
    mat.set_value(0, 8, 7);
    mat.set_value(1, 2, 6);
    mat.set_value(1, 4, 3);
    mat.set_value(1, 6, 5);
    mat.set_value(1, 7, 1);
    mat.set_value(1, 8, 4);
    mat.set_value(2, 1, 9);
    mat.set_value(2, 3, 8);
    mat.set_value(2, 5, 7);
    mat.set_value(2, 7, 3);
    mat.set_value(3, 1, 7);
    mat.set_value(3, 3, 6);
    mat.set_value(3, 4, 2);
    mat.set_value(3, 5, 8);
    mat.set_value(4, 2, 5);
    mat.set_value(4, 6, 4);
    mat.set_value(4, 8, 9);
    mat.set_value(5, 1, 3);
    mat.set_value(5, 2, 8);
    mat.set_value(5, 3, 5);
    mat.set_value(5, 4, 9);
    mat.set_value(6, 2, 1);
    mat.set_value(6, 4, 5);
    mat.set_value(6, 5, 2);
    mat.set_value(6, 6, 7);
    mat.set_value(6, 7, 6);
    mat.set_value(7, 0, 8);
    mat.set_value(7, 2, 7);
    mat.set_value(7, 3, 3);
    mat.set_value(7, 8, 1);
    mat.set_value(8, 1, 6);
    mat.set_value(8, 2, 2);
    mat.set_value(8, 3, 7);
    mat.set_value(8, 8, 5);
    
    // solve steps
    if solve_sudoku(&mut mat) {
        mat.print();
    } else {
        println!("No solution!");
    }
}
