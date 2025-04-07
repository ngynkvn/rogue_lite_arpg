use config_macros::LazyRon;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Copy, Clone, LazyRon)]
#[lazy("testdata/test.ron")]
struct RonStruct {
    wrongname: usize,
}

#[test]
#[should_panic]
fn main() {
    RonStruct::default();
}
