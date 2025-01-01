use idmangler_lib::types::{ItemType, TransformVersion};
use serde::Deserialize;
// structs for the json parsing
#[derive(Deserialize)]
pub struct Powder {
    pub r#type: char,
    pub amount: Option<u8>,
}
#[derive(Deserialize)]
pub struct Identificationer {
    pub id: String,
    pub base: i32,
    pub roll: Option<u8>,
}
#[derive(Deserialize)]
pub struct Jsonconfig {
    pub debug: Option<bool>,
    pub item_type: ItemTypeDeser,
    pub name: Option<String>,
    pub shiny: Option<Shinyjson>,
    pub ids: Option<Vec<Identificationer>>,
    pub powders: Option<Vec<Powder>>,
    pub rerolls: Option<u8>,
}

#[derive(Deserialize)]
pub struct Shinystruct {
    pub id: u8,
    pub key: String,
}
#[derive(Deserialize)]
pub struct Shinyjson {
    pub key: String,
    pub value: i64,
}

// struct for general parameters for most encode functions
pub struct FuncParams<'a> {
    pub fr_out: &'a mut Vec<u8>,
    pub fr_debug_mode: &'a bool,
    pub fr_ver: TransformVersion,
}

// I had to clone this and add Deserialize because the original idmangler_lib::types::ItemType does not
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Deserialize)]
pub enum ItemTypeDeser {
    Gear = 0,
    Tome = 1,
    Charm = 2,
    CraftedGear = 3,
    CraftedConsu = 4,
}
impl From<ItemTypeDeser> for ItemType {
    fn from(value: ItemTypeDeser) -> ItemType {
        match value {
            ItemTypeDeser::Gear => ItemType::Gear,
            ItemTypeDeser::Tome => ItemType::Tome,
            ItemTypeDeser::Charm => ItemType::Charm,
            ItemTypeDeser::CraftedConsu => ItemType::CraftedConsu,
            ItemTypeDeser::CraftedGear => ItemType::CraftedGear,
        }
    }
}

// stuff for the bit for downloading data jsons for ease of use
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Deserialize)]
pub enum DownloadJsons {
    None,
    IdKeys,
    ShinyStats,
    All,
}
impl From<String> for DownloadJsons {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str().trim() {
            "none" => {
                println!("download NONE");
                DownloadJsons::None
            }
            "id_keys" | "idkeys" | "idkeys.json" | "id_keys.json" => {
                println!("download ID_KEYS");
                DownloadJsons::IdKeys
            }
            "shiny_stats" | "shinystats" | "shiny_stats.json" | "shinystats.json" => {
                println!("download SHINY_STATS");
                DownloadJsons::ShinyStats
            }
            "all" | "everything" | "both" => {
                println!("download BOTH");
                DownloadJsons::All
            }
            _ => {
                println!("Could not understand what Jsons to download, sorry.");
                DownloadJsons::None
            }
        }
    }
}
