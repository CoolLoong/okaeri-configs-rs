#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "json")]
    Json,
    #[cfg(feature = "yaml")]
    Yaml,
}

impl TryFrom<&std::path::PathBuf> for Format {
    type Error = crate::error::ConfigError;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        if let Some(extension) = value.extension()
            && let Some(extension) = extension.to_str()
        {
            match extension {
                #[cfg(feature = "toml")]
                "toml" => Ok(Format::Toml),
                #[cfg(feature = "json")]
                "json" => Ok(Format::Json),
                #[cfg(feature = "yaml")]
                "yaml" | "yml" => Ok(Format::Yaml),
                _ => Err(crate::error::ConfigError::UnsupportedFormat(
                    extension.to_string(),
                )),
            }
        } else {
            Err(crate::error::ConfigError::UnsupportedFormat(
                value
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default()
                    .to_string(),
            ))
        }
    }
}

impl Format {
    pub fn extension(&self) -> &'static str {
        match self {
            #[cfg(feature = "toml")]
            Format::Toml => "toml",
            #[cfg(feature = "json")]
            Format::Json => "json",
            #[cfg(feature = "yaml")]
            Format::Yaml => "yaml",
        }
    }

    pub fn default() -> Self {
        #[cfg(feature = "toml")]
        {
            return Format::Toml;
        }
        #[cfg(all(feature = "json", not(feature = "toml")))]
        {
            return Format::Json;
        }
        #[cfg(all(feature = "yaml", not(feature = "toml"), not(feature = "json")))]
        {
            return Format::Yaml;
        }
        #[cfg(not(any(feature = "toml", feature = "json", feature = "yaml")))]
        compile_error!("At least one format feature (toml, json, yaml) must be enabled");
    }
}
