// Integration tests for props-util

// Import the derive macro and any other necessary items from the crate
use props_util::Properties;
// Define the struct(s) used for testing within the integration test file
// This struct can now correctly use the Properties macro from the crate.
#[derive(Properties, Debug, PartialEq)]
struct TestConfig {
    #[prop(key = "name", default = "DefaultName")]
    name: String,
    #[prop(key = "dept")] // Required
    dept: String,
    #[prop(key = "id", default = "0")]
    empid: u32,
    #[prop(key = "numeric_test", default = "999")]
    numeric: i64,
    #[prop(key = "bool_test", default = "false")]
    boolean: bool,
    #[prop(key = "spaced.key", default = "")]
    spaced: String,
    #[prop(key = "missing_default", default = "DefaultValue")]
    missing_default: String,
    #[prop(key = "missing_required")] // Required
    missing_required: String,
}

// Helper struct for default test where all fields need defaults
#[derive(Properties, Debug, PartialEq)]
struct DefaultableTestConfig {
    #[prop(key = "name", default = "DefaultName")]
    name: String,
    #[prop(key = "dept", default = "DefaultDept")] // Default added
    dept: String,
    #[prop(key = "id", default = "0")]
    empid: u32,
    #[prop(key = "numeric_test", default = "999")]
    numeric: i64,
    #[prop(key = "bool_test", default = "false")]
    boolean: bool,
    #[prop(key = "spaced.key", default = "")]
    spaced: String,
    #[prop(key = "missing_default", default = "DefaultValue")]
    missing_default: String,
    #[prop(key = "missing_required", default = "DefaultRequiredValue")] // Default added
    missing_required: String,
}

// Test struct for Vec parsing
#[derive(Properties, Debug, PartialEq)]
struct VecTestConfig {
    #[prop(key = "numbers", default = "1,2,3")]
    numbers: Vec<i32>,
    #[prop(key = "strings", default = "hello,world")]
    strings: Vec<String>,
    #[prop(key = "required_vec")] // Required
    required_vec: Vec<u64>,
}
// --- Test Functions ---

#[test]
fn test_from_file_success() {
    // Assumes `examples/test.properties` exists relative to crate root
    // and contains `missing_required=value_added_to_file` (or adjust assertion)
    let config = TestConfig::from_file("examples/test.properties").expect("Failed to load from file");

    assert_eq!(config.name, "TestName");
    assert_eq!(config.dept, "Engineering");
    assert_eq!(config.empid, 123);
    assert_eq!(config.numeric, 456);
    assert!(config.boolean);
    assert_eq!(config.spaced, "spaced value");
    assert_eq!(config.missing_default, "DefaultValue");
    // Adjust this expected value based on your examples/test.properties file
    assert_eq!(config.missing_required, "value_added_to_file");
}

#[test]
fn test_from_file_not_found() {
    let result = TestConfig::from_file("non_existent_file.properties");
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn test_default_initialization() {
    let config = DefaultableTestConfig::default().expect("Default initialization failed");

    assert_eq!(config.name, "DefaultName");
    assert_eq!(config.dept, "DefaultDept");
    assert_eq!(config.empid, 0);
    assert_eq!(config.numeric, 999);
    assert!(!config.boolean);
    assert_eq!(config.spaced, "");
    assert_eq!(config.missing_default, "DefaultValue");
    assert_eq!(config.missing_required, "DefaultRequiredValue");
}

#[test]
fn test_from_hash_map_success() {
    let mut props = std::collections::HashMap::new();
    props.insert("name", "NameFromMap");
    props.insert("dept", "DeptFromMap"); // Required
    props.insert("id", "54321");
    props.insert("numeric_test", "-100");
    props.insert("bool_test", "true");
    props.insert("spaced.key", " spaced map value ");
    props.insert("missing_required", "map_provided"); // Required

    let config = TestConfig::from_hash_map(&props).expect("from_hash_map failed");

    assert_eq!(config.name, "NameFromMap");
    assert_eq!(config.dept, "DeptFromMap");
    assert_eq!(config.empid, 54321);
    assert_eq!(config.numeric, -100);
    assert!(config.boolean);
    assert_eq!(config.spaced, "spaced map value");
    assert_eq!(config.missing_default, "DefaultValue"); // Default used
    assert_eq!(config.missing_required, "map_provided");
}

#[test]
fn test_from_hash_map_uses_defaults() {
    let mut props = std::collections::HashMap::new();
    // Provide only the required fields (those without defaults in TestConfig)
    props.insert("dept", "DeptForDefaults");
    props.insert("missing_required", "RequiredForDefaults");

    let config = TestConfig::from_hash_map(&props).expect("from_hash_map (defaults) failed");

    assert_eq!(config.name, "DefaultName"); // Default
    assert_eq!(config.dept, "DeptForDefaults"); // Provided
    assert_eq!(config.empid, 0); // Default
    assert_eq!(config.numeric, 999); // Default
    assert!(!config.boolean); // Default
    assert_eq!(config.spaced, ""); // Default
    assert_eq!(config.missing_default, "DefaultValue"); // Default
    assert_eq!(config.missing_required, "RequiredForDefaults"); // Provided
}

#[test]
fn test_from_hash_map_missing_required() {
    let mut props = std::collections::HashMap::new();
    props.insert("name", "NameFromMap");
    // "dept" is required in TestConfig and is missing
    props.insert("id", "54321");
    props.insert("missing_required", "provided"); // This one is provided

    let result = TestConfig::from_hash_map(&props);
    assert!(result.is_err());
    // Optionally check the error message
    // assert!(result.err().unwrap().to_string().contains("Missing required property 'dept'"));
}

#[test]
fn test_from_hash_map_parse_error() {
    let mut props = std::collections::HashMap::new();
    props.insert("name", "NameFromMap");
    props.insert("dept", "DeptFromMap");
    props.insert("id", "not_a_number"); // Invalid u32
    props.insert("missing_required", "provided");

    let result = TestConfig::from_hash_map(&props);
    assert!(result.is_err());
    // Optionally check the error message
    // assert!(result.err().unwrap().to_string().contains("Failed to parse value"));
}

#[test]
fn test_vec_parsing() {
    let mut props = std::collections::HashMap::new();
    props.insert("numbers", "4,5,6,7");
    props.insert("strings", "test,vec,parsing");
    props.insert("required_vec", "10,20,30");

    let config = VecTestConfig::from_hash_map(&props).expect("from_hash_map failed");

    assert_eq!(config.numbers, vec![4, 5, 6, 7]);
    assert_eq!(config.strings, vec!["test".to_string(), "vec".to_string(), "parsing".to_string()]);
    assert_eq!(config.required_vec, vec![10, 20, 30]);
}

#[test]
fn test_vec_defaults() {
    let mut props = std::collections::HashMap::new();
    props.insert("required_vec", "1,2,3");

    let config = VecTestConfig::from_hash_map(&props).expect("from_hash_map failed");

    assert_eq!(config.numbers, vec![1, 2, 3]); // Default value
    assert_eq!(config.strings, vec!["hello".to_string(), "world".to_string()]); // Default value
    assert_eq!(config.required_vec, vec![1, 2, 3]);
}

#[test]
fn test_vec_parse_error() {
    let mut props = std::collections::HashMap::new();
    props.insert("numbers", "1,invalid,3");
    props.insert("required_vec", "1,2,3");

    let result = VecTestConfig::from_hash_map(&props);
    assert!(result.is_err());
}
