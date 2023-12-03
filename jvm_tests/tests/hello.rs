use jvm_tests::run_class;

#[test]
fn test_hello() -> anyhow::Result<()> {
    let hello = include_bytes!("../../test_data/Hello.class");

    let result = run_class("Hello", hello, &[])?;
    assert_eq!(result, "Hello, world!\n");

    Ok(())
}
