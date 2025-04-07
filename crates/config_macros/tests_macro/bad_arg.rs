use config_macros::LazyRon;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Copy, Clone, LazyRon)]
#[lazy(1)]
struct RonStruct {
    count: usize,
}

fn main() {}
