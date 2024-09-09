#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ItemType {
    Gear = 0,
    Tome = 1,
    Charm = 2,
    CraftedGear = 3,
    CraftedConsu = 4,
}

impl Into<u8> for ItemType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for ItemType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Gear),
            1 => Ok(Self::Tome),
            2 => Ok(Self::Charm),
            3 => Ok(Self::CraftedGear),
            4 => Ok(Self::CraftedConsu),

            _ => Err(()),
        }
    }
}
