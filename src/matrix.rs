#[derive(Clone, Debug, Default)]
/**
 * Data class to host a sudoku matrix.
 */
pub struct SudokuMatrix {
    matrix: [[u8; 9]; 9],
}

impl SudokuMatrix {
    /**
     * Creates a new empty Sudoku matrix.
     */
    pub fn new() -> Self {
        SudokuMatrix {
            matrix: [[0u8; 9]; 9],
        }
    }

    /**
     * Set value on a cell. A value of `0` will clear the cell.
     *
     * # Arguments
     * * `r` row
     * * `c` column
     * * `v` the value to be set
     */
    pub fn set_value(&mut self, r: usize, c: usize, v: u8) {
        self.matrix[r][c] = v;
    }

    /**
     * Get the value on a cell. A value of `0` means the cell is not filled.
     *
     * # Arguments
     * * `r` row
     * * `c` column
     */
    pub fn get_value(&self, r: usize, c: usize) -> u8 {
        self.matrix[r][c]
    }

    pub fn print(&self) {
        println!("   0 1 2 3 4 5 6 7 8");
        for i in 0..9 {
            print!("  ");
            for j in 0..9 {
                if i % 3 == 0 && j % 3 == 0 {
                    print!("*-");
                } else {
                    print!("--");
                }
            }
            if i % 3 == 0 {
                print!("*\n{} |", i);
            } else {
                print!("-\n{} |", i);
            }
            for j in 0..9 {
                if self.matrix[i][j] != 0 {
                    print!("{}|", self.matrix[i][j]);
                } else {
                    print!(" |");
                }
            }
            println!();
        }
        print!("  ");
        for j in 0..9 {
            if j % 3 == 0 {
                print!("*-");
            } else {
                print!("--");
            }
        }
        println!("*");
    }

    /**
     * Determine whether the matrix is fully filled.
     */
    pub fn is_complete(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.matrix[i][j] == 0 {
                    return false;
                }
            }
        }
        true
    }

    /**
     * Determine whether there is a conflict in the matrix. Return `true` if there is no conflict.
     */
    pub fn is_compatible(&self) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if self.matrix[r][c] != 0 {
                    for idx in 0..9 {
                        if idx != c && self.matrix[r][idx] == self.matrix[r][c] {
                            return false;
                        }
                        if idx != r && self.matrix[idx][c] == self.matrix[r][c] {
                            return false;
                        }
                        let block_r = r / 3;
                        let block_c = c / 3;
                        let rr = block_r * 3 + idx / 3;
                        let cc = block_c * 3 + idx % 3;
                        if (rr != r || cc != c) && self.matrix[rr][cc] == self.matrix[r][c] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let m = SudokuMatrix::new();
        assert!(m.is_complete() == false);
        assert!(m.is_compatible());
    }

    #[test]
    fn test_set_and_get() {
        let mut m = SudokuMatrix::new();
        m.set_value(0, 0, 5);
        assert_eq!(m.get_value(0, 0), 5);
        m.set_value(8, 8, 9);
        assert_eq!(m.get_value(8, 8), 9);
    }

    #[test]
    fn test_is_complete() {
        let mut m = SudokuMatrix::new();
        assert!(!m.is_complete());
        for i in 0..9 {
            for j in 0..9 {
                m.set_value(i, j, 1);
            }
        }
        assert!(m.is_complete());
    }

    #[test]
    fn test_is_compatible_rows() {
        let mut m = SudokuMatrix::new();
        m.set_value(0, 0, 5);
        m.set_value(0, 1, 5);
        assert!(!m.is_compatible());
    }

    #[test]
    fn test_is_compatible_cols() {
        let mut m = SudokuMatrix::new();
        m.set_value(0, 0, 5);
        m.set_value(1, 0, 5);
        assert!(!m.is_compatible());
    }

    #[test]
    fn test_is_compatible_blocks() {
        let mut m = SudokuMatrix::new();
        m.set_value(1, 1, 5);
        m.set_value(2, 2, 5);
        assert!(!m.is_compatible());
    }

    #[test]
    fn test_is_compatible_valid() {
        let mut m = SudokuMatrix::new();
        m.set_value(0, 0, 1);
        m.set_value(0, 1, 2);
        m.set_value(0, 2, 3);
        m.set_value(1, 0, 4);
        m.set_value(1, 1, 5);
        m.set_value(1, 2, 6);
        m.set_value(2, 0, 7);
        m.set_value(2, 1, 8);
        m.set_value(2, 2, 9);
        assert!(m.is_compatible());
    }
}
