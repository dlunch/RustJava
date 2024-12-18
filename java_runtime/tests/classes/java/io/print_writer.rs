use java_runtime::classes::java::lang::String;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn test_print_writer() -> Result<()> {
    let jvm = test_jvm().await?;

    let sw = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let pw = jvm.new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (sw.clone(),)).await?;

    let hello = JavaLangString::from_rust_string(&jvm, "hello").await?;
    let world = JavaLangString::from_rust_string(&jvm, "world").await?;

    let _: () = jvm.invoke_virtual(&pw, "println", "(Ljava/lang/String;)V", (hello,)).await?;
    let _: () = jvm.invoke_virtual(&pw, "println", "(Ljava/lang/String;)V", (world,)).await?;

    let result: ClassInstanceRef<String> = jvm.invoke_virtual(&sw, "toString", "()Ljava/lang/String;", ()).await?;
    let result = JavaLangString::to_rust_string(&jvm, &result).await?;

    assert_eq!(result, "hello\nworld\n");

    Ok(())
}
