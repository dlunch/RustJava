use alloc::{collections::BTreeMap, vec};

use java_constants::{FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    Runtime,
    classes::java::{
        io::{InputStream, PrintStream},
        lang::{Object, Runtime as JavaRuntime, String, System},
        util::Properties,
    },
};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};
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

    let runtime_instance: ClassInstanceRef<JavaRuntime> = jvm.invoke_static("java/lang/Runtime", "getRuntime", "()Ljava/lang/Runtime;", ()).await?;
    let _: () = jvm.invoke_virtual(&runtime_instance, "java/lang/Runtime", "exit", "(I)V", (23,)).await?;
    assert_eq!(runtime.exit_status(), Some(23));

    Ok(())
}

#[tokio::test]
async fn sys_01_to_06_descriptors_streams_properties_and_identity_hash() -> Result<()> {
    let system = System::as_proto();
    for (name, descriptor) in [
        ("identityHashCode", "(Ljava/lang/Object;)I"),
        ("setIn", "(Ljava/io/InputStream;)V"),
        ("setOut", "(Ljava/io/PrintStream;)V"),
        ("setErr", "(Ljava/io/PrintStream;)V"),
        ("getProperties", "()Ljava/util/Properties;"),
        ("getProperty", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"),
    ] {
        let method = system
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing java/lang/System.{name}{descriptor}"));
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC));
    }
    let input = system
        .fields
        .iter()
        .find(|field| field.name == "in" && field.descriptor == "Ljava/io/InputStream;")
        .expect("missing java/lang/System.in");
    assert!(
        input
            .access_flags
            .contains(FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL)
    );

    let runtime = TestRuntime::new_with_stdin(BTreeMap::new(), vec![0x41, 0x42]);
    let jvm = create_test_jvm(runtime).await?;

    let stdin: ClassInstanceRef<InputStream> = jvm.get_static_field("java/lang/System", "in", "Ljava/io/InputStream;").await?;
    assert!(!stdin.is_null());
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&stdin, "java/io/InputStream", "read", "()I", ()).await?,
        0x41
    );

    let mut bytes = jvm.instantiate_array("B", 1).await?;
    jvm.store_array(&mut bytes, 0, [7i8]).await?;
    let replacement_in: ClassInstanceRef<InputStream> = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?.into();
    let _: () = jvm
        .invoke_static("java/lang/System", "setIn", "(Ljava/io/InputStream;)V", (replacement_in.clone(),))
        .await?;
    let stored_in: ClassInstanceRef<InputStream> = jvm.get_static_field("java/lang/System", "in", "Ljava/io/InputStream;").await?;
    assert_eq!(stored_in.identity(), replacement_in.identity());

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let replacement_out: ClassInstanceRef<PrintStream> = jvm.new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output,)).await?.into();
    let _: () = jvm
        .invoke_static("java/lang/System", "setOut", "(Ljava/io/PrintStream;)V", (replacement_out.clone(),))
        .await?;
    let stored_out: ClassInstanceRef<PrintStream> = jvm.get_static_field("java/lang/System", "out", "Ljava/io/PrintStream;").await?;
    assert_eq!(stored_out.identity(), replacement_out.identity());

    let _: () = jvm
        .invoke_static("java/lang/System", "setErr", "(Ljava/io/PrintStream;)V", (None,))
        .await?;
    let stored_err: ClassInstanceRef<PrintStream> = jvm.get_static_field("java/lang/System", "err", "Ljava/io/PrintStream;").await?;
    assert!(stored_err.is_null());

    let properties: ClassInstanceRef<Properties> = jvm
        .invoke_static("java/lang/System", "getProperties", "()Ljava/util/Properties;", ())
        .await?;
    let properties_again: ClassInstanceRef<Properties> = jvm
        .invoke_static("java/lang/System", "getProperties", "()Ljava/util/Properties;", ())
        .await?;
    assert_eq!(properties.identity(), properties_again.identity());

    let fallback = JavaLangString::from_rust_string(&jvm, "fallback").await?;
    let missing = JavaLangString::from_rust_string(&jvm, "missing").await?;
    let value: ClassInstanceRef<String> = jvm
        .invoke_static(
            "java/lang/System",
            "getProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            (missing, fallback.clone()),
        )
        .await?;
    assert_eq!(value.identity(), fallback.identity());

    let result: Result<ClassInstanceRef<String>> = jvm
        .invoke_static(
            "java/lang/System",
            "getProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            (None, fallback),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("System.getProperty(null, default) must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let object: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let object_hash: i32 = jvm.invoke_special(&object, "java/lang/Object", "hashCode", "()I", ()).await?;
    let identity_hash: i32 = jvm
        .invoke_static("java/lang/System", "identityHashCode", "(Ljava/lang/Object;)I", (object,))
        .await?;
    assert_eq!(identity_hash, object_hash);
    assert_eq!(
        jvm.invoke_static::<_, i32>(
            "java/lang/System",
            "identityHashCode",
            "(Ljava/lang/Object;)I",
            (ClassInstanceRef::<Object>::new(None),),
        )
        .await?,
        0
    );

    Ok(())
}
