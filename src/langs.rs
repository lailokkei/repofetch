use ansi_term::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LanguageRecord<'a> {
    pub extensions: Vec<&'a str>,
    pub name: &'a str,
    pub color: &'a str,
}

pub struct LanguageDB<'a> {
    pub langs: Vec<LanguageRecord<'a>>,
}

impl<'a> LanguageDB<'a> {}

pub fn hex_string_to_color(hex: &str) -> Result<Color, ()> {
    let chars: Vec<char> = hex.chars().collect();
    let mut rgb: [u8; 3] = [0; 3];

    for i in (1..6).step_by(2) {
        let mut dec = chars.get(i).ok_or(())?.to_digit(16).ok_or(())? as u8 * 16 as u8;
        dec += chars.get(i + 1).ok_or(())?.to_digit(16).ok_or(())? as u8;

        rgb[i / 2] = dec;
    }

    Ok(Color::RGB(rgb[0], rgb[1], rgb[2]))
}

pub fn color_bar(langs: Vec<(ansi_term::Color, f64)>) -> String {
    let total_length = 40;
    let mut bar = "".to_string();
    for lang in langs {
        let length = (lang.1 * (total_length as f64)) as usize;
        bar += lang
            .0
            .paint("█".repeat(length).as_str())
            .to_string()
            .as_str();
    }

    bar
}
