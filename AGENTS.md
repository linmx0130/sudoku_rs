# Sudoku Project - Agent Guide

## Project Overview

This is a Rust-based Sudoku library and TUI (Terminal User Interface) game implementation.

## Architecture

```
src/
├── lib.rs          # Library entry point, re-exports public APIs
├── matrix.rs       # SudokuMatrix: Core data structure for 9x9 grid
├── solver.rs       # Sudoku solver using backtracking algorithm
├── generator.rs    # Puzzle generator with configurable difficulty
└── bin/
    └── tui-game/   # Terminal UI game binary
        ├── main.rs # CLI entry with clap argument parsing
        └── app.rs  # TUI event loop and rendering
```

## Key Components

### SudokuMatrix (`matrix.rs`)
- 9x9 grid stored as `[[u8; 9]; 9]`
- Value `0` represents empty cell
- Methods: `new()`, `set_value()`, `get_value()`, `is_complete()`, `is_compatible()`

### Solver (`solver.rs`)
- `solve_sudoku(mat, debug)` - Solves puzzle using backtracking
- `SudokuSolverState` - Tracks possible values for each cell

### Generator (`generator.rs`)
- `create_matrix(filled_cnt)` - Generates puzzle with specified filled cells

### TUI Game (`bin/tui-game/`)
- Built with `ratatui` + `crossterm`
- Controls: Arrow keys (move), 0-9 (input), Ctrl+R (reset), Ctrl+A (solve), Q (quit)

## Build & Run

```bash
# Build library
cargo build

# Run TUI game
cargo run --bin tui-game

# Run with custom difficulty (filled cells)
cargo run --bin tui-game -- --filled 30

# Run tests
cargo test
```

## Dependencies

- `rand` - Random number generation
- `clap` - CLI argument parsing
- `ratatui` - Terminal UI framework
- `crossterm` - Cross-platform terminal control

## Testing

Unit tests are in `matrix.rs` covering:
- Matrix creation and manipulation
- Conflict detection (rows, columns, blocks)
- Completion checking

## Code Style

- Rust 2024 edition
- Doc comments with `/** */` style
- Public APIs documented with `# Arguments` and `# Returns` sections
