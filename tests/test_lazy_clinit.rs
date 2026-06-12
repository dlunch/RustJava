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

async fn register(jvm: &jvm::Jvm, class_data: &[u8]) -> Result<()> {
    let class = ClassDefinitionImpl::from_classfile(class_data)?;
    jvm.register_class(Box::new(class), None).await?;

    Ok(())
}

#[tokio::test]
async fn test_clinit_not_run_at_registration() -> Result<()> {
    let jvm = test_jvm().await?;

    register(&jvm, include_bytes!("../test_data/unit/Target.class")).await?;
    register(&jvm, include_bytes!("../test_data/unit/Source.class")).await?;

    assert_eq!(jvm.get_static_field::<i32>("Target", "value", "I").await?, 0);

    let _: () = jvm.invoke_static("Source", "touch", "()V", ()).await?;
    assert_eq!(jvm.get_static_field::<i32>("Target", "value", "I").await?, 42);

    Ok(())
}

#[tokio::test]
async fn test_getstatic_triggers_clinit() -> Result<()> {
    let jvm = test_jvm().await?;

    register(&jvm, include_bytes!("../test_data/unit/Target.class")).await?;
    register(&jvm, include_bytes!("../test_data/unit/Source.class")).await?;

    assert_eq!(jvm.get_static_field::<i32>("Target", "value", "I").await?, 0);
    assert_eq!(jvm.get_static_field::<i32>("Source", "own", "I").await?, 7);
    assert_eq!(jvm.get_static_field::<i32>("Target", "value", "I").await?, 42);

    Ok(())
}

#[tokio::test]
async fn test_putstatic_triggers_clinit() -> Result<()> {
    let jvm = test_jvm().await?;

    register(&jvm, include_bytes!("../test_data/unit/Target.class")).await?;
    register(&jvm, include_bytes!("../test_data/unit/Source.class")).await?;

    assert_eq!(jvm.get_static_field::<i32>("Target", "value", "I").await?, 0);

    jvm.put_static_field("Source", "own", "I", 100).await?;
    assert_eq!(jvm.get_static_field::<i32>("Target", "value", "I").await?, 42);
    assert_eq!(jvm.get_static_field::<i32>("Source", "own", "I").await?, 100);

    Ok(())
}

#[tokio::test]
async fn test_new_triggers_clinit() -> Result<()> {
    let jvm = test_jvm().await?;

    register(&jvm, include_bytes!("../test_data/unit/Target.class")).await?;
    register(&jvm, include_bytes!("../test_data/unit/Inst.class")).await?;

    assert_eq!(jvm.get_static_field::<i32>("Target", "value", "I").await?, 0);

    let _ = jvm.instantiate_class("Inst").await?;
    assert_eq!(jvm.get_static_field::<i32>("Target", "value", "I").await?, 99);

    Ok(())
}

#[tokio::test]
async fn test_superclass_initialized_first() -> Result<()> {
    let jvm = test_jvm().await?;

    register(&jvm, include_bytes!("../test_data/unit/InitLog.class")).await?;
    register(&jvm, include_bytes!("../test_data/unit/InitBase.class")).await?;
    register(&jvm, include_bytes!("../test_data/unit/InitDerived.class")).await?;

    assert_eq!(jvm.get_static_field::<i32>("InitLog", "counter", "I").await?, 0);

    let _: () = jvm.invoke_static("InitDerived", "touch", "()V", ()).await?;
    assert_eq!(jvm.get_static_field::<i32>("InitLog", "baseOrder", "I").await?, 1);
    assert_eq!(jvm.get_static_field::<i32>("InitLog", "derivedOrder", "I").await?, 2);

    Ok(())
}

#[tokio::test]
async fn test_interface_not_initialized_by_implementor() -> Result<()> {
    let jvm = test_jvm().await?;

    register(&jvm, include_bytes!("../test_data/unit/Mark.class")).await?;
    register(&jvm, include_bytes!("../test_data/unit/MarkIFace.class")).await?;
    register(&jvm, include_bytes!("../test_data/unit/MarkImpl.class")).await?;

    let _ = jvm.instantiate_class("MarkImpl").await?;
    assert_eq!(jvm.get_static_field::<i32>("Mark", "value", "I").await?, 0);

    assert_eq!(jvm.get_static_field::<i32>("MarkIFace", "X", "I").await?, 1);
    assert_eq!(jvm.get_static_field::<i32>("Mark", "value", "I").await?, 1);

    Ok(())
}
