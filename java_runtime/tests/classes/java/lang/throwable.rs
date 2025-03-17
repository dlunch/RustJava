use alloc::boxed::Box;

use java_runtime::classes::java::lang::String;
use jvm::{ClassInstance, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_to_string() -> Result<()> {
    let jvm = test_jvm().await?;

    let message = JavaLangString::from_rust_string(&jvm, "test message").await?;

    let throwable = jvm.new_class("java/lang/Throwable", "(Ljava/lang/String;)V", (message,)).await?;
    let to_string = jvm.invoke_virtual(&throwable, "toString", "()Ljava/lang/String;", ()).await?;

    let result = JavaLangString::to_rust_string(&jvm, &to_string).await?;

    assert_eq!(result, "java/lang/Throwable: test message");

    Ok(())
}

#[tokio::test]
async fn test_stacktrace() -> Result<()> {
    let jvm = test_jvm().await?;

    // get exception by creating invalid url
    let url_string = JavaLangString::from_rust_string(&jvm, "invalid://invalid").await?;
    let url: Result<Box<dyn ClassInstance>> = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_string,)).await;

    let exception = if let JavaError::JavaException(exception) = url.err().unwrap() {
        exception
    } else {
        panic!("expected JavaException");
    };

    let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let print_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
        .await?;

    let _: () = jvm
        .invoke_virtual(&exception, "printStackTrace", "(Ljava/io/PrintWriter;)V", (print_writer,))
        .await?;

    let result: ClassInstanceRef<String> = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", ()).await?;
    let result = JavaLangString::to_rust_string(&jvm, &result).await?;

    assert_eq!(
        result,
        "\
                java/net/MalformedURLException: unknown protocol: invalid\n\
                    \tat java/net/URL.<init>(Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)V\n\
                    \tat java/net/URL.<init>(Ljava/net/URL;Ljava/lang/String;)V\n\
                    \tat java/net/URL.<init>(Ljava/lang/String;)V\n\
            "
    );

    Ok(())
}
