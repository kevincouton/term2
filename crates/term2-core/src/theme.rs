//! Terminal themes and appearance configuration.
//!
//! Warp supports custom themes with ANSI colors, background/foreground colors,
//! cursor styles, and font settings.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Self { r, g, b })
        } else {
            None
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CursorStyle {
    #[default]
    Block,
    Beam,
    Underline,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub selection: Color,
    #[serde(default)]
    pub cursor_style: CursorStyle,
    #[serde(default)]
    pub ansi: HashMap<String, Color>,
    #[serde(default)]
    pub bright_ansi: HashMap<String, Color>,
}

impl Theme {
    pub fn ansi_color(&self, name: &str) -> Option<Color> {
        self.ansi.get(name).copied()
    }

    pub fn ansi_index(&self, index: u8) -> Option<Color> {
        let name = match index {
            0 => "black",
            1 => "red",
            2 => "green",
            3 => "yellow",
            4 => "blue",
            5 => "magenta",
            6 => "cyan",
            7 => "white",
            _ => return None,
        };
        self.ansi_color(name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppearanceConfig {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub font_family: Option<String>,
    #[serde(default)]
    pub font_size: u8,
    #[serde(default = "default_opacity")]
    pub background_opacity: f32,
    #[serde(default)]
    pub dim_inactive_panes: bool,
    #[serde(default)]
    pub input_position: InputPosition,
}

fn default_opacity() -> f32 {
    1.0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InputPosition {
    #[default]
    Bottom,
    Top,
    Floating,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_hex_parse_and_format() {
        let c = Color::hex("#ff5733").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 87);
        assert_eq!(c.b, 51);
        assert_eq!(c.to_hex(), "#ff5733");
    }

    #[test]
    fn invalid_hex_returns_none() {
        assert!(Color::hex("not-a-color").is_none());
        assert!(Color::hex("#xyz").is_none());
    }

    #[test]
    fn theme_ansi_colors_by_name_and_index() {
        let mut ansi = HashMap::new();
        ansi.insert("red".to_string(), Color::new(255, 0, 0));
        ansi.insert("blue".to_string(), Color::new(0, 0, 255));
        let theme = Theme {
            name: "Test".into(),
            background: Color::new(0, 0, 0),
            foreground: Color::new(255, 255, 255),
            cursor: Color::new(255, 255, 0),
            selection: Color::new(128, 128, 128),
            cursor_style: CursorStyle::Beam,
            ansi,
            bright_ansi: HashMap::new(),
        };
        assert_eq!(theme.ansi_color("red"), Some(Color::new(255, 0, 0)));
        assert_eq!(theme.ansi_index(4), Some(Color::new(0, 0, 255)));
        assert_eq!(theme.ansi_index(9), None);
    }

    #[test]
    fn appearance_config_defaults() {
        let config: AppearanceConfig = serde_json::from_str("{}").unwrap();
        assert_eq!(config.font_size, 0);
        assert!((config.background_opacity - 1.0).abs() < f32::EPSILON);
        assert_eq!(config.input_position, InputPosition::Bottom);
    }

    #[test]
    fn appearance_config_parses_input_position() {
        let json = r#"{"input_position":"top","background_opacity":0.95}"#;
        let config: AppearanceConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.input_position, InputPosition::Top);
        assert!((config.background_opacity - 0.95).abs() < f32::EPSILON);
    }

    // Warp-derived scenarios:

    #[test]
    fn theme_ansi_color_derives_tab_colors() {
        // Tab colors (Red | Green | Yellow | Blue | Magenta | Cyan) derive from the theme.
        let mut ansi = HashMap::new();
        ansi.insert("blue".to_string(), Color::new(0, 120, 255));
        let theme = Theme {
            name: "Warp".into(),
            background: Color::new(0, 0, 0),
            foreground: Color::new(255, 255, 255),
            cursor: Color::new(255, 255, 0),
            selection: Color::new(128, 128, 128),
            cursor_style: CursorStyle::Beam,
            ansi,
            bright_ansi: HashMap::new(),
        };
        assert_eq!(theme.ansi_color("blue"), Some(Color::new(0, 120, 255)));
    }

    #[test]
    #[ignore = "theme picker UI not yet implemented"]
    fn theme_picker_lists_available_themes() {
        // CTRL-CMD-T opens theme picker.
    }

    #[test]
    #[ignore = "background blur not yet implemented"]
    fn appearance_supports_background_opacity_and_blur() {
        // Appearance settings support background opacity and blur.
    }
}
