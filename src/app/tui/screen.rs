use std::io::{stdout, Stdout};

use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, 
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::{
    event::{Event,Key}, 
    screen::{Result, ScreenApi, UpdateAction}, 
    view::View,
    tui::TuiRenderer,
};

pub struct Screen {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl ScreenApi for Screen {
    fn new() -> Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        Ok(Self {
            terminal,
        })
    }

    fn update(&mut self, view: &mut Box<dyn View>) -> Result<UpdateAction> {
        if event::poll(std::time::Duration::from_millis(0))? {
            if let event::Event::Key(key) = event::read()? {
                let evt: Event = key.into();
                let _ = view.update(&evt);
                match evt.get_key() {
                    Key::ESC => return Ok(UpdateAction::from(false, None)),
                    _ => {},
                }
            }
        }
        Ok(UpdateAction::from(true, None))
    }

    fn render(&mut self, view: &mut Box<dyn View>) {
        match self.terminal.draw(move |frame| {
            {
                let mut renderer = TuiRenderer::from(frame);
                let _ = view.render(&mut renderer);
            }
        }) {
            Ok(_) => {},
            Err(_) => {},
        }
    }

    fn close(&mut self) -> Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}

impl Screen {
}
