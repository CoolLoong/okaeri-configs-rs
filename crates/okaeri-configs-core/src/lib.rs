pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod formats;
pub(crate) mod manager;
pub(crate) mod metadata;
pub(crate) mod middleware;
pub(crate) mod options;

pub use config::Config;
pub use error::{ConfigError, ConfigResult};
pub use formats::format::Format;
pub use manager::{ConfigManager, ConfigManagerBuilder};
pub use metadata::{ConfigMetadata, FieldMetadata};
pub use middleware::ConfigMiddleware;
pub use options::{ConfigOptions, FieldOrder, NamingStrategy};
pub use okaeri_configs_derive::Config as ConfigDerive;

pub mod prelude {
    pub use crate::config::Config;
    pub use crate::error::{ConfigError, ConfigResult};
    pub use crate::formats::format::Format;
    pub use crate::manager::{ConfigManager, ConfigManagerBuilder};
    pub use crate::metadata::{ConfigMetadata, FieldMetadata};
    pub use crate::middleware::ConfigMiddleware;
    pub use crate::options::{ConfigOptions, FieldOrder, NamingStrategy};
    pub use okaeri_configs_derive::Config;
    pub use serde::{Deserialize, Serialize};
}