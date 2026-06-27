use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// abstract class java.util.AbstractCollection
pub struct AbstractCollection;

impl AbstractCollection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractCollection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Collection"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("size", "()I", Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, Default::default()),
                JavaMethodProto::new_abstract("iterator", "()Ljava/util/Iterator;", Default::default()),
                JavaMethodProto::new("toArray", "()[Ljava/lang/Object;", Self::to_array, Default::default()),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, Default::default()),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, Default::default()),
                JavaMethodProto::new("clear", "()V", Self::clear, Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractCollection::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::isEmpty({this:?})");

        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;

        Ok(size == 0)
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::contains({this:?}, {element:?})");

        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "iterator", "()Ljava/util/Iterator;", ()).await?;
        loop {
            let has_next: bool = jvm.invoke_virtual(&iterator, "hasNext", "()Z", ()).await?;
            if !has_next {
                return Ok(false);
            }

            let current: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            if Self::object_equals(jvm, &element, &current).await? {
                return Ok(true);
            }
        }
    }

    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        tracing::debug!("java.util.AbstractCollection::toArray({this:?})");

        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "iterator", "()Ljava/util/Iterator;", ()).await?;
        let mut elements = vec![];
        loop {
            let has_next: bool = jvm.invoke_virtual(&iterator, "hasNext", "()Z", ()).await?;
            if !has_next {
                break;
            }

            let current: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            elements.push(current);
        }

        let mut array: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", elements.len()).await?.into();
        if !elements.is_empty() {
            jvm.store_array(&mut array, 0, elements).await?;
        }

        Ok(array)
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::add({this:?}, {element:?})");

        Err(jvm.exception("java/lang/UnsupportedOperationException", "AbstractCollection.add").await)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::remove({this:?}, {element:?})");

        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "AbstractCollection.remove")
            .await)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractCollection::clear({this:?})");

        Err(jvm.exception("java/lang/UnsupportedOperationException", "AbstractCollection.clear").await)
    }

    async fn object_equals(jvm: &Jvm, left: &ClassInstanceRef<Object>, right: &ClassInstanceRef<Object>) -> Result<bool> {
        if left.is_null() {
            return Ok(right.is_null());
        }

        if right.is_null() {
            return Ok(false);
        }

        jvm.invoke_virtual(left, "equals", "(Ljava/lang/Object;)Z", (right.clone(),)).await
    }
}
