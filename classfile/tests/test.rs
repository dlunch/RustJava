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
    assert!(matches!(class.methods[0].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(x) = &class.methods[0].attributes[0] {
        assert_eq!(x.code.len(), 3);
        assert!(matches!(x.code.get(&0).unwrap(), classfile::Opcode::Aload0));
        assert!(matches!(x.code.get(&1).unwrap(), classfile::Opcode::Invokespecial(1)));
        assert!(matches!(x.code.get(&4).unwrap(), classfile::Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }

    assert_eq!(class.methods[1].name, "main");
    assert_eq!(class.methods[1].descriptor, "([Ljava/lang/String;)V");
    assert!(matches!(class.methods[1].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(x) = &class.methods[1].attributes[0] {
        assert_eq!(x.code.len(), 4);
        assert!(matches!(x.code.get(&0).unwrap(), classfile::Opcode::Getstatic(7)));
        assert!(matches!(x.code.get(&3).unwrap(), classfile::Opcode::Ldc(13)));
        assert!(matches!(x.code.get(&5).unwrap(), classfile::Opcode::Invokevirtual(15)));
        assert!(matches!(x.code.get(&8).unwrap(), classfile::Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }

    assert!(matches!(class.attributes[0], AttributeInfo::SourceFile { .. }));

    Ok(())
}
