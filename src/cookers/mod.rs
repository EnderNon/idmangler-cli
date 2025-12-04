use std::collections::HashMap;

use idmangler_lib::encoding::string::encode_string;

use crate::{
    Errorfr, Jsonconfig, Shinystruct, encode::FuncParams, gearjson::gen_perfect, jsondl::load_gear_cache, jsonstruct::ItemTypeDeser
};

mod cconsu;
mod cgear;
mod charm;
mod gear;
mod tome;

pub(crate) fn cook(fr_params: &mut FuncParams, json_config: &mut Jsonconfig, idsmap: HashMap<String, u8>, json_shiny: Vec<Shinystruct>, namefr: &str, executable_path: &str) -> Result<String, Errorfr> {
    // ENCODE: StartData and TypeData, ALWAYS
    fr_params.encode_startdata()?;
    fr_params.encode_typedata(&json_config.item_type)?;

    // ENCODE: CustomGearTypeData / CustomConsumableTypeData
    match json_config.item_type {
        ItemTypeDeser::CraftedGear | ItemTypeDeser::CraftedConsu => {
            if let Some(real_crafted_type) = &json_config.crafted_type {
                fr_params.encode_typedata_custom(real_crafted_type)?;
            } else {
                return Err(Errorfr::JsonNotFoundCraftedType);
            }
        }
        _ => {}
    }

    // ENCODE: NameData, if ItemType is Gear, Tome, Charm
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::Tome | ItemTypeDeser::Charm => {
            if !namefr.is_empty() {
                fr_params.encode_namedata(namefr)?
            } else if let Some(real_name) = &json_config.name {
                fr_params.encode_namedata(real_name)?
            } else {
                return Err(Errorfr::JsonNotFoundName);
            }
        }
        _ => {}
    }

    // ENCODE: IdentificationData
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::Tome | ItemTypeDeser::Charm => {
            if !namefr.is_empty() {
                println!("Overriding IDs with perfect ones!");
                let fr_gear_cache = load_gear_cache(executable_path)?;
                let resultantvec = gen_perfect(fr_params, namefr, &fr_gear_cache)?;
                fr_params.encode_iddata(&resultantvec, &idsmap)?
            } else if let Some(real_ids) = &json_config.ids {
                fr_params.encode_iddata(real_ids, &idsmap)?
            }
        }
        _ => {}
    }

    // ENCODE: DurabilityData (REQUIRED for CraftedGear)
    match json_config.item_type {
        ItemTypeDeser::CraftedGear => {
            if let Some(real_dura) = &json_config.crafted_durability {
                fr_params.encode_duradata(real_dura)?;
            } else {
                return Err(Errorfr::JsonNotFoundDura);
            }
        }
        _ => {}
    }
    
    // ENCODE: DamageData (REQUIRED for CraftedGear)
    match json_config.item_type {
        ItemTypeDeser::CraftedGear => {
            if let Some(real_damagedata) = &json_config.crafted_damage {
                fr_params.encode_damagedata(real_damagedata)?
            }
        }
        _ => {}
    }
    
    // ENCODE: DefenseData (REQUIRED for CraftedGear)
    match json_config.item_type {
        ItemTypeDeser::CraftedGear => {
            if let Some(real_defencedata) = &json_config.crafted_defence {
                fr_params.encode_defensedata(real_defencedata)?
            }
        },
        _ => {}
    }

    // ENCODE: RequirementsData if ItemType is CraftedGear, CraftedConsu
    match json_config.item_type {
        ItemTypeDeser::CraftedGear | ItemTypeDeser::CraftedConsu => {
            if let Some(real_reqs) = json_config.crafted_requirements {
                fr_params.encode_reqdata(&real_reqs)?
            } else {
                return Err(Errorfr::JsonNotFoundReqs);
            }
        }
        _ => {}
    }

    // ENCODE: PowderData if ItemType is Gear, CraftedGear
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::CraftedGear => {
            if let Some(real_powders) = &json_config.powders {
                fr_params.encode_powderdata(real_powders)?
            }
        }
        _ => {}
    }

    // ENCODE: RerollData if ItemType is Gear, Tome, Charm
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::Tome | ItemTypeDeser::Charm => {
            if let Some(rerollcount) = json_config.rerolls {
                // rerolldata
                fr_params.encode_rerolldata(&rerollcount)?
            }
        }
        _ => {}
    }

    // ENCODE: ShinyData if ItemType is Gear
    match json_config.item_type {
        ItemTypeDeser::Gear => {
            if let Some(shiny) = &json_config.shiny {
                fr_params.encode_shinydata(shiny, &json_shiny)?
            }
        }
        _ => {}
    }

    // ENCODE: EndData, ALWAYS
    fr_params.encode_enddata()?;

    let mut final_string: String = encode_string(fr_params.fr_out);

    // add NameAfter. not actually an encode.
    match json_config.item_type {
        ItemTypeDeser::CraftedGear | ItemTypeDeser::CraftedConsu => {
            if let Some(real_name) = &json_config.name {
                final_string = format!("{} \"{}\"", final_string, real_name)
            } else {
                return Err(Errorfr::JsonNotFoundName);
            }
        }
        _ => {}
    }

    Ok(final_string)
}
