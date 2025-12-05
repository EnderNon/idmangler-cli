use crate::encode::FuncParams;
use crate::errorfr::Errorfr;
use crate::jsonstruct::Identificationer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// the struct for the stuff I need in in Hashmap<String, GearJson> gear.json. its a big ass pain
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct GearJsonItem {
    identifications: Option<HashMap<String, GearJsonItemInner>>,
}
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
#[serde(untagged)]
pub enum GearJsonItemInner {
    r#Struct(GearJsonItemInnerStruct),
    r#Int(i32),
}
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct GearJsonItemInnerStruct {
    max: i32,
    min: i32,
    raw: i32,
}

/// Function to generate a perfect item.  
/// It returns Result\<Vec\<Identificationer\>, Errorfr\>
pub fn gen_perfect(fr_params: &FuncParams, name: &str, frjson: &HashMap<String, GearJsonItem>) -> Result<Vec<Identificationer>, Errorfr> {
    let mut a: Vec<Identificationer> = Vec::new();
    let name = name.to_lowercase();
    match frjson.get(&name) {
        Some(fr_idents) => {
            if let Some(fr_identmap) = &fr_idents.identifications {
                for i in fr_identmap {
                    if *fr_params.fr_debug_mode {
                        println!("{i:?}");
                    }
                    if let &GearJsonItemInner::Struct(e) = &i.1 {
                        // hardcoded list of inverts. Don't question why it's like this, blame whatever the fuck wynncraft was smoking.
                        // I'm going to have to update this list manually too... why the fuck, wynncraft?
                        let invert_bool: bool = matches!(
                            i.0.to_lowercase().as_str(),
                            "4thspellcost" | "3rdspellcost" | "2ndspellcost" | "1stspellcost" | "raw4thspellcost" | "raw3rdspellcost" | "raw2ndspellcost" | "raw1stspellcost"
                        );

                        let ider: Identificationer = Identificationer {
                            id: i.0.clone(),
                            base: {
                                if invert_bool {
                                    -e.clone().raw
                                }
                                // invert value if invert mode true
                                else {
                                    e.clone().raw
                                } // else regular value
                            },
                            roll: Some(match &e.clone().raw {
                                fr if fr < &0 => {
                                    if invert_bool {
                                        130
                                    }
                                    // value 70 if invert mode true
                                    else {
                                        70
                                    } // else value 130
                                }
                                fr if fr > &0 => {
                                    if invert_bool {
                                        70
                                    }
                                    // value 70 if invert mode true
                                    else {
                                        130
                                    } // else value 130
                                }
                                _ => 0,
                            }),
                        };
                        if *fr_params.fr_debug_mode {
                            println!("ider: {ider:?}");
                        }
                        a.push(ider)
                    }
                }
            }
        }
        None => return Err(Errorfr::PerfectItemNotFound),
    }
    Ok(a)
}
