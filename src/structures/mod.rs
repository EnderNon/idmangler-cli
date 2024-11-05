use serde::Deserialize;
// structs
#[derive(Deserialize)]
pub struct Powder {
    pub r#type: char,
    pub tier: u8,
    pub amount: Option<u8>,
}
#[derive(Deserialize)]
pub struct Identificationer {
    pub id: String,
    pub base: i32,
    pub roll: Option<u8>,
}
#[derive(Deserialize)]
pub struct jsonconfig {
    pub debug: Option<bool>,
    pub name: String,
    pub shiny: Option<shinyjson>,
    pub ids: Vec<Identificationer>,
    pub powder_limit: u8,
    pub powders: Vec<Powder>,
    pub rerolls: Option<u8>,
}

#[derive(Deserialize)]
pub struct shinystruct {
    pub id: u8,
    pub key: String,
}
#[derive(Deserialize)]
pub struct shinyjson {
    pub key: String,
    pub value: i64,
}
