mod alias;
pub mod command_bar;
mod error;
pub mod interpreter;
pub mod manager;

pub use interpreter::PyConfRuntime;
pub use manager::ContextManager;

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
pub enum StyleName {
    BOLD,
    NORMAL,
    ITALIC,
}

#[derive(Debug)]
pub struct Style {
    pub style_name: StyleName,
}

impl Style {
    fn default() -> Self {
        Self {
            style_name: StyleName::NORMAL,
        }
    }

    fn from_string(style_name: &str) -> Style {
        let style = match style_name.to_lowercase().as_str() {
            "bold" => StyleName::BOLD,
            "normal" => StyleName::NORMAL,
            "italic" => StyleName::ITALIC,
            _ => StyleName::NORMAL,
        };

        Self { style_name: style }
    }
}

#[derive(Debug)]
pub struct Color {
    pub color_name: ColorName,
}

impl Color {
    fn default() -> Self {
        Self {
            color_name: ColorName::WHITE,
        }
    }

    fn from_string(color_name: &str) -> Self {
        let color = match color_name.to_lowercase().as_str() {
            "red" => ColorName::RED,
            "blue" => ColorName::BLUE,
            "yellow" => ColorName::YELLOW,
            "black" => ColorName::BLACK,
            "white" => ColorName::WHITE,
            "green" => ColorName::GREEN,
            _ => ColorName::WHITE,
        };

        Self { color_name: color }
    }
}
