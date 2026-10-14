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

    let _: () = jvm
        .invoke_virtual(&data_output_stream, &data_output_stream.class_definition().name(), "write", "(I)V", (1,))
        .await?;
    let string = JavaLangString::from_rust_string(&jvm, "hello, world").await?;
    let _: () = jvm
        .invoke_virtual(
            &data_output_stream,
            &data_output_stream.class_definition().name(),
            "writeChars",
            "(Ljava/lang/String;)V",
            (string,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &data_output_stream,
            &data_output_stream.class_definition().name(),
            "writeInt",
            "(I)V",
            (12341234,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &data_output_stream,
            &data_output_stream.class_definition().name(),
            "writeLong",
            "(J)V",
            (123412341324i64,),
        )
        .await?;

    let bytes = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "toByteArray", "()[B", ())
        .await?;

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
        .invoke_virtual(
            &data_output_stream,
            &data_output_stream.class_definition().name(),
            "writeUTF",
            "(Ljava/lang/String;)V",
            (string,),
        )
        .await?;

    let bytes = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "toByteArray", "()[B", ())
        .await?;

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

    let _: () = jvm
        .invoke_virtual(&data_output, &data_output.class_definition().name(), "writeBoolean", "(Z)V", (true,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&data_output, &data_output.class_definition().name(), "writeByte", "(I)V", (0xfe,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&data_output, &data_output.class_definition().name(), "writeShort", "(I)V", (0x1234,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&data_output, &data_output.class_definition().name(), "writeChar", "(I)V", ('한' as i32,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&data_output, &data_output.class_definition().name(), "writeInt", "(I)V", (0x12345678,))
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &data_output,
            &data_output.class_definition().name(),
            "writeLong",
            "(J)V",
            (0x0123456789abcdefi64,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&data_output, &data_output.class_definition().name(), "writeFloat", "(F)V", (1.5f32,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&data_output, &data_output.class_definition().name(), "writeDouble", "(D)V", (-2.25f64,))
        .await?;
    let low_bytes = JavaLangString::from_rust_string(&jvm, "Aé").await?;
    let _: () = jvm
        .invoke_virtual(
            &data_output,
            &data_output.class_definition().name(),
            "writeBytes",
            "(Ljava/lang/String;)V",
            (low_bytes,),
        )
        .await?;

    let bytes: ClassInstanceRef<Array<i8>> = jvm
        .invoke_virtual(&output, &output.class_definition().name(), "toByteArray", "()[B", ())
        .await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let data_input = jvm.new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input,)).await?;

    assert!(
        jvm.invoke_virtual::<_, bool>(&data_input, &data_input.class_definition().name(), "readBoolean", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&data_input, &data_input.class_definition().name(), "readUnsignedByte", "()I", ())
            .await?,
        0xfe
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i16>(&data_input, &data_input.class_definition().name(), "readShort", "()S", ())
            .await?,
        0x1234
    );
    assert_eq!(
        jvm.invoke_virtual::<_, u16>(&data_input, &data_input.class_definition().name(), "readChar", "()C", ())
            .await?,
        '한' as u16
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&data_input, &data_input.class_definition().name(), "readInt", "()I", ())
            .await?,
        0x12345678
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&data_input, &data_input.class_definition().name(), "readLong", "()J", ())
            .await?,
        0x0123456789abcdefi64
    );
    assert_eq!(
        jvm.invoke_virtual::<_, f32>(&data_input, &data_input.class_definition().name(), "readFloat", "()F", ())
            .await?,
        1.5
    );
    assert_eq!(
        jvm.invoke_virtual::<_, f64>(&data_input, &data_input.class_definition().name(), "readDouble", "()D", ())
            .await?,
        -2.25
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&data_input, &data_input.class_definition().name(), "readUnsignedByte", "()I", ())
            .await?,
        b'A' as i32
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&data_input, &data_input.class_definition().name(), "readUnsignedByte", "()I", ())
            .await?,
        0xe9
    );

    Ok(())
}

#[tokio::test]
async fn test_data_stream_modified_utf_round_trip_and_malformed_input() -> Result<()> {
    let jvm = test_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let data_output = jvm
        .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let units = [0, 0x41, 0x7f, 0x80, 0x7ff, 0x800, 0xd83d, 0xde00, 0xd800, 0x41, 0xdc00, 0xffff];
    let expected = JavaLangString::from_utf16(&jvm, units.to_vec()).await?;
    let _: () = jvm
        .invoke_virtual(
            &data_output,
            &data_output.class_definition().name(),
            "writeUTF",
            "(Ljava/lang/String;)V",
            (expected,),
        )
        .await?;

    let bytes: ClassInstanceRef<Array<i8>> = jvm
        .invoke_virtual(&output, &output.class_definition().name(), "toByteArray", "()[B", ())
        .await?;
    let encoded: Vec<i8> = jvm.load_array(&bytes, 0, jvm.array_length(&bytes).await?).await?;
    assert_eq!(
        encoded.iter().map(|&byte| byte as u8).collect::<Vec<_>>(),
        [
            0, 27, 0xc0, 0x80, 0x41, 0x7f, 0xc2, 0x80, 0xdf, 0xbf, 0xe0, 0xa0, 0x80, 0xed, 0xa0, 0xbd, 0xed, 0xb8, 0x80, 0xed, 0xa0, 0x80, 0x41,
            0xed, 0xb0, 0x80, 0xef, 0xbf, 0xbf,
        ]
    );

    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let data_input = jvm.new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input,)).await?;
    let decoded: ClassInstanceRef<rustjava_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(&data_input, &data_input.class_definition().name(), "readUTF", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_utf16(&jvm, &decoded).await?, units);

    for payload in [
        &[0xc2, 0x20][..],
        &[0xc0],
        &[0xe0, 0xa0],
        &[0xe0, 0x20, 0x80],
        &[0xe0, 0xa0, 0x20],
        &[0xf0, 0x9f, 0x98, 0x80],
        &[0x80],
        &[0xff],
    ] {
        let mut data = (payload.len() as u16).to_be_bytes().to_vec();
        data.extend_from_slice(payload);
        let mut malformed = jvm.instantiate_array("B", data.len()).await?;
        jvm.store_array(&mut malformed, 0, data.into_iter().map(|byte| byte as i8).collect::<Vec<_>>())
            .await?;
        let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (malformed,)).await?;
        let data_input = jvm.new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input,)).await?;
        let result: Result<ClassInstanceRef<rustjava_runtime::classes::java::lang::String>> = jvm
            .invoke_virtual(&data_input, &data_input.class_definition().name(), "readUTF", "()Ljava/lang/String;", ())
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("malformed modified UTF-8 must throw UTFDataFormatException: {payload:?}");
        };
        assert!(jvm.is_instance(&*exception, "java/io/UTFDataFormatException"));
    }

    Ok(())
}

#[tokio::test]
async fn test_write_utf_encoded_length_limit() -> Result<()> {
    let jvm = test_jvm().await?;
    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let data_output = jvm
        .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;

    for (length, succeeds) in [(0, true), (21845, true), (21846, false)] {
        let _: () = jvm.invoke_virtual(&output, "java/io/ByteArrayOutputStream", "reset", "()V", ()).await?;
        let value = JavaLangString::from_utf16(&jvm, vec![0x800; length]).await?;
        let result: Result<()> = jvm
            .invoke_virtual(&data_output, "java/io/DataOutputStream", "writeUTF", "(Ljava/lang/String;)V", (value,))
            .await;
        if succeeds {
            result?;
            let bytes: ClassInstanceRef<Array<i8>> = jvm
                .invoke_virtual(&output, "java/io/ByteArrayOutputStream", "toByteArray", "()[B", ())
                .await?;
            assert_eq!(jvm.array_length(&bytes).await?, length * 3 + 2);
            let header: Vec<i8> = jvm.load_array(&bytes, 0, 2).await?;
            assert_eq!(header, if length == 0 { [0, 0] } else { [-1, -1] });
        } else {
            let Err(JavaError::JavaException(exception)) = result else {
                panic!("oversized modified UTF-8 must throw UTFDataFormatException");
            };
            assert!(jvm.is_instance(&*exception, "java/io/UTFDataFormatException"));
            let size: i32 = jvm.invoke_virtual(&output, "java/io/ByteArrayOutputStream", "size", "()I", ()).await?;
            assert_eq!(size, 0);
        }
    }

    Ok(())
}
