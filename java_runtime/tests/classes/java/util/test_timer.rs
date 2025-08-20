use alloc::{boxed::Box, collections::BTreeMap, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_runtime::{RuntimeClassProto, RuntimeContext};
use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

struct TestClass;
impl TestClass {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TestClass",
            parent_class: Some("java/util/TimerTask"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("runCount", "I", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/TimerTask", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "runCount", "I").await?;
        jvm.put_field(&mut this, "runCount", "I", count + 1).await?;

        Ok(())
    }
}

#[tokio::test]
async fn test_timer() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        TestClass::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(class, None).await?;

    let test_class = jvm.new_class("TestClass", "()V", ()).await?;

    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(&timer, "schedule", "(Ljava/util/TimerTask;JJ)V", (test_class.clone(), 100i64, 0i64))
        .await?;

    let _: () = jvm.invoke_static("java/lang/Thread", "sleep", "(J)V", (200i64,)).await?;
    let run_count: i32 = jvm.get_field(&test_class, "runCount", "I").await?;
    assert_eq!(run_count, 1);

    let _: () = jvm
        .invoke_virtual(
            &timer,
            "scheduleAtFixedRate",
            "(Ljava/util/TimerTask;JJ)V",
            (test_class.clone(), 100i64, 0i64),
        )
        .await?;

    let _: () = jvm.invoke_static("java/lang/Thread", "sleep", "(J)V", (200i64,)).await?;
    let run_count: i32 = jvm.get_field(&test_class, "runCount", "I").await?;
    assert_eq!(run_count, 2);

    Ok(())
}

#[tokio::test]
async fn test_timer_periodic() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        TestClass::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(class, None).await?;

    let test_class = jvm.new_class("TestClass", "()V", ()).await?;

    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(&timer, "schedule", "(Ljava/util/TimerTask;JJ)V", (test_class.clone(), 0i64, 100i64))
        .await?;

    let _: () = jvm.invoke_static("java/lang/Thread", "sleep", "(J)V", (500i64,)).await?;
    let run_count: i32 = jvm.get_field(&test_class, "runCount", "I").await?;
    assert_eq!(run_count, 5);

    Ok(())
}
