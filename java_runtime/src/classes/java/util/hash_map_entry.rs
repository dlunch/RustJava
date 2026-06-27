use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.HashMap$Entry
pub struct HashMapEntry;

impl HashMapEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/HashMap$Entry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map$Entry"],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
                    Self::init,
                    Default::default(),
                ),
                JavaMethodProto::new("getKey", "()Ljava/lang/Object;", Self::get_key, Default::default()),
                JavaMethodProto::new("getValue", "()Ljava/lang/Object;", Self::get_value, Default::default()),
                JavaMethodProto::new("setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::set_value, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("hash", "I", Default::default()),
                JavaFieldProto::new("key", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("value", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("next", "Ljava/util/HashMap$Entry;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        hash: i32,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
        next: ClassInstanceRef<HashMapEntry>,
    ) -> Result<()> {
        tracing::debug!("java.util.HashMap$Entry::<init>({this:?}, {hash:?}, {key:?}, {value:?}, {next:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "hash", "I", hash).await?;
        jvm.put_field(&mut this, "key", "Ljava/lang/Object;", key).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        jvm.put_field(&mut this, "next", "Ljava/util/HashMap$Entry;", next).await?;

        Ok(())
    }

    async fn get_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap$Entry::getKey({this:?})");

        jvm.get_field(&this, "key", "Ljava/lang/Object;").await
    }

    async fn get_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap$Entry::getValue({this:?})");

        jvm.get_field(&this, "value", "Ljava/lang/Object;").await
    }

    async fn set_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap$Entry::setValue({this:?}, {value:?})");

        let old_value = jvm.get_field(&this, "value", "Ljava/lang/Object;").await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;

        Ok(old_value)
    }
}
