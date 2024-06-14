use ratatui::{
    style::{Color, Stylize},
    widgets::Block,
};

use crate::app::{
    renderer::{Area, Widget},
    tui::TuiRenderer,
    ui::{Style, Surface, SurfaceOnRender},
};

impl<F: SurfaceOnRender> Widget for Surface<F> {
    fn render(&mut self, renderer: &mut TuiRenderer, area: Area, style: &Style) {
        let mut block = Block::default()
            .borders(style.borders.into());
        if style.background.as_u32() != 0 {
            block = block.bg(Color::from_u32(style.background.as_u32()));
        }
        renderer.render_native(block, area.into());
        self.render_content(renderer, area);
    }
}
