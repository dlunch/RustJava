mod test_helper;

use test_helper::run_class;

#[futures_test::test]
async fn test_multi_array() -> anyhow::Result<()> {
    let class = include_bytes!("../test_data/MultiArray.class");

    let result = run_class("MultiArray", &[("MultiArray", class)], &[]).await?;
    assert_eq!(result, "test1\ntest2\ntest3\nnull\ntest4\n");

    Ok(())
}
