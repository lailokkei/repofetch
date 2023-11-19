use std::env;

use ansi_term::Color;
use repofetch;
mod langs;

fn main() {
    let langs = vec![
        (Color::RGB(34, 43, 201), 0.32),
        (Color::RGB(159, 43, 123), 0.68),
    ];
    repofetch::langs::print_color_bar(langs);
    // print_color_bar(vec![]);
    // let args: Vec<String> = env::args().collect();
    // if let Err(e) = repofetch::run(&args[1]) {
    //     eprintln!("Error: {e}");
    // }
}
