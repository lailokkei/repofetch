pub struct LanguageRecord<'a> {
    extensions: Vec<&'a str>,
    pub name: &'a str,
    color: ansi_term::Color,
}

struct LanguageDB<'a> {
    langs: Vec<LanguageRecord<'a>>,
}

impl<'a> LanguageDB<'a> {}

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
