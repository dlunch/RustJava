use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{
    runtime::{JavaLangClass, JavaLangString},
    ClassInstanceRef, Jvm, Result,
};

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

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
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

        let class = runtime.define_class_rust(&name, proto.unwrap()).await?;
        let java_class = JavaLangClass::from_rust_class(jvm, class, Some(this.into())).await?;

        Ok(java_class.into())
    }
}
