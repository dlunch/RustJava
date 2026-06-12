use jvm::Result;
use jvm_rust::ClassDefinitionImpl;

use test_utils::test_jvm;

#[tokio::test]
async fn test_clinit_runs_once() -> Result<()> {
    let jvm = test_jvm().await?;

    let class = ClassDefinitionImpl::from_classfile(include_bytes!("../test_data/unit/Counter.class"))?;
    jvm.register_class(Box::new(class), None).await?;

    let _: () = jvm.invoke_static("Counter", "touch", "()V", ()).await?;
    let _: () = jvm.invoke_static("Counter", "touch", "()V", ()).await?;

    assert_eq!(jvm.get_static_field::<i32>("Counter", "initCount", "I").await?, 1);

    Ok(())
}

#[tokio::test]
async fn test_clinit_reentrancy() -> Result<()> {
    let jvm = test_jvm().await?;

    let class = ClassDefinitionImpl::from_classfile(include_bytes!("../test_data/unit/SelfRef.class"))?;
    jvm.register_class(Box::new(class), None).await?;

    let _: () = jvm.invoke_static("SelfRef", "touch", "()V", ()).await?;

    assert_eq!(jvm.get_static_field::<i32>("SelfRef", "a", "I").await?, 1);
    assert_eq!(jvm.get_static_field::<i32>("SelfRef", "b", "I").await?, 41);

    Ok(())
}
