#![allow(clippy::single_match)]

use idmangler_lib::{encoding::string::encode_string, types::EncodingVersion};

use std::{collections::HashMap, env, fs, io, path::PathBuf};

mod encode;
mod errorfr;
mod jsondl;
mod jsonstruct;
use crate::encode::*;
use crate::errorfr::Errorfr;
use crate::jsondl::*;
use crate::jsonstruct::*;

use crate::errorfr::Errorfr::JsonNotFoundCraftedType;
use clap::Parser;
use reqwest::Url;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Path for config path
    #[arg(short, long)]
    config: Option<String>,

    /// Enable debug mode
    #[arg(long, default_value_t = false)]
    debug: bool,

    /// Download jsons (for ease of use)
    #[arg(short, long)]
    download: Option<String>,
}

fn dl_json(url: Url, savename: String) -> Result<(), Errorfr> {
    let resp = reqwest::blocking::get(url).map_err(|_| Errorfr::JsonDlReqFail)?;
    let body = resp.text().map_err(|_| Errorfr::JsonDlReqBodyInvalid)?;
    let savepath = savename.to_string();
    println!("Downloading file to {savepath}");
    let mut out = fs::File::create(savepath).map_err(|_| Errorfr::JsonDlReqFileCreateFail)?;
    io::copy(&mut body.as_bytes(), &mut out).map_err(|_| Errorfr::JsonDlReqFileWriteFail)?;
    Ok(())
}

fn main() {
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
    if let Some(t) = &args.config {
        match load_idkeys(executable_path) {
            Ok(loaded_idkeys) => {
                match load_shinystats(executable_path) {
                    Ok(loaded_shinystats) => {
                        match load_jsonconfig(t) {
                            Ok(loaded_config) => {
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
                                
                                
                                let loaded_config_borrow = &loaded_config;
                                // ENCODE: ALotOfStuff
                                // Also print any mapped errors
                                let cooking = cook(
                                    &mut out,
                                    &debug_mode,
                                    ver,
                                    loaded_config_borrow,
                                    loaded_idkeys,
                                    loaded_shinystats,
                                );
                                if let Err(e) = cooking {
                                    println!("{}", e); // print error if there is an error
                                } else {
                                    // final string print if there is no error

                                    println!("{}",cooking.unwrap())
                                }
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

fn cook(
    out: &mut Vec<u8>,
    debug_mode: &bool,
    ver: EncodingVersion,
    json_config: &Jsonconfig,
    idsmap: HashMap<String, u8>,
    json_shiny: Vec<Shinystruct>,
) -> Result<String, Errorfr> {
    let mut fr_params = FuncParams {
        fr_out: out,
        fr_debug_mode: debug_mode,
        fr_ver: ver,
    };
    
    // ENCODE: StartData and TypeData, ALWAYS
    encode_startdata(&mut fr_params)?;
    encode_typedata(&mut fr_params, json_config.item_type)?;

    // ENCODE: CustomGearTypeData / CustomConsumableTypeData
    match json_config.item_type {
        ItemTypeDeser::CraftedGear | ItemTypeDeser::CraftedConsu => {
            if let Some(real_crafted_type) = &json_config.crafted_type {
                encode_typedata_custom(&mut fr_params, real_crafted_type)?;
            } else {
                return Err(JsonNotFoundCraftedType);
            }
        }
        _ => {}
    }

    // ENCODE: NameData, if ItemType is Gear, Tome, Charm
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::Tome | ItemTypeDeser::Charm => {
            if let Some(real_name) = &json_config.name {
                encode_namedata(&mut fr_params, real_name)?
            } else {
                return Err(Errorfr::JsonNotFoundName);
            }
        }
        _ => {}
    }

    // ENCODE: IdentificationData
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::Tome | ItemTypeDeser::Charm => {
            if let Some(real_ids) = &json_config.ids {
                encode_iddata(&mut fr_params, real_ids, idsmap)?
            }
        }
        _ => {}
    }

    // ENCODE: DurabilityData (REQUIRED for CraftedGear)
    match json_config.item_type {
        ItemTypeDeser::CraftedGear => {
            if let Some(real_dura) = &json_config.durability {
                encode_duradata(&mut fr_params, real_dura)?;
            } else {
                return Err(Errorfr::JsonNotFoundDura);
            }
        }
        _ => {}
    }

    // ENCODE: RequirementsData if ItemType is CraftedGear, CraftedConsu
    match json_config.item_type {
        ItemTypeDeser::CraftedGear | ItemTypeDeser::CraftedConsu => {
            if let Some(real_reqs) = json_config.requirements {
                encode_reqdata(&mut fr_params, real_reqs)?
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
                encode_powderdata(&mut fr_params, real_powders)?
            }
        }
        _ => {}
    }

    // ENCODE: RerollData if ItemType is Gear, Tome, Charm
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::Tome | ItemTypeDeser::Charm => {
            if let Some(rerollcount) = json_config.rerolls {
                // rerolldata
                encode_rerolldata(&mut fr_params, rerollcount)?
            }
        }
        _ => {}
    }

    // ENCODE: ShinyData if ItemType is Gear
    match json_config.item_type {
        ItemTypeDeser::Gear => {
            if let Some(shiny) = &json_config.shiny {
                encode_shinydata(&mut fr_params, shiny, &json_shiny)?
            }
        }
        _ => {}
    }

    // ENCODE: EndData, ALWAYS
    encode_enddata(&mut fr_params)?;
    
    let mut final_string: String = encode_string(out);
    
    // add NameAfter
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
