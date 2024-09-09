use crate::types::{itemtype::ItemType, transform::TransformVersion};

use super::{DataDecoder, DataEncoder, DataTransformError, DataTransformerTypes, TransformId};

#[derive(Debug, Clone)]
pub struct TypeData(pub ItemType);

impl TransformId for TypeData {
    fn get_id() -> u8 {
        DataTransformerTypes::TypeDataTransformer as u8
    }
}

impl DataEncoder for TypeData {
    fn encode_data(
        &self,
        ver: TransformVersion,
        out: &mut Vec<u8>,
    ) -> Result<(), super::DataTransformError> {
        match ver {
            TransformVersion::Version1 => out.push(self.0.into()),
        }

        Ok(())
    }
}

impl<B: Iterator<Item = u8>> DataDecoder<B> for TypeData {
    fn decode_data(bytes: &mut B, ver: TransformVersion) -> Result<Self, super::DataTransformError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                let b = bytes.next().unwrap();

                Ok(Self(
                    ItemType::try_from(b).map_err(|_| DataTransformError::InvalidTypeError)?,
                ))
            }
        }
    }
}
