use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// final class java.util.TreeMap$Entry
pub struct TreeMapEntry;

impl TreeMapEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TreeMap$Entry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map$Entry"],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/TreeMap$Entry;)V",
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
            ],
            fields: vec![
                JavaFieldProto::new("key", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("value", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("left", "Ljava/util/TreeMap$Entry;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("right", "Ljava/util/TreeMap$Entry;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("parent", "Ljava/util/TreeMap$Entry;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("color", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::FINAL,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
        parent: ClassInstanceRef<Self>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "key", "Ljava/lang/Object;", key).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        jvm.put_field(&mut this, "parent", "Ljava/util/TreeMap$Entry;", parent).await?;
        jvm.put_field(&mut this, "color", "Z", true).await
    }

    async fn get_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "key", "Ljava/lang/Object;").await
    }

    async fn get_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "value", "Ljava/lang/Object;").await
    }

    async fn set_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
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
}
