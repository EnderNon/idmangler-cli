/// Potential errors thrown during encoding of id strings
#[derive(Error, Debug)]
pub enum EncodeError {
    /// Encoder was given a string with non ascii characters.
    #[error("Cannot encode non ascii string")]
    NonAsciiString,

    /// More than 255 identifications were passed for encoding
    #[error("Cannot encode more than 255 identifications per item")]
    TooManyIdentifications,
    /// Identification is missing a basevalue while using the extended encoding scheme.
    ///
    /// An id is required to have an basevalue if the extended encoding is used for idents
    #[error("Identification id: {0} was not given a base value while using extended encoding")]
    NoBasevalueGiven(u8),

    /// More than 255 powders were passed for encoding
    #[error("Cannot encode more than 255 powders per item")]
    TooManyPowders,
    /// Invalid tier for a powder was passed
    #[error("Invalid powder tier of {0} was passed")]
    InvalidPowderTier(u8),

    /// Effect strength should be a percentage between 0 and 100
    #[error("Effect strength of {0} is too high, it should be a percentage between 0 and 100")]
    EffectStrengthTooHigh(u8),

    /// More than 255 skills were passed for encoding
    #[error("Cannot encode more than 255 skills per item")]
    TooManySkills,

    /// More than 255 damage values were passed for encoding
    #[error("Cannot encode more than 255 damage values per item")]
    TooManyDamageValues,

    /// More than 255 effects were passed for encoding
    #[error("Cannot encode more than 255 effects per item")]
    TooManyEffects,

    /// More than 255 defense values were passed for encoding
    #[error("Cannot encode more than 255 defense values per item")]
    TooManyDefences,
}