mod test_helper;

use jvm::Result;

use test_helper::run_class;

#[futures_test::test]
pub async fn test_odd_even() -> Result<()> {
    let odd_even = include_bytes!("../test_data/OddEven.class");

    let result = run_class("OddEven", &[("OddEven", odd_even)], &["1234".into()]).await?;
    assert_eq!(result, "i is even\n");

    let result = run_class("OddEven", &[("OddEven", odd_even)], &["1233".into()]).await?;
    assert_eq!(result, "i is odd\n");

    Ok(())
}
