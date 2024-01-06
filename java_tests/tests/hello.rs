use java_tests::run_class;

#[futures_test::test]
async fn test_hello() -> anyhow::Result<()> {
    let hello = include_bytes!("../../test_data/Hello.class");

    let result = run_class("Hello", hello, &[]).await?;
    assert_eq!(result, "Hello, world!\n");

    Ok(())
}
