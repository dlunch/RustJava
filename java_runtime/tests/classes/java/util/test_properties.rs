use alloc::{boxed::Box, collections::BTreeMap, string::String as RustString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{InputStream, OutputStream},
        lang::{Object, String},
        util::Properties,
    },
};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct FailingInputStream;

impl FailingInputStream {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "FailingInputStream",
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("read", "()I", Self::read, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Err(jvm.exception("java/io/IOException", "read failed").await)
    }
}

struct FailingOutputStream;

impl FailingOutputStream {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "FailingOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(I)V", Self::write, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<()> {
        Err(jvm.exception("java/io/IOException", "write failed").await)
    }
}

struct FailingAfterInputStream;

impl FailingAfterInputStream {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "FailingAfterInputStream",
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("read", "()I", Self::read, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("data", "[B", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("position", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, data: ClassInstanceRef<Array<i8>>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "data", "[B", data).await?;
        jvm.put_field(&mut this, "position", "I", 0).await
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        let data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "data", "[B").await?;
        let position: i32 = jvm.get_field(&this, "position", "I").await?;
        if position as usize >= jvm.array_length(&data).await? {
            return Err(jvm.exception("java/io/IOException", "read failed after data").await);
        }

        let value: i8 = jvm.load_array(&data, position as usize, 1).await?.into_iter().next().unwrap();
        jvm.put_field(&mut this, "position", "I", position + 1).await?;
        Ok(value as u8 as i32)
    }
}

async fn properties_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    for proto in [
        FailingInputStream::as_proto(),
        FailingOutputStream::as_proto(),
        FailingAfterInputStream::as_proto(),
    ] {
        jvm.register_class(
            Box::new(ClassDefinitionImpl::from_class_proto(proto, Box::new(runtime.clone()) as Box<_>)),
            None,
        )
        .await?;
    }
    Ok(jvm)
}

#[tokio::test]
async fn test_properties_inherits_hashtable_map_contract() -> Result<()> {
    let jvm = test_jvm().await?;

    let properties = jvm.new_class("java/util/Properties", "()V", ()).await?;
    assert!(jvm.is_instance(&*properties, "java/util/Properties"));
    assert!(jvm.is_instance(&*properties, "java/util/Hashtable"));
    assert!(jvm.is_instance(&*properties, "java/util/Dictionary"));
    assert!(jvm.is_instance(&*properties, "java/util/Map"));

    let key = JavaLangString::from_rust_string(&jvm, "name").await?;
    let equal_key = JavaLangString::from_rust_string(&jvm, "name").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await?;
    assert!(old.is_null());

    let found: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (equal_key,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "value");

    let inherited_key = JavaLangString::from_rust_string(&jvm, "inherited").await?;
    let inherited_equal_key = JavaLangString::from_rust_string(&jvm, "inherited").await?;
    let inherited_value = JavaLangString::from_rust_string(&jvm, "map-value").await?;
    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (inherited_key, inherited_value),
        )
        .await?;
    assert!(old.is_null());

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (inherited_equal_key,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "map-value");

    let size: i32 = jvm
        .invoke_virtual(&properties, &properties.class_definition().name(), "size", "()I", ())
        .await?;
    assert_eq!(size, 2);
    let key_set: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&properties, &properties.class_definition().name(), "keySet", "()Ljava/util/Set;", ())
        .await?;
    assert!(jvm.is_instance(&**key_set, "java/util/Set"));

    Ok(())
}

#[tokio::test]
async fn prop_01_constructors_and_defaults_field() -> Result<()> {
    let proto = Properties::as_proto();
    assert!(proto.access_flags.contains(ClassAccessFlags::PUBLIC));
    let defaults = proto.fields.iter().find(|field| field.name == "defaults").expect("defaults field");
    assert_eq!(defaults.descriptor, "Ljava/util/Properties;");
    assert_eq!(defaults.access_flags, FieldAccessFlags::PROTECTED);
    for descriptor in ["()V", "(Ljava/util/Properties;)V"] {
        let constructor = proto
            .methods
            .iter()
            .find(|method| method.name == "<init>" && method.descriptor == descriptor)
            .expect("Properties constructor");
        assert!(constructor.access_flags.contains(MethodAccessFlags::PUBLIC));
    }

    let jvm = test_jvm().await?;
    let empty = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let defaults: ClassInstanceRef<Properties> = jvm.get_field(&empty, "defaults", "Ljava/util/Properties;").await?;
    assert!(defaults.is_null());

    let parent = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let child = jvm
        .new_class("java/util/Properties", "(Ljava/util/Properties;)V", (parent.clone(),))
        .await?;
    let actual: ClassInstanceRef<Properties> = jvm.get_field(&child, "defaults", "Ljava/util/Properties;").await?;
    let key = JavaLangString::from_rust_string(&jvm, "shared-instance").await?;
    let value = JavaLangString::from_rust_string(&jvm, "yes").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &actual,
            "java/util/Properties",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), value),
        )
        .await?;
    let inherited: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &parent,
            &parent.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (key,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &inherited).await?, "yes");

    Ok(())
}

#[tokio::test]
async fn prop_02_get_property_uses_string_values_and_defaults_chain() -> Result<()> {
    let jvm = test_jvm().await?;
    let root = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let middle = jvm
        .new_class("java/util/Properties", "(Ljava/util/Properties;)V", (root.clone(),))
        .await?;
    let child = jvm
        .new_class("java/util/Properties", "(Ljava/util/Properties;)V", (middle.clone(),))
        .await?;

    for (properties, key, value) in [
        (&root, "root", "root-value"),
        (&root, "shared", "root-shared"),
        (&middle, "middle", "middle-value"),
        (&child, "shared", "child-shared"),
    ] {
        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                properties,
                &properties.class_definition().name(),
                "setProperty",
                "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                (key, value),
            )
            .await?;
    }

    for (key, expected) in [("root", "root-value"), ("middle", "middle-value"), ("shared", "child-shared")] {
        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let value: ClassInstanceRef<String> = jvm
            .invoke_virtual(
                &child,
                &child.class_definition().name(),
                "getProperty",
                "(Ljava/lang/String;)Ljava/lang/String;",
                (key,),
            )
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, expected);
    }

    let key = JavaLangString::from_rust_string(&jvm, "root").await?;
    let non_string = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &child,
            &child.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), non_string),
        )
        .await?;
    let inherited: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &child,
            &child.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (key,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &inherited).await?, "root-value");

    let missing = JavaLangString::from_rust_string(&jvm, "missing").await?;
    let fallback = JavaLangString::from_rust_string(&jvm, "fallback").await?;
    let value: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &child,
            &child.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            (missing, fallback),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "fallback");

    Ok(())
}

#[tokio::test]
async fn prop_03_load_store_property_names_and_failures() -> Result<()> {
    let proto = Properties::as_proto();
    for (name, descriptor) in [
        ("load", "(Ljava/io/InputStream;)V"),
        ("store", "(Ljava/io/OutputStream;Ljava/lang/String;)V"),
        ("propertyNames", "()Ljava/util/Enumeration;"),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .expect("Properties method");
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC));
        assert!(method.access_flags.contains(MethodAccessFlags::SYNCHRONIZED));
    }

    let jvm = properties_jvm().await?;
    let source =
        b"# comment ending in backslash\\\ncontinued=hello\\\n  world\nescaped\\ key\\:=value\\tend\nlatin=\xE9\nunicode=\\u00E9\ntail=slash\\";
    let mut bytes = jvm.instantiate_array("B", source.len()).await?;
    jvm.store_array(&mut bytes, 0, source.iter().map(|byte| *byte as i8)).await?;
    let input: ClassInstanceRef<InputStream> = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?.into();
    let properties = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "load",
            "(Ljava/io/InputStream;)V",
            (input,),
        )
        .await?;

    for (key, expected) in [
        ("continued", "helloworld"),
        ("escaped key:", "value\tend"),
        ("latin", "é"),
        ("unicode", "é"),
        ("tail", "slash"),
    ] {
        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let value: ClassInstanceRef<String> = jvm
            .invoke_virtual(
                &properties,
                &properties.class_definition().name(),
                "getProperty",
                "(Ljava/lang/String;)Ljava/lang/String;",
                (key,),
            )
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, expected);
    }

    let defaults = jvm.new_class("java/util/Properties", "()V", ()).await?;
    for (key, value) in [("shared", "default"), ("inherited", "yes")] {
        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &defaults,
                "java/util/Properties",
                "setProperty",
                "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                (key, value),
            )
            .await?;
    }
    let child = jvm.new_class("java/util/Properties", "(Ljava/util/Properties;)V", (defaults,)).await?;
    for (key, value) in [("shared", "child"), ("local", "yes")] {
        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &child,
                &child.class_definition().name(),
                "setProperty",
                "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                (key, value),
            )
            .await?;
    }
    let names: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&child, &child.class_definition().name(), "propertyNames", "()Ljava/util/Enumeration;", ())
        .await?;
    let mut actual_names = Vec::new();
    while jvm
        .invoke_virtual::<_, bool>(&names, &names.class_definition().name(), "hasMoreElements", "()Z", ())
        .await?
    {
        let name: ClassInstanceRef<String> = jvm
            .invoke_virtual(&names, &names.class_definition().name(), "nextElement", "()Ljava/lang/Object;", ())
            .await?;
        actual_names.push(JavaLangString::to_rust_string(&jvm, &name).await?);
    }
    actual_names.sort();
    assert_eq!(actual_names, ["inherited", "local", "shared"]);

    let child_output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &child,
            &child.class_definition().name(),
            "store",
            "(Ljava/io/OutputStream;Ljava/lang/String;)V",
            (child_output.clone(), ClassInstanceRef::<String>::from(None)),
        )
        .await?;
    let child_bytes: ClassInstanceRef<Array<i8>> = jvm
        .invoke_virtual(&child_output, "java/io/ByteArrayOutputStream", "toByteArray", "()[B", ())
        .await?;
    let child_values: Vec<i8> = jvm.load_array(&child_bytes, 0, jvm.array_length(&child_bytes).await?).await?;
    let child_text = RustString::from_utf8(child_values.iter().map(|byte| *byte as u8).collect()).expect("ASCII properties output");
    assert!(child_text.contains("local=yes"));
    assert!(child_text.contains("shared=child"));
    assert!(!child_text.contains("inherited"));

    let output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let comments = JavaLangString::from_rust_string(&jvm, "round trip").await?;
    let _: () = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "store",
            "(Ljava/io/OutputStream;Ljava/lang/String;)V",
            (output.clone(), comments),
        )
        .await?;
    let stored: ClassInstanceRef<Array<i8>> = jvm
        .invoke_virtual(&output, "java/io/ByteArrayOutputStream", "toByteArray", "()[B", ())
        .await?;
    let stored_values: Vec<i8> = jvm.load_array(&stored, 0, jvm.array_length(&stored).await?).await?;
    let stored_ascii = RustString::from_utf8(stored_values.iter().map(|byte| *byte as u8).collect()).expect("ASCII properties output");
    assert!(stored_ascii.contains("#round trip"));
    assert!(stored_ascii.contains("\\u00E9"));

    let input: ClassInstanceRef<InputStream> = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (stored,)).await?.into();
    let round_trip = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(
            &round_trip,
            &round_trip.class_definition().name(),
            "load",
            "(Ljava/io/InputStream;)V",
            (input,),
        )
        .await?;
    let latin = JavaLangString::from_rust_string(&jvm, "latin").await?;
    let value: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &round_trip,
            &round_trip.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (latin,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "é");

    let malformed = b"broken=\\u12G4\n";
    let mut bytes = jvm.instantiate_array("B", malformed.len()).await?;
    jvm.store_array(&mut bytes, 0, malformed.iter().map(|byte| *byte as i8)).await?;
    let input: ClassInstanceRef<InputStream> = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?.into();
    let malformed_result: Result<()> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "load",
            "(Ljava/io/InputStream;)V",
            (input,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = malformed_result else {
        panic!("malformed unicode escape must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));
    let broken = JavaLangString::from_rust_string(&jvm, "broken").await?;
    let broken_value: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (broken,),
        )
        .await?;
    assert!(broken_value.is_null(), "a malformed logical line must not be committed");

    let partial_source = b"committed=first\r\nunfinished=discarded";
    let mut partial_bytes = jvm.instantiate_array("B", partial_source.len()).await?;
    jvm.store_array(&mut partial_bytes, 0, partial_source.iter().map(|byte| *byte as i8))
        .await?;
    let partial_input: ClassInstanceRef<InputStream> = jvm.new_class("FailingAfterInputStream", "([B)V", (partial_bytes,)).await?.into();
    let partial_properties = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let partial_result: Result<()> = jvm
        .invoke_virtual(
            &partial_properties,
            &partial_properties.class_definition().name(),
            "load",
            "(Ljava/io/InputStream;)V",
            (partial_input,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = partial_result else {
        panic!("an IOException after a complete property must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let committed = JavaLangString::from_rust_string(&jvm, "committed").await?;
    let committed_value: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &partial_properties,
            &partial_properties.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (committed,),
        )
        .await?;
    assert!(
        !committed_value.is_null(),
        "the complete property must be committed before the later read"
    );
    assert_eq!(JavaLangString::to_rust_string(&jvm, &committed_value).await?, "first");
    let unfinished = JavaLangString::from_rust_string(&jvm, "unfinished").await?;
    let unfinished_value: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &partial_properties,
            &partial_properties.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (unfinished,),
        )
        .await?;
    assert!(unfinished_value.is_null(), "an incomplete logical line must not be committed");

    let failing_input: ClassInstanceRef<InputStream> = jvm.new_class("FailingInputStream", "()V", ()).await?.into();
    let failed_load: Result<()> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "load",
            "(Ljava/io/InputStream;)V",
            (failing_input,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = failed_load else {
        panic!("load IOException must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    let null_input: ClassInstanceRef<InputStream> = None.into();
    let null_load: Result<()> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "load",
            "(Ljava/io/InputStream;)V",
            (null_input,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = null_load else {
        panic!("null input must throw NullPointerException before stream processing");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let failing_output: ClassInstanceRef<OutputStream> = jvm.new_class("FailingOutputStream", "()V", ()).await?.into();
    let failed_store: Result<()> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "store",
            "(Ljava/io/OutputStream;Ljava/lang/String;)V",
            (failing_output, ClassInstanceRef::<String>::from(None)),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = failed_store else {
        panic!("store IOException must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    for non_string_key in [true, false] {
        let invalid = jvm.new_class("java/util/Properties", "()V", ()).await?;
        let string = JavaLangString::from_rust_string(&jvm, "text").await?;
        let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
        let (key, value): (ClassInstanceRef<Object>, ClassInstanceRef<Object>) = if non_string_key {
            (object.into(), string.into())
        } else {
            (string.into(), object.into())
        };
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &invalid,
                &invalid.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (key, value),
            )
            .await?;
        let output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
        let result: Result<()> = jvm
            .invoke_virtual(
                &invalid,
                &invalid.class_definition().name(),
                "store",
                "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                (output, ClassInstanceRef::<String>::from(None)),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("non-String property must throw ClassCastException");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));
    }

    Ok(())
}

#[tokio::test]
async fn prop_03_eof_backslash_parity_and_incomplete_logical_line() -> Result<()> {
    let jvm = properties_jvm().await?;

    for (source, key, expected) in [
        (&b"odd=slash\\"[..], "odd", "slash"),
        (&b"even=slash\\\\"[..], "even", "slash\\"),
        (&b"incomplete=value"[..], "incomplete", "value"),
    ] {
        let mut bytes = jvm.instantiate_array("B", source.len()).await?;
        jvm.store_array(&mut bytes, 0, source.iter().map(|byte| *byte as i8)).await?;

        let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
        let properties = jvm.new_class("java/util/Properties", "()V", ()).await?;
        let _: () = jvm
            .invoke_virtual(
                &properties,
                &properties.class_definition().name(),
                "load",
                "(Ljava/io/InputStream;)V",
                (input,),
            )
            .await?;

        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let value: ClassInstanceRef<String> = jvm
            .invoke_virtual(
                &properties,
                &properties.class_definition().name(),
                "getProperty",
                "(Ljava/lang/String;)Ljava/lang/String;",
                (key,),
            )
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, expected);
    }

    Ok(())
}

#[tokio::test]
async fn prop_store_and_load_with_substring_key_and_value() -> Result<()> {
    let jvm = test_jvm().await?;

    let key_parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let key: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &key_parent,
            &key_parent.class_definition().name(),
            "substring",
            "(II)Ljava/lang/String;",
            (2, 7),
        )
        .await?;
    let value_parent = JavaLangString::from_rust_string(&jvm, "zzWorldzz").await?;
    let value: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &value_parent,
            &value_parent.class_definition().name(),
            "substring",
            "(II)Ljava/lang/String;",
            (2, 7),
        )
        .await?;

    let properties = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), value),
        )
        .await?;

    let output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "store",
            "(Ljava/io/OutputStream;Ljava/lang/String;)V",
            (output.clone(), ClassInstanceRef::<String>::from(None)),
        )
        .await?;

    let bytes: ClassInstanceRef<Array<i8>> = jvm
        .invoke_virtual(&output, "java/io/ByteArrayOutputStream", "toByteArray", "()[B", ())
        .await?;
    let values: Vec<i8> = jvm.load_array(&bytes, 0, jvm.array_length(&bytes).await?).await?;
    let text = RustString::from_utf8(values.iter().map(|byte| *byte as u8).collect()).expect("ASCII properties output");
    assert!(text.contains("Hello=World"));

    let input: ClassInstanceRef<InputStream> = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?.into();
    let loaded = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(&loaded, &loaded.class_definition().name(), "load", "(Ljava/io/InputStream;)V", (input,))
        .await?;
    let result: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &loaded,
            &loaded.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (key,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "World");

    Ok(())
}

#[tokio::test]
async fn prop_empty_substring_key_round_trip() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "HelloWorld").await?;
    let empty_key: ClassInstanceRef<String> = jvm
        .invoke_virtual(&parent, &parent.class_definition().name(), "substring", "(II)Ljava/lang/String;", (5, 5))
        .await?;
    let value = JavaLangString::from_rust_string(&jvm, "World").await?;

    let properties = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (empty_key.clone(), value),
        )
        .await?;

    let output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &properties,
            &properties.class_definition().name(),
            "store",
            "(Ljava/io/OutputStream;Ljava/lang/String;)V",
            (output.clone(), ClassInstanceRef::<String>::from(None)),
        )
        .await?;

    let bytes: ClassInstanceRef<Array<i8>> = jvm
        .invoke_virtual(&output, "java/io/ByteArrayOutputStream", "toByteArray", "()[B", ())
        .await?;
    let values: Vec<i8> = jvm.load_array(&bytes, 0, jvm.array_length(&bytes).await?).await?;
    let text = RustString::from_utf8(values.iter().map(|byte| *byte as u8).collect()).unwrap();
    assert!(text.contains("=World"), "store output: {text:?}");

    let input: ClassInstanceRef<InputStream> = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?.into();
    let loaded = jvm.new_class("java/util/Properties", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(&loaded, &loaded.class_definition().name(), "load", "(Ljava/io/InputStream;)V", (input,))
        .await?;
    let result: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &loaded,
            &loaded.class_definition().name(),
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (empty_key,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "World");

    Ok(())
}
