use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// the struct for the stuff I need in in Hashmap<String, GearJson> gear.json. its a big ass pain
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct GearJsonItem {
    identifications: Option<
        HashMap<
            String, GearJsonItemInner
        >
    >
}
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum GearJsonItemInner {
    r#Struct(GearJsonItemInnerStruct),
    r#Int(i32)
}
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct GearJsonItemInnerStruct {
    max: i32,
    min: i32,
    raw: i32
}