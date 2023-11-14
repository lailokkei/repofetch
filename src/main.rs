use std::{
    env,
    error::Error,
    fs::{self, DirEntry},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = walk_dirs(&args[1]) {
        eprintln!("Error: {e}");
    }
}

fn run(dir: &str) -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(dir)?;
    for path in paths {
        let path = path?;
        if path.path().is_file() {
            let ext = match path.path().extension() {
                Some(x) => x.to_str().unwrap().to_owned(),
                None => "none".to_string(),
            };

            println!("{ext}");
        }
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

fn walk_dirs(dir: &str) -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(dir)?;
    for path in paths {
        let path = path?;
        dbg!(path.path());
        if path.path().is_dir() && !is_hidden(&path) {
            walk_dirs(path.path().to_str().unwrap())?;
        }
    }
    Ok(())
}
