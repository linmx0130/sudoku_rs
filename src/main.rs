mod matrix;
mod solver;
mod generator;

use solver::solve_sudoku;
use generator::create_matrix;

fn main() {
    let mut mat = create_matrix(20);
    mat.print();
    
    // solve steps
    if solve_sudoku(&mut mat) {
        mat.print();
    } else {
        println!("No solution!");
    }
}
