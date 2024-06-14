use ratatui::{self, layout::Rect, Frame};
use crate::app::{renderer::{Area, Renderer, Widget}, ui::Style};

pub struct TuiRenderer<'a: 'b, 'b> 
{
    frame: &'b mut Frame<'a>,
}

impl<'a, 'b> TuiRenderer<'a, 'b> {
    pub fn from(frame: &'b mut Frame<'a>) -> Self {
        Self {
            frame,
        }
    }

    pub fn render_native<W>(&mut self, widget: W, area: Rect) 
        where W: ratatui::widgets::Widget
    {
        self.frame.render_widget(widget, area);
    }
}

impl<'a: 'b, 'b> Renderer for TuiRenderer<'a, 'b> {
    fn frame_size(&self) -> Area {
        self.frame.size().into()
    }

    fn render(&mut self, widget: &mut dyn Widget, area: Area, style: &Style) {
        widget.render(self, area, style);
    }
}

impl From<Rect> for Area {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            cols: value.width,
            rows: value.height,
        }
    }
}

impl From<Area> for Rect {
    fn from(value: Area) -> Self {
        Self::new(
            value.x,
            value.y,
            value.cols,
            value.rows,
        )
    }
}
