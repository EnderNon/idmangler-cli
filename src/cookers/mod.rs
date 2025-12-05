use std::collections::HashMap;

use idmangler_lib::encoding::string::encode_string;

mod cconsu;
mod cgear;
mod charm;
mod gear;
mod tome;

use crate::{
    encode::FuncParams, jsonstruct::ItemTypeDeser, Errorfr, Jsonconfig, Shinystruct,
    cookers::{
        gear::cook_gear,
        tome::cook_tome,
        charm::cook_charm,
        cgear::cook_cgear,
        cconsu::cook_cconsu
    }
};

pub(crate) fn cook(fr_params: &mut FuncParams, json_config: &mut Jsonconfig, idsmap: HashMap<String, u8>, json_shiny: Vec<Shinystruct>, namefr: &str, executable_path: &str) -> Result<String, Errorfr> {
    // ENCODE: StartData and TypeData, ALWAYS
    fr_params.encode_startdata()?;
    fr_params.encode_typedata(&json_config.item_type)?;

    match json_config.item_type {
        ItemTypeDeser::Gear => {
            cook_gear(fr_params, json_config, idsmap, json_shiny, namefr, executable_path)
        },
        ItemTypeDeser::Tome => {
            cook_tome(fr_params, json_config, idsmap, namefr, executable_path)
        },
        ItemTypeDeser::Charm => {
            cook_charm(fr_params, json_config, idsmap, namefr, executable_path)
        },
        ItemTypeDeser::CraftedGear => {
            cook_cgear(fr_params, json_config)
        },
        ItemTypeDeser::CraftedConsu => {
            cook_cconsu(fr_params, json_config)
        }
    }?;

    // ENCODE: EndData, ALWAYS
    fr_params.encode_enddata()?;

    let mut final_string: String = encode_string(fr_params.fr_out);

    // add NameAfter. not actually an encode. Its for crafted types.
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
