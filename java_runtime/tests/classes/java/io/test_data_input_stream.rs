use alloc::vec;

use bytemuck::cast_vec;

use java_runtime::classes::java::lang::String;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_data_input_stream() -> Result<()> {
    let jvm = test_jvm().await?;

    let data = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1a, 0x1b, 0, 5, b'a', b'b', b'c', b'b', b'd',
    ];
    let data_len = data.len();

    let mut data_array = jvm.instantiate_array("B", data_len).await?;
    jvm.array_raw_buffer_mut(&mut data_array).await?.write(0, &data)?;

    let input_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data_array,)).await?;
    let data_input_stream = jvm
        .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input_stream,))
        .await?;

    let available: i32 = jvm.invoke_virtual(&data_input_stream, "available", "()I", ()).await?;
    assert_eq!(available, data_len as i32);

    let byte: i8 = jvm.invoke_virtual(&data_input_stream, "readByte", "()B", ()).await?;
    assert_eq!(byte, 0x01);

    let short: i16 = jvm.invoke_virtual(&data_input_stream, "readShort", "()S", ()).await?;
    assert_eq!(short, 0x0203);

    let int: i32 = jvm.invoke_virtual(&data_input_stream, "readInt", "()I", ()).await?;
    assert_eq!(int, 0x04050607);

    let long: i64 = jvm.invoke_virtual(&data_input_stream, "readLong", "()J", ()).await?;
    assert_eq!(long, 0x08090a0b0c0d0e0f);

    let float: f32 = jvm.invoke_virtual(&data_input_stream, "readFloat", "()F", ()).await?;
    assert_eq!(float, f32::from_be_bytes([0x10, 0x11, 0x12, 0x13]));

    let double: f64 = jvm.invoke_virtual(&data_input_stream, "readDouble", "()D", ()).await?;
    assert_eq!(double, f64::from_be_bytes([0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b]));

    let utf: ClassInstanceRef<String> = jvm.invoke_virtual(&data_input_stream, "readUTF", "()Ljava/lang/String;", ()).await?;
    let string = JavaLangString::to_rust_string(&jvm, &utf).await?;
    assert_eq!(string, "abcbd");

    Ok(())
}

#[tokio::test]
async fn test_data_input_stream_high_bit() -> Result<()> {
    let jvm = test_jvm().await?;

    let data = cast_vec(vec![
        0x81u8, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
        0x98, 0x99, 0x9a, 0x9b, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a,
    ]);
    let data_len = data.len();

    let mut data_array = jvm.instantiate_array("B", data_len).await?;
    jvm.array_raw_buffer_mut(&mut data_array).await?.write(0, &data)?;

    let input_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data_array,)).await?;
    let data_input_stream = jvm
        .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input_stream,))
        .await?;

    let available: i32 = jvm.invoke_virtual(&data_input_stream, "available", "()I", ()).await?;
    assert_eq!(available, data_len as i32);

    let byte: i8 = jvm.invoke_virtual(&data_input_stream, "readByte", "()B", ()).await?;
    assert_eq!(byte, i8::from_be_bytes([0x81]));

    let short: i16 = jvm.invoke_virtual(&data_input_stream, "readShort", "()S", ()).await?;
    assert_eq!(short, i16::from_be_bytes([0x82, 0x83]));

    let int: i32 = jvm.invoke_virtual(&data_input_stream, "readInt", "()I", ()).await?;
    assert_eq!(int, i32::from_be_bytes([0x84, 0x85, 0x86, 0x87]));

    let long: i64 = jvm.invoke_virtual(&data_input_stream, "readLong", "()J", ()).await?;
    assert_eq!(long, i64::from_be_bytes([0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f]));

    let float: f32 = jvm.invoke_virtual(&data_input_stream, "readFloat", "()F", ()).await?;
    assert_eq!(float, f32::from_be_bytes([0x90, 0x91, 0x92, 0x93]));

    let double: f64 = jvm.invoke_virtual(&data_input_stream, "readDouble", "()D", ()).await?;
    assert_eq!(double, f64::from_be_bytes([0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b]));

    let array = jvm.instantiate_array("B", 6).await?;
    let _: () = jvm.invoke_virtual(&data_input_stream, "readFully", "([B)V", (array.clone(),)).await?;
    let mut buffer = vec![0; 6];
    jvm.array_raw_buffer(&array).await?.read(0, &mut buffer)?;
    assert_eq!(buffer, vec![0x10, 0x11, 0x12, 0x13, 0x14, 0x15]);

    let skipped: i32 = jvm.invoke_virtual(&data_input_stream, "skipBytes", "(I)I", (2,)).await?;
    assert_eq!(skipped, 2);

    let short: i16 = jvm.invoke_virtual(&data_input_stream, "readShort", "()S", ()).await?;
    assert_eq!(short, i16::from_be_bytes([0x18, 0x19]));

    let available: i32 = jvm.invoke_virtual(&data_input_stream, "available", "()I", ()).await?;
    assert_eq!(available, 1);

    Ok(())
}
