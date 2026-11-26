use alloc::{boxed::Box, vec::Vec};

use jvm::{Array, ClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Class, ClassLoader, String},
};

// class org.rustjava.lang.RustJarClassLoader
#[java_class(name = "org/rustjava/lang/RustJarClassLoader", extends = "java/lang/ClassLoader", public)]
pub struct RustJarClassLoader {
    #[field(private)]
    classPaths: ClassInstanceRef<Array<String>>,
}

#[java_methods]
impl RustJarClassLoader {
    #[method(constructor, public)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        class_paths: ClassInstanceRef<Array<String>>,
        parent: ClassInstanceRef<ClassLoader>,
    ) -> Result<()> {
        tracing::debug!("org.rustjava.lang.RustJarClassLoader::<init>({this:?}, {class_paths:?}, {parent:?})");

        if class_paths.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "classPaths").await);
        }

        let _: () = jvm
            .invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        jvm.put_field(
            &mut this,
            "org/rustjava/lang/RustJarClassLoader",
            "classPaths",
            "[Ljava/lang/String;",
            class_paths,
        )
        .await?;

        Ok(())
    }

    #[method(protected)]
    async fn find_class(
        jvm: &Jvm,
        runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("org.rustjava.lang.RustJarClassLoader::findClass({this:?}, {name:?})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }

        let name = JavaLangString::to_rust_string(jvm, &name).await?;
        let class_paths: Box<dyn ClassInstance> = jvm
            .get_field(&this, "org/rustjava/lang/RustJarClassLoader", "classPaths", "[Ljava/lang/String;")
            .await?;
        let class_paths: Vec<ClassInstanceRef<String>> = jvm.load_array(&class_paths, 0, jvm.array_length(&class_paths).await? as usize).await?;

        for class_path in class_paths {
            if class_path.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "classPath").await);
            }
            let class_path = JavaLangString::to_rust_string(jvm, &class_path).await?;
            if !class_path.ends_with(".rustjar") {
                continue;
            }

            if let Some(class) = runtime.find_rustjar_class(jvm, &class_path, &name).await? {
                let class = jvm.register_class(class, Some(this.clone().into())).await?;
                return Ok(class.into());
            }
        }

        Ok(None.into())
    }
}
