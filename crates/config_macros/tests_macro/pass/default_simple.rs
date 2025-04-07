use config_macros::LazyRon;
use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, LazyRon)]
#[lazy("../../../../testdata/test.ron")]
struct Rons {
    count: usize,
}

fn main() {}
