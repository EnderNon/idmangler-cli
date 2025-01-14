use crate::errorfr::Errorfr;
use crate::jsonstruct::CraftedTypesFr::{Consu, Gear};
use idmangler_lib::types::{AttackSpeed, ClassType, ConsumableType, ConsumableType::*, CraftedGearType, CraftedGearType::*, EncodingVersion, ItemType, SkillType};
use serde::Deserialize;
use std::fs;

// structs for the json parsing
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct Jsonconfig {
    // not a thing to be encoded, this just toggles debug prints. Also settable using --debug
    #[serde(alias = "Debug", alias = "DEBUG")]
    pub debug: Option<bool>,

    // Item Types (Gear, Tome, Charm, Crafted Gear, Crafted Consum)
    #[serde(alias = "itemtype", alias = "ITEM_TYPE", alias = "ITEMTYPE", alias = "Item_Type", alias = "Item_type", alias = "ItemType", alias = "Itemtype")]
    pub item_type: ItemTypeDeser,

    // Crafted type for Crafted item types
    #[serde(alias = "craftedtype", alias = "CRAFTED_TYPE", alias = "CRAFTEDTYPE", alias = "Crafted_Type", alias = "Crafted_type", alias = "CraftedType", alias = "Craftedtype")]
    pub crafted_type: Option<String>,

    // name of item
    #[serde(alias = "Name", alias = "NAME")]
    pub name: Option<String>,

    // shiny data
    #[serde(alias = "Shiny", alias = "SHINY")]
    pub shiny: Option<Shinyjson>,

    // identifications
    #[serde(alias = "Ids", alias = "IDS", alias = "identifications", alias = "Identifications", alias = "IDENTIFICATIONS")]
    pub ids: Option<Vec<Identificationer>>,

    // powders stuff
    #[serde(alias = "Powders", alias = "POWDERS", alias = "powder", alias = "Powder", alias = "POWDER")]
    pub powders: Option<Vec<PowderFr>>,

    // rerolls
    #[serde(alias = "Rerolls", alias = "REROLLS", alias = "reroll", alias = "Reroll", alias = "REROLL")]
    pub rerolls: Option<u8>,

    // durability data (Crafted Gear)
    #[serde(alias = "durability", alias = "Durability", alias = "DURABILITY", alias = "dura", alias = "Dura", alias = "DURA")]
    pub crafted_durability: Option<Durability>,

    // requirements data (Crafted)
    #[serde(alias = "requirement", alias = "Requirement", alias = "REQUIREMENT", alias = "requirements", alias = "Requirements", alias = "REQUIREMENTS")]
    pub crafted_requirements: Option<RequirementsDeser>,

    // identifications (Crafted)
    // to be honest i wish there was a better way instead of too many aliases
    #[serde(
        alias = "craftedids",
        alias = "CRAFTED_IDS",
        alias = "CRAFTEDIDS",
        alias = "Crafted_Ids",
        alias = "Crafted_ids",
        alias = "CraftedIds",
        alias = "Craftedids",
        alias = "craftedidentifications",
        alias = "CRAFTED_IDENTIFICATIONS",
        alias = "CRAFTEDIDENTIFICATIONS",
        alias = "Crafted_Identifications",
        alias = "Crafted_identifications",
        alias = "CraftedIdentifications",
        alias = "Craftedidentifications"
    )]
    pub crafted_ids: Option<Vec<IdentificationerCrafted>>,

    pub crafted_damage: Option<DamageDeser>,
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
    #[serde(alias = "Str", alias = "str", alias = "strength")]
    pub strength: Option<i32>,
    #[serde(alias = "Dex", alias = "dex", alias = "dexterity")]
    pub dexterity: Option<i32>,
    #[serde(alias = "Def", alias = "def", alias = "defense")]
    pub defense: Option<i32>,
    #[serde(alias = "Int", alias = "int", alias = "intelligence")]
    pub intelligence: Option<i32>,
    #[serde(alias = "Agi", alias = "agi", alias = "agility")]
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
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum CraftedTypesFr {
    Gear(CraftedGearType),
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
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
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
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct Identificationer {
    pub id: String,
    pub base: i32,
    pub roll: Option<u8>,
}
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct IdentificationerCrafted {
    pub name: String,
    pub max_roll: i32,
}
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct PowderFr {
    pub r#type: char,
    pub amount: Option<u8>,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct Shinyjson {
    pub key: String,
    pub value: i64,
}

// struct for general parameters for most encode functions
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct FuncParams<'a> {
    pub fr_out: &'a mut Vec<u8>,
    pub fr_debug_mode: &'a bool,
    pub fr_ver: EncodingVersion,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct DamageDeser {
    pub attack_speed: AttackSpeed,
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
    #[serde(alias = "craftedgear", alias = "cgear", alias = "CGear")]
    CraftedGear = 3,
    #[serde(alias = "craftedconsu", alias = "cconsu", alias = "CConsu")]
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
    serde_json5::from_reader(&mut fs::File::open(path).map_err(|_| Errorfr::ItemJsonMissing)?).map_err(Errorfr::ItemJsonCorrupt)
}
