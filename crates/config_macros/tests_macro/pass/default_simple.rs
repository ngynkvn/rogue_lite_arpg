use config_macros::DefaultRon;
use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, DefaultRon)]
#[ron("../../../../testdata/test.ron")]
struct Rons {
    count: usize,
}

fn main() {}
