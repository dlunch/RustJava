use alloc::vec;

use java_class_proto::{JavaMethodProto, JavaResult};
use jvm::{ClassInstanceRef, Jvm};

use crate::{
    classes::java::lang::{Class, ClassLoader, String},
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.ClassPathClassLoader
pub struct ClassPathClassLoader {}

impl ClassPathClassLoader {
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

    async fn init(jvm: &mut Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::<init>({:?}, {:?})", &this, &parent);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        Ok(())
    }

    async fn find_class(
        _jvm: &mut Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<Class>> {
        tracing::debug!("rustjava.ClassPathClassLoader::findClass({:?}, {:?})", &this, name);

        // TODO

        Ok(None.into())
    }
}
