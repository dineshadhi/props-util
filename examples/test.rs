use props_util::Properties;
use std::collections::HashMap;

#[derive(Properties, Debug)]
#[allow(unused)]
struct TestProp {
    #[prop(key = "name", default = "Dumeel")]
    name: String,

    #[prop(key = "dept")]
    dept: String,

    #[prop(key = "id")]
    empid: u32,
}

fn main() {
    let mut hm = HashMap::new();
    hm.insert("dept".into(), "zvp".into());
    hm.insert("id".into(), "34".into());
    let test = TestProp::from_hash_map(&hm).unwrap();
    dbg!(test);
}
