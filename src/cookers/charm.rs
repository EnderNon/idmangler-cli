use std::collections::HashMap;


use crate::{encode::FuncParams, gearjson::gen_perfect, jsondl::load_gear_cache, Errorfr, Jsonconfig};

pub fn cook_charm(fr_params: &mut FuncParams, json_config: &mut Jsonconfig, idsmap: HashMap<String, u8>, namefr: &str, executable_path: &str) -> Result<(), Errorfr> {
    // ENCODE: NameData
    if !namefr.is_empty() {
        fr_params.encode_namedata(namefr)?
    } else if let Some(real_name) = &json_config.name {
        fr_params.encode_namedata(real_name)?
    } else {
        return Err(Errorfr::JsonNotFoundName);
    };

    // ENCODE: IdentificationData
    if !namefr.is_empty() {
        println!("Overriding IDs with perfect ones!");
        let fr_gear_cache = load_gear_cache(executable_path)?;
        let resultantvec = gen_perfect(fr_params, namefr, &fr_gear_cache)?;
        fr_params.encode_iddata(&resultantvec, &idsmap)?
    } else if let Some(real_ids) = &json_config.ids {
        fr_params.encode_iddata(real_ids, &idsmap)?
    }

    // ENCODE: RerollData if ItemType is Gear, Tome, Charm
    if let Some(rerollcount) = json_config.rerolls {
        // rerolldata
        fr_params.encode_rerolldata(&rerollcount)?
    }

    Ok(())
}
