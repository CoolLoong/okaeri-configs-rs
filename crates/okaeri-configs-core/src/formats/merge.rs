use crate::error::ConfigResult;
use crate::metadata::ConfigMetadata;
use crate::options::ConfigOptions;
use std::collections::HashSet;

pub enum ConfigValue {
    #[cfg(feature = "json")]
    Json(serde_json::Value),
    #[cfg(feature = "toml")]
    Toml(toml::Value),
    #[cfg(feature = "yaml")]
    Yaml(serde_yaml_bw::Value),
}

impl ConfigValue {
    pub fn merge(
        old: Self,
        new: Self,
        metadata: &ConfigMetadata,
        options: &ConfigOptions,
    ) -> ConfigResult<Self> {
        match (old, new) {
            #[cfg(feature = "json")]
            (Self::Json(old_val), Self::Json(new_val)) => {
                Ok(Self::Json(merge_json(old_val, new_val, metadata, options)?))
            }
            #[cfg(feature = "toml")]
            (Self::Toml(old_val), Self::Toml(new_val)) => {
                Ok(Self::Toml(merge_toml(old_val, new_val, metadata, options)?))
            }
            #[cfg(feature = "yaml")]
            (Self::Yaml(old_val), Self::Yaml(new_val)) => {
                Ok(Self::Yaml(merge_yaml(old_val, new_val, metadata, options)?))
            }
            (_, new_val) => Ok(new_val),
        }
    }
}

fn build_valid_keys(metadata: &ConfigMetadata, options: &ConfigOptions) -> HashSet<String> {
    metadata
        .fields
        .iter()
        .filter_map(|field| {
            if field.exclude {
                return None;
            }
            if let Some(ref custom_key) = field.key {
                Some(custom_key.to_string())
            } else {
                Some(options.naming_strategy.apply(&field.name))
            }
        })
        .collect()
}

#[cfg(feature = "json")]
fn merge_json(
    old_value: serde_json::Value,
    new_value: serde_json::Value,
    metadata: &ConfigMetadata,
    options: &ConfigOptions,
) -> ConfigResult<serde_json::Value> {
    use serde_json::Value;

    match (old_value, new_value) {
        (Value::Object(mut old_map), Value::Object(new_map)) => {
            let valid_keys = build_valid_keys(metadata, options);
            for (key, value) in new_map {
                old_map.entry(key).or_insert(value);
            }
            if options.remove_orphans {
                old_map.retain(|key, _| valid_keys.contains(key));
            }
            let mut sorted_map = serde_json::Map::new();
            let mut keys: Vec<String> = old_map.keys().cloned().collect();
            keys.sort_by(|a, b| options.field_order.compare(a, b));
            for key in keys {
                if let Some(value) = old_map.remove(&key) {
                    sorted_map.insert(key, value);
                }
            }
            Ok(Value::Object(sorted_map))
        }
        (_, new_value) => Ok(new_value),
    }
}

#[cfg(feature = "toml")]
fn merge_toml(
    old_value: toml::Value,
    new_value: toml::Value,
    metadata: &ConfigMetadata,
    options: &ConfigOptions,
) -> ConfigResult<toml::Value> {
    use toml::Value;

    match (old_value, new_value) {
        (Value::Table(mut old_map), Value::Table(new_map)) => {
            let valid_keys = build_valid_keys(metadata, options);
            for (key, value) in new_map {
                old_map.entry(key).or_insert(value);
            }
            if options.remove_orphans {
                old_map.retain(|key, _| valid_keys.contains(key));
            }
            let mut sorted_map = toml::map::Map::new();
            let mut keys: Vec<String> = old_map.keys().cloned().collect();
            keys.sort_by(|a, b| options.field_order.compare(a, b));
            for key in keys {
                if let Some(value) = old_map.remove(&key) {
                    sorted_map.insert(key, value);
                }
            }
            Ok(Value::Table(sorted_map))
        }
        (_, new_value) => Ok(new_value),
    }
}

#[cfg(feature = "yaml")]
fn merge_yaml(
    old_value: serde_yaml_bw::Value,
    new_value: serde_yaml_bw::Value,
    metadata: &ConfigMetadata,
    options: &ConfigOptions,
) -> ConfigResult<serde_yaml_bw::Value> {
    use serde_yaml_bw::Value;

    match (old_value, new_value) {
        (Value::Mapping(mut old_map), Value::Mapping(new_map)) => {
            let valid_keys = build_valid_keys(metadata, options);
            for (key, value) in new_map {
                old_map.entry(key).or_insert(value);
            }
            if options.remove_orphans {
                old_map.retain(|key, _| {
                    if let Value::String(s, None) = key {
                        valid_keys.contains(s)
                    } else {
                        true
                    }
                });
            }
            let mut sorted_map = serde_yaml_bw::Mapping::new();
            let mut keys: Vec<(Value, Value)> = old_map.into_iter().collect();
            keys.sort_by(|(a, _), (b, _)| match (a, b) {
                (Value::String(a_str, None), Value::String(b_str, None)) => {
                    options.field_order.compare(a_str, b_str)
                }
                _ => std::cmp::Ordering::Equal,
            });
            for (key, value) in keys {
                sorted_map.insert(key, value);
            }
            Ok(Value::Mapping(sorted_map))
        }
        (_, new_value) => Ok(new_value),
    }
}
