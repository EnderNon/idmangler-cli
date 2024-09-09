#[derive(Clone, Copy, Debug)]
pub enum TransformVersion {
    Version1 = 0,
}

impl TransformVersion {
    pub fn version(&self) -> u8 {
        *self as u8
    }

    pub fn from_u8(byte: u8) -> Result<Self, ()> {
        match byte {
            0 => Ok(Self::Version1),
            _ => Err(()),
        }
    }
}
