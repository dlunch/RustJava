use alloc::{boxed::Box, collections::BTreeMap, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{RuntimeClassProto, RuntimeContext};
use jvm::{ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

struct TestClass;
impl TestClass {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TestClass",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("run", "()V", Self::run, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("ran", "Z", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "ran", "Z", false).await?;

        Ok(())
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "ran", "Z", true).await?;

        Ok(())
    }
}

#[tokio::test]
async fn test_thread() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        TestClass::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(class, None).await?;

    let test_class = jvm.new_class("TestClass", "()V", ()).await?;

    let thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (test_class.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&thread, &thread.class_definition().name(), "start", "()V", []).await?;

    let _: () = jvm.invoke_virtual(&thread, &thread.class_definition().name(), "join", "()V", []).await?;

    let ran: bool = jvm.get_field(&test_class, "ran", "Z").await?;
    assert!(ran);

    assert!(
        !jvm.invoke_virtual::<_, bool>(&thread, &thread.class_definition().name(), "isAlive", "()Z", ())
            .await?
    );
    let _: () = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "setDaemon", "(Z)V", (true,))
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&thread, &thread.class_definition().name(), "isDaemon", "()Z", ())
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_thread_cldc_metadata_and_state() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        TestClass::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(class, None).await?;
    let target = jvm.new_class("TestClass", "()V", ()).await?;
    let name = JavaLangString::from_rust_string(&jvm, "worker").await?;
    let thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;Ljava/lang/String;)V", (target, name))
        .await?;

    assert!(jvm.is_instance(&*thread, "java/lang/Runnable"));
    let name = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "getName", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "worker");
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&thread, &thread.class_definition().name(), "getPriority", "()I", ())
            .await?,
        5
    );
    assert_eq!(jvm.get_static_field::<i32>("java/lang/Thread", "MIN_PRIORITY", "I").await?, 1);
    assert_eq!(jvm.get_static_field::<i32>("java/lang/Thread", "NORM_PRIORITY", "I").await?, 5);
    assert_eq!(jvm.get_static_field::<i32>("java/lang/Thread", "MAX_PRIORITY", "I").await?, 10);

    let _: () = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "setPriority", "(I)V", (7,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&thread, &thread.class_definition().name(), "getPriority", "()I", ())
            .await?,
        7
    );
    let text = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "Thread[worker,7]");

    let _: () = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "interrupt", "()V", ())
        .await?;
    assert!(jvm.get_field::<bool>(&thread, "interrupted", "Z").await?);
    assert!(jvm.invoke_static::<_, i32>("java/lang/Thread", "activeCount", "()I", ()).await? >= 1);

    let result: Result<()> = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "setPriority", "(I)V", (11,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("invalid priority must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    Ok(())
}

#[tokio::test]
async fn test_thread_jdk12_metadata_name_interrupt_and_daemon_state() -> Result<()> {
    let runtime = TestRuntime::new_with_queued_spawns(BTreeMap::new());
    let jvm = create_test_jvm(runtime).await?;
    let thread = jvm.new_class("java/lang/Thread", "()V", ()).await?;

    let class = jvm.get_class("java/lang/Thread").expect("Thread must be loaded");
    for (name, descriptor, is_static, expected) in [
        (
            "setName",
            "(Ljava/lang/String;)V",
            false,
            MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
        ),
        ("isInterrupted", "()Z", false, MethodAccessFlags::PUBLIC),
        ("interrupted", "()Z", true, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
        (
            "join",
            "(J)V",
            false,
            MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL | MethodAccessFlags::SYNCHRONIZED,
        ),
        ("setDaemon", "(Z)V", false, MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
        ("isDaemon", "()Z", false, MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
    ] {
        let flags = class
            .definition
            .method(name, descriptor, is_static)
            .unwrap_or_else(|| panic!("missing Thread.{name}{descriptor}"))
            .access_flags();
        assert_eq!(flags, expected, "wrong flags for Thread.{name}{descriptor}");
    }

    let original_name = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "getName", "()Ljava/lang/String;", ())
        .await?;
    let result: Result<()> = jvm
        .invoke_virtual(
            &thread,
            &thread.class_definition().name(),
            "setName",
            "(Ljava/lang/String;)V",
            (ClassInstanceRef::<java_runtime::classes::java::lang::String>::new(None),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("setName(null) must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    let unchanged_name = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "getName", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(
        JavaLangString::to_rust_string(&jvm, &unchanged_name).await?,
        JavaLangString::to_rust_string(&jvm, &original_name).await?
    );

    let renamed = JavaLangString::from_rust_string(&jvm, "renamed").await?;
    let _: () = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "setName", "(Ljava/lang/String;)V", (renamed,))
        .await?;
    let actual_name = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "getName", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual_name).await?, "renamed");

    assert!(
        !jvm.invoke_virtual::<_, bool>(&thread, &thread.class_definition().name(), "isDaemon", "()Z", ())
            .await?
    );
    let _: () = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "setDaemon", "(Z)V", (true,))
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&thread, &thread.class_definition().name(), "isDaemon", "()Z", ())
            .await?
    );

    assert!(
        !jvm.invoke_virtual::<_, bool>(&thread, &thread.class_definition().name(), "isInterrupted", "()Z", ())
            .await?
    );
    let _: () = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "interrupt", "()V", ())
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&thread, &thread.class_definition().name(), "isInterrupted", "()Z", ())
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&thread, &thread.class_definition().name(), "isInterrupted", "()Z", ())
            .await?
    );

    let current = jvm.invoke_static("java/lang/Thread", "currentThread", "()Ljava/lang/Thread;", ()).await?;
    let _: () = jvm
        .invoke_virtual(&current, &current.class_definition().name(), "interrupt", "()V", ())
        .await?;
    assert!(jvm.invoke_static::<_, bool>("java/lang/Thread", "interrupted", "()Z", ()).await?);
    assert!(!jvm.invoke_static::<_, bool>("java/lang/Thread", "interrupted", "()Z", ()).await?);
    assert!(
        !jvm.invoke_virtual::<_, bool>(&current, &current.class_definition().name(), "isInterrupted", "()Z", ())
            .await?
    );
    assert!(!jvm.get_field::<bool>(&current, "interrupted", "Z").await?);

    let result: Result<()> = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "join", "(J)V", (-1i64,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("join(-1) must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let _: () = jvm.invoke_virtual(&thread, &thread.class_definition().name(), "start", "()V", ()).await?;
    let result: Result<()> = jvm
        .invoke_virtual(&thread, &thread.class_definition().name(), "setDaemon", "(Z)V", (false,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("setDaemon after start must throw IllegalThreadStateException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalThreadStateException"));
    assert!(
        jvm.invoke_virtual::<_, bool>(&thread, &thread.class_definition().name(), "isDaemon", "()Z", ())
            .await?
    );

    Ok(())
}
