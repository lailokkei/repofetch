use crate::langs::{LanguageDB, LanguageRecord};
use std::{
    error::Error,
    fs::{self, File},
    io::BufReader,
};

pub fn write_langs_file() -> Result<(), Box<dyn Error>> {
    let colors_string = fs::read_to_string("data/color.json")?;
    let extensions_string = fs::read_to_string("data/ext.json")?;

    let colors_json: serde_json::Value = serde_json::from_str(colors_string.as_str())?;
    let extensions_json: serde_json::Value = serde_json::from_str(extensions_string.as_str())?;

    let mut langs: Vec<LanguageRecord> = Vec::new();

    let colors_map = colors_json.as_object().unwrap();

    for ext_obj in extensions_json.as_array().unwrap() {
        let name = ext_obj["name"].as_str().unwrap().to_owned();
        if let Some(color_obj) = colors_map.get(name.as_str()) {
            if let Some(ext_array) = ext_obj["extensions"].as_array() {
                let mut extensions: Vec<String> = Vec::new();

                for ext in ext_array {
                    extensions.push(ext.as_str().unwrap().to_owned())
                }

                let lang_record = LanguageRecord {
                    extensions,
                    name,
                    color: color_obj["color"].as_str().unwrap_or("#000000").to_owned(),
                };

                langs.push(lang_record);
            }
        }
    }

    let langs_json = serde_json::to_string(&langs).unwrap();
    fs::write("data/langs.json", langs_json).expect("unable to write file");

    Ok(())
}

pub fn read_langs_file() -> Result<LanguageDB, Box<dyn Error>> {
    let reader = BufReader::new(File::open("data/langs.json")?);

    let lang_db = LanguageDB::new(serde_json::from_reader(reader)?);

    Ok(lang_db)
}
