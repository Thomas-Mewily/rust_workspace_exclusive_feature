#[allow(unused)]
use exclusive_feature::int;

fn main() {}

#[test]
fn feature_check() {
    assert_eq!(size_of::<int>(), size_of::<i64>());
}