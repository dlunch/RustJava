use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::lang::{Class, ClassLoader, String},
    loader::get_proto,
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.RuntimeClassLoader
pub struct RuntimeClassLoader {}

impl RuntimeClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "rustjava/RuntimeClassLoader",
            parent_class: Some("java/lang/ClassLoader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/ClassLoader;)V", Self::init, Default::default()),
                JavaMethodProto::new("findClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::find_class, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> Result<()> {
        tracing::debug!("rustjava.RuntimeClassLoader::<init>({:?}, {:?})", &this, &parent);

        let _: () = jvm
            .invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        Ok(())
    }

    async fn find_class(
        jvm: &Jvm,
        runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("rustjava.RuntimeClassLoader::findClass({:?}, {:?})", &this, name);

        let name = JavaLangString::to_rust_string(jvm, &name).await?;

        let proto = get_proto(&name);
        if proto.is_none() {
            return Ok(None.into());
        }

        let class = runtime.define_runtime_class(jvm, proto.unwrap()).await?;
        let java_class = jvm.register_class(class, Some(this.into())).await?;

        Ok(java_class.into())
    }
}
