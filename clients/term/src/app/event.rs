use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Clone, Debug)]
pub enum Event {
    Quit,

    FocusGained,
    FocusLost,

    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}
