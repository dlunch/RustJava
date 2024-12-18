use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.util.jar.Attributes
pub struct Attributes;

impl Attributes {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/jar/Attributes",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "putValue",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    Self::put_value,
                    Default::default(),
                ),
                JavaMethodProto::new("getValue", "(Ljava/lang/String;)Ljava/lang/String;", Self::get_value, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("map", "Ljava/util/Map;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.jar.Manifest::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        //XXX should be HashMap, but we don't have yet.
        let map = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
        jvm.put_field(&mut this, "map", "Ljava/util/Map;", map).await?;

        Ok(())
    }

    async fn put_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
        value: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.jar.Attributes::putValue({:?}, {:?}, {:?})", &this, &name, &value);

        // TODO we should store key in Attributes.Name type
        let map = jvm.get_field(&this, "map", "Ljava/util/Map;").await?;
        let old = jvm
            .invoke_virtual(&map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (name, value))
            .await?;

        Ok(old)
    }

    async fn get_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.jar.Attributes::getValue({:?}, {:?})", &this, &name);

        let map = jvm.get_field(&this, "map", "Ljava/util/Map;").await?;
        let value = jvm.invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (name,)).await?;

        Ok(value)
    }
}
