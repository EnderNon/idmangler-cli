use crate::errorfr::Errorfr;
use crate::jsonstruct::CraftedTypesFr::{Consu, Gear};
use idmangler_lib::types::{ClassType, ConsumableType::*, GearType::*, SkillType};
use idmangler_lib::types::{ConsumableType, GearType, ItemType, TransformVersion};
use serde::Deserialize;
use std::fs;

// structs for the json parsing
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Jsonconfig {
    pub debug: Option<bool>, // not a thing to be encoded, this just toggles debug prints. Also settable using --debug
    // Item Types (Gear, Tome, Charm, Crafted Gear, Crafted Consum)
    pub item_type: ItemTypeDeser,
    // Crafted type for Crafted item types (
    pub crafted_type: Option<String>,
    // name of item
    pub name: Option<String>,
    // durability data (Crafted Gear)
    pub durability: Option<Durability>,
    // requirements data (Crafted Gear, Crafted
    pub requirements: Option<RequirementsDeser>,
    // shiny data
    pub shiny: Option<Shinyjson>,
    // identifications
    pub ids: Option<Vec<Identificationer>>,
    pub powders: Option<Vec<Powder>>,
    pub rerolls: Option<u8>,
}
// reimplementing this because it doesnt have Deserialize.
// Also, changing the SkillPoint stuff into NOT a vec.
// This avoids confusing end user.
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct RequirementsDeser {
    pub level: u8,
    pub class: Option<ClassDeser>,
    pub sp: Option<SkillPointDeser>,
}
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub enum ClassDeser {
    Archer,
    Warrior,
    Assassin,
    Mage,
    Shaman,
}
impl From<ClassDeser> for ClassType {
    fn from(value: ClassDeser) -> Self {
        match value {
            ClassDeser::Archer => ClassType::Archer,
            ClassDeser::Warrior => ClassType::Warrior,
            ClassDeser::Assassin => ClassType::Assasin,
            ClassDeser::Mage => ClassType::Mage,
            ClassDeser::Shaman => ClassType::Shaman,
        }
    }
}
#[derive(Deserialize, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct SkillPointDeser {
    #[serde(alias = "Str")]
    #[serde(alias = "str")]
    #[serde(alias = "strength")]
    pub strength: Option<i32>,
    #[serde(alias = "Dex")]
    #[serde(alias = "dex")]
    #[serde(alias = "dexterity")]
    pub dexterity: Option<i32>,
    #[serde(alias = "Def")]
    #[serde(alias = "def")]
    #[serde(alias = "defense")]
    pub defense: Option<i32>,
    #[serde(alias = "Int")]
    #[serde(alias = "int")]
    #[serde(alias = "intelligence")]
    pub intelligence: Option<i32>,
    #[serde(alias = "Agi")]
    #[serde(alias = "agi")]
    #[serde(alias = "agility")]
    pub agility: Option<i32>,
}

impl From<SkillPointDeser> for Vec<(SkillType, i32)> {
    fn from(value: SkillPointDeser) -> Self {
        let mut returnedvec: Vec<(SkillType, i32)> = Vec::new();
        if let Some(fr_str) = value.strength {
            returnedvec.push((SkillType::Strength, fr_str))
        }
        if let Some(fr_dex) = value.dexterity {
            returnedvec.push((SkillType::Dexterity, fr_dex))
        }
        if let Some(fr_int) = value.intelligence {
            returnedvec.push((SkillType::Intelligence, fr_int))
        }
        if let Some(fr_def) = value.defense {
            returnedvec.push((SkillType::Defence, fr_def))
        }
        if let Some(fr_agi) = value.agility {
            returnedvec.push((SkillType::Agility, fr_agi))
        }
        returnedvec
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CraftedTypesFr {
    Gear(GearType),
    Consu(ConsumableType),
}
impl TryFrom<&str> for CraftedTypesFr {
    type Error = Errorfr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            // consu types
            "potion" | "pot" | "potions" => Ok(Consu(Potion)),
            "food" | "meal" | "foods" | "meals" => Ok(Consu(Food)),
            "scroll" | "paper" => Ok(Consu(Scroll)),
            // 5 weapon types
            "spear" => Ok(Gear(Spear)),
            "wand" => Ok(Gear(Wand)),
            "dagger" => Ok(Gear(Dagger)),
            "bow" => Ok(Gear(Bow)),
            "relik" => Ok(Gear(Relik)),
            // 4 armour types
            "helmet" | "hat" => Ok(Gear(Helmet)),
            "chestplate" | "shirt" | "chest" | "cp" => Ok(Gear(Chestplate)),
            "leggings" | "legs" | "pants" | "trousers" => Ok(Gear(Leggings)),
            "boots" | "shoes" => Ok(Gear(Boots)),
            // 4 accessory types
            "ring" => Ok(Gear(Ring)),
            "bracelet" | "brace" => Ok(Gear(Bracelet)),
            "necklace" => Ok(Gear(Necklace)),
            // General gear types (FALLBACK) (don't use these if not necessary)
            "weapon" => Ok(Gear(Weapon)),
            "accessory" => Ok(Gear(Accessory)),
            // fallback error return
            _ => Err(Errorfr::JsonInvalidCraftedType),
        }
    }
}
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Durability {
    pub effect_strength: Option<u8>,
    pub dura_cur: i32,
    pub dura_max: i32,
}
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Shinystruct {
    pub id: u8,
    pub key: String,
}
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Identificationer {
    pub id: String,
    pub base: i32,
    pub roll: Option<u8>,
}
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Powder {
    pub r#type: char,
    pub amount: Option<u8>,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Shinyjson {
    pub key: String,
    pub value: i64,
}

// struct for general parameters for most encode functions
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FuncParams<'a> {
    pub fr_out: &'a mut Vec<u8>,
    pub fr_debug_mode: &'a bool,
    pub fr_ver: TransformVersion,
}

// I had to clone this and add Deserialize because the original idmangler_lib::types::ItemType does not
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Deserialize)]
pub enum ItemTypeDeser {
    #[serde(alias = "gear")]
    Gear = 0,
    #[serde(alias = "tome")]
    Tome = 1,
    #[serde(alias = "charm")]
    Charm = 2,
    #[serde(alias = "craftedgear")]
    #[serde(alias = "cgear")]
    #[serde(alias = "CGear")]
    CraftedGear = 3,
    #[serde(alias = "craftedconsu")]
    #[serde(alias = "cconsu")]
    #[serde(alias = "CConsu")]
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

pub fn load_jsonconfig(path: &String) -> Result<Jsonconfig, Errorfr> {
    serde_json5::from_reader(&mut fs::File::open(path).map_err(|_| Errorfr::ItemJsonMissing)?)
        .map_err(Errorfr::ItemJsonCorrupt)
}
