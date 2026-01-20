mod matrix;
mod solver;
mod generator;
mod cli;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sudoku")]
#[command(about = "A Sudoku game with CLI interface", long_about = None)]
struct Args {
    /// Start the interactive CLI interface
    #[arg(long)]
    cli: bool,
}

fn main() {
    let args = Args::parse();
    
    if args.cli {
        cli::cli_main_loop();
    }
    std::process::exit(0);

}
