extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, LitStr, parse_macro_input};

#[proc_macro_derive(Properties, attributes(prop))]
pub fn parse_prop_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let prop_impl = match implement_new_prop(&input) {
        Ok(prop) => prop,
        Err(e) => return e.to_compile_error().into(),
    };

    let expanded = quote! {
        impl #struct_name {
            #prop_impl
        }
    };

    TokenStream::from(expanded)
}

fn implement_new_prop(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => &fields_named.named,
            _ => return Err(Error::new_spanned(&input.ident, "Only named structs are allowd")),
        },
        _ => return Err(Error::new_spanned(&input.ident, "Only structs can be used on Properties")),
    };

    let mut field_initialisers: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in fields {
        let (key, default) = match parse_prop_attribute(field) {
            Ok(val) => val,
            Err(_) => return Err(Error::new_spanned(field, "Expecting `key` and `default` values")),
        };

        let field_name = field.ident.as_ref().to_owned().unwrap();
        let field_type = &field.ty;

        let raw_value_str = match default {
            Some(default) => {
                quote! { propmap.get(#key).map(String::as_str).unwrap_or(#default) }
            }
            None => {
                quote! {
                    match propmap.get(#key).map(String::as_str) {
                        Some(val) => val,
                        None => panic!("`{}` value is not configured. Use default or configure in the .properties file", #key),
                    }
                }
            }
        };

        let initializer = quote! {
             #field_name : {
                let raw_value_str = #raw_value_str;
                raw_value_str.parse::<#field_type>().map_err(|e|
                    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Error Parsing `{}` with value `{}` {}", #key, raw_value_str, e))
                )?
            }
        };

        field_initialisers.push(initializer);
    }

    let new_impl = quote! {
        pub fn new(path : &str) -> std::io::Result<Self> {
            // Required imports within the generated function
            use std::collections::HashMap;
            use std::fs;
            use std::io::{self, ErrorKind}; // Explicitly import ErrorKind
            use std::path::Path; // Required for AsRef<Path> trait bound

            let mut content = String::new();

            let mut file = File::open(path).map_err(|e| std::io::Error::new(e.kind(), format!("Error opening file {}", path)))?;
            file.read_to_string(&mut content) .map_err(|e| std::io::Error::new(e.kind(), format!("Error Reading File : {}", path)))?;

            let mut propmap = HashMap::<String, String>::new();
            for (line_num, line) in content.lines().enumerate() {
                let trimmed_line = line.trim();

                if trimmed_line.is_empty() || trimmed_line.starts_with('#') || trimmed_line.starts_with('!') {
                    continue;
                }

                // Find the first '=', handling potential whitespace
                match trimmed_line.split_once('=') {
                    Some((key, value)) => propmap.insert(key.trim().to_string(), value.trim().to_string()),
                    None => return Err(io::Error::new( ErrorKind::InvalidData, format!("Malformed line {} in '{}' (missing '='): {}", line_num + 1, path, line) )),
                };
            }

            Ok(Self {
                #( #field_initialisers ),*
            })
        }
    };

    Ok(proc_macro2::TokenStream::from(new_impl))
}

fn parse_prop_attribute(field: &syn::Field) -> syn::Result<(LitStr, Option<LitStr>)> {
    let prop_attr = field.attrs.iter().find(|attr| attr.path().is_ident("prop")).ok_or_else(|| {
        syn::Error::new_spanned(
            field.ident.as_ref().unwrap(),
            format!(
                "Field '{}' is missing the #[prop(...)] attribute",
                field.ident.as_ref().map(|i| i.to_string()).unwrap_or_else(|| "<?>".into())
            ),
        )
    })?;

    let mut key: Option<LitStr> = None;
    let mut default: Option<LitStr> = None;

    // Use parse_nested_meta for more robust parsing of attribute arguments
    prop_attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("key") {
            if key.is_some() {
                return Err(meta.error("duplicate 'key' parameter"));
            }
            key = Some(meta.value()?.parse()?); // value()? gets the = LitStr part
        } else if meta.path.is_ident("default") {
            if default.is_some() {
                return Err(meta.error("duplicate 'default' parameter"));
            }
            default = Some(meta.value()?.parse()?);
        } else {
            return Err(meta.error(format!(
                "unrecognized parameter '{}' in #[prop] attribute",
                meta.path.get_ident().map(|i| i.to_string()).unwrap_or_else(|| "<?>".into())
            )));
        }
        Ok(())
    })?;

    // Check if both key and default were found
    let key_str = key.ok_or_else(|| syn::Error::new_spanned(prop_attr, "Missing 'key' parameter in #[prop] attribute"))?;

    Ok((key_str, default))
}
