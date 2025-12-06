use std::collections::HashMap;


use crate::{encode::FuncParams, gearjson::gen_perfect, jsondl::load_gear_cache, Errorfr, Jsonconfig, Shinystruct};

pub fn cook_gear(fr_params: &mut FuncParams, json_config: &mut Jsonconfig, idsmap: HashMap<String, u8>, json_shiny: Vec<Shinystruct>, namefr: &str, executable_path: &str) -> Result<(), Errorfr> {
    // ENCODE: NameData
    if !namefr.is_empty() {
        fr_params.encode_namedata(namefr)?
    } else if let Some(real_name) = &json_config.name {
        fr_params.encode_namedata(real_name)?
    } else {
        return Err(Errorfr::JsonNotFoundName);
    }









    Ok(())
}
