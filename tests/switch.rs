mod test_helper;

use jvm::Result;

use test_helper::run_class;

#[futures_test::test]
async fn test_switch() -> Result<()> {
    let result = run_class("Switch", &["1".into()]).await?;
    assert_eq!(result, "1\n1\n");

    let result = run_class("Switch", &["3".into()]).await?;
    assert_eq!(result, "3\n4\n");

    let result = run_class("Switch", &["10".into()]).await?;
    assert_eq!(result, "10\n");

    let result = run_class("Switch", &["100".into()]).await?;
    assert_eq!(result, "100\n");

    Ok(())
}
