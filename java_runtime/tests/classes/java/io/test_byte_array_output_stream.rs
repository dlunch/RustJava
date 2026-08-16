use alloc::{vec, vec::Vec};

use bytemuck::cast_vec;

use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::classes::java::io::{ByteArrayOutputStream, OutputStream};
use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn baos_01_descriptors_fields_and_access_flags() -> Result<()> {
    let proto = ByteArrayOutputStream::as_proto();
    assert!(proto.access_flags.contains(ClassAccessFlags::PUBLIC));

    for (name, descriptor) in [
        ("writeTo", "(Ljava/io/OutputStream;)V"),
        ("toString", "(Ljava/lang/String;)Ljava/lang/String;"),
    ] {
        let methods = proto
            .methods
            .iter()
            .filter(|method| method.name == name && method.descriptor == descriptor)
            .collect::<Vec<_>>();
        assert_eq!(methods.len(), 1, "missing or duplicate {name}{descriptor}");
        assert!(methods[0].access_flags.contains(MethodAccessFlags::PUBLIC));
        assert!(methods[0].access_flags.contains(MethodAccessFlags::SYNCHRONIZED));
    }

    let buf = proto.fields.iter().find(|field| field.name == "buf").expect("buf field");
    assert_eq!(buf.descriptor, "[B");
    assert_eq!(buf.access_flags, FieldAccessFlags::PROTECTED);
    let count = proto.fields.iter().find(|field| field.name == "count").expect("count field");
    assert_eq!(count.descriptor, "I");
    assert_eq!(count.access_flags, FieldAccessFlags::PROTECTED);
    assert!(proto.fields.iter().all(|field| field.name != "pos"));

    Ok(())
}

#[tokio::test]
async fn test_byte_array_output_stream() -> Result<()> {
    let jvm = test_jvm().await?;

    let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "write", "(I)V", (b'H' as i32,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "write", "(I)V", (b'e' as i32,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "write", "(I)V", (b'l' as i32,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "write", "(I)V", (b'l' as i32,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "write", "(I)V", (b'o' as i32,))
        .await?;

    let buf = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "toByteArray", "()[B", ())
        .await?;

    let mut bytes = vec![0; 5];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut bytes)?;

    assert_eq!(bytes, cast_vec(b"Hello".to_vec()));

    Ok(())
}

#[tokio::test]
async fn null_byte_arrays_throw_null_pointer_exception() -> Result<()> {
    let jvm = test_jvm().await?;
    let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let null: ClassInstanceRef<Array<i8>> = None.into();

    let result: Result<()> = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "write", "([B)V", (null.clone(),))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null write buffer must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let result: Result<()> = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "write", "([BII)V", (null, 0, 0))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null ranged write buffer must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn baos_01_write_to_and_named_encoding_use_only_logical_count_after_close() -> Result<()> {
    let jvm = test_jvm().await?;

    let stream = jvm.new_class("java/io/ByteArrayOutputStream", "(I)V", (16,)).await?;
    let mut bytes = jvm.instantiate_array("B", 2).await?;
    jvm.store_array(&mut bytes, 0, [b'O' as i8, b'K' as i8]).await?;
    let _: () = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "write", "([BII)V", (bytes, 0, 2))
        .await?;
    let _: () = jvm.invoke_virtual(&stream, &stream.class_definition().name(), "close", "()V", ()).await?;

    let destination = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let destination_output: ClassInstanceRef<OutputStream> = destination.clone().into();
    let _: () = jvm
        .invoke_virtual(
            &stream,
            &stream.class_definition().name(),
            "writeTo",
            "(Ljava/io/OutputStream;)V",
            (destination_output,),
        )
        .await?;
    let copied: ClassInstanceRef<Array<i8>> = jvm
        .invoke_virtual(&destination, &destination.class_definition().name(), "toByteArray", "()[B", ())
        .await?;
    assert_eq!(jvm.array_length(&copied).await?, 2);
    assert_eq!(jvm.load_array::<i8>(&copied, 0, 2).await?, [b'O' as i8, b'K' as i8]);

    let encoding = JavaLangString::from_rust_string(&jvm, "UTF-8").await?;
    let text = jvm
        .invoke_virtual(
            &stream,
            &stream.class_definition().name(),
            "toString",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (encoding,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "OK");

    let _: () = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "write", "(I)V", (b'!' as i32,))
        .await?;
    let text = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "OK!");

    Ok(())
}

#[tokio::test]
async fn baos_01_rejects_null_and_unknown_encoding() -> Result<()> {
    let jvm = test_jvm().await?;
    let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;

    let null_output: ClassInstanceRef<OutputStream> = None.into();
    let result: Result<()> = jvm
        .invoke_virtual(
            &stream,
            &stream.class_definition().name(),
            "writeTo",
            "(Ljava/io/OutputStream;)V",
            (null_output,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null output must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null_encoding: ClassInstanceRef<java_runtime::classes::java::lang::String> = None.into();
    let result = jvm
        .invoke_virtual::<_, ClassInstanceRef<java_runtime::classes::java::lang::String>>(
            &stream,
            &stream.class_definition().name(),
            "toString",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (null_encoding,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null encoding must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let unknown = JavaLangString::from_rust_string(&jvm, "not-an-encoding").await?;
    let result = jvm
        .invoke_virtual::<_, ClassInstanceRef<java_runtime::classes::java::lang::String>>(
            &stream,
            &stream.class_definition().name(),
            "toString",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (unknown,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unknown encoding must throw UnsupportedEncodingException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));

    Ok(())
}
