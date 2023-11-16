use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, DirEntry, File},
    io::{self, BufRead},
    path::PathBuf,
};

struct Language<'a> {
    name: &'a str,
    color: &'a str,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(lines_of_code(&args[1]));
    if let Err(e) = run(&args[1]) {
        eprintln!("Error: {e}");
    }
}

fn run(dir: &str) -> Result<(), Box<dyn Error>> {
    let languages = HashMap::from([
        (
            "rs",
            Language {
                name: "Rust",
                color: "idk",
            },
        ),
        (
            "go",
            Language {
                name: "Go",
                color: "idk",
            },
        ),
    ]);

    let mut found: HashMap<String, i32> = HashMap::new();

    let files = get_all_files(dir)?;

    for file in files {
        let file_type = file.extension();

        match file_type {
            Some(ext) => {
                let x = ext.to_str().unwrap();
                if languages.contains_key(x) {
                    let new_loc = lines_of_code(file.to_str().unwrap());
                    found
                        .entry(x.to_owned())
                        .and_modify(|loc| *loc += new_loc)
                        .or_insert(new_loc);
                }
            }
            None => (),
        }
    }

    for (k, v) in found {
        let name = languages.get(k.as_str()).unwrap().name;
        println!("{name}, {v} loc");
    }

    Ok(())
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
