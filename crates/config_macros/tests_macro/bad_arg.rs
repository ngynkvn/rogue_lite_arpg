use config_macros::DefaultRon;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Copy, Clone, DefaultRon)]
#[ron(1)]
struct RonStruct {
    count: usize,
}

fn main() {}
