use alloc::{boxed::Box, collections::BTreeMap, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_runtime::{RuntimeClassProto, RuntimeContext};
use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{create_test_jvm, TestRuntime};

struct TestClass;
impl TestClass {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TestClass",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("ran", "Z", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "ran", "Z", false).await?;

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "ran", "Z", true).await?;

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
        .invoke_virtual(&timer, "schedule", "(Ljava/util/TimerTask;JJ)V", (test_class.clone(), 0i64, 100i64))
        .await?;

    let _: () = jvm.invoke_static("java/lang/Thread", "sleep", "(J)V", (200i64,)).await?;

    let ran: bool = jvm.get_field(&test_class, "ran", "Z").await?;
    assert!(ran);

    Ok(())
}
