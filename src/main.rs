#![allow(clippy::single_match)]
#![allow(non_camel_case_types, non_snake_case)]

mod encode;
mod errorfr;
mod gearjson;
mod jsondl;
mod jsonstruct;
use crate::errorfr::Errorfr;
use crate::jsondl::*;
use crate::jsonstruct::*;
use clap::Parser;
use idmangler_lib::{encoding::string::encode_string, types::EncodingVersion};
use reqwest::Url;
use std::{collections::HashMap, env, fs, io::Write, path::PathBuf};
use crate::gearjson::gen_perfect;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Path for config path
    #[arg(short, long)]
    config: Option<String>,

    /// Enable debug mode (for now this just prints debug info)
    #[arg(long, default_value_t = false)]
    debug: bool,

    /// Download jsons (for ease of use)
    #[arg(short, long)]
    download: Option<String>,

    /// Function to generate perfect value items
    #[arg(long)]
    perfect: Option<String>,
}


fn dl_json(url: Url, savename: String) -> Result<(), Errorfr> {
    let resp = reqwest::blocking::get(url).map_err(|_| Errorfr::JsonDlReqFail)?;
    let body = resp.text().map_err(|_| Errorfr::JsonDlReqBodyInvalid)?;
    let savepath = savename.to_string();
    println!("Downloading file to {savepath}");
    let mut out = fs::File::create(savepath).map_err(|_| Errorfr::JsonDlReqFileCreateFail)?;
    out.write_all(body.as_bytes()).map_err(|_| Errorfr::JsonDlReqFileWriteFail)?;
    Ok(())
}

fn main() {
    if let Err(e) = main_2() {
        println!("{}", e)
    }
}

fn main_2() -> Result<(), Errorfr> {
    let args = Args::parse();
    let mut executable_path = env::current_exe().unwrap();
    PathBuf::pop(&mut executable_path);
    let executable_path = executable_path.to_str().unwrap();

    let mut debug_mode = false;
    if args.debug {
        debug_mode = true;
        println!("Debug mode enabled");
    };

    // download jsons if necessary
    if let Some(dlvalue) = &args.download {
        dl_json_fr(dlvalue, executable_path)
    };

    // check if files load properly and all that
    
    let loaded_config: Jsonconfig = {
        if let Some(t) = &args.config {
            match load_jsonconfig(t) {
                Ok(fr) => {
                    Ok(fr)
                }
                Err(e) => Err(e),
            }
        }
        else {
            println!("no config path provided, fallback to empty.");
            Ok(
                Jsonconfig {
                    debug: Some(false),
                    item_type: ItemTypeDeser::Gear,
                    crafted_type: None,
                    name: None,
                    shiny: None,
                    ids: None,
                    powders: None,
                    rerolls: None,
                    crafted_durability: None,
                    crafted_requirements: None,
                    crafted_ids: None,
                    crafted_damage: None,
                }
            )
        }
    }?;
    let loaded_idkeys = load_idkeys(executable_path)?;
    let loaded_shinystats = load_shinystats(executable_path)?;

    // check if perfect status and change name if so. otherwise blank yep
    let mut namefr: String = "".to_string();
    if let Some(t1) = args.perfect {
        namefr = t1
    }

    // debug mode on if in the loaded config
    if let Some(debugconfig) = loaded_config.debug {
        if debugconfig {
            debug_mode = true
        }
    }
    // main program everything starts here fr
    let mut out: Vec<u8> = Vec::new();

    // create necessary variables
    let ver = EncodingVersion::Version1;

    let mut loaded_config_clone = loaded_config.clone();


    // ENCODE: A Lot Of Stuff
    // Also print any mapped errors
    let cooking = cook(&mut out, &debug_mode, ver, &mut loaded_config_clone, loaded_idkeys, loaded_shinystats, namefr, executable_path);
    if let Err(e) = cooking {
        println!("{}", e); // print error if there is an error
    } else {
        // final string print if there is no error
        println!("{}", cooking?)
    }
    
    Ok(())
}

fn cook(out: &mut Vec<u8>, debug_mode: &bool, ver: EncodingVersion, json_config: &mut Jsonconfig, idsmap: HashMap<String, u8>, json_shiny: Vec<Shinystruct>, namefr: String, executable_path: &str) -> Result<String, Errorfr> {
    let mut fr_params = FuncParams {
        fr_out: out,
        fr_debug_mode: debug_mode,
        fr_ver: ver,
    };

    // ENCODE: StartData and TypeData, ALWAYS
    fr_params.encode_startdata()?;
    fr_params.encode_typedata(json_config.item_type)?;

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
            if namefr != *"" {
                fr_params.encode_namedata(&namefr)?
            }
            else if let Some(real_name) = &json_config.name {
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
            if namefr != *"" {
                println!("Overriding IDs with perfect ones!");
                let fr_gear_cache = load_gear_cache(executable_path)?;
                let resultantvec = gen_perfect(&namefr, &fr_gear_cache)?;
                fr_params.encode_iddata(&resultantvec, idsmap)?
            }
            else if let Some(real_ids) = &json_config.ids {
                fr_params.encode_iddata(real_ids, idsmap)?
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

    // ENCODE: RequirementsData if ItemType is CraftedGear, CraftedConsu
    match json_config.item_type {
        ItemTypeDeser::CraftedGear | ItemTypeDeser::CraftedConsu => {
            if let Some(real_reqs) = json_config.crafted_requirements {
                fr_params.encode_reqdata(real_reqs)?
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
                fr_params.encode_rerolldata(rerollcount)?
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

    let mut final_string: String = encode_string(out);

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