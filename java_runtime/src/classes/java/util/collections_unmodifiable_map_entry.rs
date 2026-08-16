use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry
pub struct CollectionsUnmodifiableMapEntry;

impl CollectionsUnmodifiableMapEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map$Entry"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Map$Entry;)V", Self::init, MethodAccessFlags::empty()),
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
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new(
                "e",
                "Ljava/util/Map$Entry;",
                FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
            )],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, entry: ClassInstanceRef<Object>) -> Result<()> {
        if entry.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "entry").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "e", "Ljava/util/Map$Entry;", entry).await
    }

    async fn get_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry: ClassInstanceRef<Object> = jvm.get_field(&this, "e", "Ljava/util/Map$Entry;").await?;
        jvm.invoke_virtual(&entry, &entry.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
            .await
    }

    async fn get_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry: ClassInstanceRef<Object> = jvm.get_field(&this, "e", "Ljava/util/Map$Entry;").await?;
        jvm.invoke_virtual(&entry, &entry.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
            .await
    }

    async fn set_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _: ClassInstanceRef<Self>,
        _: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map entry").await)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(other.as_ref(), "java/util/Map$Entry") {
            return Ok(false);
        }
        if this.identity() == other.identity() {
            return Ok(true);
        }
        let key: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &this,
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                "getKey",
                "()Ljava/lang/Object;",
                (),
            )
            .await?;
        let other_key: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&other, &other.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
            .await?;
        let keys_equal = if key.is_null() {
            other_key.is_null()
        } else {
            jvm.invoke_virtual::<_, bool>(&key, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other_key,))
                .await?
        };
        if !keys_equal {
            return Ok(false);
        }
        let value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &this,
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                "getValue",
                "()Ljava/lang/Object;",
                (),
            )
            .await?;
        let other_value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&other, &other.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
            .await?;
        if value.is_null() {
            Ok(other_value.is_null())
        } else {
            jvm.invoke_virtual(&value, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other_value,))
                .await
        }
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let entry: ClassInstanceRef<Object> = jvm.get_field(&this, "e", "Ljava/util/Map$Entry;").await?;
        jvm.invoke_virtual(&entry, "java/lang/Object", "hashCode", "()I", ()).await
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry: ClassInstanceRef<Object> = jvm.get_field(&this, "e", "Ljava/util/Map$Entry;").await?;
        jvm.invoke_virtual(&entry, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }
}
