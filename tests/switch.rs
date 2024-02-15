mod test_helper;

use jvm::JvmResult;

use test_helper::run_class;

#[futures_test::test]
async fn test_switch() -> JvmResult<()> {
    let class = include_bytes!("../test_data/Switch.class");

    let result = run_class("Switch", &[("Switch", class)], &["1".into()]).await?;
    assert_eq!(result, "1\n1\n");

    let result = run_class("Switch", &[("Switch", class)], &["3".into()]).await?;
    assert_eq!(result, "3\n4\n");

    let result = run_class("Switch", &[("Switch", class)], &["10".into()]).await?;
    assert_eq!(result, "10\n");

    let result = run_class("Switch", &[("Switch", class)], &["100".into()]).await?;
    assert_eq!(result, "100\n");

    Ok(())
}
