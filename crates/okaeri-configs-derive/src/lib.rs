use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod codegen;

use attrs::ConfigAttrs;
use codegen::generate_config_impl;

/// Derive macro for the Config trait
///
/// # Attributes
///
/// ## Structure attributes
/// - `#[comment("...")]` - Add a config header comment
///
/// ## Field attributes
/// - `#[comment("...")]` - Add a comment
/// - `#[key("...")]` - Custom config key name
/// - `#[env("ENV_VAR")]` - Override from environment variable
///
/// # Example
///
/// ```rust
/// use okaeri_configs::prelude::*;
///
/// #[derive(Config, Serialize, Deserialize, Default)]
/// #[comment(r#"My Configuration File"#)]
/// struct AppConfig {
///     #[comment("Server host")]
///     #[env("SERVER_HOST")]
///     host: String,
///
///     #[comment("Server port")]
///     port: u16,
/// }
/// ```
#[proc_macro_derive(Config, attributes(comment, key, env))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let attrs = match ConfigAttrs::from_derive_input(&input) {
        Ok(attrs) => attrs,
        Err(err) => return err.to_compile_error().into(),
    };

    generate_config_impl(&input, &attrs).into()
}
