use crate::error::ConfigResult;
use crate::metadata::ConfigMetadata;
use crate::options::ConfigOptions;
use serde::{Serialize, de::DeserializeOwned};
use serde_yaml_bw::{Mapping, Value};
use std::collections::HashMap;

use super::common::{build_comment_map, format_header};

pub fn to_string<T: Serialize>(
    value: &T,
    metadata: &ConfigMetadata,
    options: &ConfigOptions,
) -> ConfigResult<String> {
    let mut yaml_value = serde_yaml_bw::to_value(value)?;

    let key_to_field = if let Value::Mapping(ref mut map) = yaml_value {
        apply_options(map, metadata, options)
    } else {
        HashMap::new()
    };

    let mut yaml_str = serde_yaml_bw::to_string(&yaml_value)?;

    if let Some(header) = format_header(metadata, "#") {
        yaml_str = format!("{}\n\n{}", header, yaml_str);
    }

    let key_comments = build_comment_map(&key_to_field, "#");

    let mut result = String::new();
    let mut used_comments = std::collections::HashSet::new();

    for line in yaml_str.lines() {
        for (actual_key, comment) in &key_comments {
            if used_comments.contains(actual_key) {
                continue;
            }

            if line.contains(&format!("{}:", actual_key)) {
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
    map: &mut Mapping,
    metadata: &'a ConfigMetadata,
    options: &ConfigOptions,
) -> HashMap<String, &'a crate::metadata::FieldMetadata> {
    let mut key_to_field: HashMap<String, &crate::metadata::FieldMetadata> = HashMap::new();
    let mut renamed_entries: Vec<(Value, Value)> = Vec::new();
    let original_keys: Vec<Value> = map.keys().cloned().collect();

    for key in original_keys {
        if let Some(key_str) = key.as_str() {
            let field_meta = metadata.fields.iter().find(|f| f.name.as_ref() == key_str);

            if let Some(field) = field_meta {
                let new_key_str = if let Some(ref custom_key) = field.key {
                    custom_key.to_string()
                } else {
                    options.naming_strategy.apply(key_str)
                };

                key_to_field.insert(new_key_str.clone(), field);

                if new_key_str.as_str() != key_str {
                    if let Some(value) = map.remove(key_str) {
                        renamed_entries.push((Value::String(new_key_str, None), value));
                    }
                }
            }
        }
    }

    for (key, value) in renamed_entries {
        map.insert(key, value);
    }

    let mut entries: Vec<(String, Value)> = map
        .iter()
        .filter_map(|(k, v)| k.as_str().map(|s| (s.to_string(), v.clone())))
        .collect();
    entries.sort_by(|a, b| options.field_order.compare(&a.0, &b.0));
    map.clear();
    for (key, value) in entries {
        map.insert(Value::String(key, None), value);
    }

    key_to_field
}

pub fn from_str<T: DeserializeOwned>(content: &str) -> ConfigResult<T> {
    Ok(serde_yaml_bw::from_str(content)?)
}
