use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub struct ConfigMetadata {
    pub struct_comments: Vec<Cow<'static, str>>,
    pub fields: Vec<FieldMetadata>,
}

#[derive(Debug, Clone)]
pub struct FieldMetadata {
    pub name: Cow<'static, str>,
    pub key: Option<Cow<'static, str>>,
    pub comments: Vec<Cow<'static, str>>,
    pub env_var: Option<Cow<'static, str>>,
    pub exclude: bool,
}

#[allow(dead_code)]
impl FieldMetadata {
    pub(crate) fn new(name: impl Into<Cow<'static, str>>) -> Self {
        let name = name.into();
        Self {
            name,
            key: None,
            comments: Vec::new(),
            env_var: None,
            exclude: false,
        }
    }
    pub(crate) fn with_key(mut self, key: impl Into<Cow<'static, str>>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub(crate) fn with_comment(mut self, comment: impl Into<Cow<'static, str>>) -> Self {
        self.comments.push(comment.into());
        self
    }
    pub(crate) fn with_comments(mut self, comments: Vec<Cow<'static, str>>) -> Self {
        self.comments = comments;
        self
    }
    pub(crate) fn with_env_var(mut self, env_var: impl Into<Cow<'static, str>>) -> Self {
        self.env_var = Some(env_var.into());
        self
    }
    pub(crate) fn with_exclude(mut self, exclude: bool) -> Self {
        self.exclude = exclude;
        self
    }
}
