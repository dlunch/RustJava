use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::MapEntry},
};

// class java.util.Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry
#[java_class(
    name = "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
    implements("java/util/Map$Entry")
)]
pub struct CollectionsUnmodifiableMapEntry {
    #[field(private, final)]
    e: ClassInstanceRef<MapEntry>,
}

#[java_methods]
impl CollectionsUnmodifiableMapEntry {
    #[method(constructor, descriptor = "(Ljava/util/Map$Entry;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, entry: ClassInstanceRef<Object>) -> Result<()> {
        if entry.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "entry").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
            "e",
            "Ljava/util/Map$Entry;",
            entry,
        )
        .await
    }

    #[method(public)]
    async fn get_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry: ClassInstanceRef<Object> = jvm
            .get_field(
                &this,
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                "e",
                "Ljava/util/Map$Entry;",
            )
            .await?;
        jvm.invoke_virtual(&entry, &entry.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
            .await
    }

    #[method(public)]
    async fn get_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry: ClassInstanceRef<Object> = jvm
            .get_field(
                &this,
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                "e",
                "Ljava/util/Map$Entry;",
            )
            .await?;
        jvm.invoke_virtual(&entry, &entry.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
            .await
    }

    #[method(public)]
    async fn set_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _: ClassInstanceRef<Self>,
        _: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map entry").await)
    }

    #[method(public)]
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

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let entry: ClassInstanceRef<Object> = jvm
            .get_field(
                &this,
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                "e",
                "Ljava/util/Map$Entry;",
            )
            .await?;
        jvm.invoke_virtual(&entry, "java/lang/Object", "hashCode", "()I", ()).await
    }

    #[method(descriptor = "()Ljava/lang/String;", public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry: ClassInstanceRef<Object> = jvm
            .get_field(
                &this,
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                "e",
                "Ljava/util/Map$Entry;",
            )
            .await?;
        jvm.invoke_virtual(&entry, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }
}
