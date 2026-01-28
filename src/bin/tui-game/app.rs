use std::io;
use sudoku_lib::{SudokuMatrix, solve_sudoku, create_matrix};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use ratatui::{
    layout::{Rect, Alignment, Constraint, Layout, Spacing},
    style::{Stylize, Color},
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug)]
pub struct App {
    matrix: SudokuMatrix,
    is_original_matrix: [bool;81],
    cursor_pos: usize,
    exit:bool
}

impl App {
    pub fn new() -> Self {
        let mat = create_matrix(25);
        let mut is_original_matrix = [false; 81];
        for (i, mut_ref) in is_original_matrix.iter_mut().enumerate() {
            let x = i / 9 ;
            let y = i % 9;
            if mat.get_value(x, y) != 0 {
                *mut_ref = true;
            }
        }
        App {
            matrix: mat,
            is_original_matrix,
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
        self.render_instruction(frame, bottom_area);
        self.render_matrix(frame, main_area);
    }

    fn fill_value(&mut self, value: u32) {
        if self.is_original_matrix[self.cursor_pos] {
            return;
        }
        let r = self.cursor_pos / 9;
        let c = self.cursor_pos % 9 ;
        self.matrix.set_value(r, c, value.try_into().unwrap());
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
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match key_event.code {
                KeyCode::Char('r') => self.reset_matrix(),
                KeyCode::Char('a') => self.solve_matrix(),
                _ => {}
            }
            return;
        }
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.fill_value(c.to_digit(10).unwrap());
            }
            KeyCode::Left => {
                if !self.cursor_pos.is_multiple_of(9) {
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

    fn reset_matrix(&mut self) {
        for i in 0..81 {
            let r = i / 9;
            let c = i % 9;
            if !self.is_original_matrix[i] {
                self.matrix.set_value(r, c, 0);
            }
        }
    }

    fn solve_matrix(&mut self) {
        solve_sudoku(&mut self.matrix, false);
    }
    
    // Split the layout into a top row, middle space and a bottom row
    fn calculate_main_layout(&mut self, area: Rect) -> (Rect, Rect, Rect) {
        let main_layout = Layout::vertical(
            [
                Constraint::Length(1),
                Constraint::Min(19),
                Constraint::Length(2)
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

    fn render_instruction(&self, frame: &mut Frame, area: Rect) {
        let conflit_label = if self.matrix.is_compatible() {
            Line::from(vec!["".into()])
        } else {
            Line::from(vec!["CONFLICT!".white().bold().bg(Color::Red)])
        };
        let instructions = Line::from(vec![
            "Reset ".into(),
            "<C-R>".blue().bold(),
            " Solve ".into(),
            "<C-A>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold()
        ]);
        frame.render_widget(
            Paragraph::new(vec![conflit_label, instructions])
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
            let v_line = if self.is_original_matrix[i] {
                v_text.bold()
            } else {
                v_text.into()
            };
            let cell_widget = if self.cursor_pos == i {
                Paragraph::new(Line::from(vec![v_line.bg(Color::DarkGray)]))
                    .block(Block::bordered().merge_borders(MergeStrategy::Exact))
            } else {
                Paragraph::new(Line::from(vec![v_line]))
                    .block(Block::bordered().merge_borders(MergeStrategy::Exact))
            };
            frame.render_widget(cell_widget, cell);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}


