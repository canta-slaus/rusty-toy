use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToyError {
    #[error("Missing path to .toy file")]
    MissingRequiredPath,
    #[error("File does not exist")]
    FileDoesNotExist,
    #[error("File is not a .toy file")]
    InvalidFileType,
    #[error("Unkown file type")]
    UnknownFileType,
    #[error("Couldn't read file contents")]
    CouldntReadFile {
        #[from]
        source: io::Error,
    },
}
