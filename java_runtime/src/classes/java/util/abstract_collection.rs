use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, AsClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

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
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new_abstract("size", "()I", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new_abstract(
                    "iterator",
                    "()Ljava/util/Iterator;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new("toArray", "()[Ljava/lang/Object;", Self::to_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "toArray",
                    "([Ljava/lang/Object;)[Ljava/lang/Object;",
                    Self::to_typed_array,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("containsAll", "(Ljava/util/Collection;)Z", Self::contains_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("addAll", "(Ljava/util/Collection;)Z", Self::add_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("removeAll", "(Ljava/util/Collection;)Z", Self::remove_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("retainAll", "(Ljava/util/Collection;)Z", Self::retain_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
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

    async fn to_typed_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        destination: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Array<Object>>> {
        tracing::debug!("java.util.AbstractCollection::toArray({this:?}, {destination:?})");

        if destination.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }

        let snapshot: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&this, "toArray", "()[Ljava/lang/Object;", ()).await?;
        let size = jvm.array_length(&snapshot).await?;
        let elements: alloc::vec::Vec<ClassInstanceRef<Object>> = if size == 0 {
            alloc::vec::Vec::new()
        } else {
            jvm.load_array(&snapshot, 0, size).await?
        };
        let destination_length = jvm.array_length(&destination).await?;
        let mut result = if destination_length < size {
            let class_name = destination.class_definition().name();
            let component_descriptor = class_name.strip_prefix('[').unwrap();
            ClassInstanceRef::from(jvm.instantiate_array(component_descriptor, size).await?)
        } else {
            destination
        };

        for (index, element) in elements.into_iter().enumerate() {
            if !element.is_null() && !jvm.array_store_allowed(result.as_class_instance(), element.as_class_instance()) {
                return Err(jvm.exception("java/lang/ArrayStoreException", &element.class_definition().name()).await);
            }
            jvm.store_array(&mut result, index, core::iter::once(element)).await?;
        }
        if destination_length > size {
            let null: ClassInstanceRef<Object> = None.into();
            jvm.store_array(&mut result, size, core::iter::once(null)).await?;
        }

        Ok(result)
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

    async fn contains_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::containsAll({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&collection, "toArray", "()[Ljava/lang/Object;", ()).await?;
        let size = jvm.array_length(&elements).await?;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, size).await? {
            let contains: bool = jvm.invoke_virtual(&this, "contains", "(Ljava/lang/Object;)Z", (element,)).await?;
            if !contains {
                return Ok(false);
            }
        }

        Ok(true)
    }

    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::addAll({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&collection, "toArray", "()[Ljava/lang/Object;", ()).await?;
        let size = jvm.array_length(&elements).await?;
        let mut modified = false;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, size).await? {
            modified |= jvm.invoke_virtual::<_, bool>(&this, "add", "(Ljava/lang/Object;)Z", (element,)).await?;
        }

        Ok(modified)
    }

    async fn remove_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::removeAll({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&this, "toArray", "()[Ljava/lang/Object;", ()).await?;
        let size = jvm.array_length(&elements).await?;
        let same_collection = this.identity() == collection.identity();
        let mut modified = false;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, size).await? {
            let remove = if same_collection {
                true
            } else {
                jvm.invoke_virtual(&collection, "contains", "(Ljava/lang/Object;)Z", (element.clone(),))
                    .await?
            };
            if remove {
                while jvm
                    .invoke_virtual::<_, bool>(&this, "remove", "(Ljava/lang/Object;)Z", (element.clone(),))
                    .await?
                {
                    modified = true;
                }
            }
        }

        Ok(modified)
    }

    async fn retain_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::retainAll({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        if this.identity() == collection.identity() {
            return Ok(false);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&this, "toArray", "()[Ljava/lang/Object;", ()).await?;
        let size = jvm.array_length(&elements).await?;
        let mut modified = false;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, size).await? {
            let retain: bool = jvm
                .invoke_virtual(&collection, "contains", "(Ljava/lang/Object;)Z", (element.clone(),))
                .await?;
            if !retain {
                while jvm
                    .invoke_virtual::<_, bool>(&this, "remove", "(Ljava/lang/Object;)Z", (element.clone(),))
                    .await?
                {
                    modified = true;
                }
            }
        }

        Ok(modified)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractCollection::clear({this:?})");

        Err(jvm.exception("java/lang/UnsupportedOperationException", "AbstractCollection.clear").await)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let buffer: ClassInstanceRef<Object> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let open = JavaLangString::from_rust_string(jvm, "[").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (open,))
            .await?;
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "iterator", "()Ljava/util/Iterator;", ()).await?;
        let mut first = true;
        while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            if first {
                first = false;
            } else {
                let separator = JavaLangString::from_rust_string(jvm, ", ").await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (separator,))
                    .await?;
            }
            let element: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            if !element.is_null() && element.identity() == this.identity() {
                let recursive = JavaLangString::from_rust_string(jvm, "(this Collection)").await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (recursive,))
                    .await?;
            } else {
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&buffer, "append", "(Ljava/lang/Object;)Ljava/lang/StringBuffer;", (element,))
                    .await?;
            }
        }
        let close = JavaLangString::from_rust_string(jvm, "]").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (close,))
            .await?;
        jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await
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
