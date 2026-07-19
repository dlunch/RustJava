use alloc::vec;

use java_runtime::classes::{
    java::lang::{Class, ClassLoader},
    org::rustjava::lang::RustJarClassLoader,
};
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_find_class_uses_rustjar_runtime_source() -> Result<()> {
    let jvm = test_jvm().await?;

    let class_path = JavaLangString::from_rust_string(&jvm, "rt.rustjar").await?;
    let mut class_paths = jvm.instantiate_array("Ljava/lang/String;", 1).await?;
    jvm.store_array(&mut class_paths, 0, vec![class_path]).await?;
    let class_loader: ClassInstanceRef<RustJarClassLoader> = jvm
        .new_class(
            "org/rustjava/lang/RustJarClassLoader",
            "([Ljava/lang/String;Ljava/lang/ClassLoader;)V",
            (class_paths, None),
        )
        .await?
        .into();

    let name = JavaLangString::from_rust_string(&jvm, "java/util/Random").await?;
    let class: ClassInstanceRef<Class> = jvm
        .invoke_virtual(&class_loader, "findClass", "(Ljava/lang/String;)Ljava/lang/Class;", (name,))
        .await?;
    assert!(!class.is_null());

    let defining_loader: ClassInstanceRef<ClassLoader> = jvm.get_field(&class, "classLoader", "Ljava/lang/ClassLoader;").await?;
    assert!(jvm.is_instance(&**defining_loader, "org/rustjava/lang/RustJarClassLoader"));

    Ok(())
}
