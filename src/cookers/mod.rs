use std::collections::HashMap;

use serde::Deserialize;
use EncodeOps::*;
use crate::{ItemTypeDeser::*, encode::FuncParams, errorfr::Errorfr, gearjson::gen_perfect, jsondl::load_gear_cache, jsonstruct::{ItemTypeDeser, Jsonconfig, Shinystruct}};



/// Operations for encoding because thats actually the least stupid way to do this
#[derive(Deserialize,Clone)]
pub enum EncodeOps {
    StartData,
    TypeData,
    NameData,
    IdentificationData,
    PowderData,
    RerollData,
    ShinyData,
    EndData,
    CustomTypeData,
    DamageData,
    DefenseData,
    DurabilityData,
    RequirementsData,
}

impl ItemTypeDeser {
    pub fn get_ops(&self) -> &[EncodeOps] {
        let s: &[EncodeOps] = match self {
            Gear => {
                &[StartData,TypeData,NameData,IdentificationData,PowderData,RerollData,ShinyData,EndData]
            },
            Tome => {
                &[StartData,TypeData,NameData,IdentificationData,RerollData,EndData]
            },
            Charm => {
                &[StartData,TypeData,NameData,IdentificationData,RerollData,EndData]
            },
            CraftedGear => {
                &[StartData,CustomTypeData,DurabilityData,DamageData,DefenseData,RequirementsData,PowderData,EndData]
            },
            CraftedConsu => {
                &[StartData,CustomTypeData,RequirementsData,EndData]
            }
        };
        s
    }
}

impl FuncParams<'_> {
    pub fn encode_from_arr(&mut self, ops: &[EncodeOps], json_config: Jsonconfig, idsmap: HashMap<String, u8>, json_shiny: Vec<Shinystruct>, namefr: &str, executable_path: &str) -> Result<(), Errorfr> {
        for i in ops.into_iter() {
            match i {
                StartData => {
                    // ENCODE: StartData (literally always)
                    self.encode_startdata()?;
                },
                TypeData => {
                    // ENCODE: TypeData (only on non crafted)
                    self.encode_typedata(&json_config.item_type)?;
                },
                NameData => {
                    // ENCODE: NameData
                    if !namefr.is_empty() {
                        self.encode_namedata(namefr)?
                    } else if let Some(real_name) = &json_config.name {
                        self.encode_namedata(real_name)?
                    } else {
                        return Err(Errorfr::JsonNotFoundName);
                    }
                },
                IdentificationData => {
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
                PowderData => {
                    // ENCODE: PowderData if ItemType is Gear, CraftedGear
                    if let Some(real_powders) = &json_config.powders {
                        self.encode_powderdata(real_powders)?
                    }
                },
                RerollData => {
                    // ENCODE: RerollData if ItemType is Gear, Tome, Charm
                    if let Some(rerollcount) = json_config.rerolls {
                        // rerolldata
                        self.encode_rerolldata(&rerollcount)?
                    }
                },
                ShinyData => {
                    // ENCODE: ShinyData if ItemType is Gear
                    if let Some(shiny) = &json_config.shiny {
                        self.encode_shinydata(shiny, &json_shiny)?
                    }
                },
                CustomTypeData => {
                    // ENCODE: CustomGearTypeData / CustomConsumableTypeData
                    if let Some(real_crafted_type) = &json_config.crafted_type {
                        self.encode_typedata_custom(real_crafted_type)?;
                    } else {
                        return Err(Errorfr::JsonNotFoundCraftedType);
                    };
                },
                DamageData => {
                    // ENCODE: DamageData (REQUIRED for CraftedGear)
                    if let Some(real_damagedata) = &json_config.crafted_damage {
                        self.encode_damagedata(real_damagedata)?
                    }
                },
                DefenseData => {
                    // ENCODE: DefenseData (REQUIRED for CraftedGear)
                    if let Some(real_defencedata) = &json_config.crafted_defence {
                        self.encode_defensedata(real_defencedata)?
                    }
                },
                DurabilityData => {
                    // ENCODE: DurabilityData (REQUIRED for CraftedGear)
                    if let Some(real_dura) = &json_config.crafted_durability {
                        self.encode_duradata(real_dura)?;
                    } else {
                        return Err(Errorfr::JsonNotFoundDura);
                    }
                },
                RequirementsData => {
                    // ENCODE: RequirementsData if ItemType is CraftedGear, CraftedConsu
                    if let Some(real_reqs) = json_config.crafted_requirements {
                        self.encode_reqdata(&real_reqs)?
                    } else {
                        return Err(Errorfr::JsonNotFoundReqs);
                    }
                },
                EndData => {
                    // ENCODE: EndData (literally always)
                    self.encode_enddata()?;
                },
            }
        }
        Ok(())
    }
}