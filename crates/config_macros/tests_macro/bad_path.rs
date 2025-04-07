use config_macros::LazyRon;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Copy, Clone, LazyRon)]
#[lazy("testdata/test.ro")]
struct RonStruct {
    count: usize,
}

fn main() {}
