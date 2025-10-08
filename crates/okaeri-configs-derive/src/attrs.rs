use syn::{
    Attribute, Data, DeriveInput, Error, Expr, ExprLit, Fields, Lit, Meta, Result, spanned::Spanned,
};

#[derive(Debug, Default)]
pub struct ConfigAttrs {
    pub struct_comments: Vec<String>,
    pub fields: Vec<FieldAttrs>,
}

#[derive(Clone)]
pub struct FieldAttrs {
    pub name: String,
    pub ty: syn::Type,
    pub comments: Vec<String>,
    pub custom_key: Option<String>,
    pub env_var: Option<String>,
    pub exclude: bool,
}

impl std::fmt::Debug for FieldAttrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldAttrs")
            .field("name", &self.name)
            .field("ty", &"<syn::Type>")
            .field("comments", &self.comments)
            .field("custom_key", &self.custom_key)
            .field("env_var", &self.env_var)
            .field("exclude", &self.exclude)
            .finish()
    }
}

impl ConfigAttrs {
    pub fn from_derive_input(input: &DeriveInput) -> Result<Self> {
        let mut attrs = ConfigAttrs::default();

        // struct-level
        for attr in &input.attrs {
            if attr.path().is_ident("comment") {
                attrs.parse_struct_comment(attr)?;
            }
        }

        // field-level
        if let Data::Struct(data) = &input.data {
            if let Fields::Named(fields) = &data.fields {
                for field in &fields.named {
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    let field_type = field.ty.clone();
                    let mut field_attrs = FieldAttrs::new(field_name, field_type);

                    for attr in &field.attrs {
                        if attr.path().is_ident("comment") {
                            field_attrs.parse_comment(attr)?;
                        } else if attr.path().is_ident("env") {
                            field_attrs.parse_env(attr)?;
                        } else if attr.path().is_ident("key") {
                            field_attrs.parse_key(attr)?;
                        } else if attr.path().is_ident("serde") {
                            if let Meta::List(list) = &attr.meta {
                                if list.tokens.to_string().contains("skip") {
                                    field_attrs.exclude = true;
                                }
                            }
                        }
                    }

                    attrs.fields.push(field_attrs);
                }
            }
        }

        Ok(attrs)
    }

    fn parse_struct_comment(&mut self, attr: &Attribute) -> Result<()> {
        if let Meta::List(list) = &attr.meta {
            let value: Expr = syn::parse2(list.tokens.clone())?;
            if let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = value
            {
                self.struct_comments.push(s.value());
                return Ok(());
            }
        }
        Err(Error::new(attr.span(), "expected #[comment(\"...\")]"))
    }
}

impl FieldAttrs {
    pub fn new(name: String, ty: syn::Type) -> Self {
        Self {
            name,
            ty,
            comments: Vec::new(),
            custom_key: None,
            env_var: None,
            exclude: false,
        }
    }

    fn parse_comment(&mut self, attr: &Attribute) -> Result<()> {
        if let Meta::List(list) = &attr.meta {
            let value: Expr = syn::parse2(list.tokens.clone())?;
            if let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = value
            {
                self.comments.push(s.value());
                return Ok(());
            }
        }
        Err(Error::new(attr.span(), "expected #[comment(\"...\")]"))
    }

    fn parse_env(&mut self, attr: &Attribute) -> Result<()> {
        if let Meta::List(list) = &attr.meta {
            let value: Expr = syn::parse2(list.tokens.clone())?;
            if let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = value
            {
                self.env_var = Some(s.value());
                return Ok(());
            }
        }
        Err(Error::new(attr.span(), "expected #[env(\"VAR\")]"))
    }

    fn parse_key(&mut self, attr: &Attribute) -> Result<()> {
        if let Meta::List(list) = &attr.meta {
            let value: Expr = syn::parse2(list.tokens.clone())?;
            if let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = value
            {
                self.custom_key = Some(s.value());
                return Ok(());
            }
        }
        Err(Error::new(attr.span(), "expected #[key(\"name\")]"))
    }
}
