use idmangler_lib::{
    encoding::encode_string,
    types::{
        Element, ItemType, TransformVersion, {RollType, Stat},
    },
    DataEncoder, EndData, IdentificationData, NameData, PowderData, RerollData, ShinyData,
    StartData, TypeData,
};

use std::ops::Deref;
use std::{collections::HashMap, env, fs, io, panic, path::PathBuf};

mod structures;
use crate::structures::*;
mod errorfr;
use crate::errorfr::Errorfr;

use clap::Parser;
use reqwest::Url;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path for config path
    #[arg(short, long)]
    config: Option<String>,

    /// Enable debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// Download jsons (for ease of use)
    #[arg(long)]
    download: Option<String>,
}

fn dl_json(
    url: Url,
    savename: String,
    args: Args,
    executablePath: &str,
    debug_mode: bool,
) -> Result<(), Errorfr> {
    let resp = reqwest::blocking::get(url).map_err(|_| Errorfr::JsonDlReqFail)?;
    let body = resp.text().map_err(|_| Errorfr::JsonDlReqBodyInvalid)?;
    let savepath = format!("{}",savename);
    println!("Downloading to {savepath}");
    let mut out = fs::File::create(savepath)
        .map_err(|_| Errorfr::JsonDlReqFileCreateFail)?;
    io::copy(&mut body.as_bytes(), &mut out).map_err(|_| Errorfr::JsonDlReqFileWriteFail)?;
    Ok(())
}

fn main() {
    // enable fancypanic when building for release
    // fancypanic();
    let args = Args::parse();
    let mut executablePath = env::current_exe().unwrap();
    PathBuf::pop(&mut executablePath);
    let executable_path = executablePath.to_str().unwrap();

    let mut debug_mode = false;
    if args.debug == true {
        debug_mode = true;
        println!("Debug mode enabled");
    };

    // download jsons if necessary
    if let Some(dlvalue) = &args.download {
        let jsons = DownloadJsons::from(dlvalue.clone());
        if jsons == DownloadJsons::all || jsons == DownloadJsons::shiny_stats {
            if let Err(e) = dl_json(
                "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json".parse().unwrap(),
                format!("{}{}", executable_path, "shiny_stats.json"),
                args.clone(),
                executable_path,
                debug_mode
            ) { // error handling below
                println!("{} Filename: {}",e,dlvalue)
            }
        }
        if jsons == DownloadJsons::all || jsons == DownloadJsons::id_keys {
            if let Err(e) = dl_json(
                "https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json".parse().unwrap(),
                format!("{}{}", executable_path, "id_keys.json"),
                args.clone(),
                executable_path,
                debug_mode
            ) { // error handling below
                println!("{} Filename: {}",e,dlvalue)
            }
        }
    };

    if let Err(e) = cook(args, executable_path, debug_mode) {
        println!("{}", e);
    }
}

fn cook(args: Args, executable_path: &str, mut debug_mode: bool) -> Result<(), Errorfr> {
    let mut config: String = String::from(executable_path.to_owned() + "config.json");
    if let Some(configpathargs) = args.config {
        config = configpathargs;
    }

    // load configs
    let json_config: Jsonconfig =
        serde_json::from_reader(fs::File::open(config).map_err(|_| Errorfr::ItemJsonMissing)?)
            .map_err(|e| Errorfr::ItemJsonCorrupt(e))?;
    let idsmap: HashMap<String, u8> = serde_json::from_reader(
        fs::File::open(executable_path.to_owned() + "id_keys.json")
            .map_err(|_| Errorfr::IDMapJsonMissing)?,
    )
    .map_err(|_| Errorfr::IDMapJsonCorrupt)?;
    let json_shiny: Vec<Shinystruct> = serde_json::from_reader(
        fs::File::open(executable_path.to_owned() + "shiny_stats.json")
            .map_err(|_| Errorfr::ShinyJsonMissing)?,
    )
    .map_err(|_| Errorfr::ShinyJsonCorrupt)?;
    // println!("{:?}",idsmap.get("airDamage"));

    if let Some(debugconfig) = json_config.debug {
        if debugconfig {
            debug_mode = true
        }
    }

    // create necessary variables
    let mut out = Vec::new();
    let ver = TransformVersion::Version1;

    // ENCODE: StartData
    StartData(ver).encode(ver, &mut out).unwrap();

    // ENCODE: TypeData
    TypeData(ItemType::from(json_config.item_type))
        .encode(ver, &mut out)
        .unwrap();

    // ENCODE: NameData
    NameData(String::from(format!("{}", json_config.name.trim())))
        .encode(ver, &mut out)
        .unwrap();

    // json identification data handling
    let mut idvec = Vec::new();
    for eachid in json_config.ids {
        let id_id = idsmap.get(eachid.id.trim());
        let id_base = eachid.base as i32;
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
    .encode(ver, &mut out)
    .unwrap();

    // json powder data handling
    let mut powdervec = Vec::new();
    for eachpowder in json_config.powders {
        let powdertier = eachpowder.tier; // get the powder tier
        let powderamount: u8 = eachpowder.amount.unwrap_or_else(|| 1);
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
            if debug_mode {
                dbg!(powdertier);
                dbg!(eletype);
            }
            powdervec.push(Some((eletype, powdertier)));
        }
    }
    if debug_mode {
        dbg!(&powdervec);
    }

    // ENCODE: PowderData
    PowderData {
        powder_slots: json_config.powder_limit,
        powders: powdervec,
    }
    .encode(ver, &mut out)
    .unwrap();

    match json_config.rerolls {
        Some(rerollcount) => {
            if rerollcount != 0 {
                // ENCODE: RerollData if applicable
                RerollData(rerollcount).encode(ver, &mut out).unwrap();
                if debug_mode {
                    dbg!(rerollcount);
                };
            };
        }
        None => pass(),
    };

    let mut realshinykey: u8;
    if let Some(shiny) = json_config.shiny {
        if let ref _shinykey = shiny.key {
            if let shinyvalue = shiny.value {
                realshinykey = 1;
                for i in json_shiny {
                    if i.key == shiny.key {
                        realshinykey = i.id;
                        if debug_mode {
                            dbg!(&shiny.key);
                        }
                    }
                }
                if debug_mode {
                    dbg!(&realshinykey);
                    dbg!(&shinyvalue);
                }
                // ENCODE: ShinyData (if applicable)
                ShinyData {
                    id: realshinykey,
                    val: shinyvalue as i64, //- 0b0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
                                            // u16::MAX is the max value of unsigned 16bit value
                }
                .encode(ver, &mut out)
                .unwrap();
            }
        }
    }

    // prints (Water,6) 255 times
    // println!("{:?}",vec![(Powders::WATER, 6); 255]);

    // ENCODE: EndData
    EndData.encode(ver, &mut out).unwrap();

    // final string print
    println!("{}", encode_string(&out));

    // I don't even know what the fuck this does
    //for b in out {
    //    print!("{:02X}", b);
    //}

    // println!();

    // decode test
    //let input = "󰀀󰄀󰉁󶹴󶡲󶅣󶥴󶔠󴉡󶱬󶥳󷑡󰀃󰠁󰀞󾠇󵠑󳱩󳢠󱽴󴠧󷄡󱹵󳫠󰢂󱌨󵴅󲠞􏿮";
    //let bytes = decode_string(&input);
    //let mut bytes_iter = bytes.into_iter();
    //let out = decode(&mut bytes_iter).unwrap();

    // println!("{:#?}", out);
    Ok(())
}

fn fancypanic() {
    panic::set_hook(Box::new(|panic_info| {
        let panic_msg = format!("{panic_info}");
        println!(
            "{}",
            panic_msg
                .lines()
                .nth(1)
                .unwrap_or("HOW DID YOU BREAK THE PANIC HANDLER???")
        );
    }));
}

fn pass() {}

const ERROR: [&'static str; 7] = [
    "Error 0: what did you even do to get this? ",
    "Error 1: json config json is missing, obtain it from https://git.frfrnocap.men/endernon/idmangler-cli/raw/branch/main/config.json and move it to this directory. ",
    "Error 2: json config json is corrupt. Reread config.md or reobtain it from https://git.frfrnocap.men/endernon/idmangler-cli/raw/branch/main/config.json and move it to this diirectory. ",
    "Error 3: Identifications hashmap is missing. Get it from https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json and move it to this directory. ",
    "Error 4: Identifications hashmap is corrupt. Reobtain it from https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json and move it to this directory. ",
    "Error 5: Shiny data json is missing. Get it from https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json and move it to this directory. ",
    "Error 6: Shiny data json is corrupt. Get it from https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json and move it to this directory. "
];
