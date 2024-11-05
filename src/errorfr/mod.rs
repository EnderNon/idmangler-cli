use thiserror::Error;

/// Errors yep
#[derive(Error, Debug)]
pub enum Errorfr {
    /// item config json is missing
    #[error("Error 1: item config json is missing")]
    ItemJsonMissing,

    /// item config json is corrupt
    #[error("Error 2: item config json is corrupt, Reread config.md"
    )]
    ItemJsonCorrupt,

    /// idmap is missing
    #[error("Error 3: id_keys.json is missing")]
    IDMapJsonMissing,

    /// idmap is corrupt
    #[error("Error 4: id_keys.json is corrupt")]
    IDMapJsonCorrupt,

    /// shiny data json is missing
    #[error("Error 5: shiny_stats.json is missing.")]
    ShinyJsonMissing,

    /// shiny data json is corrupt
    #[error("Error 6: shiny_stats.json is corrupt.")]
    ShinyJsonCorrupt
}