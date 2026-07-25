use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableCollection
pub struct CollectionsUnmodifiableCollection;

impl CollectionsUnmodifiableCollection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableCollection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Collection", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Collection;)V", Self::init, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toArray", "()[Ljava/lang/Object;", Self::to_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "toArray",
                    "([Ljava/lang/Object;)[Ljava/lang/Object;",
                    Self::to_typed_array,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("containsAll", "(Ljava/util/Collection;)Z", Self::contains_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("addAll", "(Ljava/util/Collection;)Z", Self::add_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("removeAll", "(Ljava/util/Collection;)Z", Self::remove_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("retainAll", "(Ljava/util/Collection;)Z", Self::retain_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("c", "Ljava/util/Collection;", FieldAccessFlags::FINAL)],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<()> {
        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "c", "Ljava/util/Collection;", collection).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let collection: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&collection, "size", "()I", ()).await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let collection: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&collection, "isEmpty", "()Z", ()).await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let collection: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&collection, "contains", "(Ljava/lang/Object;)Z", (element,)).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let collection: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&collection, "iterator", "()Ljava/util/Iterator;", ()).await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableCollection$1", "(Ljava/util/Iterator;)V", (iterator,))
            .await?
            .into())
    }

    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        let collection: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&collection, "toArray", "()[Ljava/lang/Object;", ()).await
    }

    async fn to_typed_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Array<Object>>> {
        let collection: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&collection, "toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;", (array,))
            .await
    }

    async fn contains_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        let backing: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&backing, "containsAll", "(Ljava/util/Collection;)Z", (collection,))
            .await
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let collection: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&collection, "toString", "()Ljava/lang/String;", ()).await
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    async fn remove_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    async fn retain_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }
}
