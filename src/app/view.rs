mod home;

pub use home::HomeView;
pub use crate::app::renderer::Renderer;

use super::event::Event;

pub trait View {
    fn init(&mut self) -> Result<(), Error>;
    fn update(&mut self, event: &Event) -> Result<(), Error>;
    fn render(&mut self, renderer: &mut dyn Renderer) -> Result<(), Error>;
    fn close(&mut self) -> Result<(), Error>;
}

pub enum Error {
}
