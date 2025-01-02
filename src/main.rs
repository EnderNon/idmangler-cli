#![allow(
    clippy::single_match
)]

use idmangler_lib::{
    encoding::encode_string,
    types::{
        Element, ItemType, TransformVersion, {RollType, Stat},
    },
    DataEncoder, EndData, IdentificationData, NameData, PowderData, RerollData, ShinyData,
    StartData, TypeData,
};

use std::{collections::HashMap, env, fs, io, panic, path::PathBuf};

mod structures;
use crate::structures::*;
mod errorfr;
use crate::errorfr::Errorfr;

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
                                let ver = TransformVersion::Version1;

                                // ENCODE: ALotOfStuff
                                // Also print any mapped errors
                                if let Err(e) = cook(
                                    &mut out,
                                    &debug_mode,
                                    ver,
                                    loaded_config,
                                    loaded_idkeys,
                                    loaded_shinystats,
                                ) {
                                    println!("{}", e);
                                };

                                // final string print
                                println!("{}", encode_string(&out));
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
    ver: TransformVersion,
    json_config: Jsonconfig,
    idsmap: HashMap<String, u8>,
    json_shiny: Vec<Shinystruct>,
) -> Result<(), Errorfr> {
    let mut fr_params = FuncParams {
        fr_out: out,
        fr_debug_mode: debug_mode,
        fr_ver: ver,
    };

    // ENCODE: StartData and TypeData, ALWAYS
    encode_startdata(&mut fr_params);
    encode_typedata(&mut fr_params, json_config.item_type);

    // ENCODE: NameData, if ItemType is Gear, Tome, Charm
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::Tome | ItemTypeDeser::Charm => {
            if let Some(real_name) = json_config.name {
                encode_namedata(&mut fr_params, &real_name)
            }
            else {
                return Err(Errorfr::JsonNotFoundName)
            }
        }
        _ => {}
    }

    // ENCODE: IdentificationData
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::Tome | ItemTypeDeser::Charm => {
            if let Some(real_ids) = json_config.ids {
                encode_ids(&mut fr_params, real_ids, idsmap)
            }
        }
        _ => {}
    }

    // ENCODE: PowderData if ItemType is Gear, CraftedGear
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::CraftedGear => {
            if let Some(real_powders) = json_config.powders {
                encode_powder(&mut fr_params, real_powders)
            }
        }
        _ => {}
    }

    // ENCODE: RerollData if ItemType is Gear, Tome, Charm
    match json_config.item_type {
        ItemTypeDeser::Gear | ItemTypeDeser::Tome | ItemTypeDeser::Charm => {
            if let Some(rerollcount) = json_config.rerolls {
                // rerolldata
                encode_reroll(&mut fr_params, rerollcount)
            }
        }
        _ => {}
    }

    // ENCODE: ShinyData if ItemType is Gear
    match json_config.item_type {
        ItemTypeDeser::Gear => {
            if let Some(shiny) = json_config.shiny {
                encode_shiny(&mut fr_params, shiny, json_shiny)
            }
        }
        _ => {}
    }

    // ENCODE: EndData, ALWAYS
    encode_enddata(&mut fr_params);

    Ok(())
}

fn load_jsonconfig(path: &String) -> Result<Jsonconfig, Errorfr> {
    serde_json5::from_reader(&mut fs::File::open(path).map_err(|_| Errorfr::ItemJsonMissing)?)
        .map_err(Errorfr::ItemJsonCorrupt)
}
fn load_idkeys(executable_path: &str) -> Result<HashMap<String, u8>, Errorfr> {
    // id_keys.json
    serde_json5::from_reader(
        &mut fs::File::open(executable_path.to_owned() + "/id_keys.json")
            .map_err(|_| Errorfr::IDMapJsonMissing)?,
    )
    .map_err(|_| Errorfr::IDMapJsonCorrupt)
}
fn load_shinystats(executable_path: &str) -> Result<Vec<Shinystruct>, Errorfr> {
    // shiny_stats.json
    serde_json5::from_reader(
        &mut fs::File::open(executable_path.to_owned() + "/shiny_stats.json")
            .map_err(|_| Errorfr::ShinyJsonMissing)?,
    )
    .map_err(|_| Errorfr::ShinyJsonCorrupt)
}
fn dl_json_fr(dlvalue: &String, executable_path: &str) {
    let jsons = DownloadJsons::from(dlvalue.clone());
    if jsons == DownloadJsons::All || jsons == DownloadJsons::ShinyStats {
        if let Err(e) = dl_json(
            "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json".parse().unwrap(),
            format!("{}{}", executable_path, "/shiny_stats.json"),
        ) { // error handling below
            println!("{} Filename: {}",e,dlvalue)
        }
    }
    if jsons == DownloadJsons::All || jsons == DownloadJsons::IdKeys {
        if let Err(e) = dl_json(
            "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json"
                .parse()
                .unwrap(),
            format!("{}{}", executable_path, "/id_keys.json"),
        ) {
            // error handling below
            println!("{} Filename: {}", e, dlvalue)
        }
    }
}
fn encode_startdata(general_params: &mut FuncParams) {
    // ENCODE: StartData
    StartData(general_params.fr_ver)
        .encode(general_params.fr_ver, general_params.fr_out)
        .unwrap();
}
fn encode_typedata(general_params: &mut FuncParams, item_type_deser: ItemTypeDeser) {
    // ENCODE: TypeData
    TypeData(ItemType::from(item_type_deser))
        .encode(general_params.fr_ver, general_params.fr_out)
        .unwrap();
}
fn encode_namedata(general_params: &mut FuncParams, real_name: &str) {
    // ENCODE: NameData
    NameData(real_name.trim().to_string())
        .encode(general_params.fr_ver, general_params.fr_out)
        .unwrap();
}
fn encode_ids(
    general_params: &mut FuncParams,
    real_ids: Vec<Identificationer>,
    idsmap: HashMap<String, u8>,
) {
    let mut idvec = Vec::new();
    for eachid in real_ids {
        let id_id = idsmap.get(eachid.id.trim());
        let id_base = eachid.base;
        let id_roll = eachid.roll;

        idvec.push(
            Stat {
                kind: match id_id {
                    Some(ide) => *ide,
                    None => panic!("There is a mismatched ID, and this message has replaced where the line is meant to be")
                },
                base: Some(id_base),
                roll: match id_roll{
                    Some(rolle) => RollType::Value(rolle),
                    None => RollType::PreIdentified
                }
            }
        );

        // println!("{:?} {:?} {:?}",id_id,id_base,id_roll)
    }
    // ENCODE: IdentificationsData
    IdentificationData {
        identifications: idvec,
        extended_encoding: true,
    }
    .encode(general_params.fr_ver, general_params.fr_out)
    .unwrap();
}
fn encode_powder(general_params: &mut FuncParams, real_powders: Vec<Powder>) {
    let mut powdervec = Vec::new();
    for eachpowder in real_powders {
        let powderamount: u8 = eachpowder.amount.unwrap_or(1);
        // match for the powder type
        for _ in 0..powderamount {
            let eletype = match eachpowder.r#type.to_ascii_lowercase() {
                'e' => Element::Earth,
                't' => Element::Thunder,
                'w' => Element::Water,
                'f' => Element::Fire,
                'a' => Element::Air,
                _ => Element::Thunder,
            };
            if *general_params.fr_debug_mode {
                dbg!(eletype);
            }
            powdervec.push(Some((eletype, 6))); // 6 is the tier. Wynntils ONLY really uses tier 6 so theres no point keeping others.
        }
    }
    if *general_params.fr_debug_mode {
        dbg!(&powdervec);
    }

    let powderlimitfr: u8 = powdervec.len() as u8; // min of the current number of powders and 255 (if you have over 255 powders stuff breaks)

    // ENCODE: PowderData
    // only occurs if the powders array is present and the powder limit is also present
    //
    PowderData {
        powder_slots: powderlimitfr,
        powders: powdervec,
    }
    .encode(general_params.fr_ver, general_params.fr_out)
    .unwrap();
}
fn encode_reroll(general_params: &mut FuncParams, rerollcount: u8) {
    if rerollcount != 0 {
        // ENCODE: RerollData if applicable
        RerollData(rerollcount)
            .encode(general_params.fr_ver, general_params.fr_out)
            .unwrap();
        if *general_params.fr_debug_mode {
            dbg!(rerollcount);
        }
    }
}
fn encode_shiny(general_params: &mut FuncParams, shiny: Shinyjson, json_shiny: Vec<Shinystruct>) {
    let mut realshinykey: u8;
    let _shinykey = &shiny.key;
    let shinyvalue = shiny.value;
    realshinykey = 1;
    for i in json_shiny {
        if i.key == shiny.key {
            realshinykey = i.id;
            if *general_params.fr_debug_mode {
                dbg!(&shiny.key);
            }
        }
    }
    if *general_params.fr_debug_mode {
        dbg!(&realshinykey);
        dbg!(&shinyvalue);
    }
    // ENCODE: ShinyData (if applicable)
    ShinyData {
        id: realshinykey,
        val: shinyvalue,
    }
    .encode(general_params.fr_ver, general_params.fr_out)
    .unwrap();
}
fn encode_enddata(general_params: &mut FuncParams) {
    // ENCODE: EndData
    EndData
        .encode(general_params.fr_ver, general_params.fr_out)
        .unwrap();
}
