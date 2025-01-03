use thiserror::Error;

/// Errors yep
#[derive(Error, Debug)]
pub enum Errorfr {
    /// item config json is missing
    #[error("Error 1.1: item config json is missing.")]
    ItemJsonMissing,

    /// item config json is corrupt
    #[error("Error 1.2: item config json is invalid. \nReread config.md.\n{0}")]
    ItemJsonCorrupt(serde_json5::Error),

    /// idmap is missing
    #[error("Error 1.3: id_keys.json is missing. \nYou should run \"--download id_keys\" or \"--download All\".")]
    IDMapJsonMissing,

    /// idmap is corrupt
    #[error("Error 2.1: id_keys.json is corrupt. \nYou should run \"--download id_keys\" or \"--download All\".")]
    IDMapJsonCorrupt,

    /// shiny data json is missing
    #[error("Error 2.2: shiny_stats.json is missing. \nYou should run \"--download ShinyStats\" or \"--download All\".")]
    ShinyJsonMissing,

    /// shiny data json is corrupt
    #[error("Error 2.3: shiny_stats.json is corrupt. \nYou should run \"--download ShinyStats\" or \"--download All\".")]
    ShinyJsonCorrupt,

    /// could not download the file
    #[error("Error 3.1: Download request failed. Check your network settings.")]
    JsonDlReqFail,

    /// invalid body response after downloading
    #[error("Error 3.2: Download body is invalid. Something is broken.")]
    JsonDlReqBodyInvalid,

    /// unable to create file after download
    #[error("Error 3.3: Download successful, but unable to actually create the file.")]
    JsonDlReqFileCreateFail,

    /// unable to copy (write in) file content
    #[error("Error 3.4: Download successful, but unable to write to file.")]
    JsonDlReqFileWriteFail,

    /// Name value was not found in json
    #[error(
        "Error 4.1: \"name\" field was not found in the json (required for Gear, Tome, Charm)."
    )]
    JsonNotFoundName,

    /// unable to find the crafted_type field
    #[error("Error 4.2: Missing \"crafted_type\" field")]
    JsonNotFoundCraftedType,

    /// invalid crafted_type field
    #[error("Error 4.3: Invalid \"crafted_type\" value")]
    JsonInvalidCraftedType,

    /// Durability is out of allowed range (0 to 100)
    #[error("Error 4.4: Durability percentage is out of range (Should be between 0 and 100)")]
    JsonDuraOutOfRange,

    /// Durability was not found but is necessary
    #[error("Error 4.5: \"Durability\" was not found (necessary for Crafted Gear item type)")]
    JsonNotFoundDura,

    /// Durability Bad
    #[error(
        "Error 4.6: \"Requirements\" was not found (necessary for Crafted Gear / Consumable items)"
    )]
    JsonNotFoundReqs,
}
