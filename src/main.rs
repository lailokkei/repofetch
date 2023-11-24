use repofetch;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = repofetch::run(&args[1]) {
        eprintln!("Error: {e}");
    }
}
