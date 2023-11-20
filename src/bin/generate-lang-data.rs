use repofetch::langs::LanguageRecord;
use std::{
    error::Error,
    fs::{self},
};

fn main() -> Result<(), Box<dyn Error>> {
    let colors_string = fs::read_to_string("data/color.json")?;
    let extensions_string = fs::read_to_string("data/ext.json")?;

    let colors_json: serde_json::Value = serde_json::from_str(colors_string.as_str())?;
    let extensions_json: serde_json::Value = serde_json::from_str(extensions_string.as_str())?;

    let mut langs: Vec<LanguageRecord> = Vec::new();

    let colors_map = colors_json.as_object().unwrap();

    for ext_obj in extensions_json.as_array().unwrap() {
        let name = ext_obj["name"].as_str().unwrap();
        if let Some(color_obj) = colors_map.get(name) {
            if let Some(ext_array) = ext_obj["extensions"].as_array() {
                let mut extensions: Vec<&str> = Vec::new();

                for ext in ext_array {
                    extensions.push(ext.as_str().unwrap())
                }

                let lang_record = LanguageRecord {
                    extensions,
                    name,
                    color: color_obj["color"].as_str().unwrap_or("#000000"),
                };

                langs.push(lang_record);
            }
        }
    }

    let langs_json = serde_json::to_string(&langs).unwrap();
    fs::write("data/langs.json", langs_json).expect("unable to write file");

    Ok(())
}
