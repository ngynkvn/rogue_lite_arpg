use config_macros::LazyRon;
use serde::Deserialize;

#[derive(LazyRon, Debug, Deserialize, Copy, Clone)]
#[lazy("testdata/test.ron")]
struct RonStruct {
    cout: usize,
}

fn main() {
    println!("{:?}", RonStruct::default().cout);
}
