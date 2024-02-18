use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::lang::{Class, ClassLoader, String},
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.ArrayClassLoader
pub struct ArrayClassLoader {}

impl ArrayClassLoader {
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
        tracing::debug!("rustjava.ArrayClassLoader::<init>({:?}, {:?})", &this, &parent);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        Ok(())
    }

    async fn find_class(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("rustjava.ArrayClassLoader::findClass({:?}, {:?})", &this, name);

        let name = JavaLangString::to_rust_string(jvm, name.into())?;

        if let Some(element_type_name) = name.strip_prefix('[') {
            let class = jvm.define_array_class(element_type_name, this.into()).await?;

            return Ok(class.into());
        }

        Ok(None.into())
    }
}
