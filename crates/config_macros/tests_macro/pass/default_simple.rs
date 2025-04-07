use config_macros::RonDefault;
use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, RonDefault)]
#[ron("../../../../testdata/test.ron")]
struct Rons {
    count: usize,
}

fn main() {}
