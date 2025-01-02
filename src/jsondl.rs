use std::collections::HashMap;
use std::fs;
use crate::dl_json;
use crate::errorfr::Errorfr;
use crate::jsonstruct::{DownloadJsons, Jsonconfig, Shinystruct};

pub fn load_jsonconfig(path: &String) -> Result<Jsonconfig, Errorfr> {
    serde_json5::from_reader(&mut fs::File::open(path).map_err(|_| Errorfr::ItemJsonMissing)?)
        .map_err(Errorfr::ItemJsonCorrupt)
}
pub fn load_idkeys(executable_path: &str) -> Result<HashMap<String, u8>, Errorfr> {
    // id_keys.json
    serde_json5::from_reader(
        &mut fs::File::open(executable_path.to_owned() + "/id_keys.json")
            .map_err(|_| Errorfr::IDMapJsonMissing)?,
    )
        .map_err(|_| Errorfr::IDMapJsonCorrupt)
}
pub fn load_shinystats(executable_path: &str) -> Result<Vec<Shinystruct>, Errorfr> {
    // shiny_stats.json
    serde_json5::from_reader(
        &mut fs::File::open(executable_path.to_owned() + "/shiny_stats.json")
            .map_err(|_| Errorfr::ShinyJsonMissing)?,
    )
        .map_err(|_| Errorfr::ShinyJsonCorrupt)
}
pub fn dl_json_fr(dlvalue: &String, executable_path: &str) {
    let jsons = DownloadJsons::from(dlvalue.clone());
    if jsons == DownloadJsons::All || jsons == DownloadJsons::ShinyStats {
        if let Err(e) = dl_json(
            "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json".parse().unwrap(),
            format!("{}{}", executable_path, "/shiny_stats.json"),
        ) { // error handling below
            println!("{} Filename: {}",e,dlvalue)
        }
    }
    if jsons == DownloadJsons::All || jsons == DownloadJsons::IdKeys {
        if let Err(e) = dl_json(
            "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json"
                .parse()
                .unwrap(),
            format!("{}{}", executable_path, "/id_keys.json"),
        ) {
            // error handling below
            println!("{} Filename: {}", e, dlvalue)
        }
    }
}