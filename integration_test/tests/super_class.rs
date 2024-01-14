use integration_test::run_class;

// #[futures_test::test]
async fn test_superclass() -> anyhow::Result<()> {
    let super_class = include_bytes!("../../test_data/SuperClass.class");
    let inner = include_bytes!("../../test_data/SuperClass$InnerClass.class");
    let inner_derived = include_bytes!("../../test_data/SuperClass$InnerDerivedClass.class");

    let result = run_class(
        "SuperClass",
        &[
            ("SuperClass", super_class),
            ("SuperClass$InnerClass", inner),
            ("SuperClass$InnerDerivedClass", inner_derived),
        ],
        &[],
    )
    .await?;
    assert_eq!(result, "2\n1234123412341234\n2\ntest\ntest\n");

    Ok(())
}
