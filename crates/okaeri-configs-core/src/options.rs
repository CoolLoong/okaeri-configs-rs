use std::cmp::Ordering;


#[derive(Debug, Clone)]
pub struct ConfigOptions {
    pub naming_strategy: NamingStrategy,
    pub field_order: FieldOrder,
    /// Remove orphaned fields (fields that exist in file but not in struct)
    pub remove_orphans: bool
}

impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            naming_strategy: NamingStrategy::Identity,
            field_order: FieldOrder::Declaration,
            remove_orphans: false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamingStrategy {
    /// Keep original field names (my_field -> my_field)
    Identity,
    /// Convert to camelCase (my_field -> myField)
    CamelCase,
    /// Convert to PascalCase (my_field -> MyField)
    PascalCase,
    /// Convert to snake_case (myField -> my_field)
    SnakeCase,
    /// Convert to kebab-case (my_field -> my-field)
    KebabCase,
    /// Convert to SCREAMING_SNAKE_CASE (my_field -> MY_FIELD)
    ScreamingSnakeCase,
}

impl NamingStrategy {
    pub(crate) fn apply(&self, name: &str) -> String {
        match self {
            Self::Identity => name.to_string(),
            Self::CamelCase => to_camel_case(name),
            Self::PascalCase => to_pascal_case(name),
            Self::SnakeCase => to_snake_case(name),
            Self::KebabCase => to_kebab_case(name),
            Self::ScreamingSnakeCase => to_screaming_snake_case(name),
        }
    }
}

fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for (i, ch) in s.chars().enumerate() {
        if ch == '_' || ch == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else if i == 0 {
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }

    result
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for ch in s.chars() {
        if ch == '_' || ch == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }

    result
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
            result.push(ch.to_ascii_lowercase());
        } else if ch == '-' {
            result.push('_');
        } else {
            result.push(ch.to_ascii_lowercase());
        }
    }

    result
}

fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('-');
            result.push(ch.to_ascii_lowercase());
        } else if ch == '_' {
            result.push('-');
        } else {
            result.push(ch.to_ascii_lowercase());
        }
    }

    result
}

fn to_screaming_snake_case(s: &str) -> String {
    to_snake_case(s).to_uppercase()
}

#[derive(Debug, Clone)]
pub enum FieldOrder {
    /// Keep declaration order from the struct
    Declaration,
    /// Sort fields alphabetically
    Alphabetical,
    /// Sort fields in reverse alphabetical order
    AlphabeticalReverse,
    /// Custom sorting function
    Custom(fn(&str, &str) -> Ordering),
}

impl FieldOrder {
    pub(crate) fn compare(&self, a: &str, b: &str) -> Ordering {
        match self {
            Self::Declaration => Ordering::Equal, 
            Self::Alphabetical => a.cmp(b),
            Self::AlphabeticalReverse => b.cmp(a),
            Self::Custom(f) => f(a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!(to_camel_case("my_field"), "myField");
        assert_eq!(to_camel_case("my_long_field_name"), "myLongFieldName");
        assert_eq!(to_camel_case("field"), "field");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(to_pascal_case("my_field"), "MyField");
        assert_eq!(to_pascal_case("my_long_field_name"), "MyLongFieldName");
        assert_eq!(to_pascal_case("field"), "Field");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("myField"), "my_field");
        assert_eq!(to_snake_case("MyField"), "my_field");
        assert_eq!(to_snake_case("field"), "field");
    }

    #[test]
    fn test_kebab_case() {
        assert_eq!(to_kebab_case("my_field"), "my-field");
        assert_eq!(to_kebab_case("myField"), "my-field");
        assert_eq!(to_kebab_case("field"), "field");
    }

    #[test]
    fn test_screaming_snake_case() {
        assert_eq!(to_screaming_snake_case("myField"), "MY_FIELD");
        assert_eq!(to_screaming_snake_case("my_field"), "MY_FIELD");
    }

    #[test]
    fn test_field_order_declaration() {
        let order = FieldOrder::Declaration;
        assert_eq!(order.compare("field_a", "field_b"), Ordering::Equal);
        assert_eq!(order.compare("zebra", "apple"), Ordering::Equal);
        assert_eq!(order.compare("foo", "bar"), Ordering::Equal);
    }

    #[test]
    fn test_field_order_alphabetical() {
        let order = FieldOrder::Alphabetical;
        assert_eq!(order.compare("apple", "banana"), Ordering::Less);
        assert_eq!(order.compare("banana", "apple"), Ordering::Greater);
        assert_eq!(order.compare("apple", "apple"), Ordering::Equal);
        assert_eq!(order.compare("aaa", "zzz"), Ordering::Less);
        assert_eq!(order.compare("field_a", "field_b"), Ordering::Less);
        assert_eq!(order.compare("zebra", "apple"), Ordering::Greater);
    }

    #[test]
    fn test_field_order_alphabetical_reverse() {
        let order = FieldOrder::AlphabeticalReverse;
        assert_eq!(order.compare("apple", "banana"), Ordering::Greater);
        assert_eq!(order.compare("banana", "apple"), Ordering::Less);
        assert_eq!(order.compare("apple", "apple"), Ordering::Equal);
        assert_eq!(order.compare("aaa", "zzz"), Ordering::Greater);
        assert_eq!(order.compare("field_a", "field_b"), Ordering::Greater);
        assert_eq!(order.compare("zebra", "apple"), Ordering::Less);
    }
}
