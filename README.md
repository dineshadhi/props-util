[![crates.io](https://img.shields.io/crates/v/props-util)](https://crates.io/crates/props-util)
[![docs.rs](https://img.shields.io/docsrs/props-util)](https://docs.rs/props-util/)

# Props-Util

A Rust library for easily loading and parsing properties files into strongly-typed structs.

## Overview

Props-Util provides a procedural macro that allows you to derive a `Properties` trait for your structs, enabling automatic parsing of properties files into your struct fields. This makes configuration management in Rust applications more type-safe and convenient.

## Features

- Derive macro for automatic properties parsing
- Support for default values
- Type conversion from string to your struct's field types
- Error handling for missing or malformed properties
- Support for both file-based and default initialization

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
props-util = "0.1.0"  # Replace with the actual version
```

## Usage

### Basic Example

1. Define a struct with the `Properties` derive macro:

```rust
use props_util::Properties;

#[derive(Properties, Debug)]
struct Config {
    #[prop(key = "server.host", default = "localhost")]
    host: String,

    #[prop(key = "server.port", default = "8080")]
    port: u16,

    #[prop(key = "debug.enabled", default = "false")]
    debug: bool,
}
```

2. Load properties from a file:

```rust
fn main() -> std::io::Result<()> {
    let config = Config::from_file("config.properties")?;
    println!("Server: {}:{}", config.host, config.port);
    println!("Debug mode: {}", config.debug);
    Ok(())
}
```

3. Create a properties file (e.g., `config.properties`):

```properties
server.host=example.com
server.port=9090
debug.enabled=true
```

### Attribute Parameters

The `#[prop]` attribute accepts the following parameters:

- `key`: The property key to look for in the properties file (optional). If not specified, the field name will be used as the key.
- `default`: A default value to use if the property is not found in the file (optional)

### Field Types

Props-Util supports any type that implements `FromStr`. This includes:

- `String`
- Numeric types (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`)
- Boolean (`bool`)
- `Vec<T>` where `T` implements `FromStr` (values are comma-separated in the properties file)
- `Option<T>` where `T` implements `FromStr` (optional fields that may or may not be present in the properties file)
- Custom types that implement `FromStr`

### Example of using Vec and Option types:

```rust
#[derive(Properties, Debug)]
struct Config {
    #[prop(key = "numbers", default = "1,2,3")]
    numbers: Vec<i32>,
    
    #[prop(key = "strings", default = "hello,world")]
    strings: Vec<String>,

    #[prop(key = "optional_port")]  // No default needed for Option
    optional_port: Option<u16>,

    #[prop(key = "optional_host")]  // No default needed for Option
    optional_host: Option<String>,
}
```

In the properties file:
```properties
numbers=4,5,6,7
strings=test,vec,parsing
optional_port=9090
# optional_host is not set, so it will be None
```

### Converting to and from HashMap

You can convert your struct to a HashMap using the `into_hash_map` method:

```rust
let config = Config::from_file("config.properties")?;
let hashmap = config.into_hash_map();
```

This is particularly useful when you need to:
- Convert between different configuration types
- Store the configuration in a different format
- Pass the configuration to another system that expects a HashMap

When building from a HashMap, the keys and values must be of type `String`:

```rust
use std::collections::HashMap;

let mut props = HashMap::new();
props.insert("server.host".to_string(), "192.168.1.100".to_string());
props.insert("server.port".to_string(), "9999".to_string());
props.insert("debug.enabled".to_string(), "true".to_string());

let config = Config::from_hash_map(&props)?;
```

### Converting Between Different Types

You can use `into_hash_map` to convert between different configuration types. This is particularly useful when you have multiple structs that share similar configuration fields but with different types or structures:

```rust
#[derive(Properties, Debug)]
struct ServerConfig {
    #[prop(key = "host", default = "localhost")]
    host: String,
    #[prop(key = "port", default = "8080")]
    port: u16,
}

#[derive(Properties, Debug)]
struct ClientConfig {
    #[prop(key = "host", default = "localhost")]  // Note: using same key as ServerConfig
    server_host: String,
    #[prop(key = "port", default = "8080")]      // Note: using same key as ServerConfig
    server_port: u16,
}

// Convert from ServerConfig to ClientConfig
let server_config = ServerConfig::from_file("server.properties")?;
let hashmap = server_config.into_hash_map();
let client_config = ClientConfig::from_hash_map(&hashmap)?;
```

> **Important**: When converting between types using `into_hash_map`, the `key` attribute values must match between the source and target types. If no `key` is specified, the field names must match. This ensures that the configuration values are correctly mapped between the different types.

This approach is useful when:
- You need to migrate between different configuration formats
- You have multiple applications that share configuration but use different struct layouts
- You want to transform configuration between different versions of your application

### Error Handling

The `from_file` method returns a `std::io::Result<T>`, which will contain:

- `Ok(T)` if the properties file was successfully parsed
- `Err` with an appropriate error message if:
  - The file couldn't be opened or read
  - A required property is missing (and no default is provided)
  - A property value couldn't be parsed into the expected type
  - The properties file is malformed (e.g., missing `=` character)

### Default Initialization

You can also create an instance with default values without reading from a file:

```rust
let config = Config::default()?;
```

This will use the default values specified in the `#[prop]` attributes.

### Initialization from HashMap

You can also create an instance directly from a `std::collections::HashMap<&str, &str>`:

```rust
use std::collections::HashMap;

let mut props = HashMap::new();
props.insert("server.host", "192.168.1.100");
props.insert("server.port", "9999");
props.insert("debug.enabled", "true");

let config = Config::from_hash_map(&props)?;
```

This method is useful if you already have the configuration data in a HashMap, for example, loaded from a different source or constructed dynamically. Property keys and values are expected to be string slices (`&str`), and type conversion and default values work the same way as `from_file`.

## Advanced Example

Here's a more comprehensive example showing nested configuration:

```rust
use props_util::Properties;

#[derive(Properties, Debug)]
struct AppConfig {
    #[prop(key = "app.name", default = "MyApp")]
    name: String,

    #[prop(key = "app.version", default = "1.0.0")]
    version: String,

    #[prop(key = "database.url", default = "postgres://localhost:5432/mydb")]
    db_url: String,

    #[prop(key = "database.pool_size", default = "10")]
    db_pool_size: u32,

    #[prop(key = "logging.level", default = "info")]
    log_level: String,

    #[prop(key = "logging.file", default = "app.log")]
    log_file: String,

    #[prop(key = "allowed_ips", default = "127.0.0.1,192.168.1.1")]
    allowed_ips: Vec<String>,

    #[prop(key = "ports", default = "80,443,8080")]
    ports: Vec<u16>,

    #[prop(key = "enabled_features")] // Required
    enabled_features: Vec<String>,

    #[prop(key = "optional_ssl_port")] // Optional field
    optional_ssl_port: Option<u16>,
}
```

## Properties File Format

The properties file follows a simple key-value format:

- Each line represents a single property
- The format is `key=value`
- Lines starting with `#` or `!` are treated as comments and ignored
- Empty lines are ignored
- Leading and trailing whitespace around both key and value is trimmed

Example:

```properties
# Application settings
app.name=MyAwesomeApp
app.version=2.1.0

# Database configuration
database.url=postgres://user:pass@localhost:5432/mydb
database.pool_size=20

# Logging settings
logging.level=debug
logging.file=debug.log

# Network settings
allowed_ips=10.0.0.1,10.0.0.2,192.168.0.1
ports=80,443,8080,8443

# Features
enabled_features=ssl,compression,caching

# Optional settings
optional_ssl_port=8443
```