pub mod langs;

use std::{
    collections::HashMap,
    error::Error,
    fs::{self, DirEntry, File},
    io::{self, BufRead},
    path::PathBuf,
};

use langs::LanguageRecord;

struct LangFraction {
    lines: i32,
    language: String,
}

pub fn run(dir: &str) -> Result<(), Box<dyn Error>> {
    let languages: HashMap<&'static str, LanguageRecord<'_>> = HashMap::from([
        // (
        //     "rs",
        //     LanguageRecord {
        //         name: "Rust",
        //         color: RGB::from_hex_string("#00ADD8"),
        //     },
        // ),
        // (
        //     "go",
        //     LanguageRecord {
        //         name: "Go",
        //         color: RGB::from_hex_string("#00ADD8"),
        //     },
        // ),
        // (
        //     "c",
        //     LanguageRecord {
        //         name: "C",
        //         color: RGB { r: 0, g: 0, b: 0 }
        //     },
        // ),
    ]);

    let mut counter = LinesCounter::new();

    for file in get_all_files(dir)? {
        let extension_option = file.extension();

        match extension_option {
            Some(extension) => {
                let extension = extension.to_str().unwrap();
                if languages.contains_key(extension) {
                    let new_loc = lines_of_code(file.to_str().unwrap());
                    counter.add_file(extension, new_loc);
                }
            }
            None => (),
        }
    }

    let mut total_lines = 0;

    for (_, v) in counter.found.iter() {
        total_lines += v;
    }

    // let lang_fraction: Vec<(i32, String,)>

    for (k, v) in counter.found {
        let x = languages.get(k.as_str()).unwrap().name;
        let percentage = v as f64 / total_lines as f64 * 100.0;
        println!("{x} {v} lines {:.1}%", percentage);
    }

    // print_color_bar()

    Ok(())
}
struct LinesCounter {
    found: HashMap<String, i32>,
}

impl LinesCounter {
    fn new() -> LinesCounter {
        LinesCounter {
            found: HashMap::new(),
        }
    }

    fn add_file(&mut self, lang: &str, lines: i32) {
        self.found
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
