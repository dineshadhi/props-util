use props_util::Properties;

#[derive(Properties, Debug)]
#[allow(unused)]
struct TestProp {
    #[prop(key = "name", default = "Dumeel")]
    name: String,

    #[prop(key = "dept", default = "wms")]
    dept: String,

    #[prop(key = "id")]
    empid: u32,
}

fn main() {
    let test = TestProp::default().unwrap();
    dbg!(test);
}
