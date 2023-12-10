use ratatui::layout::Rect;

use crate::app::{event::Event, tui::Frame};

pub mod home;
pub mod text_input;

pub trait Component {
    /// Handle incoming events.
    ///
    /// Returns `true` if the event was handled.
    fn handle_event(&mut self, event: &Event) -> bool;

    /// Render the component on the screen.
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect);
}
