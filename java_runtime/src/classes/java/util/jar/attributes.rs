use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.util.jar.Attributes
pub struct Attributes;

impl Attributes {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/jar/Attributes",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Cloneable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "putValue",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    Self::put_value,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getValue",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_value,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![JavaFieldProto::new("map", "Ljava/util/Map;", FieldAccessFlags::PROTECTED)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.jar.Attributes::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
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
        tracing::debug!("java.util.jar.Attributes::putValue({this:?}, {name:?}, {value:?})");

        // TODO we should store key in Attributes.Name type
        let map = jvm.get_field(&this, "map", "Ljava/util/Map;").await?;
        let old = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (name, value),
            )
            .await?;

        Ok(old)
    }

    async fn get_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.jar.Attributes::getValue({this:?}, {name:?})");

        let map = jvm.get_field(&this, "map", "Ljava/util/Map;").await?;
        let value = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "get",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (name,),
            )
            .await?;

        Ok(value)
    }
}
