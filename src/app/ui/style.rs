pub struct Style {
    pub borders: Borders,
    pub background: RGBA,
    pub foreground: RGBA,
}

impl Style {
    pub fn new() -> Self {
        Self::from(Borders::none(), RGBA::transparent(), RGBA::transparent())
    }

    pub fn from(borders: Borders, background: RGBA, foreground: RGBA) -> Self {
        Self {
            borders,
            background,
            foreground,
        }
    }

    pub fn bordered() -> Self {
        Self::from(Borders::all(), RGBA::new(), RGBA::white())
    }
}

#[derive(Copy, Clone)]
pub struct Borders {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Borders {
    pub fn from(top: bool, bottom: bool, left: bool, right: bool) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn all() -> Self {
        Self::from(true, true, true, true)
    }

    pub fn none() -> Self {
        Self::from(false, false, false, false)
    }
}

#[derive(Copy, Clone)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: bool,
}

impl RGBA {
    pub fn new() -> Self {
        Self::transparent()
    }

    pub fn from(red: u8, green: u8, blue: u8, alpha: bool) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn black() -> Self {
        Self::from(0, 0, 0, false)
    }

    pub fn white() -> Self {
        Self::from(255, 255, 255, false)
    }

    pub fn red() -> Self {
        Self::from(255, 0, 0, false)
    }

    pub fn green() -> Self {
        Self::from(0, 255, 0, false)
    }

    pub fn blue() -> Self {
        Self::from(0, 0, 255, false)
    }

    pub fn transparent() -> Self {
        Self::from(0, 0, 0, true)
    }

    /*
     * Pack an RGBA value into a u32 with big endianness
     */
    pub fn as_u32(&self) -> u32 {
        let alpha_numeric_value: u32 = if self.alpha { 0 } else { 255 };
        alpha_numeric_value << 24
            | (self.red as u32) << 16
            | (self.green as u32) << 8
            | (self.blue as u32)
    }
}

pub struct Padding {
    pub top: u16,
    pub bottom: u16,
    pub left: u16,
    pub right: u16,
}

impl Padding {
    pub fn from(top: u16, bottom: u16, left: u16, right: u16) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn none() -> Self {
        Self::from(0, 0, 0, 0)
    }

    pub fn around(value: u16) -> Self {
        Self::from(value, value, value, value)
    }

    pub fn high_and_wide(high: u16, wide: u16) -> Self {
        Self::from(high, high, wide, wide)
    }
}
