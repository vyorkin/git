use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Invalid .git/objects file header")]
    InvalidHeader,

    #[error("Invalid size of .git/objects file (expected: {expected:?}, actual: {actual:?})")]
    InvalidContentSize { expected: usize, actual: usize },

    #[error("Invalid object kind `{0}`")]
    InvalidObjectKind(String),

    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),
}
