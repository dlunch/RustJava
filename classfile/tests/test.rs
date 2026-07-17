use std::collections::BTreeMap;

use java_constants::ClassAccessFlags;

use classfile::{AttributeInfo, ClassFileError, ClassInfo, ConstantPoolReference, Opcode};

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

#[test]
fn test_switch_rejects_entry_counts_larger_than_remaining_input() {
    let constant_pool = BTreeMap::new();
    let lookup_switch = [0xab, 0, 0, 0, 0, 0, 0, 0, 0x7f, 0xff, 0xff, 0xff];
    assert!(Opcode::parse(&lookup_switch, 0, &constant_pool).is_err());

    let table_switch = [0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x7f, 0xff, 0xff, 0xff];
    assert!(Opcode::parse(&table_switch, 0, &constant_pool).is_err());
}

#[test]
fn test_invokeinterface() {
    let interface = include_bytes!("../../test_data/Interface.class");

    let class = ClassInfo::parse(interface).unwrap();

    assert_eq!(class.methods[1].name, "main".to_string().into());
    if let AttributeInfo::Code(x) = &class.methods[1].attributes[0] {
        assert_eq!(x.code.len(), 7);
        assert!(matches!(x.code.get(&9).unwrap(),
            Opcode::Invokeinterface(ConstantPoolReference::InterfaceMethodref(m), 1, 0) if m.class == "Interface$IInterface".to_string().into() && m.name == "test".to_string().into()));
        assert!(!x.code.contains_key(&12));
        assert!(!x.code.contains_key(&13));
        assert!(matches!(x.code.get(&14).unwrap(), Opcode::Return));
    } else {
        panic!("Expected code attribute");
    }
}

#[test]
fn test_malformed_class_files_return_structured_errors() {
    let hello = include_bytes!("../../test_data/Hello.class");

    assert_eq!(ClassInfo::parse(&[]).err(), Some(ClassFileError::InvalidFormat));

    let mut invalid_magic = hello.to_vec();
    invalid_magic[0] = 0;
    assert_eq!(ClassInfo::parse(&invalid_magic).err(), Some(ClassFileError::InvalidFormat));

    let mut unsupported_version = hello.to_vec();
    unsupported_version[6..8].copy_from_slice(&71u16.to_be_bytes());
    assert_eq!(ClassInfo::parse(&unsupported_version).err(), Some(ClassFileError::UnsupportedVersion(71)));

    assert_eq!(ClassInfo::parse(&hello[..hello.len() / 2]).err(), Some(ClassFileError::InvalidFormat));

    let minimal_class = vec![
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x2d, 0x00, 0x05, 0x01, 0x00, 0x04, b'T', b'e', b's', b't', 0x07, 0x00, 0x01, 0x01, 0x00, 0x10,
        b'j', b'a', b'v', b'a', b'/', b'l', b'a', b'n', b'g', b'/', b'O', b'b', b'j', b'e', b'c', b't', 0x07, 0x00, 0x03, 0x00, 0x21, 0x00, 0x02,
        0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    assert!(ClassInfo::parse(&minimal_class).is_ok());

    let mut invalid_constant_pool_index = minimal_class.clone();
    invalid_constant_pool_index[44..46].copy_from_slice(&99u16.to_be_bytes());
    assert_eq!(ClassInfo::parse(&invalid_constant_pool_index).err(), Some(ClassFileError::InvalidFormat));

    let mut invalid_constant_pool_type = minimal_class;
    invalid_constant_pool_type[44..46].copy_from_slice(&1u16.to_be_bytes());
    assert_eq!(ClassInfo::parse(&invalid_constant_pool_type).err(), Some(ClassFileError::InvalidFormat));
}
