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

- `key`: The property key to look for in the properties file (required)
- `default`: A default value to use if the property is not found in the file (optional)

### Field Types

Props-Util supports any type that implements `FromStr`. This includes:

- `String`
- Numeric types (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`)
- Boolean (`bool`)
- Custom types that implement `FromStr`

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
```

## Limitations

- Only named structs are supported (not tuple structs or enums)
- All fields must have the `#[prop]` attribute
- The `key` parameter is required for all fields
- Properties files must use the `key=value` format

