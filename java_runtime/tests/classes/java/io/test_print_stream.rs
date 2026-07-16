use alloc::vec::Vec;

use jvm::{Array, ClassInstanceRef, JavaChar, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_print_stream_cldc_api() -> Result<()> {
    let jvm = test_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;

    let prefix = JavaLangString::from_rust_string(&jvm, "v=").await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(Ljava/lang/String;)V", (prefix,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(I)V", (7,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(C)V", (' ' as u16,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(Z)V", (true,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "()V", ()).await?;

    let mut chars = jvm.instantiate_array("C", 2).await?;
    jvm.store_array(&mut chars, 0, ['O' as JavaChar, 'K' as JavaChar]).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "([C)V", (chars,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(D)V", (1.5f64,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(F)V", (1.0f32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(D)V", (f64::INFINITY,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "flush", "()V", ()).await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&stream, "checkError", "()Z", ()).await?);

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    let values: Vec<i8> = jvm.load_array(&bytes, 0, jvm.array_length(&bytes).await?).await?;
    let values = values.into_iter().map(|value| value as u8).collect::<Vec<_>>();
    assert_eq!(values, b"v=7 true\nOK1.5\n1.0Infinity\n");

    Ok(())
}
