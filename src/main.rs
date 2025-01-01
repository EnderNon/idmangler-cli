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
use crate::errorfr::Errorfr::PowderLimitNotPresent;

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
                "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json".parse().unwrap(),
                format!("{}{}", executable_path, "/id_keys.json"),
            ) { // error handling below
                println!("{} Filename: {}",e,dlvalue)
            }
        }
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

                                // StartData and TypeData are always present

                                // ENCODE: StartData
                                StartData(ver).encode(ver, &mut out).unwrap();

                                // ENCODE: TypeData
                                TypeData(ItemType::from(loaded_config.item_type))
                                    .encode(ver, &mut out)
                                    .unwrap();

                                // ENCODE: ALotOfStuff
                                // Also print any mapped errors
                                if let Err(e) = cook(
                                    &debug_mode,
                                    loaded_config,
                                    loaded_idkeys,
                                    loaded_shinystats,
                                    &mut out,
                                    ver,
                                ) {
                                    println!("{}", e);
                                };

                                // ENCODE: EndData
                                EndData.encode(ver, &mut out).unwrap();

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
    debug_mode: &bool,
    json_config: Jsonconfig,
    idsmap: HashMap<String, u8>,
    json_shiny: Vec<Shinystruct>,
    out: &mut Vec<u8>,
    ver: TransformVersion,
) -> Result<(), Errorfr> {
    // ENCODE: NameData
    if let Some(real_name) = json_config.name {
        NameData(real_name.trim().to_string())
            .encode(ver, out)
            .unwrap();
    }


    // json identification data handling for type GEAR (0)
    // only occurs if identifications block is present
    if let Some(real_ids) = json_config.ids {
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
            .encode(ver, out)
            .unwrap();
    }



    // json powder data handling
    if let Some(real_powders) = json_config.powders {
        encode_powder(out, debug_mode, real_powders, ver)
    }


    if let Some(rerollcount) = json_config.rerolls {
        if rerollcount != 0 {
            // ENCODE: RerollData if applicable
            RerollData(rerollcount).encode(ver, out).unwrap();
            if *debug_mode {
                dbg!(rerollcount);
            };
        };
    };

    let mut realshinykey: u8;
    if let Some(shiny) = json_config.shiny {
        let _shinykey = &shiny.key;
        let shinyvalue = shiny.value;
        realshinykey = 1;
        for i in json_shiny {
            if i.key == shiny.key {
                realshinykey = i.id;
                if *debug_mode {
                    dbg!(&shiny.key);
                }
            }
        }
        if *debug_mode {
            dbg!(&realshinykey);
            dbg!(&shinyvalue);
        }
        // ENCODE: ShinyData (if applicable)
        ShinyData {
            id: realshinykey,
            val: shinyvalue,
        }
        .encode(ver, out)
        .unwrap();
    }

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

fn encode_powder(out: &mut Vec<u8>, debug_mode: &bool, real_powders: Vec<Powder>, ver: TransformVersion) {
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
            if *debug_mode {
                dbg!(eletype);
            }
            powdervec.push(Some((eletype, 6))); // 6 is the tier. Wynntils ONLY really uses tier 6 so theres no point keeping others.
        }
    }
    if *debug_mode {
        dbg!(&powdervec);
    }

    let powderlimitfr: u8 = (powdervec.len() as u8)
        .min(255); // min of the current number of powders and 255 (if you have over 255 powders stuff breaks)

    // ENCODE: PowderData
    // only occurs if the powders array is present and the powder limit is also present
    //
    PowderData {
        powder_slots: powderlimitfr,
        powders: powdervec,
    }
        .encode(ver, out)
        .unwrap();
}