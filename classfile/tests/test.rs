use std::collections::BTreeMap;

use jvm_types::ClassAccessFlags;

use classfile::{AttributeInfo, AttributeInfoCode, ClassFileError, ClassInfo, ConstantPoolReference, Opcode};

#[test]
fn test_hello() {
    let hello = include_bytes!("../../test-data/Hello.class");

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
        assert_eq!(x.pc_to_index, vec![0, 1, u32::MAX, u32::MAX, 2]);
        assert!(matches!(&x.code[0], (0, Opcode::Aload(0))));
        assert!(matches!(&x.code[1],
            (1, Opcode::Invokespecial(
                ConstantPoolReference::Method(x))) if x.class == "java/lang/Object".to_string().into() && x.name == "<init>".to_string().into() && x.descriptor == "()V".to_string().into()));
        assert!(matches!(&x.code[2], (4, Opcode::Return)));
    } else {
        panic!("Expected code attribute");
    }

    assert_eq!(class.methods[1].name, "main".to_string().into());
    assert_eq!(class.methods[1].descriptor, "([Ljava/lang/String;)V".to_string().into());
    assert!(matches!(class.methods[1].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(x) = &class.methods[1].attributes[0] {
        assert_eq!(x.code.len(), 4);
        assert_eq!(x.pc_to_index, vec![0, u32::MAX, u32::MAX, 1, u32::MAX, 2, u32::MAX, u32::MAX, 3]);
        assert!(matches!(&x.code[0],
            (0, Opcode::Getstatic(ConstantPoolReference::Field(x))) if x.class == "java/lang/System".to_string().into() && x.name == "out".to_string().into() && x.descriptor == "Ljava/io/PrintStream;".to_string().into()));
        assert!(matches!(&x.code[1],
            (3, Opcode::Ldc(x)) if matches!(x, ConstantPoolReference::String(y) if *y == "Hello, world!".to_string().into())));
        assert!(matches!(&x.code[2],
            (5, Opcode::Invokevirtual(ConstantPoolReference::Method(x))) if x.class == "java/io/PrintStream".to_string().into() && x.name == "println".to_string().into() && x.descriptor == "(Ljava/lang/String;)V".to_string().into()));
        assert!(matches!(&x.code[3], (8, Opcode::Return)));
    } else {
        panic!("Expected code attribute");
    }

    assert!(matches!(class.attributes[0], AttributeInfo::SourceFile { .. }));
}

#[test]
fn test_odd_even() {
    let odd_even = include_bytes!("../../test-data/OddEven.class");

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
    let super_class = include_bytes!("../../test-data/SuperClass.class");

    let class = ClassInfo::parse(super_class).unwrap();

    assert_eq!(class.methods[1].name, "run".to_string().into());
    assert!(matches!(class.methods[1].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(code_attribute) = &class.methods[2].attributes[0] {
        assert!(matches!(code_attribute.attributes[0], AttributeInfo::LineNumberTable { .. }));
    }
}

#[test]
fn test_switch() {
    let super_class = include_bytes!("../../test-data/Switch.class");

    let class = ClassInfo::parse(super_class).unwrap();

    assert_eq!(class.methods[2].name, "run".to_string().into());
    assert!(matches!(class.methods[2].attributes[0], AttributeInfo::Code { .. }));
    if let AttributeInfo::Code(code_attribute) = &class.methods[2].attributes[0] {
        let table_targets = [(1, 36), (2, 47), (3, 58), (4, 66)].map(|(key, pc)| (key, code_attribute.pc_to_index[pc] as i32));
        assert!(matches!(
            &code_attribute.code[code_attribute.pc_to_index[6] as usize],
            (6, Opcode::Tableswitch(default, pairs)) if *default == code_attribute.pc_to_index[74] as i32 && *pairs == table_targets
        ));

        let lookup_targets = [(1, 116), (10, 127), (100, 138), (1000, 149)].map(|(key, pc)| (key, code_attribute.pc_to_index[pc] as i32));
        assert!(matches!(
            &code_attribute.code[code_attribute.pc_to_index[75] as usize],
            (75, Opcode::Lookupswitch(default, pairs)) if *default == code_attribute.pc_to_index[157] as i32 && *pairs == lookup_targets));
    }
}

#[test]
fn test_branch_targets_are_resolved_to_instruction_indices() {
    // bipush 0; ifeq 8; goto 0; return
    let data = [0, 1, 0, 0, 0, 0, 0, 9, 0x10, 0, 0x99, 0, 6, 0xa7, 0xff, 0xfb, 0xb1, 0, 0, 0, 0];
    let (_, code) = AttributeInfoCode::parse(&data, &BTreeMap::new()).unwrap();

    assert!(matches!(code.code[1], (2, Opcode::Ifeq(3))));
    assert!(matches!(code.code[2], (5, Opcode::Goto(0))));

    // goto_w 7; bipush 1; jsr 5; jsr_w 5; return
    let data = [
        0, 1, 0, 1, 0, 0, 0, 16, 0xc8, 0, 0, 0, 7, 0x10, 1, 0xa8, 0xff, 0xfe, 0xc9, 0xff, 0xff, 0xff, 0xfb, 0xb1, 0, 0, 0, 0,
    ];
    let (_, code) = AttributeInfoCode::parse(&data, &BTreeMap::new()).unwrap();

    assert!(matches!(code.code[0], (0, Opcode::GotoW(2))));
    assert!(matches!(code.code[2], (7, Opcode::Jsr(1))));
    assert!(matches!(code.code[3], (10, Opcode::JsrW(1))));
}

#[test]
fn test_short_branch_can_target_an_instruction_index_above_i16_max() {
    let mut bytecode = vec![0; 32768];
    bytecode.extend_from_slice(&[0xa7, 0, 3, 0xb1]);
    let mut data = vec![0, 0, 0, 0];
    data.extend_from_slice(&(bytecode.len() as u32).to_be_bytes());
    data.extend_from_slice(&bytecode);
    data.extend_from_slice(&[0, 0, 0, 0]);
    let (_, code) = AttributeInfoCode::parse(&data, &BTreeMap::new()).unwrap();

    assert!(matches!(code.code[32768], (32768, Opcode::Goto(32769))));
    assert!(matches!(code.code[32769], (32771, Opcode::Return)));
}

#[test]
fn test_branch_targets_must_be_instruction_boundaries() {
    for bytecode in [
        &[0xa7, 0, 1][..],
        &[0xa7, 0, 3],
        &[0xa7, 0xff, 0xff],
        &[0xc8, 0x7f, 0xff, 0xff, 0xff],
        &[0xc8, 0x80, 0, 0, 0],
    ] {
        let mut data = vec![0, 0, 0, 0];
        data.extend_from_slice(&(bytecode.len() as u32).to_be_bytes());
        data.extend_from_slice(bytecode);
        data.extend_from_slice(&[0, 0, 0, 0]);

        assert!(AttributeInfoCode::parse(&data, &BTreeMap::new()).is_err());
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
    let interface = include_bytes!("../../test-data/Interface.class");

    let class = ClassInfo::parse(interface).unwrap();

    assert_eq!(class.methods[1].name, "main".to_string().into());
    if let AttributeInfo::Code(x) = &class.methods[1].attributes[0] {
        assert_eq!(x.code.len(), 7);
        assert_eq!(x.pc_to_index.len(), 15);
        assert_eq!(x.pc_to_index[9], 5);
        assert_eq!(x.pc_to_index[12], u32::MAX);
        assert_eq!(x.pc_to_index[13], u32::MAX);
        assert!(matches!(&x.code[5],
            (9, Opcode::Invokeinterface(ConstantPoolReference::InterfaceMethodref(m), 1, 0)) if m.class == "Interface$IInterface".to_string().into() && m.name == "test".to_string().into()));
        assert!(matches!(&x.code[6], (14, Opcode::Return)));
    } else {
        panic!("Expected code attribute");
    }
}

#[test]
fn test_malformed_class_files_return_structured_errors() {
    let hello = include_bytes!("../../test-data/Hello.class");

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

#[test]
fn test_class_info_validation_rejects_invalid_names_descriptors_and_code_layout() {
    let hello = include_bytes!("../../test-data/Hello.class");

    let mut invalid_name = ClassInfo::parse(hello).unwrap();
    invalid_name.this_class = "[I".to_string().into();
    assert_eq!(invalid_name.validate(), Err(ClassFileError::InvalidFormat));

    let mut invalid_descriptor = ClassInfo::parse(hello).unwrap();
    invalid_descriptor.methods[0].descriptor = "(V)V".to_string().into();
    assert_eq!(invalid_descriptor.validate(), Err(ClassFileError::InvalidFormat));

    let mut missing_code = ClassInfo::parse(hello).unwrap();
    missing_code.methods[0].attributes.clear();
    assert_eq!(missing_code.validate(), Err(ClassFileError::InvalidFormat));
}

#[test]
fn test_array_clone_method_owner_is_a_valid_class_constant() {
    assert!(ClassInfo::parse(include_bytes!("../../test-data/Array.class")).is_ok());
}
