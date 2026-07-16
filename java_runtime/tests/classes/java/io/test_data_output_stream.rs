use alloc::vec;

use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_data_output_stream() -> Result<()> {
    let jvm = test_jvm().await?;

    let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let data_output_stream = jvm
        .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream.clone(),))
        .await?;

    let _: () = jvm.invoke_virtual(&data_output_stream, "write", "(I)V", (1,)).await?;
    let string = JavaLangString::from_rust_string(&jvm, "hello, world").await?;
    let _: () = jvm
        .invoke_virtual(&data_output_stream, "writeChars", "(Ljava/lang/String;)V", (string,))
        .await?;
    let _: () = jvm.invoke_virtual(&data_output_stream, "writeInt", "(I)V", (12341234,)).await?;
    let _: () = jvm.invoke_virtual(&data_output_stream, "writeLong", "(J)V", (123412341324i64,)).await?;

    let bytes = jvm.invoke_virtual(&stream, "toByteArray", "()[B", ()).await?;

    let length = jvm.array_length(&bytes).await?;
    let mut buf = vec![0; length];
    jvm.array_raw_buffer(&bytes).await?.read(0, &mut buf)?;

    assert_eq!(
        buf,
        vec![
            1, 0, b'h', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0, b',', 0, b' ', 0, b'w', 0, b'o', 0, b'r', 0, b'l', 0, b'd', 0, 0xbc, 0x4f, 0xf2, 0, 0,
            0, 0x1c, 0xbb, 0xf2, 0xe2, 0x4c
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_data_output_stream_utf() -> Result<()> {
    let jvm = test_jvm().await?;

    let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let data_output_stream = jvm
        .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream.clone(),))
        .await?;

    let string = JavaLangString::from_rust_string(&jvm, "hello, world").await?;
    let _: () = jvm
        .invoke_virtual(&data_output_stream, "writeUTF", "(Ljava/lang/String;)V", (string,))
        .await?;

    let bytes = jvm.invoke_virtual(&stream, "toByteArray", "()[B", ()).await?;

    let length = jvm.array_length(&bytes).await?;
    let mut buf = vec![0; length];
    jvm.array_raw_buffer(&bytes).await?.read(0, &mut buf)?;

    assert_eq!(buf, vec![0, 12, b'h', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd']);

    Ok(())
}

#[tokio::test]
async fn test_data_stream_cldc_primitive_round_trip() -> Result<()> {
    let jvm = test_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let data_output = jvm
        .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;

    let _: () = jvm.invoke_virtual(&data_output, "writeBoolean", "(Z)V", (true,)).await?;
    let _: () = jvm.invoke_virtual(&data_output, "writeByte", "(I)V", (0xfe,)).await?;
    let _: () = jvm.invoke_virtual(&data_output, "writeShort", "(I)V", (0x1234,)).await?;
    let _: () = jvm.invoke_virtual(&data_output, "writeChar", "(I)V", ('한' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&data_output, "writeInt", "(I)V", (0x12345678,)).await?;
    let _: () = jvm.invoke_virtual(&data_output, "writeLong", "(J)V", (0x0123456789abcdefi64,)).await?;
    let _: () = jvm.invoke_virtual(&data_output, "writeFloat", "(F)V", (1.5f32,)).await?;
    let _: () = jvm.invoke_virtual(&data_output, "writeDouble", "(D)V", (-2.25f64,)).await?;
    let low_bytes = JavaLangString::from_rust_string(&jvm, "Aé").await?;
    let _: () = jvm
        .invoke_virtual(&data_output, "writeBytes", "(Ljava/lang/String;)V", (low_bytes,))
        .await?;

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let data_input = jvm.new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input,)).await?;

    assert!(jvm.invoke_virtual::<_, bool>(&data_input, "readBoolean", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&data_input, "readUnsignedByte", "()I", ()).await?, 0xfe);
    assert_eq!(jvm.invoke_virtual::<_, i16>(&data_input, "readShort", "()S", ()).await?, 0x1234);
    assert_eq!(jvm.invoke_virtual::<_, u16>(&data_input, "readChar", "()C", ()).await?, '한' as u16);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&data_input, "readInt", "()I", ()).await?, 0x12345678);
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&data_input, "readLong", "()J", ()).await?,
        0x0123456789abcdefi64
    );
    assert_eq!(jvm.invoke_virtual::<_, f32>(&data_input, "readFloat", "()F", ()).await?, 1.5);
    assert_eq!(jvm.invoke_virtual::<_, f64>(&data_input, "readDouble", "()D", ()).await?, -2.25);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&data_input, "readUnsignedByte", "()I", ()).await?,
        b'A' as i32
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&data_input, "readUnsignedByte", "()I", ()).await?, 0xe9);

    Ok(())
}

#[tokio::test]
async fn test_data_stream_modified_utf_round_trip_and_malformed_input() -> Result<()> {
    let jvm = test_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let data_output = jvm
        .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let expected = JavaLangString::from_rust_string(&jvm, "\0A😀한").await?;
    let _: () = jvm.invoke_virtual(&data_output, "writeUTF", "(Ljava/lang/String;)V", (expected,)).await?;

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    let encoded: Vec<i8> = jvm.load_array(&bytes, 0, jvm.array_length(&bytes).await?).await?;
    assert_eq!(&encoded[2..4], &[-64, -128]);

    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let data_input = jvm.new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input,)).await?;
    let decoded: ClassInstanceRef<java_runtime::classes::java::lang::String> =
        jvm.invoke_virtual(&data_input, "readUTF", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &decoded).await?, "\0A😀한");

    let mut malformed = jvm.instantiate_array("B", 4).await?;
    jvm.store_array(&mut malformed, 0, [0i8, 2, -62, 32]).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (malformed,)).await?;
    let data_input = jvm.new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input,)).await?;
    let result: Result<ClassInstanceRef<java_runtime::classes::java::lang::String>> =
        jvm.invoke_virtual(&data_input, "readUTF", "()Ljava/lang/String;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("malformed modified UTF-8 must throw UTFDataFormatException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UTFDataFormatException"));

    Ok(())
}
