#[test]
fn feature_check() {
    assert_eq!(size_of::<exclusive_feature::int>(), size_of::<i64>());
}