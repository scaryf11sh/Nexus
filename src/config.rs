use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Theme {
    pub background: [u8; 4],
    pub foreground: [u8; 4],
    pub highlight: [u8; 4],
    pub primary: [u8; 4],
}

impl Theme {
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read theme file");
        toml::from_str(&content).expect("Failed to parse theme file")
    }

    pub fn bg_color(&self) -> raylib::color::Color {
        raylib::color::Color::new(self.background[0], self.background[1], self.background[2], self.background[3])
    }
}