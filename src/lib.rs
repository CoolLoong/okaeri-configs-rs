//! # okaeri-configs-rs
//!
//! A flexible configuration library for Rust, inspired by [okaeri-configs](https://github.com/OkaeriPoland/okaeri-configs).
//!
//! ## Features
//!
//! - **Multiple formats**: JSON, TOML, YAML support
//! - **Comments**: Add comments to configuration fields
//! - **Headers**: Add file headers to configurations
//! - **Environment variables**: Override config values from environment
//! - **Middleware system**: Extend with custom processing logic
//! - **Type-safe**: Full compile-time type safety with derive macros
//!
//! ## Quick Start
//!
//! ```rust
//! use okaeri_configs::prelude::*;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Config, Serialize, Deserialize, Default)]
//! #[comment(r#"My Application Configuration"#)]
//! struct AppConfig {
//!     #[comment("Server host")]
//!     #[env("SERVER_HOST")]
//!     host: String,
//!
//!     #[comment("Server port")]
//!     #[env("SERVER_PORT")]
//!     port: u16,
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut manager = ConfigManager::<AppConfig>::create()
//!         .with_path("example.toml")
//!         .build()?;
//!     manager.update(|config| {
//!         config.port = 8080;
//!     })?;
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! Enable features in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! okaeri-configs = { version = "0.0.1" }
//! # Or enable specific features
//! okaeri-configs = { version = "0.0.1", features = ["toml"] }
//! ```
//!
//! Available features:
//! - `json` - JSON format support
//! - `toml` - TOML format support
//! - `yaml` - YAML format support
//! - `all` - Enable all features

pub use okaeri_configs_core::{
    Config, ConfigDerive, ConfigError, ConfigManager, ConfigManagerBuilder, ConfigMetadata,
    ConfigMiddleware, ConfigOptions, ConfigResult, FieldMetadata, FieldOrder, Format,
    NamingStrategy,
};
pub mod prelude {
    pub use okaeri_configs_core::prelude::*;
}