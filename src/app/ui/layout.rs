use crate::app::renderer::Area;

pub enum Direction {
    Vertical,
    Horizontal,
}

pub enum Layout {
    Static(u16),
    Weighted(u16),
    Percentage(f32),
}

pub struct LayoutBuffer {
    layouts: Vec<Layout>,
    areas: Vec<Area>,
}

impl LayoutBuffer {
    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self {
            layouts: Vec::with_capacity(initial_capacity),
            areas: Vec::with_capacity(initial_capacity),
        }
    }

    pub fn clear(&mut self) {
        self.layouts.clear();
    }

    pub fn add_layout(&mut self, layout: Layout) -> &mut Self {
        self.layouts.push(layout);
        self
    }

    pub fn calculate_areas(&mut self, direction: Direction, frame_area: &Area) {
        self.areas.clear();

        let mut statically_taken_size: u16 = 0;
        let mut weight_sum: f32 = 0.0;
        let total_size = match direction {
            Direction::Vertical => frame_area.rows,
            Direction::Horizontal => frame_area.cols,
        };

        let size_from_percentage = |percentage: f32| -> f32 {
            let clamped: f32 = percentage.clamp(0.0, 1.0);
            total_size as f32 * clamped
        };

        for item in &self.layouts {
            match item {
                Layout::Static(size) => statically_taken_size += size,
                Layout::Weighted(weight) => weight_sum += *weight as f32,
                Layout::Percentage(percentage) => {
                    statically_taken_size += (size_from_percentage)(*percentage) as u16;
                }
            }
        }

        let weight_unit: f32 = if weight_sum == 0.0 {
            0.0
        } else {
            (total_size - statically_taken_size) as f32 / weight_sum
        };
        let mut offset: u16 = 0;

        for item in &self.layouts {
            let area: Area = match direction {
                Direction::Vertical => {
                    let x = frame_area.x;
                    let y = frame_area.y + offset;
                    let cols = frame_area.cols;
                    let rows = match item {
                        Layout::Static(size) => *size,
                        Layout::Weighted(weight) => (*weight as f32 * weight_unit).round() as u16,
                        Layout::Percentage(percentage) => (size_from_percentage)(*percentage) as u16,
                    };
                    offset += rows;
                    Area { x, y, cols, rows }
                }
                Direction::Horizontal => {
                    let x = frame_area.x + offset;
                    let y = frame_area.y;
                    let rows = frame_area.rows;
                    let cols = match item {
                        Layout::Static(size) => *size,
                        Layout::Weighted(weight) => (*weight as f32 * weight_unit).round() as u16,
                        Layout::Percentage(percentage) => (size_from_percentage)(*percentage) as u16,
                    };
                    offset += cols;
                    Area { x, y, cols, rows }
                }
            };

            self.areas.push(area);
        }
    }

    pub fn area(&self, index: usize) -> Option<&Area> {
        self.areas.get(index)
    }

    pub fn areas(&self) -> &Vec<Area> {
        &self.areas
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_static() {
        let area = Area {
            x: 0,
            y: 0,
            cols: 10,
            rows: 10,
        };
        let mut layout_buffer = LayoutBuffer::with_capacity(3);

        layout_buffer
            .add_layout(Layout::Static(1))
            .add_layout(Layout::Static(5))
            .add_layout(Layout::Static(3))
            .calculate_areas(Direction::Vertical, &area);

        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 0,
                y: 0,
                cols: 10,
                rows: 1
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 0,
                y: 1,
                cols: 10,
                rows: 5
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 0,
                y: 6,
                cols: 10,
                rows: 3
            }
        );

        layout_buffer.calculate_areas(Direction::Horizontal, &area);

        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 0,
                y: 0,
                cols: 1,
                rows: 10
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 1,
                y: 0,
                cols: 5,
                rows: 10
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 6,
                y: 0,
                cols: 3,
                rows: 10
            }
        );
    }

    #[test]
    fn all_percentage() {
        let area = Area {
            x: 10,
            y: 12,
            cols: 32,
            rows: 60,
        };
        let mut layout_buffer = LayoutBuffer::with_capacity(5);

        layout_buffer
            .add_layout(Layout::Percentage(0.25))
            .add_layout(Layout::Percentage(0.125))
            .add_layout(Layout::Percentage(0.125))
            .add_layout(Layout::Percentage(0.3))
            .add_layout(Layout::Percentage(0.2))
            .calculate_areas(Direction::Vertical, &area);

        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 10,
                y: 12,
                cols: 32,
                rows: 15
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 10,
                y: 27,
                cols: 32,
                rows: 7
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 10,
                y: 34,
                cols: 32,
                rows: 7
            }
        );
        assert_eq!(
            *layout_buffer.area(3).unwrap(),
            Area {
                x: 10,
                y: 41,
                cols: 32,
                rows: 18
            }
        );
        assert_eq!(
            *layout_buffer.area(4).unwrap(),
            Area {
                x: 10,
                y: 59,
                cols: 32,
                rows: 12
            }
        );

        layout_buffer.calculate_areas(Direction::Horizontal, &area);
        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 10,
                y: 12,
                cols: 8,
                rows: 60
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 18,
                y: 12,
                cols: 4,
                rows: 60
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 22,
                y: 12,
                cols: 4,
                rows: 60
            }
        );
        assert_eq!(
            *layout_buffer.area(3).unwrap(),
            Area {
                x: 26,
                y: 12,
                cols: 9,
                rows: 60
            }
        );
        assert_eq!(
            *layout_buffer.area(4).unwrap(),
            Area {
                x: 35,
                y: 12,
                cols: 6,
                rows: 60
            }
        );
    }

    #[test]
    fn all_weighted() {
        let area = Area {
            x: 45,
            y: 78,
            cols: 60,
            rows: 900,
        };
        let mut layout_buffer = LayoutBuffer::with_capacity(5);

        layout_buffer
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(1))
            .calculate_areas(Direction::Vertical, &area);

        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 45,
                y: 78,
                cols: 60,
                rows: 180
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 45,
                y: 258,
                cols: 60,
                rows: 180
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 45,
                y: 438,
                cols: 60,
                rows: 180
            }
        );
        assert_eq!(
            *layout_buffer.area(3).unwrap(),
            Area {
                x: 45,
                y: 618,
                cols: 60,
                rows: 180
            }
        );
        assert_eq!(
            *layout_buffer.area(4).unwrap(),
            Area {
                x: 45,
                y: 798,
                cols: 60,
                rows: 180
            }
        );

        layout_buffer.calculate_areas(Direction::Horizontal, &area);
        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 45,
                y: 78,
                cols: 12,
                rows: 900
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 57,
                y: 78,
                cols: 12,
                rows: 900
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 69,
                y: 78,
                cols: 12,
                rows: 900
            }
        );
        assert_eq!(
            *layout_buffer.area(3).unwrap(),
            Area {
                x: 81,
                y: 78,
                cols: 12,
                rows: 900
            }
        );
        assert_eq!(
            *layout_buffer.area(4).unwrap(),
            Area {
                x: 93,
                y: 78,
                cols: 12,
                rows: 900
            }
        );

        layout_buffer.clear();
        layout_buffer
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(1))
            .add_layout(Layout::Weighted(3))
            .add_layout(Layout::Weighted(5))
            .add_layout(Layout::Weighted(5))
            .calculate_areas(Direction::Vertical, &area);

        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 45,
                y: 78,
                cols: 60,
                rows: 60
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 45,
                y: 138,
                cols: 60,
                rows: 60
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 45,
                y: 198,
                cols: 60,
                rows: 180
            }
        );
        assert_eq!(
            *layout_buffer.area(3).unwrap(),
            Area {
                x: 45,
                y: 378,
                cols: 60,
                rows: 300
            }
        );
        assert_eq!(
            *layout_buffer.area(4).unwrap(),
            Area {
                x: 45,
                y: 678,
                cols: 60,
                rows: 300
            }
        );

        layout_buffer.calculate_areas(Direction::Horizontal, &area);
        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 45,
                y: 78,
                cols: 4,
                rows: 900
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 49,
                y: 78,
                cols: 4,
                rows: 900
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 53,
                y: 78,
                cols: 12,
                rows: 900
            }
        );
        assert_eq!(
            *layout_buffer.area(3).unwrap(),
            Area {
                x: 65,
                y: 78,
                cols: 20,
                rows: 900
            }
        );
        assert_eq!(
            *layout_buffer.area(4).unwrap(),
            Area {
                x: 85,
                y: 78,
                cols: 20,
                rows: 900
            }
        );
    }

    #[test]
    fn mix() {
        let area = Area {
            x: 8,
            y: 15,
            cols: 100,
            rows: 50,
        };
        let mut layout_buffer = LayoutBuffer::with_capacity(5);

        layout_buffer
            .add_layout(Layout::Static(5))
            .add_layout(Layout::Percentage(0.2))
            .add_layout(Layout::Weighted(1))
            .calculate_areas(Direction::Vertical, &area);

        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 8,
                y: 15,
                cols: 100,
                rows: 5
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 8,
                y: 20,
                cols: 100,
                rows: 10
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 8,
                y: 30,
                cols: 100,
                rows: 35
            }
        );

        layout_buffer.calculate_areas(Direction::Horizontal, &area);
        assert_eq!(
            *layout_buffer.area(0).unwrap(),
            Area {
                x: 8,
                y: 15,
                cols: 5,
                rows: 50
            }
        );
        assert_eq!(
            *layout_buffer.area(1).unwrap(),
            Area {
                x: 13,
                y: 15,
                cols: 20,
                rows: 50
            }
        );
        assert_eq!(
            *layout_buffer.area(2).unwrap(),
            Area {
                x: 33,
                y: 15,
                cols: 75,
                rows: 50
            }
        );
    }
}
