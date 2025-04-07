use config_macros::DefaultRon;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Copy, Clone, DefaultRon)]
#[ron("testdata/test.ro")]
struct RonStruct {
    count: usize,
}

fn main() {}
