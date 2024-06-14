use crate::app::{
    event::{Event, Key},
    renderer::{Area, Renderer},
    ui::{Borders, Direction, Label, Layout, LayoutBuffer, Padding, Style, Surface, Table, RGBA},
    View,
};

const MAIN_TABLE_ROW_COUNT: u16 = 5;
const MAIN_TABLE_COL_COUNT: u16 = 3;

pub struct HomeView {
    should_draw_second: bool,
    layout_buffer: LayoutBuffer,
    horizontal_layout_buffer: LayoutBuffer,
    main_table: Table,
}

impl HomeView {
    pub fn new() -> Self {
        Self {
            should_draw_second: false,
            layout_buffer: LayoutBuffer::with_capacity(20),
            horizontal_layout_buffer: LayoutBuffer::with_capacity(5),
            main_table: Table::with_capacity(
                MAIN_TABLE_COL_COUNT as usize,
                MAIN_TABLE_ROW_COUNT as usize,
            ),
        }
    }
}

impl View for HomeView {
    fn init(&mut self) -> Result<(), super::Error> {
        Ok(())
    }

    fn update(&mut self, event: &Event) -> Result<(), super::Error> {
        match event.get_key() {
            Key::ENTER => {
                self.should_draw_second = !self.should_draw_second;
            }
            _ => {}
        }
        Ok(())
    }

    fn render(&mut self, renderer: &mut dyn Renderer) -> Result<(), super::Error> {
        let frame_size = renderer.frame_size();

        self.layout_buffer.clear();
        self.layout_buffer
            .add_layout(Layout::Static(1))
            .add_layout(Layout::Static(3));
        if self.should_draw_second {
            self.layout_buffer.add_layout(Layout::Percentage(0.05));
        }
        self.layout_buffer
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(1))
            .calculate_areas(Direction::Vertical, &frame_size);

        renderer.render(
            &mut Label::from("This is some text"),
            *self.layout_buffer.area(0).unwrap(),
            &Style::new(),
        );

        self.horizontal_layout_buffer.clear();
        self.horizontal_layout_buffer
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(1))
            .calculate_areas(Direction::Horizontal, &self.layout_buffer.area(1).unwrap());

        renderer.render(
            &mut Surface::from(Padding::none(), &|_: &mut dyn Renderer, _: &Area| {}), 
            *self.horizontal_layout_buffer.area(0).unwrap(), 
            &Style::from(Borders::none(), RGBA::red(), RGBA::transparent()));

        renderer.render(
            &mut Surface::from(Padding::none(), &|_: &mut dyn Renderer, _: &Area| {}), 
            *self.horizontal_layout_buffer.area(1).unwrap(), 
            &Style::from(Borders::none(), RGBA::green(), RGBA::transparent()));

        renderer.render(
            &mut Surface::from(Padding::none(), &|_: &mut dyn Renderer, _: &Area| {}), 
            *self.horizontal_layout_buffer.area(2).unwrap(), 
            &Style::from(Borders::none(), RGBA::blue(), RGBA::transparent()));

        renderer.render(
            &mut Surface::from(Padding::none(), &|_: &mut dyn Renderer, _: &Area| {}), 
            *self.horizontal_layout_buffer.area(3).unwrap(), 
            &Style::from(Borders::none(), RGBA::black(), RGBA::transparent()));

        if self.should_draw_second {
            renderer.render(
                &mut Label::from("This is the second text!"),
                *self.layout_buffer.area(2).unwrap(),
                &Style::new(),
            );
        }

        let mut surface = Surface::from(
            Padding::high_and_wide(1, 2),
            |internal_renderer: &mut dyn Renderer, content_area: &Area| {
                internal_renderer.render(
                    &mut Label::from("This should be far off"),
                    *content_area,
                    &Style::new(),
                );
            },
        );
        renderer.render(
            &mut surface,
            *self
                .layout_buffer
                .area(if self.should_draw_second { 3 } else { 2 })
                .unwrap(),
            &Style::bordered(),
        );

        self.main_table.clear();
        self.main_table
            .add_row(Layout::Static(3), &|layout_buffer: &mut LayoutBuffer| {
                layout_buffer.add_layout(Layout::Weighted(1));
            });

        for _ in 0..MAIN_TABLE_ROW_COUNT - 1 {
            self.main_table
                .add_row(Layout::Static(3), &|layout_buffer: &mut LayoutBuffer| {
                    for _ in 0..MAIN_TABLE_COL_COUNT {
                        layout_buffer.add_layout(Layout::Weighted(1));
                    }
                });
        }

        let table_area = self
            .layout_buffer
            .area(if self.should_draw_second { 4 } else { 3 })
            .unwrap();
        self.main_table.calculate_areas(table_area);

        let mut table_title_surface = Surface::from(
            Padding::high_and_wide(1, 2),
            |internal_renderer, content_area| {
                internal_renderer.render(
                    &mut Label::from("This is a table title"),
                    *content_area,
                    &Style::from(Borders::none(), RGBA::transparent(), RGBA::red()),
                );
            },
        );
        renderer.render(
            &mut table_title_surface,
            self.main_table.row_area(0).clone(),
            &Style::bordered(),
        );

        for row in 1..MAIN_TABLE_ROW_COUNT as usize {
            for col in 0..MAIN_TABLE_COL_COUNT as usize {
                let on_cell_draw = |internal_renderer: &mut dyn Renderer, content_area: &Area| {
                    internal_renderer.render(
                        &mut Label::from(format!("This is cell {col}, {row}")),
                        *content_area,
                        &Style::new(),
                    );
                };

                let mut table_cell = Surface::from(Padding::around(1), &on_cell_draw);
                renderer.render(
                    &mut table_cell,
                    self.main_table.area(col, row),
                    &Style::bordered(),
                );
            }
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), super::Error> {
        Ok(())
    }
}
