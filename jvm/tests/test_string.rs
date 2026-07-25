use jvm::{ClassInstance, JavaChar, JavaError, Result, runtime::JavaLangString};

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
async fn test_to_utf16_preserves_unpaired_surrogate() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars, 0, [0x61 as JavaChar, 0xd800, 0x62]).await?;

    let string = jvm.new_class("java/lang/String", "([C)V", (chars,)).await?;

    assert_eq!(JavaLangString::to_utf16(&jvm, &string).await?, [0x61, 0xd800, 0x62]);

    Ok(())
}

#[tokio::test]
async fn test_to_utf16_on_substring_preserves_unpaired_surrogate() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars, 0, [0x61 as JavaChar, 0xd800, 0x62]).await?;

    let string = jvm.new_class("java/lang/String", "([C)V", (chars,)).await?;
    let sub = jvm.invoke_virtual(&string, "substring", "(II)Ljava/lang/String;", (1, 3)).await?;

    assert_eq!(JavaLangString::to_utf16(&jvm, &sub).await?, [0xd800, 0x62]);

    Ok(())
}

#[tokio::test]
async fn test_intern_on_substring_uses_logical_slice() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let sub = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 7)).await?;

    let interned: Box<dyn ClassInstance> = jvm.invoke_virtual(&sub, "intern", "()Ljava/lang/String;", ()).await?;
    let pooled = jvm.intern_string("Hello").await?;
    assert!(interned == pooled);

    let independent = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    let independent_interned: Box<dyn ClassInstance> = jvm.invoke_virtual(&independent, "intern", "()Ljava/lang/String;", ()).await?;
    assert!(interned == independent_interned);

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

#[tokio::test]
async fn test_array_bounds_check_survives_offset_overflow() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars, 0, [0x61 as JavaChar, 0x62, 0x63]).await?;

    for offset in [usize::MAX, usize::MAX - 2, 4] {
        let result: Result<Vec<JavaChar>> = jvm.load_array(&chars, offset, 3).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("load_array at {offset} must report a java exception");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/ArrayIndexOutOfBoundsException"));

        let result = jvm.store_array(&mut chars, offset, [0x64 as JavaChar]).await;
        assert!(matches!(result, Err(JavaError::JavaException(_))));
    }

    let loaded: Vec<JavaChar> = jvm.load_array(&chars, 0, 3).await?;
    assert_eq!(loaded, [0x61, 0x62, 0x63]);

    Ok(())
}
