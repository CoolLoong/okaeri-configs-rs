use crate::config::Config;
use crate::error::{ConfigError, ConfigResult};
use crate::formats::format::Format;
use crate::middleware::ConfigMiddleware;
use crate::options::ConfigOptions;
use std::path::{Path, PathBuf};

pub struct ConfigManager<T: Config> {
    config: T,
    /// the path of config file
    path: Option<PathBuf>,
    /// the format of config
    format: Format,
    options: ConfigOptions,
    middlewares: Vec<Box<dyn ConfigMiddleware>>,
}

impl<T: Config> ConfigManager<T> {
    pub fn create() -> ConfigManagerBuilder<T> {
        ConfigManagerBuilder::new()
    }

    pub fn get(&self) -> &T {
        &self.config
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.config
    }

    pub fn update<F>(&mut self, f: F) -> ConfigResult<()>
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.config);
        self.save()
    }

    pub fn save(&self) -> ConfigResult<()> {
        if let Some(path) = &self.path {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }

            let mut content = self.serialize()?;
            let metadata = T::metadata();
            for middleware in &self.middlewares {
                middleware.on_save(&mut content, &metadata, self.format)?;
            }
            std::fs::write(path, content)?;
            Ok(())
        } else {
            Err(ConfigError::Path("No path specified".to_string()))
        }
    }

    pub fn reload(&mut self) -> ConfigResult<()> {
        if let Some(path) = &self.path {
            let mut content = std::fs::read_to_string(path)?;
            let metadata = T::metadata();
            for middleware in &self.middlewares {
                middleware.on_load(&mut content, &metadata, self.format)?;
            }
            self.config = self.deserialize(&content)?;
            Ok(())
        } else {
            Err(ConfigError::Path("No path specified".to_string()))
        }
    }

    pub fn add_middleware(&mut self, middleware: Box<dyn ConfigMiddleware>) {
        self.middlewares.push(middleware);
    }

    pub fn set_options(&mut self, options: ConfigOptions) {
        self.options = options;
    }

    fn serialize(&self) -> ConfigResult<String> {
        let metadata = T::metadata();
        match self.format {
            #[cfg(feature = "toml")]
            Format::Toml => crate::formats::toml::to_string(&self.config, &metadata, &self.options),
            #[cfg(feature = "json")]
            Format::Json => crate::formats::json::to_string(&self.config, &metadata, &self.options),
            #[cfg(feature = "yaml")]
            Format::Yaml => crate::formats::yaml::to_string(&self.config, &metadata, &self.options),
        }
    }

    fn deserialize(&self, content: &str) -> ConfigResult<T> {
        match self.format {
            #[cfg(feature = "json")]
            Format::Json => crate::formats::json::from_str(content),
            #[cfg(feature = "toml")]
            Format::Toml => crate::formats::toml::from_str(content),
            #[cfg(feature = "yaml")]
            Format::Yaml => crate::formats::yaml::from_str(content),
        }
    }
}

pub struct ConfigManagerBuilder<T: Config> {
    path: Option<PathBuf>,
    format: Option<Format>,
    options: ConfigOptions,
    middlewares: Vec<Box<dyn ConfigMiddleware>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> ConfigManagerBuilder<T> {
    pub fn new() -> Self {
        Self {
            path: None,
            format: None,
            options: ConfigOptions::default(),
            middlewares: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn with_options(mut self, options: ConfigOptions) -> Self {
        self.options = options;
        self
    }

    pub fn with_naming_strategy(mut self, strategy: crate::options::NamingStrategy) -> Self {
        self.options.naming_strategy = strategy;
        self
    }

    pub fn with_field_order(mut self, order: crate::options::FieldOrder) -> Self {
        self.options.field_order = order;
        self
    }

    pub fn with_middleware<M: ConfigMiddleware + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }

    pub fn with_remove_orphans(mut self, remove: bool) -> Self {
        self.options.remove_orphans = remove;
        self
    }

    pub fn build(self) -> ConfigResult<ConfigManager<T>> {
        let format = if let Some(fmt) = self.format {
            fmt
        } else if let Some(ref p) = self.path {
            Format::try_from(p).map_err(|_| {
                ConfigError::UnsupportedFormat(
                    "Could not determine format from file extension".to_string(),
                )
            })?
        } else {
            Format::default()
        };

        let mut config: T = if let Some(ref p) = self.path {
            if p.exists() {
                let mut content = std::fs::read_to_string(p)?;
                let metadata = T::metadata();
                for middleware in &self.middlewares {
                    middleware.on_load(&mut content, &metadata, format)?;
                }

                use crate::formats::merge::ConfigValue;
                let (old_value, new_value) = match format {
                    #[cfg(feature = "json")]
                    Format::Json => {
                        let old_value = ConfigValue::Json(json5::from_str(&content)?);
                        let new_value = ConfigValue::Json(serde_json::to_value(&T::default())?);
                        (old_value, new_value)
                    }
                    #[cfg(feature = "toml")]
                    Format::Toml => {
                        let old_value = ConfigValue::Toml(toml::from_str(&content)?);
                        let new_value = ConfigValue::Toml(toml::Value::try_from(&T::default())?);
                        (old_value, new_value)
                    }
                    #[cfg(feature = "yaml")]
                    Format::Yaml => {
                        let old_value = ConfigValue::Yaml(serde_yaml_bw::from_str(&content)?);
                        let new_value = ConfigValue::Yaml(serde_yaml_bw::to_value(&T::default())?);
                        (old_value, new_value)
                    }
                };
                let merged = ConfigValue::merge(old_value, new_value, &metadata, &self.options)?;
                match merged {
                    #[cfg(feature = "toml")]
                    ConfigValue::Toml(v) => v.try_into()?,
                    #[cfg(feature = "json")]
                    ConfigValue::Json(v) => serde_json::from_value(v)?,
                    #[cfg(feature = "yaml")]
                    ConfigValue::Yaml(v) => serde_yaml_bw::from_value(v)?,
                }
            } else {
                T::default()
            }
        } else {
            T::default()
        };
        config.apply_env()?;
        let manager = ConfigManager {
            config,
            path: self.path,
            format,
            options: self.options,
            middlewares: self.middlewares,
        };
        manager.save().map(|_| manager)
    }
}

impl<T: Config> Default for ConfigManagerBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}
