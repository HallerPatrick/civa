mod command_bar;
mod error;

#[derive(Debug)]
pub enum ColorName {
    RED,
    BLUE,
    YELLOW,
    BLACK,
    WHITE,
    GREEN,
}

#[derive(Debug)]
pub enum Style {
    BOLD,
    NORMAL,
    ITALIC,
}

#[derive(Debug)]
pub struct Color {
    color_name: Option<ColorName>,
}

impl Color {
    fn default() -> Self {
        Self {
            color_name: Some(ColorName::WHITE),
        }
    }
}

impl Color {
    fn from_rgb() {}
    fn from_hex() {}
    fn from_name() {}
}
