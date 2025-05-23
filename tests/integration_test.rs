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
fn basic_parsing() -> anyhow::Result<()> {
    let a = A::default()?;
    assert_eq!(a.option_vec1, None);
    Ok(())
}

#[test]
fn conversion_test() -> anyhow::Result<()> {
    let b = B::from(A::default()?).unwrap();
    assert_eq!(b.name_string, "props-util".to_string());
    assert_eq!(b.option_vec1, Some(vec![1, 2, 3]));
    assert_eq!(b.option_vec2, Some(vec![4, 5, 6]));
    assert_eq!(b.option_vec3, None);
    Ok(())
}

#[test]
fn hash_map_test() -> anyhow::Result<()> {
    let mut hm = HashMap::<String, String>::new();
    hm.insert("name".into(), "hash_map_string".into());
    hm.insert("option_vec1".into(), "4,5,6".into());
    hm.insert("option_vec3".into(), "s1, s2, s3".into());

    let b = B::from(hm)?;
    assert_eq!(b.name_string, "hash_map_string".to_string());
    assert_eq!(b.option_vec1, Some(vec![4, 5, 6]));
    assert_eq!(b.option_vec2, Some(vec![1, 2, 3]));
    assert_eq!(b.option_vec3, Some(vec!["s1".into(), "s2".into(), "s3".into()]));

    Ok(())
}

#[test]
fn file_test() -> anyhow::Result<()> {
    let a = A::from_file("examples/test.properties").unwrap();
    assert_eq!(a.name, "test".to_string());
    assert_eq!(a.option_vec1, Some(vec![8, 9, 10]));
    assert_eq!(a.option_vec2, Some(vec![8, 9, 10]));

    let b = B::from(a)?;
    assert_eq!(b.name_string, "test".to_string());
    assert_eq!(b.option_vec1, Some(vec![8, 9, 10]));
    assert_eq!(b.option_vec2, Some(vec![8, 9, 10]));
    assert_eq!(b.option_vec3, None);

    Ok(())
}

#[derive(Properties)]
struct EnvTest {
    #[prop(env = "NAME", default = "props-util")]
    name: String,
}

#[test]
fn env_test() -> anyhow::Result<()> {
    let t = EnvTest::default()?;
    assert_eq!(t.name, "props-util".to_string());

    unsafe {
        std::env::set_var("NAME", "changed-name");
    }

    let t = EnvTest::default()?;
    assert_eq!(t.name, "changed-name");

    Ok(())
}

#[derive(Properties, Debug)]
struct EnvFailTest {
    #[prop(env = "NAME_FAIL")]
    name: String,
}

#[test]
fn env_fail_test() -> anyhow::Result<()> {
    let t = EnvFailTest::default();
    assert!(t.is_err());

    let t = EnvFailTest::from_file("examples/test.properties")?;
    assert_eq!(t.name, "test".to_string());

    let mut hm = HashMap::<String, String>::new();
    hm.insert("name".into(), "test".into());

    let t = EnvFailTest::from(hm)?;
    assert_eq!(t.name, "test".to_string());

    unsafe {
        std::env::set_var("NAME_FAIL", "changed-name");
    }

    let t = EnvFailTest::default()?;
    assert_eq!(t.name, "changed-name");

    Ok(())
}
