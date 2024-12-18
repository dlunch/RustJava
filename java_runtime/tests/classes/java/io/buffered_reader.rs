use java_runtime::classes::java::lang::String;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn test_buffered_reader() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut buffer = jvm.instantiate_array("B", 11).await?;
    jvm.array_raw_buffer_mut(&mut buffer).await?.write(0, b"Hello\nWorld")?;

    let is = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (buffer,)).await?;
    let isr = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (is,)).await?;
    let reader = jvm.new_class("java/io/BufferedReader", "(Ljava/io/Reader;)V", (isr,)).await?;

    let line = jvm.invoke_virtual(&reader, "readLine", "()Ljava/lang/String;", ()).await?;
    let line = JavaLangString::to_rust_string(&jvm, &line).await?;
    assert_eq!(line, "Hello");

    let line = jvm.invoke_virtual(&reader, "readLine", "()Ljava/lang/String;", ()).await?;
    let line = JavaLangString::to_rust_string(&jvm, &line).await?;
    assert_eq!(line, "World");

    let line: ClassInstanceRef<String> = jvm.invoke_virtual(&reader, "readLine", "()Ljava/lang/String;", ()).await?;
    assert!(line.is_null());

    Ok(())
}
