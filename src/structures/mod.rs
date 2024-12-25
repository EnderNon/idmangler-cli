use idmangler_lib::types::ItemType;
use serde::Deserialize;
// structs
#[derive(Deserialize)]
pub struct Powder {
    pub r#type: char,
    pub tier: u8,
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
    pub name: String,
    pub item_type: ItemTypeDeser,
    pub shiny: Option<Shinyjson>,
    pub ids: Vec<Identificationer>,
    pub powder_limit: u8,
    pub powders: Vec<Powder>,
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
            "ShinyStats" | "shinystats" | "ShinyStats.json" | "shinystats.json" => {
                println!("download SHINY_STATS");
                DownloadJsons::ShinyStats
            }
            "All" | "everything" | "both" => {
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
