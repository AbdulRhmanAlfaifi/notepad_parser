use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotepadErrors {
    #[error("Encountered an error. Error: '{0}', Function: '{1}', Additinal: '{2}'")]
    Generic(String, String, String),
    #[error(
        "File signature does't match the correct TabState file format. Expected 'NP', found '{0}'"
    )]
    Signature(String),
    #[error("Unable to read data. Error: '{0}', Field: '{1}'")]
    ReadError(String, String),
    #[error("Unable to read data. Error: '{0}', Field: '{1}', Size: '{2}'")]
    ReadErrorWithSize(String, String, String),
    #[error("Unexpected value found. Expected: '{0}', Found: '{1}', Field: '{2}'")]
    UnexpectedValue(String, String, String),
    #[error("EoF Reached")]
    EoF,
    #[error("No data to parse")]
    NA,
    #[error("Error while opening a file. ERROR: '{0}', PATH: '{1}'")]
    FileOpen(String, String),
    #[error("CLI error. ERROR: '{0}', MSG: '{1}'")]
    CLIError(String, String),
}
