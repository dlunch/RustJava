use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::lang::{Object, String},
    RuntimeClassProto, RuntimeContext,
};

// class java.util.Properties
pub struct Properties {}

impl Properties {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Properties",
            parent_class: Some("java/util/Hashtable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_property,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                    Self::set_property,
                    Default::default(),
                ),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Properties::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/util/Hashtable", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_property(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Properties::getProperty({:?}, {:?})", &this, &key);

        let result = jvm.invoke_virtual(&this, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,)).await?;

        // TODO defaults

        Ok(result)
    }

    async fn set_property(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<String>,
        value: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Properties::setProperty({:?}, {:?}, {:?})", &this, &key, &value);

        let old = jvm
            .invoke_virtual(&this, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;

        Ok(old)
    }
}
