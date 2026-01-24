use sudoku_lib::{SudokuMatrix, solve_sudoku, create_matrix};
use std::io;
use std::io::Write;


fn generate_valid_matrix(filled: usize) -> SudokuMatrix{
    loop {
        let mat = create_matrix(filled);
        let mut mmat = mat.clone();
        if solve_sudoku(&mut mmat, false) {
            return mat;
        }
    }
}

fn print_game_state(mat: &SudokuMatrix) {
    mat.print();
    if !mat.is_compatible() {
        println!("The matrix is not compatible!!!");
    }
}

fn check_solvability(mat: &SudokuMatrix) {
    let mut vmat = mat.clone();
    if !solve_sudoku(&mut vmat, false) {
        println!("This matrix is not solvable!");
    }
}

fn process_solve_command(mat: &mut SudokuMatrix) -> bool {
    if !solve_sudoku(mat, true) {
        println!("No solution found!");
    }
    true
}

fn process_reset_command(mat: &mut SudokuMatrix, original_mat: &SudokuMatrix) {
    *mat = original_mat.clone();
}

fn process_cell_input(mat: &mut SudokuMatrix, original_mat: &SudokuMatrix, input: &str) {
    let splitted: Vec<&str> = input.trim().split(" ").collect();
    if splitted.len() >= 3 {
        if let (Ok(r), Ok(c), Ok(val)) = (
            splitted[0].parse::<usize>(),
            splitted[1].parse::<usize>(),
            splitted[2].parse::<u8>()
        ) {
            if original_mat.get_value(r, c) != 0 {
                println!("({}, {}) cannot be set.", r, c);
            } else {
                mat.set_value(r, c, val);
            }
        }
    } else {
        println!("Input should be in the format: [row] [col] [value]");
    }
}

fn read_user_input() -> String {
    let mut input = String::new();
    print!("> ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}

pub fn cli_main_loop() {
    let mut mat = generate_valid_matrix(25);
    let original_mat = mat.clone();
    
    println!("> solve    => Solve the sudoku");
    println!("> reset    => Reset to the original matrix");
    println!("Input in the format of: [r] [c] [val]");

    while !mat.is_complete() {
        print_game_state(&mat);
        check_solvability(&mat);
        
        let input = read_user_input();
        
        if input.starts_with("solve") {
            if process_solve_command(&mut mat) {
                break;
            }
        } else if input.starts_with("reset") {
            process_reset_command(&mut mat, &original_mat);
        } else {
            process_cell_input(&mut mat, &original_mat, &input);
        }
    }
}