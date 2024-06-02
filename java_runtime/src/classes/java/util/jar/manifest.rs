use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{io::InputStream, util::jar::Attributes},
    RuntimeClassProto, RuntimeContext,
};

// class java.util.jar.Manifest
pub struct Manifest {}

impl Manifest {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("read", "(Ljava/io/InputStream;)V", Self::read, Default::default()),
                JavaMethodProto::new(
                    "getMainAttributes",
                    "()Ljava/util/jar/Attributes;",
                    Self::get_main_attributes,
                    Default::default(),
                ),
            ],
            fields: vec![JavaFieldProto::new("attrs", "Ljava/util/jar/Attributes;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, is: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.util.jar.Manifest::<init>({:?}, {:?})", &this, &is);

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.invoke_virtual(&this, "read", "(Ljava/io/InputStream;)V", (is,)).await?;

        Ok(())
    }

    async fn read(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, is: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.util.jar.Manifest::read({:?}, {:?})", &this, &is);

        Ok(())
    }

    async fn get_main_attributes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Attributes>> {
        tracing::debug!("java.util.jar.Manifest::getMainAttributes({:?})", &this);

        jvm.get_field(&this, "attrs", "Ljava/util/jar/Attributes;").await
    }
}
