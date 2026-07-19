use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Class, ClassLoader, String},
};

// class org.rustjava.lang.RustJarClassLoader
pub struct RustJarClassLoader;

impl RustJarClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/rustjava/lang/RustJarClassLoader",
            parent_class: Some("java/lang/ClassLoader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([Ljava/lang/String;Ljava/lang/ClassLoader;)V", Self::init, Default::default()),
                JavaMethodProto::new("findClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::find_class, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("classPaths", "[Ljava/lang/String;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        class_paths: ClassInstanceRef<Array<String>>,
        parent: ClassInstanceRef<ClassLoader>,
    ) -> Result<()> {
        tracing::debug!("org.rustjava.lang.RustJarClassLoader::<init>({this:?}, {class_paths:?}, {parent:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        jvm.put_field(&mut this, "classPaths", "[Ljava/lang/String;", class_paths).await?;

        Ok(())
    }

    async fn find_class(
        jvm: &Jvm,
        runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("org.rustjava.lang.RustJarClassLoader::findClass({this:?}, {name:?})");

        let name = JavaLangString::to_rust_string(jvm, &name).await?;
        let class_paths = jvm.get_field(&this, "classPaths", "[Ljava/lang/String;").await?;
        let class_paths: Vec<ClassInstanceRef<String>> = jvm.load_array(&class_paths, 0, jvm.array_length(&class_paths).await? as usize).await?;

        for class_path in class_paths {
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
