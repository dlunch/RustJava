use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        lang::{Class, String},
        net::URL,
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.net.URLClassLoader
pub struct URLClassLoader {}

impl URLClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/ClassLoader"), // TODO java.security.SecureClassLoader
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([Ljava/net/URL;)V", Self::init, Default::default()),
                JavaMethodProto::new("findClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::find_class, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, urls: ClassInstanceRef<Array<URL>>) -> Result<()> {
        tracing::debug!("java.net.URLClassLoader::<init>({:?}, {:?})", &this, &urls);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (None,))
            .await?;

        Ok(())
    }

    async fn find_class(
        _jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.net.URLClassLoader::findClass({:?}, {:?})", &this, name);

        Ok(None.into())
    }
}
