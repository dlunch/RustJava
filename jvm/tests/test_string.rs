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

#[tokio::test]
async fn test_intern_identity_survives_gc() -> Result<()> {
    let jvm = test_jvm().await?;

    // collect first for a clean baseline, then intern and assert the very next GC keeps it
    jvm.collect_garbage()?;

    let a = jvm.intern_string("interned").await?;
    let b = jvm.intern_string("interned").await?;
    assert!(a == b);

    // the interned string is held by no frame, but the string pool roots it (and its [C),
    // so the first GC after interning must collect nothing
    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 0);

    let c = jvm.intern_string("interned").await?;
    assert!(a == c);

    Ok(())
}
