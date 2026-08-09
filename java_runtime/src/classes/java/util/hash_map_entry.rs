use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::HashMap;

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
                JavaMethodProto::new("getKey", "()Ljava/lang/Object;", Self::get_key, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getValue", "()Ljava/lang/Object;", Self::get_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "setValue",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                    Self::set_value,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("onAccess", "(Ljava/util/HashMap;)V", Self::on_access, Default::default()),
                JavaMethodProto::new("onRemoval", "(Ljava/util/HashMap;)V", Self::on_removal, Default::default()),
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

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(other.as_ref(), "java/util/Map$Entry") {
            return Ok(false);
        }

        let key: ClassInstanceRef<Object> = jvm.get_field(&this, "key", "Ljava/lang/Object;").await?;
        let other_key: ClassInstanceRef<Object> = jvm.invoke_virtual(&other, "getKey", "()Ljava/lang/Object;", ()).await?;
        let keys_equal = if key.is_null() {
            other_key.is_null()
        } else {
            jvm.invoke_virtual(&key, "equals", "(Ljava/lang/Object;)Z", (other_key,)).await?
        };
        if !keys_equal {
            return Ok(false);
        }

        let value: ClassInstanceRef<Object> = jvm.get_field(&this, "value", "Ljava/lang/Object;").await?;
        let other_value: ClassInstanceRef<Object> = jvm.invoke_virtual(&other, "getValue", "()Ljava/lang/Object;", ()).await?;
        if value.is_null() {
            Ok(other_value.is_null())
        } else {
            jvm.invoke_virtual(&value, "equals", "(Ljava/lang/Object;)Z", (other_value,)).await
        }
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let key: ClassInstanceRef<Object> = jvm.get_field(&this, "key", "Ljava/lang/Object;").await?;
        let value: ClassInstanceRef<Object> = jvm.get_field(&this, "value", "Ljava/lang/Object;").await?;
        let key_hash = if key.is_null() {
            0
        } else {
            jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?
        };
        let value_hash = if value.is_null() {
            0
        } else {
            jvm.invoke_virtual(&value, "hashCode", "()I", ()).await?
        };
        Ok(key_hash ^ value_hash)
    }

    async fn on_access(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<HashMap>) -> Result<()> {
        Ok(())
    }

    async fn on_removal(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<HashMap>) -> Result<()> {
        Ok(())
    }
}
