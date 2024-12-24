use thiserror::Error;

/// Errors yep
#[derive(Error, Debug)]
pub enum Errorfr {
    /// item config json is missing
    #[error("Error 1.1: item config json is missing.")]
    ItemJsonMissing,

    /// item config json is corrupt
    #[error("Error 1.2: item config json is corrupt, Reread config.md.\n{0}")]
    ItemJsonCorrupt(serde_json::Error),

    /// idmap is missing
    #[error("Error 1.3: id_keys.json is missing")]
    IDMapJsonMissing,

    /// idmap is corrupt
    #[error("Error 2.1: id_keys.json is corrupt")]
    IDMapJsonCorrupt,

    /// shiny data json is missing
    #[error("Error 2.2: shiny_stats.json is missing.")]
    ShinyJsonMissing,

    /// shiny data json is corrupt
    #[error("Error 2.3: shiny_stats.json is corrupt.")]
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
    #[error("Error 9: Download successful, but unable to write to file.")]
    JsonDlReqFileWriteFail,
}
