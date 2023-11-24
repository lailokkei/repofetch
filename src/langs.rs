use std::collections::HashMap;

use ansi_term::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LanguageRecord {
    pub extensions: Vec<String>,
    pub name: String,
    pub color: String,
}

pub struct LanguageDB {
    pub langs: Vec<LanguageRecord>,
    pub ext_map: HashMap<String, usize>,
}

impl LanguageDB {
    pub fn new(langs: Vec<LanguageRecord>) -> LanguageDB {
        let mut ext_map: HashMap<String, usize> = HashMap::new();
        for (i, lang) in langs.iter().enumerate() {
            for ext in lang.extensions.iter() {
                ext_map.insert(ext.to_owned(), i);
            }
        }
        LanguageDB { langs, ext_map }
    }

    pub fn contains_ext(&self, ext: &str) -> bool {
        self.ext_map.contains_key(ext)
    }

    pub fn get_by_ext(&self, ext: &str) -> &LanguageRecord {
        let i = self.ext_map.get(ext).expect(format!("{}", ext).as_str());
        self.langs.get(*i).unwrap()
    }
}

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
