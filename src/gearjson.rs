use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errorfr::Errorfr;
use crate::jsonstruct::Identificationer;

// the struct for the stuff I need in in Hashmap<String, GearJson> gear.json. its a big ass pain
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct GearJsonItem {
    identifications: Option<
        HashMap<
            String, GearJsonItemInner
        >
    >
}
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
#[serde(untagged)]
pub enum GearJsonItemInner {
    r#Struct(GearJsonItemInnerStruct),
    r#Int(i32)
}
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct GearJsonItemInnerStruct {
    max: i32,
    min: i32,
    raw: i32
}

pub fn gen_perfect(name: &str, frjson: &HashMap<String, GearJsonItem>) -> Result<Vec<Identificationer>, Errorfr> {
    let mut a: Vec<Identificationer> = Vec::new();
    let name = name.to_lowercase();
    match frjson.get(&name) {
        Some(fr_idents) => {
            if let Some(fr_identmap) = &fr_idents.identifications {
                for i in fr_identmap {
                    if let &GearJsonItemInner::Struct(e) = &i.1 {
                        a.push(
                            Identificationer {
                                id: i.0.clone(),
                                base: e.clone().raw,
                                roll: Some(match &e.clone().raw {
                                    fr if fr<&0 => 69,
                                    fr if fr>&0 => 130,
                                    _ => 0
                                }),
                            }
                        )
                    }
                }
            } 
        },
        None => return Err(Errorfr::PerfectItemNotFound)
    }
    Ok(a)
}