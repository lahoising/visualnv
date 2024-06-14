use screen::Screen;

use self::{
    screen::{ScreenApi, ViewLoader},
    view::HomeView,
};

mod event;
mod screen;
mod view;
mod renderer;
mod ui;

#[cfg(feature = "ratatui")]
mod tui;

use view::View;

pub struct App {
    running: bool,
    screen: Screen,
    view: Box<dyn View>,
}

impl App {
    pub fn new() -> Result<Self, Error> {
        let screen = Screen::new()?;
        Ok(Self {
            running: false,
            screen,
            view: Box::new(HomeView::new()),
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.start()?;
        while self.running {
            self.update()?;
            self.render();
        }
        self.close()?;
        Ok(())
    }

    fn start(&mut self) -> Result<(), Error> {
        self.running = true;
        Ok(())
    }

    fn update(&mut self) -> Result<(), Error> {
        let update_action = self.screen.update(&mut self.view)?;
        self.running = update_action.get_keep_running();
        if let Some(view_loader) = update_action.get_view_loader() {
            self.load_view(view_loader);
        }
        Ok(())
    }

    fn render(&mut self) {
        self.screen.render(&mut self.view);
    }

    fn close(&mut self) -> Result<(), Error> {
        self.screen.close()?;
        Ok(())
    }

    fn load_view(&mut self, view_loader: ViewLoader) {
        self.view = view_loader();
        let _ = self.view.init();
    }
}

#[derive(Debug)]
pub enum Error {
    ScreenError(screen::Error),
}

impl From<screen::Error> for Error {
    fn from(value: screen::Error) -> Self {
        Self::ScreenError(value)
    }
}
