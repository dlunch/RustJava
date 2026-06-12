use jvm::{JavaChar, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_to_rust_string_unpaired_surrogate() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars, 0, [0x61 as JavaChar, 0xd800, 0x62]).await?;

    let string = jvm.new_class("java/lang/String", "([C)V", (chars,)).await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "a\u{fffd}b");

    Ok(())
}
