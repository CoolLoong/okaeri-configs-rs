use crate::attrs::ConfigAttrs;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_config_impl(input: &DeriveInput, attrs: &ConfigAttrs) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let metadata_impl = generate_metadata(attrs);
    let apply_env_impl = generate_apply_env(attrs);

    quote! {
        impl #impl_generics ::okaeri_configs::Config for #name #ty_generics #where_clause {
            fn metadata() -> ::okaeri_configs::ConfigMetadata {
                #metadata_impl
            }

            fn apply_env(&mut self) -> ::okaeri_configs::ConfigResult<()> {
                #apply_env_impl
                Ok(())
            }
        }
    }
}

fn generate_metadata(attrs: &ConfigAttrs) -> TokenStream {
    let struct_comments: Vec<&String> = attrs.struct_comments.iter().collect();
    let struct_comments_tokens = quote! {
        vec![#(::std::borrow::Cow::Borrowed(#struct_comments)),*]
    };

    let field_metadata: Vec<TokenStream> = attrs
        .fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let key = if let Some(custom_key) = &field.custom_key {
                quote! { Some(::std::borrow::Cow::Borrowed(#custom_key)) }
            } else {
                quote! { None }
            };
            let comments: Vec<&String> = field.comments.iter().collect();
            let env_var = if let Some(env) = &field.env_var {
                quote! { Some(::std::borrow::Cow::Borrowed(#env)) }
            } else {
                quote! { None }
            };
            let exclude = field.exclude;

            quote! {
                ::okaeri_configs::FieldMetadata {
                    name: ::std::borrow::Cow::Borrowed(#name),
                    key: #key,
                    comments: vec![#(::std::borrow::Cow::Borrowed(#comments)),*],
                    env_var: #env_var,
                    exclude: #exclude,
                }
            }
        })
        .collect();

    quote! {
        ::okaeri_configs::ConfigMetadata {
            struct_comments: #struct_comments_tokens,
            fields: vec![#(#field_metadata),*],
        }
    }
}

fn generate_apply_env(attrs: &ConfigAttrs) -> TokenStream {
    let field_assignments: Vec<TokenStream> = attrs
        .fields
        .iter()
        .filter_map(|field| {
            if let Some(env_var) = &field.env_var {
                let field_name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
                let field_type = &field.ty;

                let is_string_type = matches!(field_type, syn::Type::Path(type_path)
                    if type_path.path.segments.last().map(|s| s.ident == "String").unwrap_or(false));

                if is_string_type {
                    Some(quote! {
                        if let Ok(env_value) = ::std::env::var(#env_var) {
                            self.#field_name = env_value;
                        }
                    })
                } else {
                    Some(quote! {
                        if let Ok(env_value) = ::std::env::var(#env_var) {
                            if let Ok(parsed_value) = env_value.parse::<#field_type>() {
                                self.#field_name = parsed_value;
                            }
                        }
                    })
                }
            } else {
                None
            }
        })
        .collect();

    quote! {
        #(#field_assignments)*
    }
}
