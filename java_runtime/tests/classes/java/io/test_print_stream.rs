use alloc::vec::Vec;

use java_runtime::classes::java::{
    io::OutputStream,
    lang::{Object, String},
};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};

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

#[tokio::test]
async fn test_print_stream_remaining_overloads_and_close() -> Result<()> {
    let jvm = test_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let text = JavaLangString::from_rust_string(&jvm, "obj").await?;
    let object: ClassInstanceRef<Object> = text.clone().into();
    let null_object: ClassInstanceRef<Object> = None.into();
    let null_string: ClassInstanceRef<String> = None.into();

    let _: () = jvm.invoke_virtual(&stream, "print", "(Ljava/lang/Object;)V", (object.clone(),)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(Ljava/lang/Object;)V", (null_object,)).await?;
    let _: () = jvm
        .invoke_virtual(&stream, "print", "(Ljava/lang/String;)V", (null_string.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(J)V", (9i64,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", ('|' as i32,)).await?;

    let _: () = jvm.invoke_virtual(&stream, "println", "(Ljava/lang/Object;)V", (object,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Ljava/lang/String;)V", (null_string,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(I)V", (-1,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(J)V", (2i64,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(C)V", ('A' as JavaChar,)).await?;

    let mut chars = jvm.instantiate_array("C", 2).await?;
    jvm.store_array(&mut chars, 0, ['B' as JavaChar, 'C' as JavaChar]).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "([C)V", (chars,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(B)V", (-3i8,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(S)V", (4i16,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Z)V", (false,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(F)V", (2.5f32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&stream, "checkError", "()Z", ()).await?);

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    let values: Vec<i8> = jvm.load_array(&bytes, 0, jvm.array_length(&bytes).await?).await?;
    assert_eq!(
        values.into_iter().map(|value| value as u8).collect::<Vec<_>>(),
        b"objnullnull9|obj\nnull\n-1\n2\nA\nBC\n-3\n4\nfalse\n2.5\n"
    );

    let null_output: ClassInstanceRef<OutputStream> = None.into();
    let result = jvm.new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (null_output,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null output must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
