use serde::Deserialize;
use std::collections::HashMap;

// the struct for each item in Hashmap<String, GearJson> gear.json. its a big ass pain
#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct GearJsonItem {
    attackSpeed: Option<String>,
    averageDps: Option<i16>,
    base: HashMap<String, ItemId>,
    dropMeta: Option<dropMeta>,
    dropRestriction: Option<String>,
    icon: Icon,
    identifications: Option<HashMap<String, i16>>,
    identified: Option<bool>,
    internalName: String,
    lore: Option<String>,
    powderSlots: Option<u8>,
    rarity: String,
    restrictions: Option<String>,
    r#type: String,
    weaponType: Option<String>,
    armourMaterial: Option<String>,
    armourType: Option<String>,
}
#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct ItemId {
    max: i8,
    min: i8,
    raw: i8,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct dropMeta {
    coordinates: [i64; 3],
    name: String,
    r#type: String,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Icon {
    format: String,
    value: IconValue,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct IconValue {
    customModelData: i16,
    id: String,
    name: String,
}
