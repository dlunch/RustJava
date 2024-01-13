use integration_test::run_class;

#[futures_test::test]
async fn test_control_flow() -> anyhow::Result<()> {
    let control_flo = include_bytes!("../../test_data/ControlFlow.class");

    let result = run_class("ControlFlow", &[("ControlFlow", control_flo)], &[]).await?;
    assert_eq!(
        result,
        "5\n16\n8\n4\n2\n1\n0\n1\n2\n3\n4\n5\n6\n7\n8\n9\na is not null\nx < 10\nx is true\n"
    );

    Ok(())
}
