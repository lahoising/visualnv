use ratatui::style::{Color, Stylize};
use ratatui::widgets::Paragraph;

use crate::app::renderer::{Area, Widget};
use crate::app::tui::renderer::TuiRenderer;
use crate::app::ui::{Label, Style};

impl<'a, 'b> Widget for Label {
    fn render(&mut self, renderer: &mut TuiRenderer, area: Area, style: &Style) {
        let mut paragraph = Paragraph::new(self.get_text().clone());
        if style.foreground.as_u32() != 0 {
            paragraph = paragraph.fg(Color::from_u32(style.foreground.as_u32()));
        }
        renderer.render_native(paragraph, area.into());
    }
}
