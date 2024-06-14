use crate::app::{
    renderer::{Area, Renderer},
    ui::Padding,
};

pub struct Surface<F>
where
    F: SurfaceOnRender,
{
    on_render: F,
    padding: Padding,
}

impl<F: SurfaceOnRender> Surface<F> {
    pub fn from(padding: Padding, on_render: F) -> Self {
        Self { on_render, padding }
    }

    pub fn render_content(&self, renderer: &mut dyn Renderer, area: Area) {
        let content_area = Area {
            x: area.x + self.padding.left,
            y: area.y + self.padding.top,
            cols: area.cols - self.padding.left - self.padding.right,
            rows: area.rows - self.padding.top - self.padding.bottom,
        };
        (self.on_render)(renderer, &content_area);
    }
}

pub trait SurfaceOnRender: Fn(&mut dyn Renderer, &Area) {}
impl<F> SurfaceOnRender for F where F: Fn(&mut dyn Renderer, &Area) {}
