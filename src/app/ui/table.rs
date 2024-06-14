use crate::app::renderer::Area;

use super::{Direction, Layout, LayoutBuffer};

pub struct Table {
    vertical_layout_buf: LayoutBuffer,
    rows_layout_bufs: Vec<LayoutBuffer>,
    row_count: usize,
}

impl Table {
    pub fn with_capacity(cols: usize, rows: usize) -> Self {
        let mut rows_layout_bufs = Vec::with_capacity(rows);
        for _ in 0..rows {
            let buf = LayoutBuffer::with_capacity(cols);
            rows_layout_bufs.push(buf);
        }

        Self {
            vertical_layout_buf: LayoutBuffer::with_capacity(rows),
            rows_layout_bufs,
            row_count: 0,
        }
    }

    pub fn add_row(&mut self, row_layout: Layout, formatter: &dyn Fn(&mut LayoutBuffer)) -> &mut Self {
        self.vertical_layout_buf.add_layout(row_layout);
        formatter(self.rows_layout_bufs.get_mut(self.row_count).unwrap());
        self.row_count += 1;
        self
    }

    pub fn calculate_areas(&mut self, area: &Area) {
        self.vertical_layout_buf
            .calculate_areas(Direction::Vertical, area);

        let mut i: usize = 0;
        for row_area in self.vertical_layout_buf.areas() {
            self.rows_layout_bufs
                .get_mut(i)
                .unwrap()
                .calculate_areas(Direction::Horizontal, row_area);
            i += 1;
        }
    }

    pub fn area(&self, col: usize, row: usize) -> Area {
        *self.rows_layout_bufs.get(row).unwrap().area(col).unwrap()
    }

    pub fn row_area(&self, row: usize) -> Area {
        *self.vertical_layout_buf.area(row).unwrap()
    }

    pub fn clear(&mut self) {
        self.row_count = 0;
        self.vertical_layout_buf.clear();
        for layout in &mut self.rows_layout_bufs {
            layout.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::ui::{Layout, LayoutBuffer};

    use super::*;

    #[test]
    fn regular_table() {
        let mut table = Table::with_capacity(5, 10);
        table
            .add_row(Layout::Static(5), &|layout_buffer: &mut LayoutBuffer| {
                layout_buffer.add_layout(Layout::Weighted(1));
            })
            .add_row(Layout::Weighted(1), &|layout_buffer: &mut LayoutBuffer| {
                layout_buffer
                    .add_layout(Layout::Static(5))
                    .add_layout(Layout::Weighted(1))
                    .add_layout(Layout::Percentage(0.1));
            });

        let area = Area {
            x: 0,
            y: 0,
            cols: 100,
            rows: 25,
        };
        table.calculate_areas(&area);

        assert_eq!(
            table.area(0, 0),
            Area {
                x: 0,
                y: 0,
                cols: 100,
                rows: 5
            }
        );
        assert_eq!(
            table.area(0, 1),
            Area {
                x: 0,
                y: 5,
                cols: 5,
                rows: 20
            }
        );
        assert_eq!(
            table.area(1, 1),
            Area {
                x: 5,
                y: 5,
                cols: 85,
                rows: 20
            }
        );
        assert_eq!(
            table.area(2, 1),
            Area {
                x: 90,
                y: 5,
                cols: 10,
                rows: 20
            }
        );
    }
}
