use classfile::{AttributeInfo, ClassInfo, Opcode, ValueConstant};

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
        assert!(matches!(x.code.get(&0).unwrap(), Opcode::Aload0));
        assert!(
            matches!(x.code.get(&1).unwrap(), Opcode::Invokespecial(x) if x.class == "java/lang/Object" && x.name == "<init>" && x.descriptor == "()V")
        );
        assert!(matches!(x.code.get(&4).unwrap(), Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }

    assert_eq!(class.methods[1].name, "main");
    assert_eq!(class.methods[1].descriptor, "([Ljava/lang/String;)V");
    assert!(matches!(class.methods[1].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(x) = &class.methods[1].attributes[0] {
        assert_eq!(x.code.len(), 4);
        assert!(
            matches!(x.code.get(&0).unwrap(), Opcode::Getstatic(x) if x.class == "java/lang/System" && x.name == "out" && x.descriptor == "Ljava/io/PrintStream;")
        );
        assert!(matches!(x.code.get(&3).unwrap(), Opcode::Ldc(x) if matches!(x, ValueConstant::String(y) if y == "Hello, world!")));
        assert!(
            matches!(x.code.get(&5).unwrap(), Opcode::Invokevirtual(x) if x.class == "java/io/PrintStream" && x.name == "println" && x.descriptor == "(Ljava/lang/String;)V")
        );
        assert!(matches!(x.code.get(&8).unwrap(), Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }

    assert!(matches!(class.attributes[0], AttributeInfo::SourceFile { .. }));

    Ok(())
}
