extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, Field, LitStr, parse_macro_input, punctuated::Punctuated, token::Comma};

#[proc_macro_derive(Properties, attributes(prop))]
pub fn parse_prop_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    match generate_prop_fns(&input) {
        Ok(prop_impl) => quote! {
            impl #struct_name { #prop_impl }
        }
        .into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn extract_named_fields(input: &DeriveInput) -> syn::Result<Punctuated<Field, Comma>> {
    let fields = match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => &fields_named.named,
            _ => return Err(Error::new_spanned(&input.ident, "Only named structs are allowd")),
        },
        _ => return Err(Error::new_spanned(&input.ident, "Only structs can be used on Properties")),
    };

    Ok(fields.to_owned())
}

fn generate_result_quote(field_type: &syn::Type, field_name: &proc_macro2::Ident, raw_value_str: proc_macro2::TokenStream, key: LitStr, is_option: bool) -> proc_macro2::TokenStream {
    match field_type {
        syn::Type::Path(tpath) if tpath.path.segments.last().is_some_and(|segment| segment.ident == "Vec") => match is_option {
            false => quote! {
                #field_name : match #raw_value_str {
                    Some(val) => Self::parse_vec::<_>(val).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Error Parsing `{}` with value `{}` {}", #key, val, e)))?,
                    None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("`{}` value is not configured. Use default or set it in the .properties file", #key)))
                }
            },
            true => quote! {
                #field_name : match #raw_value_str {
                    Some(val) => Some(Self::parse_vec::<_>(val).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Error Parsing `{}` with value `{}` {}", #key, val, e)))?),
                    None => None
                }
            },
        },
        _ => match is_option {
            false => quote! {
                #field_name : match #raw_value_str {
                    Some(val) => Self::parse(val).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Error Parsing `{}` with value `{}` {}", #key, val, e)))?,
                    None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("`{}` value is not configured. Use default or set it in the .properties file", #key)))
                }
            },
            true => quote! {
                #field_name : match #raw_value_str {
                    Some(val) => Some(Self::parse(val).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Error Parsing `{}` with value `{}` {}", #key, val, e)))?),
                    None => None
                }
            },
        },
    }
}

fn generate_initalizers(fields: Punctuated<Field, Comma>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut init_arr: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in fields {
        let (key, default) = parse_key_default(&field).map_err(|_| Error::new_spanned(field.clone(), "Expecting `key` and `default` values"))?;
        let field_name = field.ident.as_ref().to_owned().unwrap();
        let field_type = &field.ty;

        let raw_value_str = match default {
            Some(default) => quote! { Some(propmap.get(#key).map(String::as_str).unwrap_or(#default)) },
            None => quote! { propmap.get(#key).map(String::as_str) },
        };

        let init = match field_type {
            syn::Type::Path(tpath) if tpath.path.segments.last().is_some_and(|segment| segment.ident == "Option") => match tpath.path.segments.last().unwrap().to_owned().arguments {
                syn::PathArguments::AngleBracketed(arguments) if arguments.args.first().is_some() => match arguments.args.first().unwrap() {
                    syn::GenericArgument::Type(ftype) => generate_result_quote(ftype, field_name, raw_value_str, key, true),
                    _ => panic!("Option not configured {field_name} properly"),
                },
                _ => panic!("Option not configured {field_name} properly"),
            },
            _ => generate_result_quote(field_type, field_name, raw_value_str, key, false),
        };

        init_arr.push(init);
    }

    Ok(init_arr)
}

fn generate_prop_fns(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = extract_named_fields(input)?;
    let init_arr = generate_initalizers(fields)?;

    let new_impl = quote! {

        fn parse_vec<T: std::str::FromStr>(string: &str) -> anyhow::Result<Vec<T>> {
            Ok(string
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<T>().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Error Parsing with value `{s}`"))))
                .collect::<std::io::Result<Vec<T>>>()?)
        }

        fn parse<T : std::str::FromStr>(string : &str) -> anyhow::Result<T> {
            Ok(string.parse::<T>().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Error Parsing with value `{string}`")))?)
        }

        pub fn from_file(path : &str) -> std::io::Result<Self> {
            use std::collections::HashMap;
            use std::fs;
            use std::io::{self, ErrorKind}; // Explicitly import ErrorKind
            use std::path::Path; // Required for AsRef<Path> trait bound
            use std::{fs::File, io::Read};

            let mut content = String::new();

            let mut file = File::open(path).map_err(|e| std::io::Error::new(e.kind(), format!("Error opening file {}", path)))?;
            file.read_to_string(&mut content) .map_err(|e| std::io::Error::new(e.kind(), format!("Error Reading File : {}", path)))?;

            let mut propmap = std::collections::HashMap::<String, String>::new();
            for (line_num, line) in content.lines().enumerate() {
                let line = line.trim();

                if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
                    continue;
                }

                // Find the first '=', handling potential whitespace
                match line.split_once('=') {
                    Some((key, value)) => propmap.insert(key.trim().to_string(), value.trim().to_string()),
                    None => return Err(io::Error::new( ErrorKind::InvalidData, format!("Malformed line {} in '{}' (missing '='): {}", line_num + 1, path, line) )),
                };
            }

            Ok(Self { #( #init_arr ),* })
        }

        pub fn from_hash_map(propmap : &std::collections::HashMap<&str, &str>) -> std::io::Result<Self> {
            let propmap : std::collections::HashMap<String, String> = propmap.iter().map(|(k, v)| (k.trim().to_string(), v.trim().to_string())).collect();
            Ok(Self { #( #init_arr ),* })
        }

        pub fn default() -> std::io::Result<Self> {
            use std::collections::HashMap;
            let mut propmap = HashMap::<String, String>::new();
            Ok(Self { #( #init_arr ),* })
        }
    };

    Ok(new_impl)
}

fn parse_key_default(field: &syn::Field) -> syn::Result<(LitStr, Option<LitStr>)> {
    let prop_attr = field.attrs.iter().find(|attr| attr.path().is_ident("prop"));
    let prop_attr = match prop_attr {
        Some(attr) => attr,
        None => {
            // If there is no "prop" attr, simply return the field name with None default
            let ident = field.ident.to_owned().unwrap();
            let key = LitStr::new(&ident.to_string(), ident.span());
            return Ok((key, None));
        }
    };

    let mut key: Option<LitStr> = None;
    let mut default: Option<LitStr> = None;

    // parse the metadata to find `key` and `default` values
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
            return Err(meta.error(format!("unrecognized parameter '{}' in #[prop] attribute", meta.path.get_ident().map(|i| i.to_string()).unwrap_or_else(|| "<?>".into()))));
        }
        Ok(())
    })?;

    // if there is no key, simple use the ident field name
    let key_str = match key {
        Some(key) => key,
        None => match field.ident.to_owned() {
            Some(key) => LitStr::new(&key.to_string(), key.span()),
            None => return Err(syn::Error::new_spanned(prop_attr, "Missing 'key' parameter in #[prop] attribute")),
        },
    };

    Ok((key_str, default))
}
