mod matrix;
mod solver;
mod generator;

use std::io;
use std::io::Write;
use matrix::SudokuMatrix;
use solver::solve_sudoku;
use generator::create_matrix;

fn generate_valid_matrix(filled: usize) -> SudokuMatrix{
    loop {
        let mat = create_matrix(filled);
        let mut mmat = mat.clone();
        if solve_sudoku(&mut mmat, false) {
            return mat;
        }
    }
}
fn main() {
    let mut mat = generate_valid_matrix(25);
    let original_mat = mat.clone();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    println!("> solve    => Solve the sudoku");
    println!("> reset    => Reset to the original matrix");
    println!("Input in the format of: [r] [c] [val]");

    while !mat.is_complete() {
        mat.print();
        if !mat.is_compatible() {
            println!("The matrix is not compatible!!!");
        } 
        {
            let mut vmat = mat.clone();
            if !solve_sudoku(&mut vmat, false) {
                println!("This matrix is not solvable!");
            }
        }
        let mut input = String::new();
        print!("> ");
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut input);
        if input.starts_with("solve") {
            if !solve_sudoku(&mut mat, true) {
                println!("No solution found!");
            }
            break;
        }
        if input.starts_with("reset") {
            mat = original_mat.clone();
        }
        let splitted: Vec<&str> = input.trim().split(" ").collect();
        let r: usize = splitted[0].parse().unwrap();
        let c: usize = splitted[1].parse().unwrap();
        let val: u8 = splitted[2].parse().unwrap();
        if original_mat.get_value(r, c) != 0 {
            println!("({}, {}) cannot be set.", r, c);
        } else {
            mat.set_value(r, c, val);
        }
    }
}
