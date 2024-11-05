use thiserror::Error;

/// Errors yep
#[derive(Error, Debug)]
pub enum Errorfr {
    /// config json is missing
    #[error("Error 1: json config json is missing, obtain it from https://git.frfrnocap.men/endernon/idmangler-cli/raw/branch/main/config.json and move it to this directory. "
    )]
    JsonMissing,

    /// config json is corrupt
    #[error("Error 2: json config json is corrupt. Reread config.md or reobtain it from https://git.frfrnocap.men/endernon/idmangler-cli/raw/branch/main/config.json and move it to this diirectory. "
    )]
    JsonCorrupt,

    /// idmap is missing
    #[error("Error 3: Identifications hashmap is missing. Get it from https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json and move it to this directory. ")]
    IDMapMissing,

    /// idmap is corrupt
    #[error("Error 4: Identifications hashmap is corrupt. Reobtain it from https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json and move it to this directory. ")]
    IDMapCorrupt,

    /// shiny data json is missing
    #[error("Error 5: Shiny data json is missing. Get it from https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json and move it to this directory. ")]
    ShinyJsonMissing,

    /// shiny data json is corrupt
    #[error("Error 6: Shiny data json is corrupt. Get it from https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json and move it to this directory. ")]
    ShinyJsonCorrupt
}