use jvm::{ClassInstance, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_manifest_parsing() -> Result<()> {
    let jvm = test_jvm().await?;

    for (data, expected) in [
        (&b"Main-Class: test"[..], "test"),
        (
            &b"Main-Class:  a\xf0\x9f\r\n \x98\x80\r b  \n\nName: entry\nMain-Class: other\n"[..],
            " a😀b  ",
        ),
    ] {
        let mut bytes = jvm.instantiate_array("B", data.len()).await?;
        jvm.array_raw_buffer_mut(&mut bytes).await?.write(0, data)?;

        let byte_array_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
        let manifest = jvm
            .new_class("java/util/jar/Manifest", "(Ljava/io/InputStream;)V", (byte_array_stream,))
            .await?;

        let main_attributes: Box<dyn ClassInstance> = jvm
            .invoke_virtual(
                &manifest,
                &manifest.class_definition().name(),
                "getMainAttributes",
                "()Ljava/util/jar/Attributes;",
                (),
            )
            .await?;

        let key = JavaLangString::from_rust_string(&jvm, "Main-Class").await?;
        let value = jvm
            .invoke_virtual(
                &main_attributes,
                &main_attributes.class_definition().name(),
                "getValue",
                "(Ljava/lang/String;)Ljava/lang/String;",
                (key,),
            )
            .await?;

        assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, expected);
    }

    for data in [&b"Bad Name: value\r"[..], b"Main-Class:missing-space\n", b" continuation\n"] {
        let mut bytes = jvm.instantiate_array("B", data.len()).await?;
        jvm.array_raw_buffer_mut(&mut bytes).await?.write(0, data)?;
        let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
        let result = jvm.new_class("java/util/jar/Manifest", "(Ljava/io/InputStream;)V", (input,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("malformed manifest headers must throw IOException");
        };
        assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    }

    let result = jvm.new_class("java/util/jar/Manifest", "(Ljava/io/InputStream;)V", (None,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("a null input stream must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    Ok(())
}
