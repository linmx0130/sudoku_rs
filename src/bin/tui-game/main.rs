use std::io;
use sudoku_lib::{SudokuMatrix, solve_sudoku, create_matrix};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    buffer::Buffer,
    layout::{Rect, Alignment, Constraint, Layout, Spacing},
    style::{Stylize, Color},
    symbols::{border, merge::MergeStrategy},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn generate_valid_matrix(filled: usize) -> SudokuMatrix{
    loop {
        let mat = create_matrix(filled);
        let mut mmat = mat.clone();
        if solve_sudoku(&mut mmat, false) {
            return mat;
        }
    }
}

#[derive(Debug)]
pub struct App {
    matrix: SudokuMatrix,
    cursor_pos: usize,
    exit:bool
}

impl App {
    pub fn new() -> Self {
        let mut mat = generate_valid_matrix(25);
        App {
            matrix: mat,
            cursor_pos: 0,
            exit: false
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let (title_area, main_area, bottom_area) = self.calculate_main_layout(frame.area());
        Self::render_title(frame, title_area);
        Self::render_instruction(frame, bottom_area);
        self.render_matrix(frame, main_area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => {
                if self.cursor_pos % 9 > 0 {
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_pos % 9 < 8 {
                    self.cursor_pos += 1;
                }
            }
            KeyCode::Up => {
                if self.cursor_pos >= 9 {
                    self.cursor_pos -= 9;
                }
            }
            KeyCode::Down => {
                if self.cursor_pos < 72 {
                    self.cursor_pos += 9;
                }
            }

            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    
    // Split the layout into a top row, middle space and a bottom row
    fn calculate_main_layout(&mut self, area: Rect) -> (Rect, Rect, Rect) {
        let main_layout = Layout::vertical(
            [
                Constraint::Length(1),
                Constraint::Min(19),
                Constraint::Length(1)
            ]
        );
        let [top_row, main, bottom_row] = main_layout.areas(area);
        (top_row, main, bottom_row)
    }

    fn render_title(frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(" Sudoku ".bold())
                .alignment(Alignment::Center),
            area
        );
    }

    fn render_instruction(frame: &mut Frame, area: Rect) {
        let instructions = Line::from(vec![
            "Quit ".into(),
            "<Q>".blue().bold()
        ]);
        frame.render_widget(
            Paragraph::new(instructions)
                .alignment(Alignment::Center),
            area
        );
    }

    fn render_matrix(&self, frame: &mut Frame, area: Rect) {
        let centered_area = area.centered(
            Constraint::Length(37),
            Constraint::Length(19)
        );
        let col_constraints = (0..9).map(|_| Constraint::Length(5));
        let row_constraints = (0..9).map(|_| Constraint::Length(3));
        let horizontal = Layout::horizontal(col_constraints).spacing(Spacing::Overlap(1));
        let vertical = Layout::vertical(row_constraints).spacing(Spacing::Overlap(1));
        let rows = vertical.split(centered_area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let x = i / 9;
            let y = i % 9;
            let v = self.matrix.get_value(x, y);
            let v_text = if v == 0 {
                "   ".to_string()
            } else {
                format!(" {} ", v)
            };
            let cell_widget = if self.cursor_pos == i {
                Paragraph::new(Line::from(vec![v_text.bold().bg(Color::DarkGray)]))
                    .block(Block::bordered().merge_borders(MergeStrategy::Exact))
            } else {
                Paragraph::new(v_text)
                    .block(Block::bordered().merge_borders(MergeStrategy::Exact))
            };
            frame.render_widget(cell_widget, cell);
        }
    }
}

fn main() -> io::Result<()>{
    ratatui::run(|terminal| App::new().run(terminal))
}
