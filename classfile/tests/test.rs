use classfile::parse_class;

#[test]
fn test_hello() -> anyhow::Result<()> {
    let hello = include_bytes!("../../test_data/Hello.class");

    let class = parse_class(hello)?;

    assert_eq!(class.magic, 0xCAFEBABE);
    assert_eq!(class.major_version, 65);
    assert_eq!(class.minor_version, 0);
    assert_eq!(class.constant_pool.len(), 28);
    assert_eq!(class.access_flags, 0x20);
    assert_eq!(class.this_class, 0x15);
    assert_eq!(class.super_class, 0x02);
    assert_eq!(class.interfaces.len(), 0);
    assert_eq!(class.fields.len(), 0);
    assert_eq!(class.methods.len(), 2);
    assert_eq!(class.attributes.len(), 1);

    assert_eq!(class.methods[0].name_index, 5);
    assert_eq!(class.methods[0].descriptor_index, 6);

    assert_eq!(class.attributes[0].attribute_name_index, 27);
    assert_eq!(class.attributes[0].attribute_length, 2);
    assert_eq!(class.attributes[0].info, [0, 28]);

    Ok(())
}
