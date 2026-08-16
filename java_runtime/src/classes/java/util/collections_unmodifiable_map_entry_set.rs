use alloc::{vec, vec::Vec};

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, AsClassInstance, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableMap$UnmodifiableEntrySet
pub struct CollectionsUnmodifiableMapEntrySet;

impl CollectionsUnmodifiableMapEntrySet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet",
            parent_class: Some("java/util/Collections$UnmodifiableSet"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Set;)V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toArray", "()[Ljava/lang/Object;", Self::to_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "toArray",
                    "([Ljava/lang/Object;)[Ljava/lang/Object;",
                    Self::to_typed_array,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("containsAll", "(Ljava/util/Collection;)Z", Self::contains_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, set: ClassInstanceRef<Object>) -> Result<()> {
        jvm.invoke_special(&this, "java/util/Collections$UnmodifiableSet", "<init>", "(Ljava/util/Set;)V", (set,))
            .await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&set, &set.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        Ok(jvm
            .new_class(
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1",
                "(Ljava/util/Iterator;)V",
                (iterator,),
            )
            .await?
            .into())
    }

    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        let entries: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&set, &set.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let length = jvm.array_length(&entries).await?;
        let mut wrapped: Vec<ClassInstanceRef<Object>> = Vec::with_capacity(length);
        for entry in jvm.load_array::<ClassInstanceRef<Object>>(&entries, 0, length).await? {
            wrapped.push(
                jvm.new_class(
                    "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                    "(Ljava/util/Map$Entry;)V",
                    (entry,),
                )
                .await?
                .into(),
            );
        }
        let mut result: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", length).await?.into();
        if !wrapped.is_empty() {
            jvm.store_array(&mut result, 0, wrapped).await?;
        }
        Ok(result)
    }

    async fn to_typed_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        destination: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Array<Object>>> {
        if destination.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let class_name = destination.class_definition().name();
        let component_descriptor = class_name.strip_prefix('[').unwrap();
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        let entries: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&set, &set.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let length = jvm.array_length(&entries).await?;
        let mut wrapped: Vec<ClassInstanceRef<Object>> = Vec::with_capacity(length);
        for entry in jvm.load_array::<ClassInstanceRef<Object>>(&entries, 0, length).await? {
            wrapped.push(
                jvm.new_class(
                    "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                    "(Ljava/util/Map$Entry;)V",
                    (entry,),
                )
                .await?
                .into(),
            );
        }

        let destination_length = jvm.array_length(&destination).await?;
        let mut result = if destination_length < length {
            ClassInstanceRef::from(jvm.instantiate_array(component_descriptor, length).await?)
        } else {
            destination
        };
        for (index, entry) in wrapped.into_iter().enumerate() {
            if !jvm.array_store_allowed(result.as_class_instance(), entry.as_class_instance()) {
                return Err(jvm.exception("java/lang/ArrayStoreException", &entry.class_definition().name()).await);
            }
            jvm.store_array(&mut result, index, core::iter::once(entry)).await?;
        }
        if destination_length > length {
            jvm.store_array(&mut result, length, core::iter::once(ClassInstanceRef::<Object>::from(None)))
                .await?;
        }
        Ok(result)
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        if candidate.is_null() || !jvm.is_instance(candidate.as_ref(), "java/util/Map$Entry") {
            return Ok(false);
        }
        let safe: ClassInstanceRef<Object> = jvm
            .new_class(
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                "(Ljava/util/Map$Entry;)V",
                (candidate,),
            )
            .await?
            .into();
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&set, &set.class_definition().name(), "contains", "(Ljava/lang/Object;)Z", (safe,))
            .await
    }

    async fn contains_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &collection,
                &collection.class_definition().name(),
                "iterator",
                "()Ljava/util/Iterator;",
                (),
            )
            .await?;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let entry: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            if !jvm
                .invoke_virtual::<_, bool>(
                    &this,
                    "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet",
                    "contains",
                    "(Ljava/lang/Object;)Z",
                    (entry,),
                )
                .await?
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if !other.is_null() && this.identity() == other.identity() {
            return Ok(true);
        }
        if other.is_null() || !jvm.is_instance(other.as_ref(), "java/util/Set") {
            return Ok(false);
        }
        let size: i32 = jvm
            .invoke_virtual(&this, "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet", "size", "()I", ())
            .await?;
        let other_size: i32 = jvm.invoke_virtual(&other, &other.class_definition().name(), "size", "()I", ()).await?;
        if size != other_size {
            return Ok(false);
        }
        jvm.invoke_virtual(
            &this,
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet",
            "containsAll",
            "(Ljava/util/Collection;)Z",
            (other,),
        )
        .await
    }
}
