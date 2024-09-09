use enddata::EndData;
use identdata::IdentificationData;
use namedata::NameData;
use powderdata::PowderData;
use rerolldata::RerollData;
use startdata::StartData;
use typedata::TypeData;

use crate::types::transform::TransformVersion;

pub mod enddata;
pub mod identdata;
pub mod namedata;
pub mod powderdata;
pub mod rerolldata;
pub mod shinydata;
pub mod startdata;
pub mod typedata;

pub trait TransformId {
    fn get_id() -> u8;
}

pub trait DataEncoder: TransformId {
    fn encode(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), DataTransformError> {
        // skip encoding data which should not be encoded
        if !self.should_encode_data(ver) {
            return Ok(());
        }

        // encode the id
        out.push(Self::get_id());

        // encode the data
        self.encode_data(ver, out)?;

        Ok(())
    }

    fn encode_data(
        &self,
        ver: TransformVersion,
        out: &mut Vec<u8>,
    ) -> Result<(), DataTransformError>;

    fn should_encode_data(&self, _ver: TransformVersion) -> bool {
        true
    }
}

pub trait DataDecoder<B: Iterator<Item = u8>>: TransformId {
    fn decode_data(bytes: &mut B, ver: TransformVersion) -> Result<Self, DataTransformError>
    where
        Self: Sized;
}

pub fn decode<B: Iterator<Item = u8>>(bytes: &mut B) -> Result<Vec<AnyData>, DataTransformError> {
    let mut out = Vec::new();

    // decode the start byte and version
    let ver = StartData::decode_start_bytes(bytes)?;

    while let Some(id) = bytes.next() {
        match id {
            0 => return Err(DataTransformError::StartReparse),
            1 => out.push(AnyData::TypeData(TypeData::decode_data(bytes, ver)?)),
            2 => out.push(AnyData::NameData(NameData::decode_data(bytes, ver)?)),
            3 => out.push(AnyData::IdentificationData(
                IdentificationData::decode_data(bytes, ver)?,
            )),
            // TODO
            255 => out.push(AnyData::EndData(EndData::decode_data(bytes, ver)?)),
            _ => return Err(DataTransformError::UnknownTransformer(id)),
        }
    }

    Ok(out)
}

#[derive(Debug)]
pub enum DataTransformError {
    NoStartBlock,
    UnknownVersion(u8),
    /// Attempt to parse start data. Start data is specially handled.
    StartReparse,

    InvalidTypeError,

    BadString,

    TooManyIdentifications,
    NoBasevalueForIdent,
    NoPotentialValuesForIdent,
    InvalidIntRoll,

    UnexpectedEndOfBytes,
    UnknownTransformer(u8),
}

pub enum DataTransformerTypes {
    StartDataTransformer = 0,
    TypeDataTransformer = 1,
    NameDataTransformer = 2,
    IdentificationDataTransformer = 3,
    PowderDataTransformer = 4,
    RerollDataTransformer = 5,
    ShinyDataTransformer = 6,
    CustomGearTypeTransformer = 7,
    DurabilityDataTransformer = 8,
    RequirementsDataTransformer = 9,
    DamageDataTransformer = 10,
    DefenseDataTransformer = 11,
    CustomIdentificationDataTransformer = 12,
    CustomConsumableTypeDataTransformer = 13,
    UsesDataTransformer = 14,
    EffectsDataTransformer = 15,
    EndDataTransformer = 255,
}

#[derive(Debug)]
pub enum AnyData {
    StartData(StartData),
    TypeData(TypeData),
    NameData(NameData),
    IdentificationData(IdentificationData),
    PowderData(PowderData),
    RerollData(RerollData),
    // TODO
    EndData(EndData),
}
