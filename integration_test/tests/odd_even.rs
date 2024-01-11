use integration_test::run_class;

#[futures_test::test]
pub async fn test_odd_even() -> anyhow::Result<()> {
    let odd_even = include_bytes!("../../test_data/OddEven.class");

    let result = run_class("OddEven", odd_even, &["1234"]).await?;
    assert_eq!(result, "i is even\n");

    let result = run_class("OddEven", odd_even, &["1233"]).await?;
    assert_eq!(result, "i is odd\n");

    Ok(())
}
