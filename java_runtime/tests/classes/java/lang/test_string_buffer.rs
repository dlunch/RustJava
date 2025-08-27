use java_runtime::classes::java::lang::StringBuffer;
use jvm::{ClassInstanceRef, JavaChar, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_string_buffer() -> Result<()> {
    let jvm = test_jvm().await?;

    let string_buffer = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
    let string = JavaLangString::from_rust_string(&jvm, "Hello, ").await?;

    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&string_buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (string,))
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm.invoke_virtual(&string_buffer, "append", "(I)Ljava/lang/StringBuffer;", (42,)).await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&string_buffer, "append", "(C)Ljava/lang/StringBuffer;", (b'H' as JavaChar,))
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(&string_buffer, "append", "(J)Ljava/lang/StringBuffer;", (42i64,))
        .await?;

    let length: i32 = jvm.invoke_virtual(&string_buffer, "length", "()I", ()).await?;
    assert_eq!(length, 12);

    let result = jvm.invoke_virtual(&string_buffer, "toString", "()Ljava/lang/String;", ()).await?;
    let result = JavaLangString::to_rust_string(&jvm, &result).await?;

    assert_eq!("Hello, 42H42", result);

    Ok(())
}
