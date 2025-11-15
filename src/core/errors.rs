use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("authentication failed: {0}")]
    Auth(String),

    #[error("network error: {0}")]
    Network(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("validation error: {field}: {message}")]
    Validation { field: String, message: String },

    #[error("not found: {0}")]
    NotFound(String),

    #[error("rate limited")]
    RateLimit,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("toml error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("toml serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn auth(msg: impl Into<String>) -> Self {
        Self::Auth(msg.into())
    }

    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }

    pub fn parse(msg: impl Into<String>) -> Self {
        Self::Parse(msg.into())
    }

    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn io(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }

    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
}
