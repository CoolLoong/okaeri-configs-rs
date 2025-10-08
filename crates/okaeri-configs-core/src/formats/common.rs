use crate::metadata::{ConfigMetadata, FieldMetadata};
use std::collections::HashMap;

pub fn format_header(metadata: &ConfigMetadata, comment_prefix: &str) -> Option<String> {
    if metadata.struct_comments.is_empty() {
        return None;
    }

    let all_lines: Vec<String> = metadata
        .struct_comments
        .iter()
        .flat_map(|s| s.lines())
        .map(|line| line.trim_end())
        .filter(|line| !line.is_empty())
        .map(|line| format!("{} {}", comment_prefix, line))
        .collect();

    if all_lines.is_empty() {
        return None;
    }

    Some(format!("{}\n", all_lines.join("\n")))
}

pub fn resolve_field_comments(field: &FieldMetadata) -> Vec<String> {
    let mut comments = Vec::new();

    for comment in field.comments.iter() {
        comments.extend(
            comment
                .lines()
                .map(|line| line.trim_end())
                .filter(|line| !line.is_empty())
                .map(|line| line.to_string())
        );
    }

    comments
}

pub fn build_comment_map<'a>(
    key_to_field: &HashMap<String, &'a FieldMetadata>,
    comment_prefix: &str,
) -> HashMap<String, String> {
    let mut key_comments = HashMap::new();

    for (actual_key, field) in key_to_field.iter() {
        if field.exclude {
            continue;
        }

        let resolved_comments = resolve_field_comments(field);
        if resolved_comments.is_empty() {
            continue;
        }

        let comment = resolved_comments
            .iter()
            .map(|c| format!("{} {}", comment_prefix, c))
            .collect::<Vec<_>>()
            .join("\n");

        key_comments.insert(actual_key.clone(), comment);
    }

    key_comments
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::FieldMetadata;
    use std::borrow::Cow;

    #[test]
    fn test_format_header() {
        let mut metadata = ConfigMetadata::default();
        metadata.struct_comments = vec![
            Cow::Borrowed("Line 1"),
            Cow::Borrowed("Line 2"),
        ];

        let result = format_header(&metadata, "#");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "# Line 1\n# Line 2\n");
    }

    #[test]
    fn test_resolve_field_comments() {
        let field = FieldMetadata::new("test")
            .with_comment("Comment 1")
            .with_comment("Comment 2");

        let comments = resolve_field_comments(&field);
        assert_eq!(comments, vec!["Comment 1", "Comment 2"]);
    }

    #[test]
    fn test_build_comment_map() {
        let field = FieldMetadata::new("test")
            .with_comment("Test comment");

        let mut key_to_field = HashMap::new();
        key_to_field.insert("test".to_string(), &field);

        let result = build_comment_map(&key_to_field, "#");

        assert_eq!(result.len(), 1);
        assert_eq!(result.get("test").unwrap(), "# Test comment");
    }

    #[test]
    fn test_format_header_multiline() {
        let mut metadata = ConfigMetadata::default();
        metadata.struct_comments = vec![
            Cow::Borrowed("Line 1\nLine 2\nLine 3"),
        ];

        let result = format_header(&metadata, "#");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "# Line 1\n# Line 2\n# Line 3\n");
    }

    #[test]
    fn test_resolve_field_comments_multiline() {
        let field = FieldMetadata::new("test")
            .with_comment("Comment line 1\nComment line 2");

        let comments = resolve_field_comments(&field);
        assert_eq!(comments, vec!["Comment line 1", "Comment line 2"]);
    }

    #[test]
    fn test_build_comment_map_multiline() {
        let field = FieldMetadata::new("test")
            .with_comment("Line 1\nLine 2");

        let mut key_to_field = HashMap::new();
        key_to_field.insert("test".to_string(), &field);

        let result = build_comment_map(&key_to_field, "#");

        assert_eq!(result.len(), 1);
        assert_eq!(result.get("test").unwrap(), "# Line 1\n# Line 2");
    }
}