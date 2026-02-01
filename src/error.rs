use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("NBT parsing error: {0}")]
    Nbt(#[from] fastnbt::error::Error),

    #[error("Unknown schematic format")]
    UnknownFormat,

    #[error("Invalid schematic: {0}")]
    Invalid(String),

    #[error("Unsupported version: {0}")]
    UnsupportedVersion(i32),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid block data at index {0}")]
    InvalidBlockData(usize),
}
