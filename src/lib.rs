extern crate alloc;

mod runtime;

use std::{io::Write, path::Path};

use java_runtime::{get_bootstrap_class_loader, Runtime};
use jvm::{runtime::JavaLangString, JavaValue, Jvm, Result};

use runtime::RuntimeImpl;

pub enum StartType<'a> {
    Jar(&'a Path),
    Class(&'a Path),
}

pub async fn run<'a, T, S>(stdout: T, start_type: StartType<'a>, args: &[S], class_path: &[&Path]) -> Result<()>
where
    T: Sync + Send + Write + 'static,
    S: AsRef<str>,
{
    let runtime = Box::new(RuntimeImpl::new(stdout)) as Box<dyn Runtime>;

    let bootstrap_class_loader = get_bootstrap_class_loader(runtime.clone());

    let mut class_path_str = class_path.iter().map(|x| x.to_str().unwrap()).collect::<Vec<_>>().join(":");
    if let StartType::Jar(x) = start_type {
        class_path_str = format!("{}:{}", x.to_str().unwrap(), class_path_str);
    }

    let properties = [("java.class.path", class_path_str.as_str())].into_iter().collect();

    let jvm = Jvm::new(bootstrap_class_loader, properties).await?;

    let main_class_name = match start_type {
        StartType::Jar(x) => &get_jar_main_class(&jvm, x).await?,
        StartType::Class(x) => x.file_stem().unwrap().to_str().unwrap(),
    };

    let mut java_args = Vec::with_capacity(args.len());
    for arg in args {
        java_args.push(JavaLangString::from_rust_string(&jvm, arg.as_ref()).await?);
    }
    let mut array = jvm.instantiate_array("Ljava/lang/String;", args.len()).await?;
    jvm.store_array(&mut array, 0, java_args).await.unwrap();

    let normalized_name = main_class_name.replace('.', "/");
    let _: () = jvm
        .invoke_static(&normalized_name, "main", "([Ljava/lang/String;)V", [JavaValue::Object(Some(array))])
        .await?;

    Ok(())
}

async fn get_jar_main_class(jvm: &Jvm, jar_path: &Path) -> Result<String> {
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
