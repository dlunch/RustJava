use alloc::{boxed::Box, collections::BTreeMap, vec, vec::Vec};

use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_bytecode::ClassDefinitionImpl;
use jvm_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use rustjava_runtime::classes::java::lang::Object;
use rustjava_runtime::{RuntimeClassProto, RuntimeContext};

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct OneByteInputStream;

impl OneByteInputStream {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "OneByteInputStream",
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("read", "()I", Self::read, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("read", "([BII)I", Self::read_offset_length, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("data", "[B", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("position", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, data: ClassInstanceRef<Array<i8>>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "OneByteInputStream", "data", "[B", data).await?;
        jvm.put_field(&mut this, "OneByteInputStream", "position", "I", 0).await?;

        Ok(())
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        let data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "OneByteInputStream", "data", "[B").await?;
        let position: i32 = jvm.get_field(&this, "OneByteInputStream", "position", "I").await?;
        if position == jvm.array_length(&data).await? as i32 {
            return Ok(-1);
        }

        let value = jvm.load_array::<i8>(&data, position as usize, 1).await?[0];
        jvm.put_field(&mut this, "OneByteInputStream", "position", "I", position + 1).await?;

        Ok(value as u8 as i32)
    }

    async fn read_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut target: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        if length == 0 {
            return Ok(0);
        }

        let value: i32 = jvm.invoke_virtual(&this, "OneByteInputStream", "read", "()I", ()).await?;
        if value == -1 {
            return Ok(-1);
        }

        jvm.store_array(&mut target, offset as usize, [value as i8]).await?;
        Ok(1)
    }
}

#[tokio::test]
async fn test_isr() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut buffer = jvm.instantiate_array("B", 11).await?;
    jvm.array_raw_buffer_mut(&mut buffer).await?.write(0, b"Hello\nWorld")?;

    let is = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (buffer,)).await?;
    let isr = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (is,)).await?;

    let buf = jvm.instantiate_array("C", 10).await?;
    let read: i32 = jvm
        .invoke_virtual(&isr, &isr.class_definition().name(), "read", "([CII)I", (buf.clone(), 0, 5))
        .await?;

    assert_eq!(read, 5);
    let buf_data: Vec<JavaChar> = jvm.load_array(&buf, 0, 5).await?;
    assert_eq!(buf_data, vec![72, 101, 108, 108, 111]);

    let read: i32 = jvm
        .invoke_virtual(&isr, &isr.class_definition().name(), "read", "([CII)I", (buf.clone(), 0, 6))
        .await?;

    assert_eq!(read, 6);
    let buf_data: Vec<JavaChar> = jvm.load_array(&buf, 0, 6).await?;
    assert_eq!(buf_data, vec![10, 87, 111, 114, 108, 100]);

    Ok(())
}

#[tokio::test]
async fn test_input_stream_reader_drains_available_input() -> Result<()> {
    let jvm = test_jvm().await?;
    let value = "123456789한글\r\n가나다".repeat(20);
    let expected: Vec<JavaChar> = value.encode_utf16().collect();

    for encoding in [encoding_rs::UTF_8, encoding_rs::EUC_KR] {
        let (encoded, _, _) = encoding.encode(&value);
        let mut bytes = jvm.instantiate_array("B", encoded.len()).await?;
        jvm.store_array(&mut bytes, 0, encoded.iter().map(|byte| *byte as i8)).await?;
        let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
        let charset = JavaLangString::from_rust_string(&jvm, encoding.name()).await?;
        let reader = jvm
            .new_class(
                "java/io/InputStreamReader",
                "(Ljava/io/InputStream;Ljava/lang/String;)V",
                (input.clone(), charset),
            )
            .await?;
        let mut chars = jvm.instantiate_array("C", expected.len() + 4).await?;
        jvm.store_array(&mut chars, 0, vec!['!' as JavaChar; expected.len() + 4]).await?;

        let read: i32 = jvm
            .invoke_virtual(
                &reader,
                "java/io/Reader",
                "read",
                "([CII)I",
                (chars.clone(), 2, (expected.len() + 2) as i32),
            )
            .await?;
        assert_eq!(read as usize, expected.len(), "{}", encoding.name());
        assert_eq!(jvm.load_array::<JavaChar>(&chars, 2, expected.len()).await?, expected);
        assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 2).await?, ['!' as JavaChar; 2]);
        assert_eq!(jvm.load_array::<JavaChar>(&chars, expected.len() + 2, 2).await?, ['!' as JavaChar; 2]);
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&input, "java/io/InputStream", "available", "()I", ())
                .await?,
            0
        );
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&reader, "java/io/Reader", "read", "([C)I", (chars,)).await?,
            -1
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_input_stream_reader_preserves_split_multibyte_and_buffered_eof() -> Result<()> {
    let jvm = test_jvm().await?;
    let value = "123456789한";
    let mut bytes = jvm.instantiate_array("B", value.len()).await?;
    jvm.store_array(&mut bytes, 0, value.as_bytes().iter().map(|byte| *byte as i8)).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let reader = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (input,)).await?;
    let chars = jvm.instantiate_array("C", 16).await?;

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "([CII)I", (chars.clone(), 0, 1))
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "([CII)I", (chars.clone(), 1, 15))
            .await?,
        9
    );
    let decoded: Vec<JavaChar> = jvm.load_array(&chars, 0, 10).await?;
    assert_eq!(alloc::string::String::from_utf16(&decoded).unwrap(), value);

    let invalid: Result<i32> = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "read", "([CII)I", (chars, -1, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    Ok(())
}

#[tokio::test]
async fn test_input_stream_reader_waits_only_until_a_character_is_available() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            OneByteInputStream::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;

    for (encoding, trailing_lead) in [(encoding_rs::UTF_8, 0xed), (encoding_rs::EUC_KR, 0xc7)] {
        let (encoded, _, _) = encoding.encode("한글");
        let mut bytes = jvm.instantiate_array("B", encoded.len() + 1).await?;
        jvm.store_array(&mut bytes, 0, encoded.iter().map(|byte| *byte as i8)).await?;
        jvm.store_array(&mut bytes, encoded.len(), [trailing_lead as i8]).await?;
        let input = jvm.new_class("OneByteInputStream", "([B)V", (bytes,)).await?;
        let charset = JavaLangString::from_rust_string(&jvm, encoding.name()).await?;
        let reader = jvm
            .new_class(
                "java/io/InputStreamReader",
                "(Ljava/io/InputStream;Ljava/lang/String;)V",
                (input.clone(), charset),
            )
            .await?;
        let chars = jvm.instantiate_array("C", 16).await?;

        for (expected, position) in [('한', encoded.len() / 2), ('글', encoded.len()), ('\u{fffd}', encoded.len() + 1)] {
            assert_eq!(
                jvm.invoke_virtual::<_, i32>(&reader, "java/io/Reader", "read", "([C)I", (chars.clone(),))
                    .await?,
                1
            );
            assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 1).await?, [expected as JavaChar]);
            assert_eq!(
                jvm.get_field::<i32>(&input, "OneByteInputStream", "position", "I").await?,
                position as i32
            );
        }
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&reader, "java/io/Reader", "read", "([C)I", (chars,)).await?,
            -1
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_input_stream_reader_rejects_unknown_encoding() -> Result<()> {
    let jvm = test_jvm().await?;

    let bytes = jvm.instantiate_array("B", 0).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let encoding = JavaLangString::from_rust_string(&jvm, "not-an-encoding").await?;
    let result = jvm
        .new_class(
            "java/io/InputStreamReader",
            "(Ljava/io/InputStream;Ljava/lang/String;)V",
            (input, encoding),
        )
        .await;

    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unknown encoding must throw UnsupportedEncodingException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));

    Ok(())
}

#[tokio::test]
async fn test_reader_default_contract_and_lifecycle() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut bytes = jvm.instantiate_array("B", 3).await?;
    jvm.store_array(&mut bytes, 0, [b'a' as i8, b'b' as i8, b'c' as i8]).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let reader = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (input,)).await?;

    assert!(
        jvm.invoke_virtual::<_, bool>(&reader, &reader.class_definition().name(), "ready", "()Z", ())
            .await?
    );
    assert!(!jvm.invoke_special::<_, bool>(&reader, "java/io/Reader", "ready", "()Z", ()).await?);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        'a' as i32
    );

    let chars = jvm.instantiate_array("C", 4).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "([C)I", (chars.clone(),))
            .await?,
        2
    );
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 2).await?, ['b' as JavaChar, 'c' as JavaChar]);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        -1
    );

    let empty = jvm.instantiate_array("C", 0).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "([CII)I", (empty, 0, 0))
            .await?,
        0
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&reader, &reader.class_definition().name(), "markSupported", "()Z", ())
            .await?
    );

    let mark: Result<()> = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "mark", "(I)V", (1,)).await;
    let Err(JavaError::JavaException(exception)) = mark else {
        panic!("default mark must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    let reset: Result<()> = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "reset", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = reset else {
        panic!("default reset must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    let negative_skip: Result<i64> = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "skip", "(J)J", (-1i64,))
        .await;
    let Err(JavaError::JavaException(exception)) = negative_skip else {
        panic!("negative skip must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let mut bytes = jvm.instantiate_array("B", 3).await?;
    jvm.store_array(&mut bytes, 0, [b'x' as i8, b'y' as i8, b'z' as i8]).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let reader = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (input,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&reader, &reader.class_definition().name(), "skip", "(J)J", (2i64,))
            .await?,
        2
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        'z' as i32
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&reader, &reader.class_definition().name(), "skip", "(J)J", (2i64,))
            .await?,
        0
    );
    let _: () = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "close", "()V", ()).await?;

    let null_lock: ClassInstanceRef<Object> = None.into();
    let result: Result<()> = jvm
        .invoke_special(&reader, "java/io/Reader", "<init>", "(Ljava/lang/Object;)V", (null_lock,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null lock must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
