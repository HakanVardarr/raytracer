use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot open the file: {0}")]
    CannotOpenFile(String),
    #[error("Cannot write to the file.")]
    CannotWriteToTheFile,
}
