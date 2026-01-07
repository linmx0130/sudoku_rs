#[derive(Clone)]
pub struct SudokuMatrix{
    matrix: [[u8;9];9]
}

impl SudokuMatrix {
    pub fn new() -> Self {
        SudokuMatrix {
            matrix: [[0u8; 9]; 9]
        }
    }

    pub fn set_value(&mut self, r: usize, c: usize, v: u8) {
        self.matrix[r][c] = v;
    }

    pub fn get_value(&self, r: usize, c: usize) -> u8 {
        self.matrix[r][c]
    }

    pub fn print(&self) {
        for i in 0..9 {
            for _i in 0..9 {
                print!("--");
            }
            print!("-\n|");
            for j in 0..9 {
                if self.matrix[i][j] != 0 {
                    print!("{}|", self.matrix[i][j]);
                } else {
                    print!(" |");
                }
            }
            println!();
        }
        for _i in 0..9 {
            print!("--");
        }
        println!("-");
    }

    pub fn is_complete(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.matrix[i][j] == 0 {
                    return false
                }
            }
        }
        true
    }

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

