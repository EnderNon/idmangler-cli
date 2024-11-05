use idmangler_lib::{
    encoding::encode_string,
    types::{
        ItemType, Powders, TransformVersion, {RollType, Stat},
    },
    DataEncoder, EndData, IdentificationData, NameData, PowderData, RerollData, ShinyData,
    StartData, TypeData,
};

use std::collections::HashMap;
use std::fs;
use std::panic;
mod structures;
use crate::structures::*;
mod errorfr;
use crate::errorfr::Errorfr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path for config path
    #[arg(short, long)]
    configpath: Option<String>,

    /// Enable debug mode
    #[arg(short, long, default_value_t = false)]
    debugmode: bool,
}

// const fallbackconfigpath: String = "config.json".to_owned();

fn main() {
    if let Err(e) = cook() {
        println!("{}",e);
    }
}
fn cook() -> Result<(), Errorfr> {
    // enable fancypanic when building for release
    // fancypanic();
    let args = Args::parse();

    let mut debug_mode = false;
    if args.debugmode == true {
        debug_mode = true;
        println!("Debug mode enabled");
    };
    let mut configpath: String = String::from("config.json");
    if let Some(configpathargs) = args.configpath {
        configpath = configpathargs;
    }

    // newest json reading code
    let json_config: Jsonconfig = serde_json::from_reader(fs::File::open(configpath)
        .map_err(|_| Errorfr::ItemJsonMissing)?)
        .map_err(|_| Errorfr::ItemJsonCorrupt)?;
    let idsmap: HashMap<String, u8> = serde_json::from_reader(fs::File::open("id_keys.json")
        .map_err(|_| Errorfr::IDMapJsonMissing)?)
        .map_err(|_| Errorfr::IDMapJsonCorrupt)?;
    let json_shiny: Vec<Shinystruct> = serde_json::from_reader(fs::File::open("shiny_stats.json")
        .map_err(|_| Errorfr::ShinyJsonMissing)?)
        .map_err(|_| Errorfr::ShinyJsonCorrupt)?;
    // println!("{:?}",idsmap.get("airDamage"));

    let mut out = Vec::new();
    let ver = TransformVersion::Version1;

    StartData(ver).encode(ver, &mut out).unwrap();

    TypeData(ItemType::Gear).encode(ver, &mut out).unwrap();

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
        // no need to return to variable or i'll need to rematch AGAIN
        match eachpowder.r#type.to_ascii_lowercase() {
            'e' => {
                for _i in 0..powderamount {
                    powdervec.push((Powders::EARTH, powdertier))
                }
                if debug_mode {
                    println!("Powder type: Earth");
                }
            }
            't' => {
                for _i in 0..powderamount {
                    powdervec.push((Powders::THUNDER, powdertier))
                }
                if debug_mode {
                    println!("Powder type: Thunder");
                }
            }
            'w' => {
                for _i in 0..powderamount {
                    powdervec.push((Powders::WATER, powdertier))
                }
                if debug_mode {
                    println!("Powder type: Water");
                }
            }
            'f' => {
                for _i in 0..powderamount {
                    powdervec.push((Powders::FIRE, powdertier))
                }
                if debug_mode {
                    println!("Powder type: Fire");
                }
            }
            'a' => {
                for _i in 0..powderamount {
                    powdervec.push((Powders::AIR, powdertier))
                }
                if debug_mode {
                    println!("Powder type: Air");
                }
            }
            _ => {
                for _i in 0..powderamount {
                    powdervec.push((Powders::THUNDER, powdertier))
                }
                if debug_mode {
                    println!("Powder type: Broken, fallback Thunder");
                }
            }
        };

        if debug_mode {
            println!("Powder tier: {}", powdertier);
            println!("Powder amount: {}", powderamount);
        }
    }
    if debug_mode {
        println!("Powders Vec: {:?}", powdervec);
    }

    // old powder data encode kinda, takes data from new encode
    PowderData {
        powder_slots: json_config.powder_limit,
        powders: powdervec,
    }
    .encode(ver, &mut out)
    .unwrap();

    match json_config.rerolls {
        Some(i) => {
            if i != 0 {
                RerollData(i).encode(ver, &mut out).unwrap();
                if debug_mode {
                    println!("Rerolls: {}", i)
                }
            }
        }
        None => pass(),
    }

    let mut realshinykey: u8;
    if let Some(shiny) = json_config.shiny {
        if let ref _shinykey = shiny.key {
            if let shinyvalue = shiny.value {

                realshinykey = 1;
                for i in json_shiny {
                    if i.key == shiny.key {
                        realshinykey = i.id;
                        if debug_mode {
                            println!("shiny key {}", shiny.key);
                        }
                    }
                }
                if debug_mode {
                    dbg!(&realshinykey);
                    dbg!(&shinyvalue);
                }

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
