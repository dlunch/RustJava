use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_date_cldc11_value_contract() -> Result<()> {
    let jvm = test_jvm().await?;
    let epoch = jvm.new_class("java/util/Date", "(J)V", (0i64,)).await?;
    let same = jvm.new_class("java/util/Date", "(J)V", (0i64,)).await?;
    let later = jvm.new_class("java/util/Date", "(J)V", (1i64,)).await?;

    assert!(jvm.invoke_virtual::<_, bool>(&epoch, "equals", "(Ljava/lang/Object;)Z", (same,)).await?);
    assert!(!jvm.invoke_virtual::<_, bool>(&epoch, "equals", "(Ljava/lang/Object;)Z", (later,)).await?);
    let null: ClassInstanceRef<Object> = None.into();
    assert!(!jvm.invoke_virtual::<_, bool>(&epoch, "equals", "(Ljava/lang/Object;)Z", (null,)).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&epoch, "hashCode", "()I", ()).await?, 0);

    let text: ClassInstanceRef<Object> = jvm.invoke_virtual(&epoch, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "Thu Jan 01 00:00:00 GMT 1970");

    Ok(())
}
