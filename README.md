[![crates.io](https://img.shields.io/crates/v/props-util)](https://crates.io/crates/props-util)
[![docs.rs](https://img.shields.io/docsrs/props-util)](https://docs.rs/props-util/)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/dineshadhi/props-util)

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
- `env`: The environment variable name to look for (optional). If the environment variable is set, its value will be used instead of the value from the properties file.

### Example of using environment variables:

```rust
use props_util::Properties;
use std::io::Result;

#[derive(Properties, Debug)]
struct Config {
    #[prop(key = "server.host", env = "SERVER_HOST", default = "localhost")]
    host: String,

    #[prop(key = "server.port", env = "SERVER_PORT", default = "8080")]
    port: u16,

    #[prop(key = "api.key", env = "API_KEY")]  // No default, must be set in env or props file
    api_key: String,
}

fn main() -> Result<()> {
    // Set environment variables for testing
    std::env::set_var("SERVER_HOST", "env.example.com");
    std::env::set_var("SERVER_PORT", "9090");
    
    // Create a properties file with different values
    let temp_file = tempfile::NamedTempFile::new()?;
    std::fs::write(&temp_file, "server.host=file.example.com\nserver.port=8080\napi.key=test123")?;
    
    let config = Config::from_file(temp_file.path().to_str().unwrap())?;
    
    // Environment variables take precedence over file values
    println!("Host: {}", config.host);  // Will print "env.example.com"
    println!("Port: {}", config.port);  // Will print "9090"
    println!("API Key: {}", config.api_key);  // Will print "test123" (from file)
    
    Ok(())
}
```

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


### Converting Between Different Types

You can use the `from` function to convert between different configuration types. This is particularly useful when you have multiple structs that share similar configuration fields but with different types or structures:

```rust
use props_util::Properties;
use std::io::Result;

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

fn main() -> Result<()> {
    // Create a temporary file for testing
    let temp_file = tempfile::NamedTempFile::new()?;
    std::fs::write(&temp_file, "host=example.com\nport=9090")?;
    
    // Convert from ServerConfig to ClientConfig using the from function
    let server_config = ServerConfig::from_file(temp_file.path().to_str().unwrap())?;
    let client_config = ClientConfig::from(server_config)?;
    
    println!("Server host: {}", client_config.server_host);
    println!("Server port: {}", client_config.server_port);
    Ok(())
}
```

> **Important**: When converting between types using `from`, the `key` attribute values must match between the source and target types. If no `key` is specified, the field names must match. This ensures that the configuration values are correctly mapped between the different types.

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
