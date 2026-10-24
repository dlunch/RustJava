use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};
use rustjava_runtime::classes::java::net::URLConnection;
use test_utils::test_jvm;

#[tokio::test]
async fn test_null_constructor_arguments() -> Result<()> {
    let jvm = test_jvm().await?;
    for result in [
        jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (None,)).await,
        jvm.new_class("java/net/URL", "(Ljava/net/URL;Ljava/lang/String;)V", (None, None)).await,
        jvm.new_class(
            "java/net/URL",
            "(Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)V",
            (None, None, None),
        )
        .await,
    ] {
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("expected MalformedURLException for null spec");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/net/MalformedURLException"));
    }
    let protocol = JavaLangString::from_rust_string(&jvm, "file").await?;
    let file = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
    for (protocol, file) in [(None, Some(file.clone())), (Some(protocol.clone()), None)] {
        let result = jvm
            .new_class(
                "java/net/URL",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (protocol, None, file),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("expected NullPointerException for null protocol or file");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));
    }
    jvm.new_class(
        "java/net/URL",
        "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/net/URLStreamHandler;)V",
        (protocol, None, -1, file, None),
    )
    .await?;
    let spec = JavaLangString::from_rust_string(&jvm, "file:test.txt").await?;
    jvm.new_class(
        "java/net/URL",
        "(Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)V",
        (None, spec, None),
    )
    .await?;
    Ok(())
}

#[tokio::test]
async fn test_handler_null_arguments() -> Result<()> {
    let jvm = test_jvm().await?;
    let handler = jvm.new_class("org/rustjava/net/FileURLHandler", "()V", ()).await?;
    let spec = JavaLangString::from_rust_string(&jvm, "file:test.txt").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (spec.clone(),)).await?;
    for (url, spec) in [(None, Some(spec)), (Some(url), None)] {
        let result: Result<()> = jvm
            .invoke_virtual(
                &handler,
                "java/net/URLStreamHandler",
                "parseURL",
                "(Ljava/net/URL;Ljava/lang/String;II)V",
                (url, spec, 0, 13),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("expected NullPointerException for null parseURL argument");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));
    }
    let result: Result<()> = jvm
        .invoke_virtual(
            &handler,
            "java/net/URLStreamHandler",
            "setURL",
            "(Ljava/net/URL;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
            (None, None, None, -1, None, None),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("expected NullPointerException for null setURL target");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));
    for class in ["org/rustjava/net/FileURLHandler", "org/rustjava/net/JarURLHandler"] {
        let handler = jvm.new_class(class, "()V", ()).await?;
        let result: Result<ClassInstanceRef<URLConnection>> = jvm
            .invoke_virtual(
                &handler,
                "java/net/URLStreamHandler",
                "openConnection",
                "(Ljava/net/URL;)Ljava/net/URLConnection;",
                (None,),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("expected NullPointerException for null connection URL");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));
    }
    Ok(())
}
