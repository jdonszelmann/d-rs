use thiserror::Error;

#[derive(Error, Debug)]
pub enum DError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    Regex(#[from] fancy_regex::Error),
}

impl<'a> From<&'a str> for DError {
    fn from(s: &'a str) -> Self {
        DError::Message(s.to_string())
    }
}

impl<'a> From<&'a String> for DError {
    fn from(s: &'a String) -> Self {
        DError::Message(s.to_string())
    }
}

impl From<String> for DError {
    fn from(s: String) -> Self {
        DError::Message(s)
    }
}

pub type DResult<T> = Result<T, DError>;