pub mod files;
mod langs;

use std::{
    collections::HashMap,
    error::Error,
    fs::{self, DirEntry, File},
    io::{self, BufRead},
    path::PathBuf,
};

pub fn run(dir: &str) -> Result<(), Box<dyn Error>> {
    let lang_db = files::read_langs_file()?;

    let mut counter = LinesCounter::new();

    for file in get_all_files(dir)? {
        if let Some(ext) = file.extension() {
            let ext = ".".to_owned() + ext.to_str().unwrap();
            if lang_db.contains_ext(ext.as_str()) {
                let new_loc = lines_of_code(file.to_str().unwrap());
                counter.add_file(ext.as_str(), new_loc);
            }
        } else {
        }
    }

    let mut total_lines = 0;

    for (_, v) in counter.langs_loc.iter() {
        total_lines += v;
    }

    let mut color_fractions: Vec<(ansi_term::Color, f64)> = Vec::new();

    for (ext, loc) in counter.langs_loc {
        let lang_record = lang_db.get_by_ext(ext.as_str());
        let fraction = loc as f64 / total_lines as f64;
        let percentage = fraction * 100.0;

        let color = langs::hex_string_to_color(lang_record.color.as_str()).unwrap();
        color_fractions.push((color, fraction));

        println!("{} {loc} lines {:.1}%", lang_record.name, percentage);
    }

    println!("{}", langs::color_bar(color_fractions));

    Ok(())
}

#[derive(Debug)]
struct LinesCounter {
    langs_loc: HashMap<String, i32>,
}

impl LinesCounter {
    fn new() -> LinesCounter {
        LinesCounter {
            langs_loc: HashMap::new(),
        }
    }

    fn add_file(&mut self, lang: &str, lines: i32) {
        self.langs_loc
            .entry(lang.to_owned())
            .and_modify(|loc| *loc += lines)
            .or_insert(lines);
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn get_all_files(dir: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dir = fs::read_dir(dir)?;

    let mut files: Vec<PathBuf> = Vec::new();
    for entry in dir {
        let entry = entry?;

        if entry.path().is_dir() && !is_hidden(&entry) {
            files.append(&mut get_all_files(entry.path().to_str().unwrap())?);
        } else {
            files.push(entry.path());
        }
    }

    Ok(files)
}

fn lines_of_code(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    io::BufReader::new(file).lines().count() as i32
}
