use crate::app::ui;
use ratatui::widgets::Borders;

impl From<ui::Borders> for Borders {
    fn from(borders: ui::Borders) -> Self {
        let mut ret = Borders::NONE;
        if borders.top { ret |= Borders::TOP; }
        if borders.bottom { ret |= Borders::BOTTOM; }
        if borders.left { ret |= Borders::LEFT; }
        if borders.right { ret |= Borders::RIGHT; }
        ret
    }
}
