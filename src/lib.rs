extern crate alloc;

mod runtime;

use core::future::ready;
use std::{io::Write, path::Path};

use java_runtime::{get_bootstrap_class_loader, Runtime};
use jvm::{runtime::JavaLangString, JavaValue, Jvm, Result};
use jvm_rust::{ClassDefinitionImpl, JvmDetailImpl};

use runtime::RuntimeImpl;

pub async fn create_jvm<T>(stdout: T, class_path: &[&Path]) -> Result<Jvm>
where
    T: Sync + Send + Write + 'static,
{
    let runtime = Box::new(RuntimeImpl::new(stdout)) as Box<dyn Runtime>;

    let bootstrap_class_loader = get_bootstrap_class_loader(move |name: &str, proto| {
        ready(Box::new(ClassDefinitionImpl::from_class_proto(name, proto, runtime.clone())) as Box<_>)
    });

    let class_path_str = class_path.iter().map(|x| x.to_str().unwrap()).collect::<Vec<_>>().join(":");

    let properties = [("java.class.path", class_path_str.as_str())].into_iter().collect();

    Jvm::new(JvmDetailImpl, bootstrap_class_loader, properties).await
}

pub async fn get_main_class_name(jvm: &Jvm, jar_path: &Path) -> Result<String> {
    let filename = JavaLangString::from_rust_string(jvm, jar_path.to_str().unwrap()).await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (filename,)).await?;
    let jar_file = jvm.new_class("java/util/jar/JarFile", "(Ljava/io/File;)V", (file,)).await?;

    let manifest = jvm.invoke_virtual(&jar_file, "getManifest", "()Ljava/util/jar/Manifest;", ()).await?;
    let attributes = jvm
        .invoke_virtual(&manifest, "getMainAttributes", "()Ljava/util/jar/Attributes;", ())
        .await?;

    let main_class = jvm
        .invoke_virtual(
            &attributes,
            "getValue",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (JavaLangString::from_rust_string(jvm, "Main-Class").await?,),
        )
        .await?;

    JavaLangString::to_rust_string(jvm, &main_class).await
}

pub async fn run_java_main(jvm: &Jvm, main_class_name: &str, args: &[String]) -> Result<()> {
    let mut java_args = Vec::with_capacity(args.len());
    for arg in args {
        java_args.push(JavaLangString::from_rust_string(jvm, arg).await?);
    }
    let mut array = jvm.instantiate_array("Ljava/lang/String;", args.len()).await?;
    jvm.store_array(&mut array, 0, java_args).await.unwrap();

    let normalized_name = main_class_name.replace('.', "/");
    jvm.invoke_static(&normalized_name, "main", "([Ljava/lang/String;)V", [JavaValue::Object(Some(array))])
        .await?;

    Ok(())
}
