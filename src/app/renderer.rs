#[cfg(feature = "ratatui")]
pub use crate::app::tui::TuiRenderer as SpecializedRenderer;

use super::ui::Style;

pub trait Renderer {
    fn frame_size(&self) -> Area;
    fn render(&mut self, widget: &mut dyn Widget, area: Area, style: &Style);
}

pub trait Widget 
{
    fn render(&mut self, renderer: &mut SpecializedRenderer, area: Area, style: &Style);
}

#[derive(Copy, Clone, PartialEq,  Eq, Debug)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub cols: u16,
    pub rows: u16,
}
