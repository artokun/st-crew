use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};

use crate::app::{event::Event, AppCx};

use super::{text_input::TextInput, Component, Frame};

pub struct Home {
    text_input: TextInput,

    app: AppCx,
}

impl Home {
    pub fn new(app: impl Into<AppCx>) -> Self {
        let app = app.into();

        Self {
            text_input: TextInput::new(&app).with_placeholder("Start typing...".into()),

            app,
        }
    }
}

impl Component for Home {
    fn handle_event(&mut self, event: &Event) -> bool {
        if self.text_input.handle_event(event) {
            return true;
        }

        if let Event::Key(key) = event {
            if key.code == KeyCode::Esc && self.app.quit() {
                return true;
            }
        }

        false
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        f.render_widget(Paragraph::new("hello world"), area);

        // Draw the text input at the bottom of the screen.
        self.text_input.draw(
            f,
            Rect::new(
                area.x,
                area.y + area.height.saturating_sub(1),
                area.width,
                1,
            ),
        );
    }
}
