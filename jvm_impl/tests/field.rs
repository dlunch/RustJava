use test_utils::run_class;

// #[futures_test::test]
async fn test_field() -> anyhow::Result<()> {
    let field = include_bytes!("../../test_data/Field.class");

    let result = run_class("test_data", field, &[]).await?;
    assert_eq!(result, "1\ntest1\n");

    Ok(())
}
