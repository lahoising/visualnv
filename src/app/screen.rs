use std::{io, result};

#[cfg(feature = "ratatui")]
pub use crate::app::tui::Screen;

use super::view::View;

pub trait ScreenApi {
    fn new() -> Result<Self>
    where
        Self: Sized;
    /**
     * # Return a boolean indicating whether the program should keep running
     */
    fn update(&mut self, view: &mut Box<dyn View>) -> Result<UpdateAction>;
    fn render(&mut self, view: &mut Box<dyn View>);
    fn close(&mut self) -> Result<()>;
}

pub type ViewLoader = fn() -> Box<dyn View>;

pub struct UpdateAction {
    keep_running: bool,
    view_loader: Option<ViewLoader>,
}

impl UpdateAction {
    pub fn from(keep_running: bool, view_loader: Option<ViewLoader>) -> Self {
        Self {
            keep_running,
            view_loader,
        }
    }

    pub fn get_keep_running(&self) -> bool {
        self.keep_running
    }
    pub fn get_view_loader(&self) -> Option<ViewLoader> {
        self.view_loader
    }
}

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
