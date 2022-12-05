use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Error when running DB query: {0}")]
    DbQueryErr(String),
    #[error("Error serializing/deserializing data: {0}")]
    SerDeErr(String),
    #[error("Empty set: {0}")]
    Empty(String),
}
