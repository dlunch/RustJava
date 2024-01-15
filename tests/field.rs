mod test_helper;

use test_helper::run_class;

#[futures_test::test]
async fn test_field() -> anyhow::Result<()> {
    let field = include_bytes!("../test_data/Field.class");

    let result = run_class("Field", &[("Field", field)], &[]).await?;
    assert_eq!(result, "1\ntest1\n1234\n");

    Ok(())
}
