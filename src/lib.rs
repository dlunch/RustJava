extern crate alloc;

mod runtime;

use std::{env, ffi::OsStr, io::Write, path::Path};

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

    if let Err(JavaError::JavaException(x)) = result {
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

        Err(anyhow::anyhow!(
            "Java Exception:\n{}",
            JavaLangString::to_rust_string(&jvm, &trace).await.unwrap()
        ))
    } else {
        Ok(result?)
    }
}

async fn create_jvm<T>(stdout: T, start_type: &StartType<'_>, class_path: &[&Path]) -> anyhow::Result<Jvm>
where
    T: Sync + Send + Write + 'static,
{
    let runtime = Box::new(RuntimeImpl::new(stdout)) as Box<dyn Runtime>;

    let bootstrap_class_loader = get_bootstrap_class_loader(runtime.clone());

    let class_path_str = build_class_path(start_type, class_path)?;
    let properties = [("java.class.path", class_path_str.as_str())].into_iter().collect();

    Ok(Jvm::new(bootstrap_class_loader, move || runtime.current_task_id(), properties).await?)
}

fn build_class_path(start_type: &StartType<'_>, class_path: &[&Path]) -> anyhow::Result<String> {
    let mut entries = vec![OsStr::new(RT_RUSTJAR)];
    if let StartType::Jar(path) = start_type {
        entries.push(path.as_os_str());
    }
    entries.extend(class_path.iter().map(|path| path.as_os_str()));

    env::join_paths(entries)?
        .into_string()
        .map_err(|_| anyhow::anyhow!("Class path contains a non-UTF-8 path"))
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

#[cfg(test)]
mod tests {
    use std::{env, path::Path};

    use java_runtime::RT_RUSTJAR;

    use super::{StartType, build_class_path};

    #[test]
    fn class_launch_classpath_preserves_order_and_empty_entries() {
        assert_eq!(
            build_class_path(
                &StartType::Class(Path::new("Main")),
                &[Path::new("classes"), Path::new(""), Path::new("lib/dependency.jar")],
            )
            .unwrap(),
            env::join_paths([RT_RUSTJAR, "classes", "", "lib/dependency.jar"])
                .unwrap()
                .into_string()
                .unwrap()
        );
    }

    #[test]
    fn jar_launch_classpath_has_no_trailing_separator_without_user_entries() {
        assert_eq!(
            build_class_path(&StartType::Jar(Path::new("app.jar")), &[]).unwrap(),
            env::join_paths([RT_RUSTJAR, "app.jar"]).unwrap().into_string().unwrap()
        );
    }

    #[test]
    fn jar_library_api_preserves_explicit_user_classpath() {
        assert_eq!(
            build_class_path(&StartType::Jar(Path::new("app.jar")), &[Path::new("lib/dependency.jar")]).unwrap(),
            env::join_paths([RT_RUSTJAR, "app.jar", "lib/dependency.jar"])
                .unwrap()
                .into_string()
                .unwrap()
        );
    }

    #[cfg(unix)]
    #[test]
    fn classpath_rejects_non_utf8_entries() {
        use std::{ffi::OsString, os::unix::ffi::OsStringExt, path::PathBuf};

        let path = PathBuf::from(OsString::from_vec(vec![0xff]));
        let error = build_class_path(&StartType::Class(Path::new("Main")), &[&path]).unwrap_err();

        assert_eq!(error.to_string(), "Class path contains a non-UTF-8 path");
    }
}
