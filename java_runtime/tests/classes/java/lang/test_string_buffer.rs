use java_constants::{ClassAccessFlags, MethodAccessFlags};
use java_runtime::{
    classes::java::lang::{CharSequence, String as JavaString, StringBuffer},
    get_runtime_class_proto,
};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn string_buffer_char_sequence_sub_sequence_is_a_synchronized_snapshot() -> Result<()> {
    let proto = get_runtime_class_proto("java/lang/StringBuffer").expect("StringBuffer must be registered");
    assert_eq!(proto.parent_class, Some("java/lang/AbstractStringBuilder"));
    assert_eq!(proto.interfaces, vec!["java/io/Serializable", "java/lang/CharSequence"]);
    assert_eq!(proto.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL);
    assert!(proto.fields.is_empty());
    let method = proto
        .methods
        .iter()
        .find(|method| method.name == "subSequence" && method.descriptor == "(II)Ljava/lang/CharSequence;")
        .expect("StringBuffer.subSequence must be registered");
    assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED);

    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (source,)).await?.into();
    let subsequence: ClassInstanceRef<CharSequence> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "subSequence", "(II)Ljava/lang/CharSequence;", (1, 4))
        .await?;
    let text: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(&subsequence, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "ell");

    let _: () = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "setCharAt", "(IC)V", (2, 'X' as JavaChar))
        .await?;
    let text: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(&subsequence, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "ell");

    Ok(())
}

#[tokio::test]
async fn test_string_buffer() -> Result<()> {
    let jvm = test_jvm().await?;

    let string_buffer = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
    let string = JavaLangString::from_rust_string(&jvm, "Hello, ").await?;

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &string_buffer,
            &string_buffer.class_definition().name(),
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (string,),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &string_buffer,
            &string_buffer.class_definition().name(),
            "append",
            "(I)Ljava/lang/StringBuffer;",
            (42,),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &string_buffer,
            &string_buffer.class_definition().name(),
            "append",
            "(Z)Ljava/lang/StringBuffer;",
            (true,),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &string_buffer,
            &string_buffer.class_definition().name(),
            "append",
            "(C)Ljava/lang/StringBuffer;",
            (b'H' as JavaChar,),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &string_buffer,
            &string_buffer.class_definition().name(),
            "append",
            "(J)Ljava/lang/StringBuffer;",
            (42i64,),
        )
        .await?;

    let length: i32 = jvm
        .invoke_virtual(&string_buffer, &string_buffer.class_definition().name(), "length", "()I", ())
        .await?;
    assert_eq!(length, 16);

    let char: JavaChar = jvm
        .invoke_virtual(&string_buffer, &string_buffer.class_definition().name(), "charAt", "(I)C", (7,))
        .await?;
    assert_eq!(char, '4' as JavaChar);

    let result = jvm
        .invoke_virtual(
            &string_buffer,
            &string_buffer.class_definition().name(),
            "toString",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    let result = JavaLangString::to_rust_string(&jvm, &result).await?;

    assert_eq!("Hello, 42trueH42", result);

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &string_buffer,
            &string_buffer.class_definition().name(),
            "delete",
            "(II)Ljava/lang/StringBuffer;",
            (5, 7),
        )
        .await?;
    let result = jvm
        .invoke_virtual(
            &string_buffer,
            &string_buffer.class_definition().name(),
            "toString",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    let result = JavaLangString::to_rust_string(&jvm, &result).await?;
    assert_eq!("Hello42trueH42", result);

    Ok(())
}

#[tokio::test]
async fn test_sb_01_constructors_capacity_and_exceptions() -> Result<()> {
    let jvm = test_jvm().await?;
    let empty = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&empty, &empty.class_definition().name(), "capacity", "()I", ())
            .await?,
        16
    );

    let source = JavaLangString::from_rust_string(&jvm, "abc").await?;
    let from_string = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (source,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&from_string, &from_string.class_definition().name(), "capacity", "()I", ())
            .await?,
        19
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&from_string, &from_string.class_definition().name(), "length", "()I", ())
            .await?,
        3
    );

    let result = jvm.new_class("java/lang/StringBuffer", "(I)V", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("negative StringBuffer capacity must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NegativeArraySizeException"));

    let null = ClassInstanceRef::<java_runtime::classes::java::lang::String>::new(None);
    let result = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (null,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("StringBuffer(null) must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_sb_02_capacity_and_ensure_capacity_growth() -> Result<()> {
    let jvm = test_jvm().await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "(I)V", (2,)).await?;

    let _: () = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "ensureCapacity", "(I)V", (3,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
            .await?,
        6
    );
    let _: () = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "ensureCapacity", "(I)V", (20,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
            .await?,
        20
    );
    let _: () = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "ensureCapacity", "(I)V", (-1,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
            .await?,
        20
    );

    Ok(())
}

#[tokio::test]
async fn test_sb_03_length_char_access_and_synchronized_flags() -> Result<()> {
    let jvm = test_jvm().await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "(I)V", (1,)).await?;

    let _: () = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "setLength", "(I)V", (3,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
            .await?,
        3
    );
    assert_eq!(
        jvm.invoke_virtual::<_, JavaChar>(&buffer, "java/lang/StringBuffer", "charAt", "(I)C", (1,))
            .await?,
        0
    );
    let _: () = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "setCharAt", "(IC)V", (1, 'x' as JavaChar))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, JavaChar>(&buffer, "java/lang/StringBuffer", "charAt", "(I)C", (1,))
            .await?,
        'x' as JavaChar
    );
    let _: () = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "setLength", "(I)V", (1,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
            .await?,
        1
    );

    for (name, descriptor) in [
        ("append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;"),
        ("append", "(Ljava/lang/Object;)Ljava/lang/StringBuffer;"),
        ("append", "(Z)Ljava/lang/StringBuffer;"),
        ("append", "(C)Ljava/lang/StringBuffer;"),
        ("append", "(I)Ljava/lang/StringBuffer;"),
        ("append", "(J)Ljava/lang/StringBuffer;"),
        ("append", "(F)Ljava/lang/StringBuffer;"),
        ("append", "(D)Ljava/lang/StringBuffer;"),
        ("append", "([C)Ljava/lang/StringBuffer;"),
        ("append", "([CII)Ljava/lang/StringBuffer;"),
        ("insert", "(ILjava/lang/String;)Ljava/lang/StringBuffer;"),
        ("insert", "(ILjava/lang/Object;)Ljava/lang/StringBuffer;"),
        ("insert", "(IC)Ljava/lang/StringBuffer;"),
        ("insert", "(I[C)Ljava/lang/StringBuffer;"),
        ("delete", "(II)Ljava/lang/StringBuffer;"),
        ("deleteCharAt", "(I)Ljava/lang/StringBuffer;"),
        ("replace", "(IILjava/lang/String;)Ljava/lang/StringBuffer;"),
        ("substring", "(I)Ljava/lang/String;"),
        ("substring", "(II)Ljava/lang/String;"),
        ("capacity", "()I"),
        ("ensureCapacity", "(I)V"),
        ("length", "()I"),
        ("setLength", "(I)V"),
        ("charAt", "(I)C"),
        ("setCharAt", "(IC)V"),
        ("getChars", "(II[CI)V"),
        ("reverse", "()Ljava/lang/StringBuffer;"),
        ("toString", "()Ljava/lang/String;"),
    ] {
        let flags = jvm
            .get_class("java/lang/StringBuffer")
            .expect("StringBuffer must be loaded")
            .definition
            .method(name, descriptor, false)
            .unwrap_or_else(|| panic!("missing {name}{descriptor}"))
            .access_flags();
        assert!(flags.contains(MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED));
    }

    for descriptor in [
        "(ILjava/lang/CharSequence;)Ljava/lang/StringBuffer;",
        "(IZ)Ljava/lang/StringBuffer;",
        "(II)Ljava/lang/StringBuffer;",
        "(IJ)Ljava/lang/StringBuffer;",
        "(IF)Ljava/lang/StringBuffer;",
        "(ID)Ljava/lang/StringBuffer;",
    ] {
        let flags = jvm
            .get_class("java/lang/StringBuffer")
            .expect("StringBuffer must be loaded")
            .definition
            .method("insert", descriptor, false)
            .unwrap_or_else(|| panic!("missing insert{descriptor}"))
            .access_flags();
        assert!(flags.contains(MethodAccessFlags::PUBLIC));
        assert!(!flags.contains(MethodAccessFlags::SYNCHRONIZED));
    }

    for index in [-1, 1] {
        let result: Result<JavaChar> = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "charAt", "(I)C", (index,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("charAt must reject index {index}");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));

        let result: Result<()> = jvm
            .invoke_virtual(&buffer, "java/lang/StringBuffer", "setCharAt", "(IC)V", (index, 'z' as JavaChar))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("setCharAt must reject index {index}");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
    }

    let before_text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    let before_text = JavaLangString::to_rust_string(&jvm, &before_text).await?;
    let before_length: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "length", "()I", ()).await?;
    let before_capacity: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "capacity", "()I", ()).await?;
    let result: Result<()> = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "setLength", "(I)V", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("setLength must reject a negative length");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, before_text);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
            .await?,
        before_length
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
            .await?,
        before_capacity
    );

    Ok(())
}

#[tokio::test]
async fn test_sb_04_get_chars_checks_source_and_destination_ranges() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "abcd").await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (source,)).await?;
    let mut destination = jvm.instantiate_array("C", 5).await?;
    jvm.store_array(&mut destination, 0, ['_' as JavaChar; 5]).await?;

    let _: () = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "getChars", "(II[CI)V", (1, 3, destination.clone(), 2))
        .await?;
    assert_eq!(
        jvm.load_array::<JavaChar>(&destination, 0, 5).await?,
        ['_' as JavaChar, '_' as JavaChar, 'b' as JavaChar, 'c' as JavaChar, '_' as JavaChar]
    );

    for (start, end) in [(-1, 1), (2, 1), (0, 5)] {
        let result: Result<()> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "getChars",
                "(II[CI)V",
                (start, end, destination.clone(), 0),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("getChars must reject source range ({start}, {end})");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
    }

    for destination_offset in [-1, 4] {
        let result: Result<()> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "getChars",
                "(II[CI)V",
                (0, 2, destination.clone(), destination_offset),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("getChars must reject destination offset {destination_offset}");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    }

    let null: ClassInstanceRef<Array<JavaChar>> = None.into();
    let result: Result<()> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "getChars", "(II[CI)V", (0, 1, null, 0))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("getChars must reject a null destination");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_sb_05_append_float_double_and_char_array() -> Result<()> {
    let jvm = test_jvm().await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
    let mut chars = jvm.instantiate_array("C", 2).await?;
    jvm.store_array(&mut chars, 0, ['x' as JavaChar, 'y' as JavaChar]).await?;

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "append", "(F)Ljava/lang/StringBuffer;", (1.0f32,))
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "append", "(D)Ljava/lang/StringBuffer;", (-0.0f64,))
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "append", "([C)Ljava/lang/StringBuffer;", (chars,))
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "1.0-0.0xy");

    let null: ClassInstanceRef<Array<JavaChar>> = None.into();
    let result: Result<ClassInstanceRef<StringBuffer>> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "append", "([C)Ljava/lang/StringBuffer;", (null,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("append(char[]) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null_string: ClassInstanceRef<java_runtime::classes::java::lang::String> = None.into();
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (null_string,),
        )
        .await?;
    let null_object: ClassInstanceRef<java_runtime::classes::java::lang::Object> = None.into();
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/Object;)Ljava/lang/StringBuffer;",
            (null_object,),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "1.0-0.0xynullnull");

    Ok(())
}

#[tokio::test]
async fn test_sb_06_insert_overloads_and_boundaries() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "ab").await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (source,)).await?;
    let text = JavaLangString::from_rust_string(&jvm, "S").await?;
    let object = JavaLangString::from_rust_string(&jvm, "O").await?;
    let mut chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut chars, 0, ['C' as JavaChar]).await?;

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (0, text),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/Object;)Ljava/lang/StringBuffer;",
            (1, object),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "insert", "(IZ)Ljava/lang/StringBuffer;", (2, true))
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(IC)Ljava/lang/StringBuffer;",
            (6, '!' as JavaChar),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "insert", "(II)Ljava/lang/StringBuffer;", (7, 12))
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "insert", "(IJ)Ljava/lang/StringBuffer;", (9, 34i64))
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "insert", "(IF)Ljava/lang/StringBuffer;", (11, 1.0f32))
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "insert", "(ID)Ljava/lang/StringBuffer;", (14, 2.0f64))
        .await?;
    let length: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "length", "()I", ()).await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(I[C)Ljava/lang/StringBuffer;",
            (length, chars),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "SOtrue!12341.02.0abC");

    let sequence = JavaLangString::from_rust_string(&jvm, "string").await?;
    let sequence: ClassInstanceRef<CharSequence> = sequence.into();
    let length: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "length", "()I", ()).await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/CharSequence;)Ljava/lang/StringBuffer;",
            (length, sequence),
        )
        .await?;
    let sequence = JavaLangString::from_rust_string(&jvm, "buffer").await?;
    let sequence = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (sequence,)).await?;
    let sequence: ClassInstanceRef<CharSequence> = sequence.into();
    let length: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "length", "()I", ()).await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/CharSequence;)Ljava/lang/StringBuffer;",
            (length, sequence),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "SOtrue!12341.02.0abCstringbuffer");

    let sequence: ClassInstanceRef<CharSequence> = None.into();
    let length: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "length", "()I", ()).await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/CharSequence;)Ljava/lang/StringBuffer;",
            (length, sequence),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "SOtrue!12341.02.0abCstringbuffernull");

    let null_string: ClassInstanceRef<java_runtime::classes::java::lang::String> = None.into();
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (0, null_string),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "substring", "(II)Ljava/lang/String;", (0, 4))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "null");

    let null_object: ClassInstanceRef<java_runtime::classes::java::lang::Object> = None.into();
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/Object;)Ljava/lang/StringBuffer;",
            (4, null_object),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "substring", "(II)Ljava/lang/String;", (0, 8))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "nullnull");

    let before_text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    let before_text = JavaLangString::to_rust_string(&jvm, &before_text).await?;
    let before_length: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "length", "()I", ()).await?;
    let before_capacity: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "capacity", "()I", ()).await?;
    for offset in [-1, 100] {
        let result: Result<ClassInstanceRef<StringBuffer>> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "insert",
                "(IC)Ljava/lang/StringBuffer;",
                (offset, 'x' as JavaChar),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("insert must reject offset {offset}");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
        let after_text = jvm
            .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &after_text).await?, before_text);
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
                .await?,
            before_length
        );
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
                .await?,
            before_capacity
        );
    }

    let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
    let result: Result<ClassInstanceRef<StringBuffer>> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(I[C)Ljava/lang/StringBuffer;",
            (0, null_chars),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("insert(char[]) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for offset in [-1, before_length + 1] {
        let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
        let result: Result<ClassInstanceRef<StringBuffer>> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "insert",
                "(I[C)Ljava/lang/StringBuffer;",
                (offset, null_chars),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("insert(char[]) must validate invalid offset {offset} before null");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
        let after_text = jvm
            .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &after_text).await?, before_text);
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
                .await?,
            before_length
        );
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
                .await?,
            before_capacity
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_sb_07_reverse_uses_raw_utf16_code_units() -> Result<()> {
    let jvm = test_jvm().await?;
    let mut chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars, 0, [0xd800, 'a' as JavaChar, 0xdc00]).await?;
    let string = jvm.new_class("java/lang/String", "([C)V", (chars,)).await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (string,)).await?;

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "reverse", "()Ljava/lang/StringBuffer;", ())
        .await?;
    let reversed = jvm.instantiate_array("C", 3).await?;
    let _: () = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "getChars", "(II[CI)V", (0, 3, reversed.clone(), 0))
        .await?;
    assert_eq!(jvm.load_array::<JavaChar>(&reversed, 0, 3).await?, [0xdc00, 'a' as JavaChar, 0xd800]);

    Ok(())
}

#[tokio::test]
async fn test_sb_08_delete_and_delete_char_at_ranges() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "abcdef").await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (source,)).await?;
    let initial_capacity: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "capacity", "()I", ()).await?;

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "delete", "(II)Ljava/lang/StringBuffer;", (2, 100))
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "deleteCharAt", "(I)Ljava/lang/StringBuffer;", (1,))
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "a");
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
            .await?,
        initial_capacity
    );

    let before_text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    let before_text = JavaLangString::to_rust_string(&jvm, &before_text).await?;
    let before_length: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "length", "()I", ()).await?;
    for (start, end) in [(-1, 1), (2, 1), (2, 2)] {
        let result: Result<ClassInstanceRef<StringBuffer>> = jvm
            .invoke_virtual(&buffer, "java/lang/StringBuffer", "delete", "(II)Ljava/lang/StringBuffer;", (start, end))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("delete must reject ({start}, {end})");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
        let after_text = jvm
            .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &after_text).await?, before_text);
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
                .await?,
            before_length
        );
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
                .await?,
            initial_capacity
        );
    }

    for index in [-1, 1] {
        let result: Result<ClassInstanceRef<StringBuffer>> = jvm
            .invoke_virtual(&buffer, "java/lang/StringBuffer", "deleteCharAt", "(I)Ljava/lang/StringBuffer;", (index,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("deleteCharAt must reject {index}");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
        let after_text = jvm
            .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &after_text).await?, before_text);
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
                .await?,
            before_length
        );
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
                .await?,
            initial_capacity
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_sb_09_substring_uses_logical_count() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "abcdef").await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (source,)).await?;

    let tail = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "substring", "(I)Ljava/lang/String;", (2,))
        .await?;
    let middle = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "substring", "(II)Ljava/lang/String;", (1, 4))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &tail).await?, "cdef");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &middle).await?, "bcd");

    for (start, end) in [(-1, 1), (3, 2), (0, 7)] {
        let result: Result<ClassInstanceRef<java_runtime::classes::java::lang::String>> = jvm
            .invoke_virtual(&buffer, "java/lang/StringBuffer", "substring", "(II)Ljava/lang/String;", (start, end))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("substring must reject ({start}, {end})");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
    }

    Ok(())
}

#[tokio::test]
async fn test_sb_10_replace_clamps_end_and_checks_ranges() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "abcdef").await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (source,)).await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "XY").await?;

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "replace",
            "(IILjava/lang/String;)Ljava/lang/StringBuffer;",
            (2, 100, replacement),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "abXY");

    let before_text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    let before_text = JavaLangString::to_rust_string(&jvm, &before_text).await?;
    let before_length: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "length", "()I", ()).await?;
    let before_capacity: i32 = jvm.invoke_virtual(&buffer, "java/lang/StringBuffer", "capacity", "()I", ()).await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "z").await?;
    for (start, end) in [(-1, 1), (3, 2), (5, 5)] {
        let result: Result<ClassInstanceRef<StringBuffer>> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "replace",
                "(IILjava/lang/String;)Ljava/lang/StringBuffer;",
                (start, end, replacement.clone()),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("replace must reject ({start}, {end})");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
        let after_text = jvm
            .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &after_text).await?, before_text);
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
                .await?,
            before_length
        );
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
                .await?,
            before_capacity
        );
    }

    let null: ClassInstanceRef<java_runtime::classes::java::lang::String> = None.into();
    let result: Result<ClassInstanceRef<StringBuffer>> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "replace",
            "(IILjava/lang/String;)Ljava/lang/StringBuffer;",
            (0, 1, null),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("replace must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    let after_text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &after_text).await?, before_text);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
            .await?,
        before_length
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
            .await?,
        before_capacity
    );

    let null: ClassInstanceRef<java_runtime::classes::java::lang::String> = None.into();
    let result: Result<ClassInstanceRef<StringBuffer>> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "replace",
            "(IILjava/lang/String;)Ljava/lang/StringBuffer;",
            (-1, 1, null),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("replace must validate its range before dereferencing the replacement");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
    let after_text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &after_text).await?, before_text);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
            .await?,
        before_length
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "capacity", "()I", ())
            .await?,
        before_capacity
    );

    Ok(())
}

#[tokio::test]
async fn test_sb_11_string_arguments_backed_by_shared_substring() -> Result<()> {
    let jvm = test_jvm().await?;

    let hello_parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let hello: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(
            &hello_parent,
            &hello_parent.class_definition().name(),
            "substring",
            "(II)Ljava/lang/String;",
            (2, 7),
        )
        .await?;
    let world_parent = JavaLangString::from_rust_string(&jvm, "zzWorldzz").await?;
    let world: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(
            &world_parent,
            &world_parent.class_definition().name(),
            "substring",
            "(II)Ljava/lang/String;",
            (2, 7),
        )
        .await?;

    let buffer = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (hello.clone(),)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&buffer, "java/lang/StringBuffer", "length", "()I", ())
            .await?,
        5
    );

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (hello.clone(),),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "HelloHello");

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (0, world.clone()),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "WorldHelloHello");

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "replace",
            "(IILjava/lang/String;)Ljava/lang/StringBuffer;",
            (5, 10, world),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "WorldWorldHello");

    Ok(())
}

#[tokio::test]
async fn test_sb_12_to_string_snapshot_is_exact_size_and_immutable() -> Result<()> {
    let jvm = test_jvm().await?;

    let buffer = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
    let hello = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (hello,),
        )
        .await?;

    let string = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "Hello");

    let buffer_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&buffer, "value", "[C").await?;
    let string_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
    assert_ne!(buffer_value.identity(), string_value.identity());
    assert_eq!(jvm.array_length(&string_value).await?, 5);
    assert_eq!(jvm.get_field::<i32>(&string, "offset", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&string, "count", "I").await?, 5);

    let world = JavaLangString::from_rust_string(&jvm, "World").await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (world,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "setCharAt", "(IC)V", (0, b'X' as JavaChar))
        .await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "Hello");

    Ok(())
}

#[tokio::test]
async fn test_sb_13_insert_substring_then_reverse_and_delete() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let sub: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(&parent, &parent.class_definition().name(), "substring", "(II)Ljava/lang/String;", (2, 7))
        .await?;

    let buffer = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
    let ab = JavaLangString::from_rust_string(&jvm, "ab").await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (ab,),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (1, sub.clone()),
        )
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "aHellob");

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "reverse", "()Ljava/lang/StringBuffer;", ())
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "bolleHa");

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "delete", "(II)Ljava/lang/StringBuffer;", (1, 5))
        .await?;
    let text = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "bHa");

    // the buffer mutations must never leak into the shared parent
    assert_eq!(JavaLangString::to_rust_string(&jvm, &parent).await?, "xxHelloyy");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &sub).await?, "Hello");

    Ok(())
}

#[tokio::test]
async fn test_sb_14_append_surrogate_substring_preserves_code_unit() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars, 0, [0x61 as JavaChar, 0xd800, 0x62]).await?;
    let string = jvm.new_class("java/lang/String", "([C)V", (chars,)).await?;
    let sub: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(&string, &string.class_definition().name(), "substring", "(II)Ljava/lang/String;", (1, 2))
        .await?;

    let buffer = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &buffer,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (sub,),
        )
        .await?;
    let text: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&text, "java/lang/String", "length", "()I", ()).await?, 1);
    assert_eq!(
        jvm.invoke_virtual::<_, JavaChar>(&text, "java/lang/String", "charAt", "(I)C", (0,))
            .await?,
        0xd800
    );

    Ok(())
}
