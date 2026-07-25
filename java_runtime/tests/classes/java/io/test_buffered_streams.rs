use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::get_runtime_class_proto;
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};

use java_runtime::classes::java::io::{InputStream, OutputStream, Writer};
use java_runtime::classes::java::lang::Object;
use test_utils::test_jvm;

#[tokio::test]
async fn bio_01_bio_02_bio_03_buffered_input_stream_contract() -> Result<()> {
    let jvm = test_jvm().await?;

    let null_input: ClassInstanceRef<InputStream> = None.into();
    let result = jvm
        .new_class("java/io/BufferedInputStream", "(Ljava/io/InputStream;)V", (null_input,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null input must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let data = jvm.instantiate_array("B", 0).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data,)).await?;
    let result = jvm
        .new_class("java/io/BufferedInputStream", "(Ljava/io/InputStream;I)V", (input, 0))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("zero buffer size must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let mut data = jvm.instantiate_array("B", 1).await?;
    jvm.store_array(&mut data, 0, [7i8]).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data,)).await?;
    let default_stream = jvm.new_class("java/io/BufferedInputStream", "(Ljava/io/InputStream;)V", (input,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&default_stream, "read", "()I", ()).await?, 7);

    let mut data = jvm.instantiate_array("B", 6).await?;
    jvm.store_array(&mut data, 0, [10i8, 20, 30, 40, 50, 60]).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data,)).await?;
    let stream = jvm
        .new_class("java/io/BufferedInputStream", "(Ljava/io/InputStream;I)V", (input, 2))
        .await?;

    assert!(jvm.invoke_virtual::<_, bool>(&stream, "markSupported", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&stream, "available", "()I", ()).await?, 6);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&stream, "read", "()I", ()).await?, 10);

    let _: () = jvm.invoke_virtual(&stream, "mark", "(I)V", (4,)).await?;
    let target = jvm.instantiate_array("B", 6).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&stream, "read", "([BII)I", (target.clone(), 1, 3)).await?, 3);
    assert_eq!(jvm.load_array::<i8>(&target, 0, 6).await?, [0, 20, 30, 40, 0, 0]);
    let _: () = jvm.invoke_virtual(&stream, "reset", "()V", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&stream, "read", "()I", ()).await?, 20);
    assert_eq!(jvm.invoke_virtual::<_, i64>(&stream, "skip", "(J)J", (2i64,)).await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&stream, "read", "()I", ()).await?, 50);
    assert_eq!(jvm.invoke_virtual::<_, i64>(&stream, "skip", "(J)J", (-1i64,)).await?, 0);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&stream, "read", "()I", ()).await?, 60);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&stream, "read", "()I", ()).await?, -1);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&stream, "read", "([BII)I", (target.clone(), 0, 0)).await?, 0);

    let invalid: Result<i32> = jvm.invoke_virtual(&stream, "read", "([BII)I", (target, -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let mut data = jvm.instantiate_array("B", 5).await?;
    jvm.store_array(&mut data, 0, [1i8, 2, 3, 4, 5]).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data,)).await?;
    let stream = jvm
        .new_class("java/io/BufferedInputStream", "(Ljava/io/InputStream;I)V", (input, 2))
        .await?;
    let null_bytes: ClassInstanceRef<Array<i8>> = None.into();
    let invalid: Result<i32> = jvm.invoke_virtual(&stream, "read", "([BII)I", (null_bytes, -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("open stream must reject a null array with NullPointerException before validating the range");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    let _: () = jvm.invoke_virtual(&stream, "mark", "(I)V", (2,)).await?;
    let bytes = jvm.instantiate_array("B", 3).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&stream, "read", "([BII)I", (bytes, 0, 3)).await?, 3);
    let reset: Result<()> = jvm.invoke_virtual(&stream, "reset", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = reset else {
        panic!("reading past mark limit must invalidate the mark");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    let closed: Result<i32> = jvm.invoke_virtual(&stream, "read", "()I", ()).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("read after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let closed: Result<i32> = jvm.invoke_virtual(&stream, "available", "()I", ()).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("available after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let empty = jvm.instantiate_array("B", 0).await?;
    let closed: Result<i32> = jvm.invoke_virtual(&stream, "read", "([BII)I", (empty, 0, 0)).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("zero-length read after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let null_bytes: ClassInstanceRef<Array<i8>> = None.into();
    let closed: Result<i32> = jvm.invoke_virtual(&stream, "read", "([BII)I", (null_bytes, -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("closed stream must reject a null array with IOException before validating arguments");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let bytes = jvm.instantiate_array("B", 1).await?;
    let closed: Result<i32> = jvm.invoke_virtual(&stream, "read", "([BII)I", (bytes, -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("closed stream must reject an invalid range with IOException before validating arguments");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let closed: Result<i64> = jvm.invoke_virtual(&stream, "skip", "(J)J", (0i64,)).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("zero skip after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let position: i32 = jvm.get_field(&stream, "pos", "I").await?;
    let _: () = jvm.invoke_virtual(&stream, "mark", "(I)V", (17,)).await?;
    assert_eq!(jvm.get_field::<i32>(&stream, "marklimit", "I").await?, 17);
    assert_eq!(jvm.get_field::<i32>(&stream, "markpos", "I").await?, position);
    let closed: Result<()> = jvm.invoke_virtual(&stream, "reset", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("reset after a post-close mark must still throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    Ok(())
}

#[tokio::test]
async fn bio_04_bio_05_buffered_output_stream_contract() -> Result<()> {
    let jvm = test_jvm().await?;

    let null_output: ClassInstanceRef<OutputStream> = None.into();
    let result = jvm
        .new_class("java/io/BufferedOutputStream", "(Ljava/io/OutputStream;)V", (null_output,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null output must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let result = jvm
        .new_class("java/io/BufferedOutputStream", "(Ljava/io/OutputStream;I)V", (output, 0))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("zero buffer size must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let default_stream = jvm
        .new_class("java/io/BufferedOutputStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&default_stream, "write", "(I)V", (8,)).await?;
    let _: () = jvm.invoke_virtual(&default_stream, "flush", "()V", ()).await?;
    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    assert_eq!(jvm.load_array::<i8>(&bytes, 0, 1).await?, [8]);

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let stream = jvm
        .new_class("java/io/BufferedOutputStream", "(Ljava/io/OutputStream;I)V", (output.clone(), 3))
        .await?;

    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (1,)).await?;
    let mut bytes = jvm.instantiate_array("B", 5).await?;
    jvm.store_array(&mut bytes, 0, [10i8, 20, 30, 40, 50]).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "([BII)V", (bytes.clone(), 1, 2)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&output, "size", "()I", ()).await?, 0);

    let _: () = jvm.invoke_virtual(&stream, "write", "([BII)V", (bytes.clone(), 2, 3)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&output, "size", "()I", ()).await?,
        6,
        "a large write must flush buffered bytes before writing through"
    );

    let invalid: Result<()> = jvm.invoke_virtual(&stream, "write", "([BII)V", (bytes.clone(), -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    let _: () = jvm.invoke_virtual(&stream, "write", "([BII)V", (bytes, 5, 0)).await?;

    let _: () = jvm.invoke_virtual(&stream, "flush", "()V", ()).await?;
    let actual: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    assert_eq!(jvm.load_array::<i8>(&actual, 0, 6).await?, [1, 20, 30, 30, 40, 50]);

    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (99,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    let actual: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    assert_eq!(jvm.load_array::<i8>(&actual, 0, 7).await?, [1, 20, 30, 30, 40, 50, 99]);

    let closed: Result<()> = jvm.invoke_virtual(&stream, "write", "(I)V", (100,)).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("write after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let closed: Result<()> = jvm.invoke_virtual(&stream, "flush", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("flush after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    Ok(())
}

#[tokio::test]
async fn bio_06_bio_07_buffered_writer_contract() -> Result<()> {
    let jvm = test_jvm().await?;

    let null_writer: ClassInstanceRef<Writer> = None.into();
    let result = jvm.new_class("java/io/BufferedWriter", "(Ljava/io/Writer;)V", (null_writer,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null writer must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let output = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let result = jvm.new_class("java/io/BufferedWriter", "(Ljava/io/Writer;I)V", (output, 0)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("zero buffer size must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let key = JavaLangString::from_rust_string(&jvm, "line.separator").await?;
    let separator = JavaLangString::from_rust_string(&jvm, "|").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key, separator),
        )
        .await?;

    let default_output = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let default_writer = jvm
        .new_class("java/io/BufferedWriter", "(Ljava/io/Writer;)V", (default_output.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&default_writer, "write", "(I)V", ('V' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&default_writer, "flush", "()V", ()).await?;
    let actual: ClassInstanceRef<java_runtime::classes::java::lang::String> =
        jvm.invoke_virtual(&default_output, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, "V");

    let output = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let writer = jvm
        .new_class("java/io/BufferedWriter", "(Ljava/io/Writer;I)V", (output.clone(), 4))
        .await?;
    let lock: ClassInstanceRef<Object> = jvm.get_field(&writer, "lock", "Ljava/lang/Object;").await?;
    assert_eq!(
        lock.identity(),
        output.identity(),
        "BufferedWriter must use its backing writer as Writer.lock"
    );
    let _: () = jvm.invoke_virtual(&writer, "write", "(I)V", ('A' as i32,)).await?;
    let mut chars = jvm.instantiate_array("C", 5).await?;
    jvm.store_array(
        &mut chars,
        0,
        ['0' as JavaChar, 'B' as JavaChar, 'C' as JavaChar, 'D' as JavaChar, '4' as JavaChar],
    )
    .await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "([CII)V", (chars.clone(), 1, 2)).await?;
    let value: ClassInstanceRef<java_runtime::classes::java::lang::String> =
        jvm.invoke_virtual(&output, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "");

    let value = JavaLangString::from_rust_string(&jvm, "xyZ!").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;II)V", (value, 1, 3)).await?;
    let _: () = jvm.invoke_virtual(&writer, "newLine", "()V", ()).await?;

    let invalid: Result<()> = jvm.invoke_virtual(&writer, "write", "([CII)V", (chars, -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let _: () = jvm.invoke_virtual(&writer, "flush", "()V", ()).await?;
    let actual: ClassInstanceRef<java_runtime::classes::java::lang::String> =
        jvm.invoke_virtual(&output, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, "ABCyZ!|");

    let _: () = jvm.invoke_virtual(&writer, "write", "(I)V", ('Q' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;
    let actual: ClassInstanceRef<java_runtime::classes::java::lang::String> =
        jvm.invoke_virtual(&output, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, "ABCyZ!|Q");

    let closed: Result<()> = jvm.invoke_virtual(&writer, "write", "(I)V", ('R' as i32,)).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("write after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let closed: Result<()> = jvm.invoke_virtual(&writer, "flush", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("flush after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
    let closed: Result<()> = jvm.invoke_virtual(&writer, "write", "([CII)V", (null_chars, -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("closed writer must reject a null array with IOException before validating arguments");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let null_string: ClassInstanceRef<java_runtime::classes::java::lang::String> = None.into();
    let closed: Result<()> = jvm
        .invoke_virtual(&writer, "write", "(Ljava/lang/String;II)V", (null_string, -1, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("closed writer must reject a null string with IOException before validating arguments");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    let output = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let writer = jvm
        .new_class("java/io/BufferedWriter", "(Ljava/io/Writer;I)V", (output.clone(), 2))
        .await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(I)V", ('P' as i32,)).await?;
    let mut chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars, 0, ['A' as JavaChar, 'B' as JavaChar, 'C' as JavaChar])
        .await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "([CII)V", (chars, 0, 3)).await?;
    let actual: ClassInstanceRef<java_runtime::classes::java::lang::String> =
        jvm.invoke_virtual(&output, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(
        JavaLangString::to_rust_string(&jvm, &actual).await?,
        "PABC",
        "a large write must flush pending chars before writing through"
    );

    Ok(())
}

#[test]
fn buffered_stream_descriptors_access_and_jdk_state_fields() {
    for (class_name, parent, methods, synchronized_methods) in [
        (
            "java/io/BufferedInputStream",
            "java/io/FilterInputStream",
            &[
                ("<init>", "(Ljava/io/InputStream;)V"),
                ("<init>", "(Ljava/io/InputStream;I)V"),
                ("read", "()I"),
                ("read", "([BII)I"),
                ("skip", "(J)J"),
                ("available", "()I"),
                ("mark", "(I)V"),
                ("reset", "()V"),
                ("markSupported", "()Z"),
                ("close", "()V"),
            ][..],
            &[
                ("read", "()I"),
                ("read", "([BII)I"),
                ("skip", "(J)J"),
                ("available", "()I"),
                ("mark", "(I)V"),
                ("reset", "()V"),
            ][..],
        ),
        (
            "java/io/BufferedOutputStream",
            "java/io/FilterOutputStream",
            &[
                ("<init>", "(Ljava/io/OutputStream;)V"),
                ("<init>", "(Ljava/io/OutputStream;I)V"),
                ("write", "(I)V"),
                ("write", "([BII)V"),
                ("flush", "()V"),
            ][..],
            &[("write", "(I)V"), ("write", "([BII)V"), ("flush", "()V")][..],
        ),
        (
            "java/io/BufferedWriter",
            "java/io/Writer",
            &[
                ("<init>", "(Ljava/io/Writer;)V"),
                ("<init>", "(Ljava/io/Writer;I)V"),
                ("write", "(I)V"),
                ("write", "([CII)V"),
                ("write", "(Ljava/lang/String;II)V"),
                ("newLine", "()V"),
                ("flush", "()V"),
                ("close", "()V"),
            ][..],
            &[][..],
        ),
        (
            "java/io/StringReader",
            "java/io/Reader",
            &[
                ("<init>", "(Ljava/lang/String;)V"),
                ("read", "()I"),
                ("read", "([CII)I"),
                ("skip", "(J)J"),
                ("ready", "()Z"),
                ("markSupported", "()Z"),
                ("mark", "(I)V"),
                ("reset", "()V"),
                ("close", "()V"),
            ][..],
            &[][..],
        ),
        (
            "java/io/CharArrayReader",
            "java/io/Reader",
            &[
                ("<init>", "([C)V"),
                ("<init>", "([CII)V"),
                ("read", "()I"),
                ("read", "([CII)I"),
                ("skip", "(J)J"),
                ("ready", "()Z"),
                ("markSupported", "()Z"),
                ("mark", "(I)V"),
                ("reset", "()V"),
                ("close", "()V"),
            ][..],
            &[][..],
        ),
        (
            "java/io/CharArrayWriter",
            "java/io/Writer",
            &[
                ("<init>", "()V"),
                ("<init>", "(I)V"),
                ("write", "(I)V"),
                ("write", "([CII)V"),
                ("write", "(Ljava/lang/String;II)V"),
                ("writeTo", "(Ljava/io/Writer;)V"),
                ("reset", "()V"),
                ("toCharArray", "()[C"),
                ("size", "()I"),
                ("toString", "()Ljava/lang/String;"),
                ("flush", "()V"),
                ("close", "()V"),
            ][..],
            &[][..],
        ),
        (
            "java/io/FileReader",
            "java/io/InputStreamReader",
            &[
                ("<init>", "(Ljava/lang/String;)V"),
                ("<init>", "(Ljava/io/File;)V"),
                ("<init>", "(Ljava/io/FileDescriptor;)V"),
            ][..],
            &[][..],
        ),
        (
            "java/io/FileWriter",
            "java/io/OutputStreamWriter",
            &[
                ("<init>", "(Ljava/lang/String;)V"),
                ("<init>", "(Ljava/lang/String;Z)V"),
                ("<init>", "(Ljava/io/File;)V"),
                ("<init>", "(Ljava/io/File;Z)V"),
                ("<init>", "(Ljava/io/FileDescriptor;)V"),
            ][..],
            &[][..],
        ),
    ] {
        let proto = get_runtime_class_proto(class_name).unwrap_or_else(|| panic!("missing {class_name}"));
        assert_eq!(proto.parent_class, Some(parent), "wrong parent for {class_name}");
        assert!(proto.access_flags.contains(ClassAccessFlags::PUBLIC), "{class_name} must be public");
        for (name, descriptor) in methods {
            let method = proto
                .methods
                .iter()
                .find(|method| method.name == *name && method.descriptor == *descriptor)
                .unwrap_or_else(|| panic!("missing {class_name}.{name}{descriptor}"));
            assert!(
                method.access_flags.contains(MethodAccessFlags::PUBLIC),
                "{class_name}.{name}{descriptor} must be public"
            );
            assert_eq!(
                method.access_flags.contains(MethodAccessFlags::SYNCHRONIZED),
                synchronized_methods.contains(&(*name, *descriptor)),
                "wrong synchronized flag for {class_name}.{name}{descriptor}"
            );
        }
    }

    let buffered_output = get_runtime_class_proto("java/io/BufferedOutputStream").expect("missing java/io/BufferedOutputStream");
    assert!(
        !buffered_output
            .methods
            .iter()
            .any(|method| method.name == "close" && method.descriptor == "()V"),
        "BufferedOutputStream must inherit close() from FilterOutputStream"
    );
    let filter_output = get_runtime_class_proto("java/io/FilterOutputStream").expect("missing java/io/FilterOutputStream");
    let close = filter_output
        .methods
        .iter()
        .find(|method| method.name == "close" && method.descriptor == "()V")
        .expect("missing java/io/FilterOutputStream.close()V");
    assert!(close.access_flags.contains(MethodAccessFlags::PUBLIC));
    assert!(!close.access_flags.contains(MethodAccessFlags::SYNCHRONIZED));

    for (class_name, fields) in [
        (
            "java/io/BufferedInputStream",
            &[
                ("buf", "[B", FieldAccessFlags::PROTECTED | FieldAccessFlags::VOLATILE),
                ("count", "I", FieldAccessFlags::PROTECTED),
                ("pos", "I", FieldAccessFlags::PROTECTED),
                ("markpos", "I", FieldAccessFlags::PROTECTED),
                ("marklimit", "I", FieldAccessFlags::PROTECTED),
            ][..],
        ),
        (
            "java/io/BufferedOutputStream",
            &[("buf", "[B", FieldAccessFlags::PROTECTED), ("count", "I", FieldAccessFlags::PROTECTED)][..],
        ),
        (
            "java/io/BufferedWriter",
            &[
                ("out", "Ljava/io/Writer;", FieldAccessFlags::PRIVATE),
                ("cb", "[C", FieldAccessFlags::PRIVATE),
                ("nChars", "I", FieldAccessFlags::PRIVATE),
                ("nextChar", "I", FieldAccessFlags::PRIVATE),
                ("lineSeparator", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
            ][..],
        ),
        (
            "java/io/StringReader",
            &[
                ("str", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                ("length", "I", FieldAccessFlags::PRIVATE),
                ("next", "I", FieldAccessFlags::PRIVATE),
                ("mark", "I", FieldAccessFlags::PRIVATE),
            ][..],
        ),
        (
            "java/io/CharArrayReader",
            &[
                ("buf", "[C", FieldAccessFlags::PROTECTED),
                ("pos", "I", FieldAccessFlags::PROTECTED),
                ("markedPos", "I", FieldAccessFlags::PROTECTED),
                ("count", "I", FieldAccessFlags::PROTECTED),
            ][..],
        ),
        (
            "java/io/CharArrayWriter",
            &[("buf", "[C", FieldAccessFlags::PROTECTED), ("count", "I", FieldAccessFlags::PROTECTED)][..],
        ),
    ] {
        let proto = get_runtime_class_proto(class_name).unwrap_or_else(|| panic!("missing {class_name}"));
        for (name, descriptor, access_flags) in fields {
            let field = proto
                .fields
                .iter()
                .find(|field| field.name == *name && field.descriptor == *descriptor)
                .unwrap_or_else(|| panic!("missing {class_name}.{name}:{descriptor}"));
            assert_eq!(field.access_flags, *access_flags, "wrong field access for {class_name}.{name}");
        }
    }
}
