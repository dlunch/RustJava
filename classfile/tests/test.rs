use java_constants::ClassAccessFlags;

use classfile::{AttributeInfo, ClassInfo, ConstantPoolReference, Opcode};

#[test]
fn test_hello() {
    let hello = include_bytes!("../../test_data/Hello.class");

    let class = ClassInfo::parse(hello).unwrap();

    assert_eq!(class.magic, 0xCAFEBABE);
    assert_eq!(class.major_version, 65);
    assert_eq!(class.minor_version, 0);
    assert_eq!(class.constant_pool.len(), 28);
    assert!(class.access_flags == ClassAccessFlags::SUPER);
    assert_eq!(class.this_class, "Hello".to_string().into());
    assert_eq!(class.super_class, Some("java/lang/Object".to_string().into()));
    assert_eq!(class.interfaces.len(), 0);
    assert_eq!(class.fields.len(), 0);
    assert_eq!(class.methods.len(), 2);
    assert_eq!(class.attributes.len(), 1);

    assert_eq!(class.methods[0].name, "<init>".to_string().into());
    assert_eq!(class.methods[0].descriptor, "()V".to_string().into());
    assert!(matches!(class.methods[0].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(x) = &class.methods[0].attributes[0] {
        assert_eq!(x.code.len(), 3);
        assert!(matches!(x.code.get(&0).unwrap(), Opcode::Aload(0)));
        assert!(matches!(x.code.get(&1).unwrap(),
            Opcode::Invokespecial(
                ConstantPoolReference::Method(x)) if x.class == "java/lang/Object".to_string().into() && x.name == "<init>".to_string().into() && x.descriptor == "()V".to_string().into()));
        assert!(matches!(x.code.get(&4).unwrap(), Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }

    assert_eq!(class.methods[1].name, "main".to_string().into());
    assert_eq!(class.methods[1].descriptor, "([Ljava/lang/String;)V".to_string().into());
    assert!(matches!(class.methods[1].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(x) = &class.methods[1].attributes[0] {
        assert_eq!(x.code.len(), 4);
        assert!(matches!(x.code.get(&0).unwrap(),
            Opcode::Getstatic(ConstantPoolReference::Field(x)) if x.class == "java/lang/System".to_string().into() && x.name == "out".to_string().into() && x.descriptor == "Ljava/io/PrintStream;".to_string().into()));
        assert!(matches!(x.code.get(&3).unwrap(),
            Opcode::Ldc(x) if matches!(x, ConstantPoolReference::String(y) if *y == "Hello, world!".to_string().into())));
        assert!(matches!(x.code.get(&5).unwrap(),
            Opcode::Invokevirtual(ConstantPoolReference::Method(x)) if x.class == "java/io/PrintStream".to_string().into() && x.name == "println".to_string().into() && x.descriptor == "(Ljava/lang/String;)V".to_string().into()));
        assert!(matches!(x.code.get(&8).unwrap(), Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }

    assert!(matches!(class.attributes[0], AttributeInfo::SourceFile { .. }));
}

#[test]
fn test_odd_even() {
    let odd_even = include_bytes!("../../test_data/OddEven.class");

    let class = ClassInfo::parse(odd_even).unwrap();

    assert_eq!(class.methods[2].name, "run".to_string().into());
    assert!(matches!(class.methods[2].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(code_attribute) = &class.methods[2].attributes[0] {
        assert!(matches!(code_attribute.attributes[0], AttributeInfo::LineNumberTable { .. }));
        assert!(matches!(code_attribute.attributes[1], AttributeInfo::LocalVariableTable { .. }));
        assert!(matches!(code_attribute.attributes[2], AttributeInfo::StackMapTable { .. }));

        if let AttributeInfo::LocalVariableTable(local_variable_table) = &code_attribute.attributes[1] {
            assert_eq!(local_variable_table.len(), 3);
            assert_eq!(local_variable_table[0].name, "this".to_string().into());
            assert_eq!(local_variable_table[0].descriptor, "LOddEven;".to_string().into());
            assert_eq!(local_variable_table[0].index, 0);
            assert_eq!(local_variable_table[1].name, "arg".to_string().into());
            assert_eq!(local_variable_table[1].descriptor, "Ljava/lang/String;".to_string().into());
            assert_eq!(local_variable_table[1].index, 1);
            assert_eq!(local_variable_table[2].name, "i".to_string().into());
            assert_eq!(local_variable_table[2].descriptor, "I".to_string().into());
            assert_eq!(local_variable_table[2].index, 2);
        }
    }
}

#[test]
fn test_superclass() {
    let super_class = include_bytes!("../../test_data/SuperClass.class");

    let class = ClassInfo::parse(super_class).unwrap();

    assert_eq!(class.methods[1].name, "run".to_string().into());
    assert!(matches!(class.methods[1].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(code_attribute) = &class.methods[2].attributes[0] {
        assert!(matches!(code_attribute.attributes[0], AttributeInfo::LineNumberTable { .. }));
    }
}

#[test]
fn test_switch() {
    let super_class = include_bytes!("../../test_data/Switch.class");

    let class = ClassInfo::parse(super_class).unwrap();

    assert_eq!(class.methods[2].name, "run".to_string().into());
    assert!(matches!(class.methods[2].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(code_attribute) = &class.methods[2].attributes[0] {
        assert!(matches!(
            code_attribute.code.get(&6).unwrap(),
            Opcode::Tableswitch(default, pairs) if *default == 68 && *pairs == vec![(1, 30), (2, 41), (3, 52), (4, 60)]
        ));

        assert!(matches!(
            code_attribute.code.get(&75).unwrap(),
            Opcode::Lookupswitch(default, pairs) if *default == 82 && *pairs == vec![(1, 41), (10, 52), (100, 63), (1000, 74)]));
    }
}
