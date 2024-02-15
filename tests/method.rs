mod test_helper;

use jvm::JvmResult;

use test_helper::run_class;

#[futures_test::test]
async fn test_method() -> JvmResult<()> {
    let method = include_bytes!("../test_data/Method.class");

    let result = run_class("Method", &[("Method", method)], &[]).await?;
    assert_eq!(result, "1\n2\n3\n");

    Ok(())
}
