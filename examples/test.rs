use props_util::Properties;
use std::{fs::File, io::Read};

#[derive(Properties, Debug)]
#[allow(unused)]
struct TestProp {
    #[prop(key = "name", default = "Dumeel")]
    name: String,

    #[prop(key = "dept", default = "wms")]
    dept: String,

    #[prop(key = "id", default = "323423")]
    empid: u32,
}

fn main() {
    let test = TestProp::from_file("test.properties").unwrap();
    dbg!(test);
}
