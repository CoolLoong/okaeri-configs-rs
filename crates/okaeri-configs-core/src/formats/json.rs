use crate::error::ConfigResult;
use crate::metadata::ConfigMetadata;
use crate::options::ConfigOptions;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::common::{build_comment_map, format_header};

pub fn to_string<T: Serialize>(
    value: &T,
    metadata: &ConfigMetadata,
    options: &ConfigOptions,
) -> ConfigResult<String> {
    let mut json_value: Value = serde_json::to_value(value)?;
    let key_to_field = if let Value::Object(ref mut map) = json_value {
        apply_options(map, metadata, options)
    } else {
        HashMap::new()
    };
    
    let key_comments = build_comment_map(&key_to_field, "//");
    let json_str = serde_json::to_string_pretty(&json_value)?;

    let mut result = String::new();

    if let Some(header) = format_header(metadata, "//") {
        result.push_str(&header);
        result.push_str("\n");
    }
    let mut used_comments = std::collections::HashSet::new();

    for line in json_str.lines() {
        for (actual_key, comment) in &key_comments {
            if used_comments.contains(actual_key) {
                continue;
            }

            if line.contains(&format!("\"{}\":", actual_key)) {
                let indent = line
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>();

                for (i, comment_line) in comment.lines().enumerate() {
                    if i > 0 {
                        result.push('\n');
                    }
                    result.push_str(&indent);
                    result.push_str(comment_line);
                }
                result.push('\n');
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
    map: &mut serde_json::Map<String, Value>,
    metadata: &'a ConfigMetadata,
    options: &ConfigOptions,
) -> HashMap<String, &'a crate::metadata::FieldMetadata> {
    let mut key_to_field: HashMap<String, &crate::metadata::FieldMetadata> = HashMap::new();
    let mut ordered_map = serde_json::Map::new();

    let mut field_names: Vec<String> = Vec::new();
    for field in &metadata.fields {
        if field.exclude {
            continue;
        }

        let new_key_str = if let Some(ref custom_key) = field.key {
            custom_key.to_string()
        } else {
            options.naming_strategy.apply(&field.name)
        };

        key_to_field.insert(new_key_str.clone(), field);
        field_names.push(new_key_str);
    }

    field_names.sort_by(|a, b| options.field_order.compare(a, b));

    for key in field_names {
        if let Some(value) = map.remove(&key) {
            ordered_map.insert(key, value);
        }
    }

    *map = ordered_map;
    key_to_field
}

pub fn from_str<T: DeserializeOwned>(content: &str) -> ConfigResult<T> {
    Ok(json5::from_str(content)?)
}
