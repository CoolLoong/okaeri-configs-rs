use crate::error::ConfigResult;
use crate::metadata::ConfigMetadata;
use crate::options::ConfigOptions;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

use super::common::{build_comment_map, format_header};

pub fn to_string<T: Serialize>(
    value: &T,
    metadata: &ConfigMetadata,
    options: &ConfigOptions,
) -> ConfigResult<String> {
    let toml_value = toml::to_string(value)?;
    let mut toml_table: toml::Value = toml::from_str(&toml_value)?;

    let key_to_field = if let toml::Value::Table(ref mut map) = toml_table {
        apply_options(map, metadata, options)
    } else {
        HashMap::new()
    };

    let key_comments = build_comment_map(&key_to_field, "#");

    let mut origin_str = toml::to_string_pretty(&toml_table)?;

    if let Some(header) = format_header(metadata, "#") {
        origin_str = format!("{}\n\n{}", header, origin_str);
    }

    let mut result = String::new();
    let mut used_comments = std::collections::HashSet::new();

    for line in origin_str.lines() {
        for (actual_key, comment) in &key_comments {
            if used_comments.contains(actual_key) {
                continue;
            }

            // Match TOML field patterns:
            // 1. "key = " or "key=" - field assignment
            // 2. "[key]" or "[key." - structure header
            let pattern_assign1 = format!("{} = ", actual_key);
            let pattern_assign2 = format!("{}=", actual_key);
            let pattern_section1 = format!("[{}]", actual_key);
            let pattern_section2 = format!("[{}.", actual_key);

            if line.contains(&pattern_assign1)
                || line.contains(&pattern_assign2)
                || line.contains(&pattern_section1)
                || line.contains(&pattern_section2)
            {
                let indent = line
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>();
                for comment_line in comment.lines() {
                    result.push_str(&indent);
                    result.push_str(comment_line);
                    result.push('\n');
                }
                used_comments.insert(actual_key.clone());
                break;
            }
        }

        result.push_str(line);
        result.push('\n');
    }
    Ok(result)
}

fn apply_options<'a>(
    map: &mut toml::map::Map<String, toml::Value>,
    metadata: &'a ConfigMetadata,
    options: &ConfigOptions,
) -> HashMap<String, &'a crate::metadata::FieldMetadata> {
    let mut key_to_field: HashMap<String, &crate::metadata::FieldMetadata> = HashMap::new();
    let mut renamed_entries: Vec<(String, toml::Value)> = Vec::new();
    let original_keys: Vec<String> = map.keys().cloned().collect();

    for key in original_keys {
        let field_meta = metadata
            .fields
            .iter()
            .find(|f| f.name.as_ref() == key.as_str());

        if let Some(field) = field_meta {
            let new_key_str = if let Some(ref custom_key) = field.key {
                custom_key.to_string()
            } else {
                options.naming_strategy.apply(&key)
            };

            key_to_field.insert(new_key_str.clone(), field);

            if new_key_str != key {
                if let Some(value) = map.remove(&key) {
                    renamed_entries.push((new_key_str, value));
                }
            }
        }
    }

    for (key, value) in renamed_entries {
        map.insert(key, value);
    }
    let mut entries: Vec<(String, toml::Value)> =
        map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    entries.sort_by(|a, b| options.field_order.compare(&a.0, &b.0));
    map.clear();
    for (key, value) in entries {
        map.insert(key, value);
    }

    key_to_field
}

pub fn from_str<T: DeserializeOwned>(content: &str) -> ConfigResult<T> {
    Ok(toml::from_str(content)?)
}
