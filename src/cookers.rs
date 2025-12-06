use std::collections::HashMap;

use serde::Deserialize;
use EncodeOps::*;
use crate::{ItemTypeDeser::*, encode::FuncParams, errorfr::Errorfr, gearjson::gen_perfect, jsondl::load_gear_cache, jsonstruct::{ItemTypeDeser, Jsonconfig, Shinystruct}};



/// Operations for encoding because thats actually the least stupid way to do this
#[derive(Deserialize,Clone)]
pub enum EncodeOps {
    Start,
    Type,
    Name,
    Identification,
    Powder,
    Reroll,
    Shiny,
    End,
    CustomType,
    Damage,
    Defense,
    Durability,
    Requirements,
}

impl ItemTypeDeser {
    pub fn get_ops(&self) -> &[EncodeOps] {
        let s: &[EncodeOps] = match self {
            Gear => {
                &[Start,Type,Name,Identification,Powder,Reroll,Shiny,End]
            },
            Tome => {
                &[Start,Type,Name,Identification,Reroll,End]
            },
            Charm => {
                &[Start,Type,Name,Identification,Reroll,End]
            },
            CraftedGear => {
                &[Start,CustomType,Durability,Damage,Defense,Requirements,Powder,End]
            },
            CraftedConsu => {
                &[Start,CustomType,Requirements,End]
            }
        };
        s
    }
}

impl FuncParams<'_> {
    pub fn encode_from_arr(&mut self, ops: &[EncodeOps], json_config: Jsonconfig, idsmap: HashMap<String, u8>, json_shiny: Vec<Shinystruct>, namefr: &str, executable_path: &str) -> Result<(), Errorfr> {
        for i in ops.iter() {
            match i {
                Start => {
                    // ENCODE: StartData (literally always)
                    self.encode_startdata()?;
                },
                Type => {
                    // ENCODE: TypeData (only on non crafted)
                    self.encode_typedata(&json_config.item_type)?;
                },
                Name => {
                    // ENCODE: NameData
                    if !namefr.is_empty() {
                        self.encode_namedata(namefr)?
                    } else if let Some(real_name) = &json_config.name {
                        self.encode_namedata(real_name)?
                    } else {
                        return Err(Errorfr::JsonNotFoundName);
                    }
                },
                Identification => {
                    // ENCODE: IdentificationData
                    if !namefr.is_empty() {
                        println!("Overriding IDs with perfect ones!");
                        let fr_gear_cache = load_gear_cache(executable_path)?;
                        let resultantvec = gen_perfect(self, namefr, &fr_gear_cache)?;
                        self.encode_iddata(&resultantvec, &idsmap)?
                    } else if let Some(real_ids) = &json_config.ids {
                        self.encode_iddata(real_ids, &idsmap)?
                    }
                },
                Powder => {
                    // ENCODE: PowderData if ItemType is Gear, CraftedGear
                    if let Some(real_powders) = &json_config.powders {
                        self.encode_powderdata(real_powders)?
                    }
                },
                Reroll => {
                    // ENCODE: RerollData if ItemType is Gear, Tome, Charm
                    if let Some(rerollcount) = json_config.rerolls {
                        // rerolldata
                        self.encode_rerolldata(&rerollcount)?
                    }
                },
                Shiny => {
                    // ENCODE: ShinyData if ItemType is Gear
                    if let Some(shiny) = &json_config.shiny {
                        self.encode_shinydata(shiny, &json_shiny)?
                    }
                },
                CustomType => {
                    // ENCODE: CustomGearTypeData / CustomConsumableTypeData
                    if let Some(real_crafted_type) = &json_config.crafted_type {
                        self.encode_typedata_custom(real_crafted_type)?;
                    } else {
                        return Err(Errorfr::JsonNotFoundCraftedType);
                    };
                },
                Damage => {
                    // ENCODE: DamageData (REQUIRED for CraftedGear)
                    if let Some(real_damagedata) = &json_config.crafted_damage {
                        self.encode_damagedata(real_damagedata)?
                    }
                },
                Defense => {
                    // ENCODE: DefenseData (REQUIRED for CraftedGear)
                    if let Some(real_defencedata) = &json_config.crafted_defence {
                        self.encode_defensedata(real_defencedata)?
                    }
                },
                Durability => {
                    // ENCODE: DurabilityData (REQUIRED for CraftedGear)
                    if let Some(real_dura) = &json_config.crafted_durability {
                        self.encode_duradata(real_dura)?;
                    } else {
                        return Err(Errorfr::JsonNotFoundDura);
                    }
                },
                Requirements => {
                    // ENCODE: RequirementsData if ItemType is CraftedGear, CraftedConsu
                    if let Some(real_reqs) = json_config.crafted_requirements {
                        self.encode_reqdata(&real_reqs)?
                    } else {
                        return Err(Errorfr::JsonNotFoundReqs);
                    }
                },
                End => {
                    // ENCODE: EndData (literally always)
                    self.encode_enddata()?;
                },
            }
        }
        Ok(())
    }
}