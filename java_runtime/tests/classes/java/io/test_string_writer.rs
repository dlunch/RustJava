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

    let _: i32 = jvm.invoke_virtual(&string_writer, "write", "([CII)I", (buf.clone(), 0, 3)).await.unwrap();

    let _: i32 = jvm.invoke_virtual(&string_writer, "write", "([CII)I", (buf, 1, 2)).await.unwrap();

    let string = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", ()).await.unwrap();

    let string = JavaLangString::to_rust_string(&jvm, &string).await?;

    assert_eq!(string, "abcbc"); // cspell: disable-line

    Ok(())
}
