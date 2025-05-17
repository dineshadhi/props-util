//! # Props-Util
//!
//! A Rust library for easily loading and parsing properties files into strongly-typed structs.
//!
//! ## Overview
//!
//! Props-Util provides a procedural macro that allows you to derive a `Properties` trait for your structs,
//! enabling automatic parsing of properties files into your struct fields. This makes configuration
//! management in Rust applications more type-safe and convenient.
//!
//! ## Features
//!
//! - Derive macro for automatic properties parsing
//! - Support for default values
//! - Type conversion from string to your struct's field types
//! - Error handling for missing or malformed properties
//! - Support for both file-based and default initialization
//! - Type conversion between different configuration types
//!
//! ## Usage
//!
//! ### Basic Example
//!
//! ```rust
//! use props_util::Properties;
//! use std::io::Result;
//!
//! #[derive(Properties, Debug)]
//! struct Config {
//!     #[prop(key = "server.host", default = "localhost")]
//!     host: String,
//!
//!     #[prop(key = "server.port", default = "8080")]
//!     port: u16,
//!
//!     #[prop(key = "debug.enabled", default = "false")]
//!     debug: bool,
//! }
//!
//! fn main() -> Result<()> {
//!     // Create a temporary file for testing
//!     let temp_file = tempfile::NamedTempFile::new()?;
//!     std::fs::write(&temp_file, "server.host=example.com\nserver.port=9090\ndebug.enabled=true")?;
//!     
//!     let config = Config::from_file(temp_file.path().to_str().unwrap())?;
//!     println!("Server: {}:{}", config.host, config.port);
//!     println!("Debug mode: {}", config.debug);
//!     Ok(())
//! }
//! ```
//!
//! ### Attribute Parameters
//!
//! The `#[prop]` attribute accepts the following parameters:
//!
//! - `key`: The property key to look for in the properties file (optional). If not specified, the field name will be used as the key.
//! - `default`: A default value to use if the property is not found in the file (optional)
//!
//! ### Field Types
//!
//! Props-Util supports any type that implements `FromStr`. This includes:
//!
//! - `String`
//! - Numeric types (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`)
//! - Boolean (`bool`)
//! - `Vec<T>` where `T` implements `FromStr` (values are comma-separated in the properties file)
//! - `Option<T>` where `T` implements `FromStr` (optional fields that may or may not be present in the properties file)
//! - Custom types that implement `FromStr`
//!
//! ### Example of using Vec and Option types:
//!
//! ```rust
//! use props_util::Properties;
//! use std::io::Result;
//!
//! #[derive(Properties, Debug)]
//! struct Config {
//!     #[prop(key = "numbers", default = "1,2,3")]
//!     numbers: Vec<i32>,
//!     
//!     #[prop(key = "strings", default = "hello,world")]
//!     strings: Vec<String>,
//!
//!     #[prop(key = "optional_port")]  // No default needed for Option
//!     optional_port: Option<u16>,
//!
//!     #[prop(key = "optional_host")]  // No default needed for Option
//!     optional_host: Option<String>,
//! }
//!
//! fn main() -> Result<()> {
//!     // Create a temporary file for testing
//!     let temp_file = tempfile::NamedTempFile::new()?;
//!     std::fs::write(&temp_file, "numbers=4,5,6,7\nstrings=test,vec,parsing\noptional_port=9090")?;
//!     
//!     let config = Config::from_file(temp_file.path().to_str().unwrap())?;
//!     println!("Numbers: {:?}", config.numbers);
//!     println!("Strings: {:?}", config.strings);
//!     println!("Optional port: {:?}", config.optional_port);
//!     println!("Optional host: {:?}", config.optional_host);
//!     Ok(())
//! }
//! ```
//!
//! ### Converting Between Different Types
//!
//! You can use the `from` function to convert between different configuration types. This is particularly useful
//! when you have multiple structs that share similar configuration fields but with different types or structures:
//!
//! ```rust
//! use props_util::Properties;
//! use std::io::Result;
//!
//! #[derive(Properties, Debug)]
//! struct ServerConfig {
//!     #[prop(key = "host", default = "localhost")]
//!     host: String,
//!     #[prop(key = "port", default = "8080")]
//!     port: u16,
//! }
//!
//! #[derive(Properties, Debug)]
//! struct ClientConfig {
//!     #[prop(key = "host", default = "localhost")]  // Note: using same key as ServerConfig
//!     server_host: String,
//!     #[prop(key = "port", default = "8080")]      // Note: using same key as ServerConfig
//!     server_port: u16,
//! }
//!
//! fn main() -> Result<()> {
//!     let server_config = ServerConfig::default()?;
//!     let client_config = ClientConfig::from(server_config)?;
//!     println!("Server host: {}", client_config.server_host);
//!     println!("Server port: {}", client_config.server_port);
//!     Ok(())
//! }
//! ```
//!
//! > **Important**: When converting between types using `from`, the `key` attribute values must match between the source and target types. If no `key` is specified, the field names must match. This ensures that the configuration values are correctly mapped between the different types.
//!
//! ### Error Handling
//!
//! The `from_file` method returns a `std::io::Result<T>`, which will contain:
//!
//! - `Ok(T)` if the properties file was successfully parsed
//! - `Err` with an appropriate error message if:
//!   - The file couldn't be opened or read
//!   - A required property is missing (and no default is provided)
//!   - A property value couldn't be parsed into the expected type
//!   - The properties file is malformed (e.g., missing `=` character)
//!
//! ### Default Initialization
//!
//! You can also create an instance with default values without reading from a file:
//!
//! ```rust
//! use props_util::Properties;
//! use std::io::Result;
//!
//! #[derive(Properties, Debug)]
//! struct Config {
//!     #[prop(key = "server.host", default = "localhost")]
//!     host: String,
//!     #[prop(key = "server.port", default = "8080")]
//!     port: u16,
//! }
//!
//! fn main() -> Result<()> {
//!     let config = Config::default()?;
//!     println!("Host: {}", config.host);
//!     println!("Port: {}", config.port);
//!     Ok(())
//! }
//! ```
//!
//! ## Properties File Format
//!
//! The properties file follows a simple key-value format:
//!
//! - Each line represents a single property
//! - The format is `key=value`
//! - Lines starting with `#` or `!` are treated as comments and ignored
//! - Empty lines are ignored
//! - Leading and trailing whitespace around both key and value is trimmed
//!
//! Example:
//!
//! ```properties
//! # Application settings
//! app.name=MyAwesomeApp
//! app.version=2.1.0
//!
//! # Database configuration
//! database.url=postgres://user:pass@localhost:5432/mydb
//! database.pool_size=20
//!
//! # Logging settings
//! logging.level=debug
//! logging.file=debug.log
//!
//! # Network settings
//! allowed_ips=10.0.0.1,10.0.0.2,192.168.0.1
//! ports=80,443,8080,8443
//!
//! # Features
//! enabled_features=ssl,compression,caching
//!
//! # Optional settings
//! optional_ssl_port=8443
//! ```
//!
//! ## Limitations
//!
//! - Only named structs are supported (not tuple structs or enums)
//! - All fields must have the `#[prop]` attribute
//! - Properties files must use the `key=value` format

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, Field, LitStr, parse_macro_input, punctuated::Punctuated, token::Comma};

/// Derive macro for automatically implementing properties parsing functionality.
///
/// This macro generates implementations for:
/// - `from_file`: Load properties from a file
/// - `from`: Create instance from a type that implements Into<HashMap<String, String>>
/// - `default`: Create instance with default values
///
/// # Example
///
/// ```rust
/// use props_util::Properties;
/// use std::io::Result;
///
/// #[derive(Properties, Debug)]
/// struct Config {
///     #[prop(key = "server.host", default = "localhost")]
///     host: String,
///     #[prop(key = "server.port", default = "8080")]
///     port: u16,
/// }
///
/// fn main() -> Result<()> {
///     let config = Config::default()?;
///     println!("Host: {}", config.host);
///     println!("Port: {}", config.port);
///     Ok(())
/// }
/// ```
#[proc_macro_derive(Properties, attributes(prop))]
pub fn parse_prop_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    match generate_prop_fns(&input) {
        Ok(prop_impl) => quote! {
            impl #struct_name { #prop_impl }

            impl std::convert::Into<std::collections::HashMap<String, String>> for #struct_name {
                fn into(self) -> std::collections::HashMap<String, String> {
                    self.into_hash_map()
                }
            }
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

fn generate_field_init_quote(field_type: &syn::Type, field_name: &proc_macro2::Ident, raw_value_str: proc_macro2::TokenStream, key: LitStr, is_option: bool) -> proc_macro2::TokenStream {
    // Pregenerated token streams to generate values
    let vec_parsing = quote! { Self::parse_vec::<_>(val).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Error Parsing `{}` with value `{}` {}", #key, val, e)))? };
    let parsing = quote! { Self::parse(val).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Error Parsing `{}` with value `{}` {}", #key, val, e)))? };
    let error = quote! { Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("`{}` value is not configured which is required", #key))) };

    match field_type {
        syn::Type::Path(tpath) if tpath.path.segments.last().is_some_and(|segment| segment.ident == "Vec") => match is_option {
            false => quote! {
                #field_name : match #raw_value_str {
                    Some(val) => #vec_parsing,
                    None => return #error
                }
            },
            true => quote! {
                #field_name : match #raw_value_str {
                    Some(val) => Some(#vec_parsing),
                    None => None
                }
            },
        },
        _ => match is_option {
            false => quote! {
                #field_name : match #raw_value_str {
                    Some(val) => #parsing,
                    None => return #error
                }
            },
            true => quote! {
                #field_name : match #raw_value_str {
                    Some(val) => Some(#parsing),
                    None => None
                }
            },
        },
    }
}

fn generate_init_token_streams(fields: Punctuated<Field, Comma>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut init_arr: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in fields {
        let (key, default) = parse_key_default(&field).map_err(|_| Error::new_spanned(field.clone(), "Expecting `key` and `default` values"))?;
        let field_name = field.ident.as_ref().to_owned().unwrap();
        let field_type = &field.ty;

        let val_token_stream = match default {
            Some(default) => quote! { Some(propmap.get(#key).map(String::as_str).unwrap_or(#default)) },
            None => quote! { propmap.get(#key).map(String::as_str) },
        };

        let init = match field_type {
            syn::Type::Path(tpath) if tpath.path.segments.last().is_some_and(|segment| segment.ident == "Option") => match tpath.path.segments.last().unwrap().to_owned().arguments {
                syn::PathArguments::AngleBracketed(arguments) if arguments.args.first().is_some() => match arguments.args.first().unwrap() {
                    syn::GenericArgument::Type(ftype) => generate_field_init_quote(ftype, field_name, val_token_stream, key, true),
                    _ => panic!("Option not configured {field_name} properly"),
                },
                _ => panic!("Option not configured {field_name} properly"),
            },
            _ => generate_field_init_quote(field_type, field_name, val_token_stream, key, false),
        };

        init_arr.push(init);
    }

    Ok(init_arr)
}

fn generate_field_hm_token_stream(key: LitStr, field_type: &syn::Type, field_name: &proc_macro2::Ident, is_option: bool) -> proc_macro2::TokenStream {
    let field_name_str = field_name.to_string();
    match field_type {
        syn::Type::Path(tpath) if tpath.path.segments.last().is_some_and(|segment| segment.ident == "Vec") => match is_option {
            false => quote! {
                // When convert to a hashmap, we insert #filed_name and #key. This will be very helpful
                // when using the resultant Hashmap to construct some other type which may or may not configure key in the props. That type can look up
                // either #key or #field_name whichever it wants to construct its values.
                hm.insert(#field_name_str.to_string() ,self.#field_name.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(","));
                hm.insert(#key.to_string(), self.#field_name.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(","));
            },
            true => quote! {
                if self.#field_name.is_some() {
                    hm.insert(#field_name_str.to_string() ,self.#field_name.clone().unwrap().iter().map(|s| s.to_string()).collect::<Vec<String>>().join(","));
                    hm.insert(#key.to_string() ,self.#field_name.unwrap().iter().map(|s| s.to_string()).collect::<Vec<String>>().join(","));
                }
            },
        },
        _ => match is_option {
            false => quote! {
                hm.insert(#field_name_str.to_string(), self.#field_name.clone().to_string());
                hm.insert(#key.to_string(), self.#field_name.to_string());
            },
            true => quote! {
                if self.#field_name.is_some() {
                    hm.insert(#field_name_str.to_string(), self.#field_name.clone().unwrap().to_string());
                    hm.insert(#key.to_string(), self.#field_name.unwrap().to_string());
                }
            },
        },
    }
}

fn generate_hashmap_token_streams(fields: Punctuated<Field, Comma>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut init_arr: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in fields {
        let (key, _) = parse_key_default(&field).map_err(|_| Error::new_spanned(field.clone(), "Expecting `key` and `default` values"))?;
        let field_name = field.ident.as_ref().to_owned().unwrap();
        let field_type = &field.ty;

        let quote = match field_type {
            syn::Type::Path(tpath) if tpath.path.segments.last().is_some_and(|segment| segment.ident == "Option") => match tpath.path.segments.last().unwrap().to_owned().arguments {
                syn::PathArguments::AngleBracketed(arguments) if arguments.args.first().is_some() => match arguments.args.first().unwrap() {
                    syn::GenericArgument::Type(ftype) => generate_field_hm_token_stream(key, ftype, field_name, true),
                    _ => return Err(Error::new_spanned(field, "Optional {field_name} is not configured properly")),
                },
                _ => return Err(Error::new_spanned(field, "Optional {field_name} not configured properly")),
            },
            _ => generate_field_hm_token_stream(key, field_type, field_name, false),
        };

        init_arr.push(quote);
    }

    Ok(init_arr)
}

fn generate_prop_fns(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = extract_named_fields(input)?;
    let init_arr = generate_init_token_streams(fields.clone())?;
    let ht_arr = generate_hashmap_token_streams(fields)?;

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

        /// Loads properties from a file into an instance of this struct.
        ///
        /// # Example
        ///
        /// ```rust,no_run
        /// use props_util::Properties;
        /// use std::io::Result;
        ///
        /// #[derive(Properties, Debug)]
        /// struct Config {
        ///     #[prop(key = "server.host", default = "localhost")]
        ///     host: String,
        ///
        ///     #[prop(key = "server.port", default = "8080")]
        ///     port: u16,
        ///
        ///     #[prop(key = "debug.enabled", default = "false")]
        ///     debug: bool,
        /// }
        ///
        /// fn main() -> Result<()> {
        ///
        ///     let config = Config::from_file("config.properties")?;
        ///     println!("Server: {}:{}", config.host, config.port);
        ///     println!("Debug mode: {}", config.debug);
        ///     Ok(())
        /// }
        /// ```
        ///
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

        fn into_hash_map(self) -> std::collections::HashMap<String, String> {
            use std::collections::HashMap;
            let mut hm = HashMap::<String, String>::new();
            #( #ht_arr )*
            hm
        }

        /// Convert from another type that implements `Properties` into this type.
        ///
        /// This function uses `into_hash_map` internally to perform the conversion.
        /// The conversion will succeed only if the source type's keys match this type's keys. All the required keys must be present in the source type.
        ///
        ///
        /// # Example
        ///
        /// ```rust,no_run
        /// use props_util::Properties;
        /// use std::io::Result;
        ///
        /// #[derive(Properties, Debug)]
        /// struct ServerConfig {
        ///     #[prop(key = "host", default = "localhost")]
        ///     host: String,
        ///     #[prop(key = "port", default = "8080")]
        ///     port: u16,
        /// }
        ///
        /// #[derive(Properties, Debug)]
        /// struct ClientConfig {
        ///     #[prop(key = "host", default = "localhost")]  // Note: using same key as ServerConfig
        ///     server_host: String,
        ///     #[prop(key = "port", default = "8080")]      // Note: using same key as ServerConfig
        ///     server_port: u16,
        /// }
        ///
        /// fn main() -> Result<()> {
        ///     let server_config = ServerConfig::default()?;
        ///     let client_config = ClientConfig::from(server_config)?;
        ///     println!("Server host: {}", client_config.server_host);
        ///     println!("Server port: {}", client_config.server_port);
        ///     Ok(())
        /// }
        /// ```
        pub fn from<T>(other: T) -> std::io::Result<Self>
        where
            T: Into<std::collections::HashMap<String, String>>
        {
            let propmap = other.into();
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
        match () {
            _ if meta.path.is_ident("key") => match key {
                Some(_) => return Err(meta.error("duplicate 'key' parameter")),
                None => key = Some(meta.value()?.parse()?),
            },
            _ if meta.path.is_ident("default") => match default {
                Some(_) => return Err(meta.error("duplicate 'default' parameter")),
                None => default = Some(meta.value()?.parse()?),
            },
            _ => return Err(meta.error(format!("unrecognized parameter '{}' in #[prop] attribute", meta.path.get_ident().map(|i| i.to_string()).unwrap_or_else(|| "<?>".into())))),
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
