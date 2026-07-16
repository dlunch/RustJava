use alloc::vec;

use jvm::{JavaChar, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_string_writer() -> Result<()> {
    let jvm = test_jvm().await?;

    let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await.unwrap();

    let mut buf = jvm.instantiate_array("C", 3).await?;

    jvm.store_array(&mut buf, 0, vec![b'a' as JavaChar, b'b' as JavaChar, b'c' as JavaChar])
        .await?;

    let _: () = jvm.invoke_virtual(&string_writer, "write", "([CII)V", (buf.clone(), 0, 3)).await?;

    let _: () = jvm.invoke_virtual(&string_writer, "write", "([CII)V", (buf, 1, 2)).await?;
    let _: () = jvm.invoke_virtual(&string_writer, "write", "(I)V", ('d' as i32,)).await?;

    let string = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", ()).await.unwrap();

    let string = JavaLangString::to_rust_string(&jvm, &string).await?;

    assert_eq!(string, "abcbcd"); // cspell: disable-line

    Ok(())
}
