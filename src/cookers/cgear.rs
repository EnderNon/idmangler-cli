use std::collections::HashMap;

use idmangler_lib::encoding::string::encode_string;

use crate::{encode::FuncParams, gearjson::gen_perfect, jsondl::load_gear_cache, jsonstruct::ItemTypeDeser, Errorfr, Jsonconfig, Shinystruct};

pub fn cook_cgear(fr_params: &mut FuncParams, json_config: &mut Jsonconfig) -> Result<(), Errorfr> {
    // ENCODE: CustomGearTypeData / CustomConsumableTypeData
    if let Some(real_crafted_type) = &json_config.crafted_type {
        fr_params.encode_typedata_custom(real_crafted_type)?;
    } else {
        return Err(Errorfr::JsonNotFoundCraftedType);
    };

    // ENCODE: DurabilityData (REQUIRED for CraftedGear)
    if let Some(real_dura) = &json_config.crafted_durability {
        fr_params.encode_duradata(real_dura)?;
    } else {
        return Err(Errorfr::JsonNotFoundDura);
    }

    // ENCODE: DamageData (REQUIRED for CraftedGear)
    if let Some(real_damagedata) = &json_config.crafted_damage {
        fr_params.encode_damagedata(real_damagedata)?
    }

    // ENCODE: DefenseData (REQUIRED for CraftedGear)
    if let Some(real_defencedata) = &json_config.crafted_defence {
        fr_params.encode_defensedata(real_defencedata)?
    }

    // ENCODE: RequirementsData if ItemType is CraftedGear, CraftedConsu
    if let Some(real_reqs) = json_config.crafted_requirements {
        fr_params.encode_reqdata(&real_reqs)?
    } else {
        return Err(Errorfr::JsonNotFoundReqs);
    }

    // ENCODE: PowderData if ItemType is Gear, CraftedGear
    if let Some(real_powders) = &json_config.powders {
        fr_params.encode_powderdata(real_powders)?
    }

    Ok(())
}
