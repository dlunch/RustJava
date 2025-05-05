extern crate alloc;

mod runtime;

use std::{io::Write, path::Path};

use java_runtime::{RT_RUSTJAR, Runtime, get_bootstrap_class_loader};
use jvm::{JavaError, JavaValue, Jvm, Result, runtime::JavaLangString};

use runtime::RuntimeImpl;

pub enum StartType<'a> {
    Jar(&'a Path),
    Class(&'a Path),
}

pub async fn run<T, S>(stdout: T, start_type: StartType<'_>, args: &[S], class_path: &[&Path]) -> anyhow::Result<()>
where
    T: Sync + Send + Write + 'static,
    S: AsRef<str>,
{
    let jvm = create_jvm(stdout, &start_type, class_path).await?;

    let result = invoke_entrypoint(&jvm, &start_type, args).await;

    if let Err(x) = result {
        Err(match x {
            JavaError::JavaException(x) => {
                let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await.unwrap();
                let print_writer = jvm
                    .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
                    .await
                    .unwrap();

                let _: () = jvm
                    .invoke_virtual(&x, "printStackTrace", "(Ljava/io/PrintWriter;)V", (print_writer,))
                    .await
                    .unwrap();

                let trace = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", []).await.unwrap();

                anyhow::anyhow!("Java Exception:\n{}", JavaLangString::to_rust_string(&jvm, &trace).await.unwrap())
            }
            JavaError::FatalError(x) => anyhow::anyhow!("Fatal error: {x}"),
        })
    } else {
        Ok(result?)
    }
}

async fn create_jvm<T>(stdout: T, start_type: &StartType<'_>, class_path: &[&Path]) -> Result<Jvm>
where
    T: Sync + Send + Write + 'static,
{
    let runtime = Box::new(RuntimeImpl::new(stdout)) as Box<dyn Runtime>;

    let bootstrap_class_loader = get_bootstrap_class_loader(runtime.clone());

    let mut class_path_str = class_path.iter().map(|x| x.to_str().unwrap()).collect::<Vec<_>>().join(":");
    if let StartType::Jar(x) = start_type {
        class_path_str = format!("{}:{}", x.to_str().unwrap(), class_path_str);
    }

    // add rt.rustjar
    // TODO do we need boot class path?
    let class_path_str = format!("{RT_RUSTJAR}:{class_path_str}");
    let properties = [("java.class.path", class_path_str.as_str())].into_iter().collect();

    Jvm::new(bootstrap_class_loader, move || runtime.current_task_id(), properties).await
}

async fn invoke_entrypoint<S>(jvm: &Jvm, start_type: &StartType<'_>, args: &[S]) -> Result<()>
where
    S: AsRef<str>,
{
    let main_class_name = match start_type {
        StartType::Jar(x) => &get_jar_main_class(jvm, x).await?,
        StartType::Class(x) => x.file_stem().unwrap().to_str().unwrap(),
    };

    let mut java_args = Vec::with_capacity(args.len());
    for arg in args {
        java_args.push(JavaLangString::from_rust_string(jvm, arg.as_ref()).await?);
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
