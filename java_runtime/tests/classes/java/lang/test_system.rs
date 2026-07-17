use alloc::collections::BTreeMap;

use java_runtime::Runtime;
use jvm::Result;
use test_utils::{TestRuntime, create_test_jvm};

#[tokio::test]
async fn test_system_time_yield_and_exit_runtime_contract() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let before = runtime.now();
    let now: i64 = jvm.invoke_static("java/lang/System", "currentTimeMillis", "()J", ()).await?;
    assert!(now >= before as i64);

    let _: () = jvm.invoke_static("java/lang/Thread", "yield", "()V", ()).await?;
    let _: () = jvm.invoke_static("java/lang/System", "exit", "(I)V", (17,)).await?;
    assert_eq!(runtime.exit_status(), Some(17));
    let _: () = jvm.invoke_static("java/lang/System", "exit", "(I)V", (i32::MIN,)).await?;
    assert_eq!(runtime.exit_status(), Some(i32::MIN));

    Ok(())
}
