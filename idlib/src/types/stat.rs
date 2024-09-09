#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct StatId(pub u8);

#[derive(Debug, Clone)]
pub struct Stat {
    pub kind: StatId,
    pub base: Option<i32>,
    pub roll: RollType,
}

#[derive(Debug, Clone)]
pub enum RollType {
    Value(u8),
    PreIdentified,
}

impl Stat {
    pub fn pre_identified(&self) -> bool {
        match self.roll {
            RollType::Value(_) => false,
            RollType::PreIdentified => true,
        }
    }
}
