#![allow(clippy::single_match)]
#![allow(non_camel_case_types, non_snake_case)]

mod cookers;
mod encode;
mod errorfr;
mod gearjson;
mod jsondl;
mod jsonstruct;
use crate::encode::FuncParams;
use crate::errorfr::Errorfr;
use crate::jsondl::*;
use crate::jsonstruct::*;
use clap::Parser;
use idmangler_lib::encoding::string::encode_string;
use idmangler_lib::types::EncodingVersion;
use reqwest::Url;
use std::collections::HashMap;
use std::{env, fs, io::Write, path::PathBuf};

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
    let args: &Args = &Args::parse();
    let do_not_cook = do_not_cook(args);
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
                Ok(fr) => Ok(fr),
                Err(e) => Err(e),
            }
        } else {
            if !do_not_cook {
                println!("no config path provided, fallback to empty.");
            }
            Ok(Jsonconfig {
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
                crafted_defence: None,
            })
        }
    }?;
    let loaded_idkeys = load_idkeys(executable_path)?;
    let loaded_shinystats = load_shinystats(executable_path)?;

    // check if perfect status and change name if so. otherwise blank yep
    let mut namefr: &str = "";
    if let Some(t1) = &args.perfect {
        namefr = t1
    }

    // debug mode on if in the loaded config
    if let Some(debugconfig) = loaded_config.debug
        && debugconfig {
            debug_mode = true
        }

    if !do_not_cook {
        // main program everything starts here fr
        let mut out: Vec<u8> = Vec::new();

        // create necessary variables
        let ver = EncodingVersion::V2;

        let mut loaded_config_clone = loaded_config.clone();

        let mut funcparamsfr: FuncParams = FuncParams {
            fr_out: &mut out,
            fr_debug_mode: &debug_mode,
            fr_ver: ver,
        };

        // ENCODE: A Lot Of Stuff
        // Also print any mapped errors
        let cooking = cook(&mut funcparamsfr, &mut loaded_config_clone, loaded_idkeys, loaded_shinystats, namefr, executable_path);
        if let Err(e) = cooking {
            println!("{}", e); // print error if there is an error
        } else {
            // final string print if there is no error
            println!("{}", cooking?)
        }
    }
    Ok(())
}
// Checks for if you should actually do any encoding with the function.
// There's a few cases to account for:
// - If config flag is passed, then do it
// - If perfect flag is passed, then do it
// - If only download flag is passed, then don't do it
// - If none of these flags are passed then don't do it
// - Fallback: try and do it
fn do_not_cook(args: &Args) -> bool {
    if args.config.is_some() {
        false
    } else {
        args.perfect.is_none()
    }
}


pub(crate) fn cook(fr_params: &mut FuncParams, json_config: &mut Jsonconfig, idsmap: HashMap<String, u8>, json_shiny: Vec<Shinystruct>, namefr: &str, executable_path: &str) -> Result<String, Errorfr> {
    // ENCODE: StartData and TypeData, ALWAYS

    
    let temp_jsonconfig = json_config.clone();
    let ops = temp_jsonconfig.item_type.get_ops();

    fr_params.encode_from_arr(ops, json_config.clone(), idsmap, json_shiny, namefr, executable_path)?;

    let mut final_string: String = encode_string(fr_params.fr_out);

    // to fix an extremely funny newline exploit and prevent future trolling,
    // wynntils made it so crafteds have their name block seperately.
    // as: ` [][][][] "Crafted Item Name" `
    // this is not really an encode and it comes AFTER its all been encoded already.
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