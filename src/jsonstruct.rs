use crate::errorfr::Errorfr;
use crate::jsonstruct::CraftedTypesFr::{Consu, Gear};
use idmangler_lib::block::{DamageData, DefenseData};
use idmangler_lib::types::{AttackSpeed, ClassType, ConsumableType, ConsumableType::*, CraftedGearType, CraftedGearType::*, Element, ItemType, SkillType};
use serde::Deserialize;
use std::fs;
use std::ops::Range;

// structs for the json parsing
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct Jsonconfig {
    /// not a thing to be encoded, this just toggles debug prints. Also settable using --debug
    #[serde(alias = "Debug", alias = "DEBUG")]
    pub debug: Option<bool>,

    /// Item Types (Gear, Tome, Charm, Crafted Gear, Crafted Consum)
    #[serde(alias = "ITEMTYPE", alias = "ItemType", alias = "Itemtype", alias = "itemtype")]
    #[serde(alias = "ITEM_TYPE", alias = "Item_Type", alias = "Item_type")]
    pub item_type: ItemTypeDeser,

    /// Crafted type for Crafted item types
    #[serde(alias = "CRAFTEDTYPE", alias = "CraftedType", alias = "Craftedtype", alias = "craftedtype")]
    #[serde(alias = "CRAFTED_TYPE", alias = "Crafted_Type", alias = "Crafted_type")]
    pub crafted_type: Option<String>,

    /// name of item
    #[serde(alias = "Name", alias = "NAME")]
    pub name: Option<String>,

    /// shiny data
    #[serde(alias = "Shiny", alias = "SHINY")]
    pub shiny: Option<Shinyjson>,

    /// identifications
    #[serde(alias = "identifications", alias = "Identifications", alias = "IDENTIFICATIONS")]
    #[serde(alias = "Ids", alias = "IDS")]
    pub ids: Option<Vec<Identificationer>>,

    /// powders stuff
    #[serde(alias = "powder", alias = "Powder", alias = "POWDER")]
    #[serde(alias = "Powders", alias = "POWDERS")]
    pub powders: Option<Vec<PowderFr>>,

    /// rerolls
    #[serde(alias = "reroll", alias = "Reroll", alias = "REROLL")]
    #[serde(alias = "Rerolls", alias = "REROLLS")]
    pub rerolls: Option<u8>,

    /// durability data (Crafted Gear)
    #[serde(alias = "durability", alias = "Durability", alias = "DURABILITY")]
    #[serde(alias = "dura", alias = "Dura", alias = "DURA")]
    pub crafted_durability: Option<Durability>,

    /// requirements data (Crafted)
    #[serde(alias = "requirement", alias = "Requirement", alias = "REQUIREMENT")]
    #[serde(alias = "requirements", alias = "Requirements", alias = "REQUIREMENTS")]
    pub crafted_requirements: Option<RequirementsDeser>,

    /// identifications (Crafted)  
    ///
    /// to be honest i wish there was a better way instead of too many aliases
    #[serde(alias = "CRAFTEDIDS", alias = "CraftedIds", alias = "Craftedids", alias = "craftedids")]
    #[serde(alias = "CRAFTED_IDS", alias = "Crafted_Ids", alias = "Crafted_ids")]
    #[serde(alias = "CRAFTED_IDENTIFICATIONS", alias = "Crafted_Identifications", alias = "Crafted_identifications", alias = "crafted_identifications")]
    #[serde(alias = "CRAFTEDIDENTIFICATIONS", alias = "CraftedIdentifications", alias = "Craftedidentifications", alias = "craftedidentifications")]
    #[serde(alias = "CIDS", alias = "CIds", alias = "cids")]
    #[serde(alias = "CIDS", alias = "Cids")]
    pub crafted_ids: Option<Vec<IdentificationerCrafted>>,

    #[serde(alias = "CRAFTEDDMG", alias = "CraftedDmg", alias = "Crafteddmg", alias = "crafteddmg")]
    #[serde(alias = "CRAFTED_DMG", alias = "Crafted_Dmg", alias = "Crafted_dmg")]
    #[serde(alias = "CRAFTED_DAMAGE", alias = "Crafted_Damage", alias = "Crafted_damage", alias = "crafted_damage")]
    #[serde(alias = "CRAFTEDDAMAGE", alias = "CraftedDamage", alias = "Crafteddamage", alias = "crafteddamage")]
    #[serde(alias = "DMG", alias = "Dmg", alias = "dmg")]
    #[serde(alias = "DAMAGE", alias = "Damage", alias = "damage")]
    pub crafted_damage: Option<DamageDeser>,

    pub crafted_defence: Option<DefenceDeser>,
}
/// reimplementing this because it doesnt have Deserialize.
/// Also, changing the SkillPoint stuff into NOT a vec.
/// This avoids confusing end user.
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
    #[serde(alias = "Str", alias = "str", alias = "Strength")]
    pub strength: Option<i32>,
    #[serde(alias = "Dex", alias = "dex", alias = "Dexterity")]
    pub dexterity: Option<i32>,
    #[serde(alias = "Def", alias = "def", alias = "Defense")]
    pub defense: Option<i32>,
    #[serde(alias = "Int", alias = "int", alias = "Intelligence")]
    pub intelligence: Option<i32>,
    #[serde(alias = "Agi", alias = "agi", alias = "Agility")]
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
    pub reroll: Option<u8>,
    pub value: i64,
}

// this one isn't even because it can't deser, it's because I want to restructure and add alias for the fields
#[derive(Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DamageDeserElement {
    #[serde(alias = "min", alias = "Min", alias = "MIN")]
    #[serde(alias = "Lower", alias = "LOWER")]
    lower: i32,
    #[serde(alias = "max", alias = "Max", alias = "MAX")]
    #[serde(alias = "Upper", alias = "UPPER")]
    upper: i32,
}
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct DamageDeser {
    #[serde(alias = "AttackSpeed", alias = "ATTACKSPEED", alias = "attackspeed")]
    #[serde(alias = "Attack_Speed", alias = "ATTACK_SPEED")]
    #[serde(alias = "atkspd", alias = "AtkSpd", alias = "ATKSPD")]
    #[serde(alias = "atk_spd", alias = "Atk_Spd", alias = "ATKSPD")]
    pub attack_speed: AttackSpeed,
    #[serde(alias = "N", alias = "Neutral", alias = "NEUTRAL")]
    #[serde(alias = "normal", alias = "Normal", alias = "NORMAL")]
    pub neutral: Option<DamageDeserElement>,
    #[serde(alias = "E", alias = "Earth", alias = "EARTH")]
    pub earth: Option<DamageDeserElement>,
    #[serde(alias = "T", alias = "Thunder", alias = "THUNDER")]
    pub thunder: Option<DamageDeserElement>,
    #[serde(alias = "W", alias = "Water", alias = "WATER")]
    pub water: Option<DamageDeserElement>,
    #[serde(alias = "F", alias = "Fire", alias = "FIRE")]
    pub fire: Option<DamageDeserElement>,
    #[serde(alias = "A", alias = "Air", alias = "AIR")]
    pub air: Option<DamageDeserElement>,
}
impl TryFrom<&DamageDeser> for DamageData {
    type Error = Errorfr;
    fn try_from(value: &DamageDeser) -> Result<Self, Self::Error> {
        let mut damagesfr: Vec<(Option<Element>, Range<i32>)> = Vec::new();
        if let Some(T) = value.neutral {
            damagesfr.push((None, Range { start: T.lower, end: T.upper }))
        };
        if let Some(T) = value.earth {
            damagesfr.push((Some(Element::Earth), Range { start: T.lower, end: T.upper }))
        };
        if let Some(T) = value.thunder {
            damagesfr.push((Some(Element::Thunder), Range { start: T.lower, end: T.upper }))
        };
        if let Some(T) = value.water {
            damagesfr.push((Some(Element::Water), Range { start: T.lower, end: T.upper }))
        };
        if let Some(T) = value.fire {
            damagesfr.push((Some(Element::Fire), Range { start: T.lower, end: T.upper }))
        };
        if let Some(T) = value.air {
            damagesfr.push((Some(Element::Air), Range { start: T.lower, end: T.upper }))
        };
        Ok(Self {
            attack_speed: value.attack_speed,
            damages: damagesfr,
        })
    }
}
#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct DefenceDeser {
    #[serde(alias = "HP", alias = "Hp", alias = "hP", alias = "hp")]
    #[serde(alias = "Health", alias = "HEALTH")]
    pub health: i32,
    #[serde(alias = "E", alias = "Earth", alias = "EARTH")]
    pub earth: Option<i32>,
    #[serde(alias = "T", alias = "Thunder", alias = "THUNDER")]
    pub thunder: Option<i32>,
    #[serde(alias = "W", alias = "Water", alias = "WATER")]
    pub water: Option<i32>,
    #[serde(alias = "F", alias = "Fire", alias = "FIRE")]
    pub fire: Option<i32>,
    #[serde(alias = "A", alias = "Air", alias = "AIR")]
    pub air: Option<i32>,
}
impl From<&DefenceDeser> for DefenseData {
    fn from(value: &DefenceDeser) -> Self {
        let mut defencesfr: Vec<(Element, i32)> = Vec::new();
        if let Some(T) = value.air {
            defencesfr.push((Element::Air, T));
        }
        if let Some(T) = value.earth {
            defencesfr.push((Element::Earth, T));
        }
        if let Some(T) = value.thunder {
            defencesfr.push((Element::Thunder, T));
        }
        if let Some(T) = value.water {
            defencesfr.push((Element::Water, T));
        }
        if let Some(T) = value.fire {
            defencesfr.push((Element::Fire, T));
        }
        DefenseData {
            health: value.health,
            defences: defencesfr,
        }
    }
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
