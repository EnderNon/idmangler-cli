use crate::errorfr::Errorfr;
use crate::jsonstruct::{
    CraftedTypesFr, Durability, FuncParams, Identificationer, ItemTypeDeser, PowderFr,
    RequirementsDeser, Shinyjson, Shinystruct,
};
use idmangler_lib::{
    types::{ClassType, Element, ItemType, RollType, SkillType, Stat, Powder},
    block::{
        CraftedConsumableTypeData, CraftedGearTypeData, DurabilityData, EndData,
        IdentificationData, NameData, PowderData, RequirementsData, RerollData, ShinyData, StartData,
        TypeData,
    }
};
use idmangler_lib::encoding::DataEncoder;
use std::collections::HashMap;

impl FuncParams<'_> {
    pub fn encode_startdata(&mut self) -> Result<(), Errorfr> {
        if *self.fr_debug_mode {
            println!("Encoding StartData")
        }
        // ENCODE: StartData
        StartData(self.fr_ver)
            .encode(self.fr_ver, self.fr_out)
            .unwrap();
        Ok(())
    }
    pub fn encode_typedata(&mut self, item_type_deser: ItemTypeDeser) -> Result<(), Errorfr> {
        if *self.fr_debug_mode {
            println!("Encoding TypeData: {:?}", item_type_deser);
        }
        // ENCODE: TypeData
        TypeData(ItemType::from(item_type_deser))
            .encode(self.fr_ver, self.fr_out)
            .unwrap();
        Ok(())
    }
    pub fn encode_typedata_custom(&mut self, crafted_type: &str) -> Result<(), Errorfr> {
        let frfr_type = CraftedTypesFr::try_from(crafted_type)?;
        match frfr_type {
            CraftedTypesFr::Gear(a) => {
                if *self.fr_debug_mode {
                    println!("Encoding CustomTypeData: Gear");
                }
                CraftedGearTypeData(a)
                    .encode(self.fr_ver, self.fr_out)
                    .unwrap()
            }
            CraftedTypesFr::Consu(a) => {
                if *self.fr_debug_mode {
                    println!("Encoding CustomTypeData: Consumable");
                }
                CraftedConsumableTypeData(a)
                    .encode(self.fr_ver, self.fr_out)
                    .unwrap()
            }
        }
        Ok(())
    }
    pub fn encode_duradata(&mut self, real_dura: &Durability) -> Result<(), Errorfr> {
        let effect_strength_fr: u8; // but actually it should be 0 to 100, not 0 to 255. But i dunno how to use u7 data type.
        if let Some(effstr) = real_dura.effect_strength {
            if *self.fr_debug_mode {
                println!("Encoding DurabilityData: Defined");
            }
            effect_strength_fr = effstr
        } else {
            let current_percentage = real_dura.dura_cur / real_dura.dura_max; // percentage of max durability
            if current_percentage > 100 {
                return Err(Errorfr::JsonDuraOutOfRange);
            }
            if current_percentage >= 50 {
                // for 100% dura to 50% dura, the effectiveness is 100%
                effect_strength_fr = 100
            } else if current_percentage >= 10 {
                // for 50% dura to 10% dura, the effectiveness is 100% to 50%
                // see this answer from Stackoverflow for transcribing range
                // https://stackoverflow.com/a/929107
    
                // old range is 50-10 = 40
                let old_range = 40;
                // new range is 100-50 = 50
                let new_range = 50;
                // NewValue = (((OldValue - OldMin) * NewRange) / OldRange) + NewMin
                effect_strength_fr = ((((current_percentage - 10) * new_range) / old_range) + 50) as u8
            } else if current_percentage >= 0 {
                // for 10% dura to 0% dura, the effectiveness is 50% to 10%
                // old range is 10-0 = 10
                let old_range = 10;
                // new range is 50-10 = 40
                let new_range = 40;
                // NewValue = (((OldValue - OldMin) * NewRange) / OldRange) + NewMin
                effect_strength_fr = ((((current_percentage) * new_range) / old_range) + 10) as u8
            } else {
                return Err(Errorfr::JsonDuraOutOfRange);
            }
        }
        if *self.fr_debug_mode {
            println!(
                "Encoding DurabilityData.effect_strenght: {}",
                effect_strength_fr
            );
            println!("Encoding DurabilityData.current: {}", real_dura.dura_cur);
            println!("Encoding DurabilityData.current: {}", real_dura.dura_max);
        }
        DurabilityData {
            effect_strenght: effect_strength_fr,
            current: real_dura.dura_cur,
            max: real_dura.dura_max,
        }
        .encode(self.fr_ver, self.fr_out)
        .unwrap();
        Ok(())
    }
    pub fn encode_reqdata(&mut self, real_reqdata: RequirementsDeser) -> Result<(), Errorfr> {
        if *self.fr_debug_mode {
            println!(
                "Encoding RequirementData.Level: {:?}",
                real_reqdata.level.clone()
            )
        }
        let mut fr_class: Option<ClassType> = None;
        if let Some(actualclass) = real_reqdata.class {
            fr_class = Some(ClassType::from(actualclass));
            if *self.fr_debug_mode {
                println!("Encoding RequirementData.Class: {:?}", actualclass.clone())
            }
        } else if *self.fr_debug_mode {
            println!("Encoding RequirementData.Class: Undefined");
        }
        
        let spvec: Vec<(SkillType, i32)> = match real_reqdata.sp {
            Some(real_sp) => {Vec::<(SkillType, i32)>::from(real_sp)},
            None => {Vec::new()}
        };
        if *self.fr_debug_mode {
            println!("Encoding RequirementData.Skills: {:?}", spvec.clone())
        }
        RequirementsData {
            level: real_reqdata.level,
            class: fr_class,
            skills: spvec,
        }
        .encode(self.fr_ver, self.fr_out)
        .unwrap();
        Ok(())
    }
    pub fn encode_namedata(&mut self, real_name: &str) -> Result<(), Errorfr> {
        // ENCODE: NameData
        if *self.fr_debug_mode {
            println!("Encoding NameData: {:?}", &real_name)
        }
        NameData(real_name.trim().to_string())
            .encode(self.fr_ver, self.fr_out)
            .unwrap();
        Ok(())
    }
    pub fn encode_iddata(&mut self, real_ids: &Vec<Identificationer>, idsmap: HashMap<String, u8>) -> Result<(), Errorfr> {
        let mut idvec = Vec::new();
        for eachid in real_ids {
            let id_id = idsmap.get(eachid.id.trim());
            let id_base = eachid.base;
            let id_roll = eachid.roll;
    
            idvec.push(
                Stat {
                    kind: match id_id {
                        Some(ide) => *ide,
                        None => std::panic!("There is a mismatched ID, and this message has replaced where the line is meant to be")
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
        if *self.fr_debug_mode {
            println!("Encoding IdentificationData: {:?}", &idvec)
        }
        // ENCODE: IdentificationsData
        IdentificationData {
            identifications: idvec,
            extended_encoding: true,
        }
        .encode(self.fr_ver, self.fr_out)
        .unwrap();
        Ok(())
    }
    pub fn encode_powderdata(&mut self, real_powders: &Vec<PowderFr>) -> Result<(), Errorfr> {
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
                    _ => {return Err(Errorfr::JsonUnknownPowderElement)},
                };
                if *self.fr_debug_mode {
                    dbg!(eletype);
                }
                powdervec.push(
                    Powder::new(eletype, 6).map_err(|_| Errorfr::JsonUnknownPowderTier)?
                ); // 6 is the tier. Wynntils ONLY really uses tier 6 so theres no point keeping others.
            }
        }
        if *self.fr_debug_mode {
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
        .encode(self.fr_ver, self.fr_out)
        .unwrap();
        Ok(())
    }
    pub fn encode_rerolldata(&mut self, rerollcount: u8) -> Result<(), Errorfr> {
        if rerollcount != 0 {
            // ENCODE: RerollData if applicable
            RerollData(rerollcount)
                .encode(self.fr_ver, self.fr_out)
                .unwrap();
            if *self.fr_debug_mode {
                dbg!(rerollcount);
            }
        }
        Ok(())
    }
    pub fn encode_shinydata(&mut self, shiny: &Shinyjson, json_shiny: &Vec<Shinystruct>) -> Result<(), Errorfr> {
        let mut realshinykey: u8;
        let _shinykey = &shiny.key;
        let shinyvalue = shiny.value;
        realshinykey = 1;
        for i in json_shiny {
            if i.key == shiny.key {
                realshinykey = i.id;
                if *self.fr_debug_mode {
                    dbg!(&shiny.key);
                }
            }
        }
        if *self.fr_debug_mode {
            dbg!(&realshinykey);
            dbg!(&shinyvalue);
        }
        // ENCODE: ShinyData (if applicable)
        ShinyData {
            id: realshinykey,
            val: shinyvalue,
        }
        .encode(self.fr_ver, self.fr_out)
        .unwrap();
        Ok(())
    }
    pub fn encode_enddata(&mut self) -> Result<(), Errorfr> {
        if *self.fr_debug_mode {
            println!("Encoding EndData")
        }
        // ENCODE: EndData
        EndData
            .encode(self.fr_ver, self.fr_out)
            .unwrap();
        Ok(())
    }
}