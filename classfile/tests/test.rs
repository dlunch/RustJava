use classfile::{AttributeInfo, ClassInfo};

#[test]
fn test_hello() -> anyhow::Result<()> {
    let hello = include_bytes!("../../test_data/Hello.class");

    let class = ClassInfo::parse(hello)?;

    assert_eq!(class.magic, 0xCAFEBABE);
    assert_eq!(class.major_version, 65);
    assert_eq!(class.minor_version, 0);
    assert_eq!(class.constant_pool.len(), 28);
    assert_eq!(class.access_flags, 0x20);
    assert_eq!(class.this_class, "Hello".to_string());
    assert_eq!(class.super_class, Some("java/lang/Object".to_string()));
    assert_eq!(class.interfaces.len(), 0);
    assert_eq!(class.fields.len(), 0);
    assert_eq!(class.methods.len(), 2);
    assert_eq!(class.attributes.len(), 1);

    assert_eq!(class.methods[0].name, "<init>");
    assert_eq!(class.methods[0].descriptor, "()V");

    assert!(matches!(class.attributes[0], AttributeInfo::SourceFile { .. }));

    Ok(())
}
