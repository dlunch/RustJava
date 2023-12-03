use jvm_tests::run_class;

// #[test]
pub fn test_odd_even() -> anyhow::Result<()> {
    let odd_even = include_bytes!("../../test_data/OddEven.class");

    let result = run_class("OddEven", odd_even, &["1234"])?;
    assert_eq!(result, "i is even\n");

    let result = run_class("OddEven", odd_even, &["1233"])?;
    assert_eq!(result, "i is odd\n");

    Ok(())
}
