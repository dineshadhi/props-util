use std::collections::HashMap;

use props_util::Properties;

#[derive(Properties)]
struct A {
    #[prop(default = "props-util")]
    name: String,
    option_vec1: Option<Vec<u32>>, // For none check
    #[prop(default = "4, 5, 6")]
    option_vec2: Option<Vec<u32>>, // For none check
    option_vec3: Option<Vec<String>>,
}

#[derive(Properties)]
struct B {
    #[prop(key = "name")]
    name_string: String,
    #[prop(default = "1,2,3")]
    option_vec1: Option<Vec<u32>>,
    #[prop(default = "1, 2, 3")]
    option_vec2: Option<Vec<u32>>, // For none check
    option_vec3: Option<Vec<String>>,
}

#[test]
fn basic_parsing() {
    let a = A::default().unwrap();
    assert_eq!(a.option_vec1, None);
}

#[test]
fn conversion_test() {
    let b = B::from_hash_map(&A::default().unwrap().into_hash_map()).unwrap();
    assert_eq!(b.name_string, "props-util".to_string());
    assert_eq!(b.option_vec1, Some(vec![1, 2, 3]));
    assert_eq!(b.option_vec2, Some(vec![4, 5, 6]));
    assert_eq!(b.option_vec3, None);
}

#[test]
fn hash_map_test() {
    let mut hm = HashMap::<String, String>::new();
    hm.insert("name".into(), "hash_map_string".into());
    hm.insert("option_vec1".into(), "4,5,6".into());
    hm.insert("option_vec3".into(), "s1, s2, s3".into());

    let b = B::from_hash_map(&hm).unwrap();
    assert_eq!(b.name_string, "hash_map_string".to_string());
    assert_eq!(b.option_vec1, Some(vec![4, 5, 6]));
    assert_eq!(b.option_vec2, Some(vec![1, 2, 3]));
    assert_eq!(b.option_vec3, Some(vec!["s1".into(), "s2".into(), "s3".into()]))
}

#[test]
fn file_test() {
    let a = A::from_file("examples/test.properties").unwrap();
    assert_eq!(a.name, "test".to_string());
    assert_eq!(a.option_vec1, Some(vec![8, 9, 10]));
    assert_eq!(a.option_vec2, Some(vec![8, 9, 10]));

    let b = B::from_hash_map(&a.into_hash_map()).unwrap();
    assert_eq!(b.name_string, "test".to_string());
    assert_eq!(b.option_vec1, Some(vec![8, 9, 10]));
    assert_eq!(b.option_vec2, Some(vec![8, 9, 10]));
    assert_eq!(b.option_vec3, None);
}
