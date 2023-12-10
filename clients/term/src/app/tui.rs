use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use color_eyre::eyre::Result;
use crossterm::{
    cursor,
    event::{
        DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
        Event as CrosstermEvent, KeyEventKind,
    },
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend as Backend;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::app::event::Event;

pub type Frame<'a> = ratatui::Frame<'a>;

pub struct Tui {
    pub terminal: ratatui::Terminal<Backend<std::io::Stdout>>,
    pub task: JoinHandle<()>,
    pub cancellation_token: CancellationToken,

    pub event_rx: UnboundedReceiver<Event>,
    pub event_tx: UnboundedSender<Event>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = ratatui::Terminal::new(Backend::new(std::io::stdout()))?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let cancellation_token = CancellationToken::new();
        let task = tokio::spawn(async {});

        Ok(Self {
            terminal,
            task,
            cancellation_token,
            event_rx,
            event_tx,
        })
    }

    pub fn start(&mut self) {
        self.cancel();

        self.cancellation_token = CancellationToken::new();
        let _cancellation_token = self.cancellation_token.clone();
        let _event_tx = self.event_tx.clone();

        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();

            loop {
                let crossterm_event = reader.next().fuse();

                let event = tokio::select! {
                  _ = _cancellation_token.cancelled() => {
                    break;
                  }

                  event = crossterm_event => event,
                };

                let Some(event) = event else {
                    continue;
                };

                match event {
                    Ok(evt) => match evt {
                        CrosstermEvent::Key(key) => {
                            if key.kind == KeyEventKind::Press {
                                _event_tx.send(Event::Key(key)).unwrap();
                            }
                        }

                        CrosstermEvent::Mouse(mouse) => {
                            _event_tx.send(Event::Mouse(mouse)).unwrap();
                        }

                        CrosstermEvent::Resize(x, y) => {
                            _event_tx.send(Event::Resize(x, y)).unwrap();
                        }

                        CrosstermEvent::FocusLost => {
                            _event_tx.send(Event::FocusLost).unwrap();
                        }

                        CrosstermEvent::FocusGained => {
                            _event_tx.send(Event::FocusGained).unwrap();
                        }

                        CrosstermEvent::Paste(s) => {
                            _event_tx.send(Event::Paste(s)).unwrap();
                        }
                    },

                    Err(err) => {
                        tracing::error!("{err:?}");
                    }
                }
            }
        });
    }

    pub fn stop(&self) -> Result<()> {
        self.cancel();

        let mut counter = 0;

        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));

            counter += 1;

            if counter > 50 {
                self.task.abort();
            }

            if counter > 100 {
                tracing::error!("Failed to abort task in 100 milliseconds for unknown reason");
                break;
            }
        }

        Ok(())
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide)?;
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;
        crossterm::execute!(std::io::stdout(), EnableBracketedPaste)?;

        self.start();

        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.stop()?;

        if crossterm::terminal::is_raw_mode_enabled()? {
            self.flush()?;

            crossterm::execute!(std::io::stdout(), DisableBracketedPaste)?;
            crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;
            crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
        }

        Ok(())
    }

    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }
}

impl Deref for Tui {
    type Target = ratatui::Terminal<Backend<std::io::Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
