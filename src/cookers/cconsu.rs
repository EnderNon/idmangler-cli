

use crate::{encode::FuncParams, Errorfr, Jsonconfig};

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
