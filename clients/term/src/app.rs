use color_eyre::eyre::Result;
use ratatui::prelude::Rect;
use tokio::{
    sync::{mpsc, watch},
    task::JoinHandle,
};

use crate::components::Component;

use self::{action::Action, event::Event};

pub mod action;
pub mod event;
pub mod tui;

pub struct AppCx {
    action_tx: mpsc::UnboundedSender<Action>,

    size_rx: watch::Receiver<Rect>,

    local_tasks: Vec<JoinHandle<()>>,
}

impl AppCx {
    pub fn proxy(&self) -> AppCx {
        AppCx {
            action_tx: self.action_tx.clone(),

            size_rx: self.size_rx.clone(),

            local_tasks: Vec::new(),
        }
    }

    pub fn size(&self) -> Rect {
        *self.size_rx.borrow()
    }

    pub fn render(&self) -> bool {
        self.action_tx.send(Action::Render).is_ok()
    }

    pub fn quit(&self) -> bool {
        self.action_tx.send(Action::Quit).is_ok()
    }

    pub fn on_resize<F>(&mut self, handler: F)
    where
        F: Fn(&AppCx, Rect) + Send + Sync + 'static,
    {
        let mut size_rx = self.size_rx.clone();
        let cx = self.proxy();

        self.local_tasks.push(tokio::spawn(async move {
            while size_rx.changed().await.is_ok() {
                handler(&cx, *size_rx.borrow());
            }
        }));
    }
}

impl Drop for AppCx {
    fn drop(&mut self) {
        for task in self.local_tasks.drain(..) {
            task.abort();
        }
    }
}

impl From<&AppCx> for AppCx {
    fn from(cx: &AppCx) -> Self {
        cx.proxy()
    }
}

pub async fn run_app<F, C>(func: F) -> Result<()>
where
    F: FnOnce(AppCx) -> C,
    C: Component,
{
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();
    let (size_tx, size_rx) = watch::channel(Rect::default());

    let mut tui = tui::Tui::new()?;
    tui.enter()?;

    let proxy = AppCx {
        action_tx: action_tx.clone(),

        size_rx,

        local_tasks: Vec::new(),
    };

    let mut root = func(proxy);

    let mut should_quit = false;

    loop {
        if let Some(e) = tui.next().await {
            match e {
                Event::Quit => action_tx.send(Action::Quit)?,

                Event::Resize(w, h) => {
                    tui.resize(Rect::new(0, 0, w, h))?;

                    size_tx.send(Rect::new(0, 0, w, h))?;

                    action_tx.send(Action::Render)?;
                }

                _ => {}
            }

            root.handle_event(&e);
        }

        while let Ok(action) = action_rx.try_recv() {
            if action != Action::Render {
                tracing::debug!("{action:?}");
            }

            match action {
                Action::Quit => should_quit = true,

                Action::Render => {
                    tui.draw(|f| root.draw(f, f.size()))?;
                }
            }
        }

        if should_quit {
            tui.stop()?;

            break;
        }
    }

    tui.exit()?;

    Ok(())
}
