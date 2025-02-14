use crate::{data_transformer::DataTransformError, types::transform::TransformVersion};

use super::{DataDecoder, DataEncoder, DataTransformerTypes, TransformId};

#[derive(Debug, Clone)]
pub struct NameData(pub String);

impl TransformId for NameData {
    fn get_id() -> u8 {
        DataTransformerTypes::NameDataTransformer as u8
    }
}

impl DataEncoder for NameData {
    fn encode_data(
        &self,
        ver: TransformVersion,
        out: &mut Vec<u8>,
    ) -> Result<(), super::DataTransformError> {
        match ver {
            TransformVersion::Version1 => {
                // check that the string is valid ascii
                if self.0.chars().any(|c| !c.is_ascii()) {
                    return Err(DataTransformError::BadString);
                }

                // push the bytes
                out.extend_from_slice(self.0.as_bytes());
                // push the null terminator
                out.push(0);
            }
        }

        Ok(())
    }
}

impl<B: Iterator<Item = u8>> DataDecoder<B> for NameData {
    fn decode_data(bytes: &mut B, ver: TransformVersion) -> Result<Self, super::DataTransformError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                let b: Vec<u8> = bytes.take_while(|b| *b != 0).collect();

                // UTF-8 and ASCII share the same set of characters
                Ok(NameData(
                    String::from_utf8(b).map_err(|_| DataTransformError::BadString)?,
                ))
            }
        }
    }
}
