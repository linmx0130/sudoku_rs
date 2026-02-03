use crate::matrix::SudokuMatrix;
use std::collections::HashSet;

/**
 * Sudoku solver internal state
 *
 * This structure keeps track of the possible values for each cell
 * during the solving process.
 */
#[derive(Debug, Clone)]
pub struct SudokuSolverState {
    pub avail_vals: Vec<Vec<HashSet<u8>>>,
    pub print_debug_info: bool,
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
            avail_vals,
            print_debug_info: false,
        }
    }

    pub fn init_state_from_matrix(mat: &SudokuMatrix) -> SudokuSolverState {
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
                        state.avail_vals[r][c]
                            .remove(&mat.get_value(block_r * 3 + idx / 3, block_c * 3 + idx % 3));
                    }
                }
            }
        }
        state
    }

    pub fn update_with_new_value(&mut self, r: usize, c: usize, v: u8) {
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
                if state.print_debug_info {
                    println!("Set pos ({}, {}) to {}", r, c, v);
                }
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
            if state.print_debug_info {
                mat.print();
            }
        } else {
            break;
        }
        if !mat.is_compatible() {
            if state.print_debug_info {
                println!("Failed!");
            }
            break;
        }
    }
}

/**
 * Solve a partially-filled Sudoku puzzle by back-tracking.
 *
 * Return true on success, false on failure.
 */
pub fn solve_sudoku(mat: &mut SudokuMatrix, print_debug_info: bool) -> bool {
    if print_debug_info {
        mat.print();
    }
    if !mat.is_compatible() {
        return false;
    }
    let mut state = SudokuSolverState::init_state_from_matrix(mat);
    if print_debug_info {
        state.print_debug_info = true;
    }
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
            if state.print_debug_info {
                println!("Try set ({}, {}) to {}", cr, cc, v);
            }
            let mut new_mat = mat.clone();
            new_mat.set_value(cr, cc, *v);
            if solve_sudoku(&mut new_mat, print_debug_info) {
                *mat = new_mat;
                return true;
            } else if state.print_debug_info {
                println!("Trial failed");
            }
        }
        false
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_state_from_matrix() {
        let mut mat = SudokuMatrix::new();
        mat.set_value(0, 0, 5);
        mat.set_value(0, 1, 3);
        let state = SudokuSolverState::init_state_from_matrix(&mat);
        assert_eq!(state.avail_vals[0][0].len(), 0);
        assert!(!state.avail_vals[0][2].contains(&5));
        assert!(!state.avail_vals[0][2].contains(&3));
    }

    #[test]
    fn test_derive_from_state_happy_path() {
        let mut mat = SudokuMatrix::new();
        // Fill row 0 with values 1-8, leaving the last cell empty
        for c in 0..8 {
            mat.set_value(0, c, (c + 1) as u8);
        }
        let mut state = SudokuSolverState::init_state_from_matrix(&mat);
        // The last cell in row 0 should now be derivable (must be 9)
        assert!(solve_sudoku_derive(&mut mat, &mut state));
        assert_eq!(mat.get_value(0, 8), 9);
        // The state for that cell should now be empty (since it's filled)
        assert_eq!(state.avail_vals[0][8].len(), 0);
    }
}
