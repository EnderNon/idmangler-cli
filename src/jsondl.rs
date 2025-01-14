use crate::dl_json;
use crate::errorfr::Errorfr;
use crate::gearjson;
use crate::jsonstruct::Shinystruct;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub fn load_idkeys(executable_path: &str) -> Result<HashMap<String, u8>, Errorfr> {
    // id_keys.json
    serde_json5::from_reader(
        &mut fs::File::open(executable_path.to_owned() + "/data/id_keys.json")
        .map_err(|_| Errorfr::IDMapJsonMissing)?)
        .map_err(|_| Errorfr::IDMapJsonCorrupt)
}
pub fn load_shinystats(executable_path: &str) -> Result<Vec<Shinystruct>, Errorfr> {
    // shiny_stats.json
    serde_json5::from_reader(
        &mut fs::File::open(executable_path.to_owned() + "/data/shiny_stats.json")
        .map_err(|_| Errorfr::ShinyJsonMissing)?)
        .map_err(|_| Errorfr::ShinyJsonCorrupt)
}

pub fn load_gear(executable_path: &str) -> Result<HashMap<String, gearjson::GearJsonItem>, Errorfr> {
    // gear.json (ONLY FOR PERFECT ITEM FUNCTION GEN)
    serde_json5::from_reader(
        &mut fs::File::open(executable_path.to_owned() + "/data/gear.json")
        .map_err(|_| Errorfr::GearJsonMissing)?)
        .map_err(Errorfr::GearJsonCorrupt)
}
pub fn dl_json_fr(dlvalue: &String, executable_path: &str) {
    let jsons = DownloadJsons::from(dlvalue.clone());
    if let Err(e) = fs::create_dir_all(format!("{}{}", executable_path, "/data/")) {
        println!("Unable to create path. Path: {} ", e)
    }

    if jsons == DownloadJsons::All || jsons == DownloadJsons::ShinyStats {
        if let Err(e) = dl_json(
            "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json".parse().unwrap(),
            format!("{}{}", executable_path, "/data/shiny_stats.json"),
        ) {
            // error handling below
            println!("{} Filename: {}", e, dlvalue)
        }
    }
    if jsons == DownloadJsons::All || jsons == DownloadJsons::IdKeys {
        if let Err(e) = dl_json(
            "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json".parse().unwrap(),
            format!("{}{}", executable_path, "/data/id_keys.json"),
        ) {
            // error handling below
            println!("{} Filename: {}", e, dlvalue)
        }
    }
    if jsons == DownloadJsons::All || jsons == DownloadJsons::Gear {
        if let Err(e) = dl_json(
            "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/gear.json".parse().unwrap(),
            format!("{}{}", executable_path, "/data/gear.json"),
        ) {
            // error handling below
            println!("{} Filename: {}", e, dlvalue)
        }
    }
}

// stuff for the bit for downloading data jsons for ease of use
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Deserialize)]
pub enum DownloadJsons {
    None,
    IdKeys,
    ShinyStats,
    Gear,
    All,
}
impl From<String> for DownloadJsons {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str().trim() {
            "none" => {
                println!("downloading NONE (Why?)");
                DownloadJsons::None
            }
            "id_keys" | "idkeys" | "idkeys.json" | "id_keys.json" => {
                println!("downloading ID_KEYS");
                DownloadJsons::IdKeys
            }
            "shiny_stats" | "shinystats" | "shiny_stats.json" | "shinystats.json" => {
                println!("downloading SHINY_STATS");
                DownloadJsons::ShinyStats
            }
            "gear" | "gear.json" => {
                println!("downloading GEAR");
                DownloadJsons::Gear
            }
            "all" | "everything" | "both" => {
                println!("downloading ALL jsons");
                DownloadJsons::All
            }
            _ => {
                println!("downloading NONE (unable to understand prompt)");
                DownloadJsons::None
            }
        }
    }
}
