use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Format not supported: {0}")]
    UnsupportedFormat(String),

    #[error("Environment variable error: {0}")]
    EnvVar(String),

    #[error("Middleware error: {0}")]
    Middleware(String),

    #[error("Path error: {0}")]
    Path(String),

    #[cfg(feature = "json")]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "json")]
    #[error("JSONC error: {0}")]
    Jsonc(#[from] json5::Error),

    #[cfg(feature = "toml")]
    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[cfg(feature = "toml")]
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[cfg(feature = "yaml")]
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml_bw::Error),
}

pub type ConfigResult<T> = std::result::Result<T, ConfigError>;
