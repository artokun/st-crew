use crossterm::event::{KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use ratatui::{prelude::*, widgets::*};

use crate::app::{event::Event, AppCx};

use super::{Component, Frame};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    Normal,
    Editing,
}

pub struct TextInput {
    placeholder: String,

    input: String,
    cursor_position: usize,
    input_mode: InputMode,

    area: Rect,

    app: AppCx,
}

impl TextInput {
    pub fn new(app: impl Into<AppCx>) -> Self {
        Self {
            placeholder: String::new(),

            input: String::new(),
            cursor_position: 0,
            input_mode: InputMode::Normal,

            area: Rect::default(),

            app: app.into(),
        }
    }

    pub fn with_placeholder(self, placeholder: String) -> Self {
        Self {
            placeholder,
            ..self
        }
    }

    pub fn set_placeholder(&mut self, placeholder: String) -> &mut Self {
        self.placeholder = placeholder;
        self
    }

    pub fn set_input(&mut self, input: String) -> &mut Self {
        self.input = input;
        self.reset_cursor();
        self
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn submit_message(&mut self) {
        self.input.clear();
        self.reset_cursor();
    }
}

impl Component for TextInput {
    fn handle_event(&mut self, event: &Event) -> bool {
        if self.input_mode == InputMode::Editing {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),

                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                        }

                        _ => {}
                    }
                }

                self.app.render();

                return true;
            }
        } else if let Event::Mouse(mouse) = event {
            if mouse.kind == MouseEventKind::Down(MouseButton::Left)
                && self
                    .area
                    .intersects(Rect::new(mouse.column, mouse.row, 1, 1))
            {
                self.input_mode = InputMode::Editing;

                return true;
            }
        }

        false
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        self.area = area;

        if self.input.is_empty() {
            f.render_widget(
                Paragraph::new(self.placeholder.as_str()).set_style(Style::new().fg(
                    match self.input_mode {
                        InputMode::Normal => Color::DarkGray,
                        InputMode::Editing => Color::Gray,
                    },
                )),
                area,
            );

            return;
        }

        f.render_widget(
            Paragraph::new(self.input.as_str()).set_style(Style::new().fg(match self.input_mode {
                InputMode::Normal => Color::Gray,
                InputMode::Editing => Color::White,
            })),
            area,
        );
    }
}
