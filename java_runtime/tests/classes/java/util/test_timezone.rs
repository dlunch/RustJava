use java_runtime::classes::java::util::TimeZone;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_timezone() -> Result<()> {
    let jvm = test_jvm().await?;

    let id = JavaLangString::from_rust_string(&jvm, "UTC").await?;
    let timezone: ClassInstanceRef<TimeZone> = jvm
        .invoke_static("java/util/TimeZone", "getTimeZone", "(Ljava/lang/String;)Ljava/util/TimeZone;", (id,))
        .await?;

    assert!(!timezone.is_null());

    Ok(())
}
