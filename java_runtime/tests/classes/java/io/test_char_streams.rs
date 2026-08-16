use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};

use java_runtime::classes::java::lang::String;
use test_utils::test_jvm;

#[tokio::test]
async fn cr_01_cr_02_string_reader_contract() -> Result<()> {
    let jvm = test_jvm().await?;

    let null_string: ClassInstanceRef<String> = None.into();
    let result = jvm.new_class("java/io/StringReader", "(Ljava/lang/String;)V", (null_string,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null string must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let value = JavaLangString::from_rust_string(&jvm, "A한BC").await?;
    let reader = jvm.new_class("java/io/StringReader", "(Ljava/lang/String;)V", (value,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&reader, &reader.class_definition().name(), "ready", "()Z", ())
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&reader, &reader.class_definition().name(), "markSupported", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        'A' as i32
    );

    let _: () = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "mark", "(I)V", (10,))
        .await?;
    let chars = jvm.instantiate_array("C", 5).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "([CII)I", (chars.clone(), 1, 2))
            .await?,
        2
    );
    assert_eq!(
        jvm.load_array::<JavaChar>(&chars, 0, 5).await?,
        [0, '한' as JavaChar, 'B' as JavaChar, 0, 0]
    );
    let _: () = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "reset", "()V", ()).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        '한' as i32
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&reader, &reader.class_definition().name(), "skip", "(J)J", (-2i64,))
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&reader, &reader.class_definition().name(), "skip", "(J)J", (10i64,))
            .await?,
        2
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "([CII)I", (chars.clone(), 0, 0))
            .await?,
        0
    );

    let invalid: Result<i32> = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "read", "([CII)I", (chars, -1, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    let invalid_mark: Result<()> = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "mark", "(I)V", (-1,))
        .await;
    let Err(JavaError::JavaException(exception)) = invalid_mark else {
        panic!("negative read-ahead limit must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let _: () = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "close", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "close", "()V", ()).await?;
    let closed: Result<bool> = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "ready", "()Z", ()).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("ready after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let closed: Result<i32> = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "read", "()I", ()).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("read after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
    let closed: Result<i32> = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "read", "([CII)I", (null_chars, -1, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("closed StringReader must throw IOException before validating a null target");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let target = jvm.instantiate_array("C", 1).await?;
    let closed: Result<i32> = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "read", "([CII)I", (target, 2, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("closed StringReader must throw IOException before validating the target range");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    Ok(())
}

#[tokio::test]
async fn cr_03_cr_04_char_array_reader_contract() -> Result<()> {
    let jvm = test_jvm().await?;

    let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
    let result = jvm.new_class("java/io/CharArrayReader", "([C)V", (null_chars,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null array must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let chars = jvm.instantiate_array("C", 3).await?;
    let result = jvm.new_class("java/io/CharArrayReader", "([CII)V", (chars, -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("invalid source range must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let mut chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut chars, 0, ['X' as JavaChar]).await?;
    let default_reader = jvm.new_class("java/io/CharArrayReader", "([C)V", (chars,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&default_reader, &default_reader.class_definition().name(), "read", "()I", ())
            .await?,
        'X' as i32
    );

    let mut chars = jvm.instantiate_array("C", 5).await?;
    jvm.store_array(
        &mut chars,
        0,
        ['0' as JavaChar, 'A' as JavaChar, 'B' as JavaChar, 'C' as JavaChar, '4' as JavaChar],
    )
    .await?;
    let reader = jvm.new_class("java/io/CharArrayReader", "([CII)V", (chars, 1, 3)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&reader, &reader.class_definition().name(), "ready", "()Z", ())
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&reader, &reader.class_definition().name(), "markSupported", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        'A' as i32
    );
    let _: () = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "reset", "()V", ()).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        'A' as i32
    );
    let _: () = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "mark", "(I)V", (2,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&reader, &reader.class_definition().name(), "skip", "(J)J", (1i64,))
            .await?,
        1
    );
    let _: () = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "reset", "()V", ()).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        'B' as i32
    );

    let target = jvm.instantiate_array("C", 2).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "([CII)I", (target.clone(), 0, 1))
            .await?,
        1
    );
    assert_eq!(jvm.load_array::<JavaChar>(&target, 0, 2).await?, ['C' as JavaChar, 0]);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "()I", ())
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reader, &reader.class_definition().name(), "read", "([CII)I", (target.clone(), 0, 0))
            .await?,
        0
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&reader, &reader.class_definition().name(), "ready", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&reader, &reader.class_definition().name(), "skip", "(J)J", (-1i64,))
            .await?,
        0
    );

    let invalid: Result<i32> = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "read", "([CII)I", (target, 2, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid target range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let _: () = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "close", "()V", ()).await?;
    let closed: Result<i32> = jvm.invoke_virtual(&reader, &reader.class_definition().name(), "read", "()I", ()).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("read after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
    let closed: Result<i32> = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "read", "([CII)I", (null_chars, -1, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("closed CharArrayReader must throw IOException before validating a null target");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let target = jvm.instantiate_array("C", 1).await?;
    let closed: Result<i32> = jvm
        .invoke_virtual(&reader, &reader.class_definition().name(), "read", "([CII)I", (target, 2, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("closed CharArrayReader must throw IOException before validating the target range");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    Ok(())
}

#[tokio::test]
async fn cw_01_cw_02_char_array_writer_contract() -> Result<()> {
    let jvm = test_jvm().await?;

    let result = jvm.new_class("java/io/CharArrayWriter", "(I)V", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("negative size must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let writer = jvm.new_class("java/io/CharArrayWriter", "(I)V", (1,)).await?;
    let _: () = jvm
        .invoke_virtual(&writer, &writer.class_definition().name(), "write", "(I)V", ('A' as i32,))
        .await?;
    let mut chars = jvm.instantiate_array("C", 4).await?;
    jvm.store_array(&mut chars, 0, ['0' as JavaChar, 'B' as JavaChar, 'C' as JavaChar, '3' as JavaChar])
        .await?;
    let _: () = jvm
        .invoke_virtual(&writer, &writer.class_definition().name(), "write", "([CII)V", (chars.clone(), 1, 2))
        .await?;
    let value = JavaLangString::from_rust_string(&jvm, "xDEy").await?;
    let _: () = jvm
        .invoke_virtual(
            &writer,
            &writer.class_definition().name(),
            "write",
            "(Ljava/lang/String;II)V",
            (value, 1, 2),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&writer, &writer.class_definition().name(), "size", "()I", ())
            .await?,
        5
    );

    let invalid: Result<()> = jvm
        .invoke_virtual(&writer, &writer.class_definition().name(), "write", "([CII)V", (chars, -1, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let copy: ClassInstanceRef<Array<JavaChar>> = jvm
        .invoke_virtual(&writer, &writer.class_definition().name(), "toCharArray", "()[C", ())
        .await?;
    assert_eq!(
        jvm.load_array::<JavaChar>(&copy, 0, 5).await?,
        ['A' as JavaChar, 'B' as JavaChar, 'C' as JavaChar, 'D' as JavaChar, 'E' as JavaChar]
    );
    let mut copy = copy;
    jvm.store_array(&mut copy, 0, ['Z' as JavaChar]).await?;

    let string: ClassInstanceRef<String> = jvm
        .invoke_virtual(&writer, &writer.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "ABCDE");
    let output = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(
            &writer,
            &writer.class_definition().name(),
            "writeTo",
            "(Ljava/io/Writer;)V",
            (output.clone(),),
        )
        .await?;
    let string: ClassInstanceRef<String> = jvm
        .invoke_virtual(&output, &output.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "ABCDE");

    let _: () = jvm.invoke_virtual(&writer, &writer.class_definition().name(), "flush", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&writer, &writer.class_definition().name(), "close", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(&writer, &writer.class_definition().name(), "write", "(I)V", ('F' as i32,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&writer, &writer.class_definition().name(), "size", "()I", ())
            .await?,
        6
    );
    let _: () = jvm.invoke_virtual(&writer, &writer.class_definition().name(), "reset", "()V", ()).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&writer, &writer.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );

    let default_writer = jvm.new_class("java/io/CharArrayWriter", "()V", ()).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&default_writer, &default_writer.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );

    Ok(())
}
