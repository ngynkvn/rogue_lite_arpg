use config_macros::RonDefault;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Copy, Clone, RonDefault)]
#[ron("testdata/test.ron")]
struct RonStruct {
    count: usize,
}

fn main() {}
