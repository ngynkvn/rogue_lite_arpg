use config_macros::RonDefault;
use serde::Deserialize;

#[test]
fn ron_default() {
    #[derive(Deserialize, Copy, Clone, RonDefault)]
    #[ron("testdata/test.ron")]
    struct Rons {
        count: usize,
    }
    assert_eq!(Rons::default().count, 5);
}
