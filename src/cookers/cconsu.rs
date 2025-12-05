use std::collections::HashMap;

use idmangler_lib::encoding::string::encode_string;

use crate::{encode::FuncParams, gearjson::gen_perfect, jsondl::load_gear_cache, jsonstruct::ItemTypeDeser, Errorfr, Jsonconfig, Shinystruct};

pub fn cook_cconsu(fr_params: &mut FuncParams, json_config: &mut Jsonconfig) -> Result<(), Errorfr> {
    // ENCODE: CustomGearTypeData / CustomConsumableTypeData
    if let Some(real_crafted_type) = &json_config.crafted_type {
        fr_params.encode_typedata_custom(real_crafted_type)?;
    } else {
        return Err(Errorfr::JsonNotFoundCraftedType);
    };

    // ENCODE: RequirementsData if ItemType is CraftedGear, CraftedConsu
    if let Some(real_reqs) = json_config.crafted_requirements {
        fr_params.encode_reqdata(&real_reqs)?
    } else {
        return Err(Errorfr::JsonNotFoundReqs);
    }

    Ok(())
}
