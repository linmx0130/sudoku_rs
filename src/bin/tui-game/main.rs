mod app;

use std::io;
use app::App;
use clap::Parser;

#[derive(Parser)]
#[command(name = "tui-game")]
#[command(about = "A TUI Sudoku game")]
struct Cli {
    /// Number of cells to be filled in the sudoku matrix
    #[arg(short, long, default_value_t = 25)]
    filled: usize,
}

fn main() -> io::Result<()>{
    let cli = Cli::parse();
    ratatui::run(|terminal| App::new(cli.filled).run(terminal))
}
