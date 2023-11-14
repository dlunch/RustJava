use classfile::parse_class;

#[test]
fn test_hello() -> anyhow::Result<()> {
    let hello = include_bytes!("../../test_data/Hello.class");

    parse_class(hello)?;

    Ok(())
}
