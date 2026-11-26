use alloc::vec;

use jvm::{Array, AsClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::Iterator},
};

// abstract class java.util.AbstractCollection
#[java_class(name = "java/util/AbstractCollection", implements("java/util/Collection"), public, abstract)]
pub struct AbstractCollection;

#[java_methods]
impl AbstractCollection {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractCollection::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::isEmpty({this:?})");

        let size: i32 = jvm.invoke_virtual(&this, "java/util/AbstractCollection", "size", "()I", ()).await?;

        Ok(size == 0)
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::contains({this:?}, {element:?})");

        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "java/util/AbstractCollection", "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        loop {
            let has_next: bool = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
                .await?;
            if !has_next {
                return Ok(false);
            }

            let current: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            if Self::object_equals(jvm, &element, &current).await? {
                return Ok(true);
            }
        }
    }

    #[method(public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Iterator>>;

    #[method(public)]
    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        tracing::debug!("java.util.AbstractCollection::toArray({this:?})");

        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "java/util/AbstractCollection", "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        let mut elements = vec![];
        loop {
            let has_next: bool = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
                .await?;
            if !has_next {
                break;
            }

            let current: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            elements.push(current);
        }

        let mut array: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", elements.len()).await?.into();
        if !elements.is_empty() {
            jvm.store_array(&mut array, 0, elements).await?;
        }

        Ok(array)
    }

    #[method(name = "toArray", public)]
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

        let snapshot: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&this, "java/util/AbstractCollection", "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let size = jvm.array_length(&snapshot).await?;
        let elements: alloc::vec::Vec<ClassInstanceRef<Object>> = if size == 0 {
            alloc::vec::Vec::new()
        } else {
            jvm.load_array(&snapshot, 0, size).await?
        };
        let destination_length = jvm.array_length(&destination).await?;
        let mut result = if destination_length < size {
            let class_definition = destination.class_definition();
            let class_name = class_definition.name();
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

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::add({this:?}, {element:?})");

        Err(jvm.exception("java/lang/UnsupportedOperationException", "AbstractCollection.add").await)
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::remove({this:?}, {element:?})");

        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "AbstractCollection.remove")
            .await)
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public)]
    async fn contains_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::containsAll({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&collection, &collection.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let size = jvm.array_length(&elements).await?;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, size).await? {
            let contains: bool = jvm
                .invoke_virtual(&this, "java/util/AbstractCollection", "contains", "(Ljava/lang/Object;)Z", (element,))
                .await?;
            if !contains {
                return Ok(false);
            }
        }

        Ok(true)
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public)]
    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::addAll({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&collection, &collection.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let size = jvm.array_length(&elements).await?;
        let mut modified = false;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, size).await? {
            modified |= jvm
                .invoke_virtual::<_, bool>(&this, "java/util/AbstractCollection", "add", "(Ljava/lang/Object;)Z", (element,))
                .await?;
        }

        Ok(modified)
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public)]
    async fn remove_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::removeAll({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&this, "java/util/AbstractCollection", "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let size = jvm.array_length(&elements).await?;
        let same_collection = this.identity() == collection.identity();
        let mut modified = false;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, size).await? {
            let remove = if same_collection {
                true
            } else {
                jvm.invoke_virtual(
                    &collection,
                    &collection.class_definition().name(),
                    "contains",
                    "(Ljava/lang/Object;)Z",
                    (element.clone(),),
                )
                .await?
            };
            if remove {
                while jvm
                    .invoke_virtual::<_, bool>(
                        &this,
                        "java/util/AbstractCollection",
                        "remove",
                        "(Ljava/lang/Object;)Z",
                        (element.clone(),),
                    )
                    .await?
                {
                    modified = true;
                }
            }
        }

        Ok(modified)
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public)]
    async fn retain_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.AbstractCollection::retainAll({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        if this.identity() == collection.identity() {
            return Ok(false);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&this, "java/util/AbstractCollection", "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let size = jvm.array_length(&elements).await?;
        let mut modified = false;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, size).await? {
            let retain: bool = jvm
                .invoke_virtual(
                    &collection,
                    &collection.class_definition().name(),
                    "contains",
                    "(Ljava/lang/Object;)Z",
                    (element.clone(),),
                )
                .await?;
            if !retain {
                while jvm
                    .invoke_virtual::<_, bool>(
                        &this,
                        "java/util/AbstractCollection",
                        "remove",
                        "(Ljava/lang/Object;)Z",
                        (element.clone(),),
                    )
                    .await?
                {
                    modified = true;
                }
            }
        }

        Ok(modified)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractCollection::clear({this:?})");

        Err(jvm.exception("java/lang/UnsupportedOperationException", "AbstractCollection.clear").await)
    }

    #[method(descriptor = "()Ljava/lang/String;", public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let buffer: ClassInstanceRef<Object> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let open = JavaLangString::from_rust_string(jvm, "[").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "append",
                "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                (open,),
            )
            .await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "java/util/AbstractCollection", "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        let mut first = true;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            if first {
                first = false;
            } else {
                let separator = JavaLangString::from_rust_string(jvm, ", ").await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(
                        &buffer,
                        "java/lang/StringBuffer",
                        "append",
                        "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                        (separator,),
                    )
                    .await?;
            }
            let element: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            if !element.is_null() && element.identity() == this.identity() {
                let recursive = JavaLangString::from_rust_string(jvm, "(this Collection)").await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(
                        &buffer,
                        "java/lang/StringBuffer",
                        "append",
                        "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                        (recursive,),
                    )
                    .await?;
            } else {
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(
                        &buffer,
                        "java/lang/StringBuffer",
                        "append",
                        "(Ljava/lang/Object;)Ljava/lang/StringBuffer;",
                        (element,),
                    )
                    .await?;
            }
        }
        let close = JavaLangString::from_rust_string(jvm, "]").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "append",
                "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                (close,),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }

    async fn object_equals(jvm: &Jvm, left: &ClassInstanceRef<Object>, right: &ClassInstanceRef<Object>) -> Result<bool> {
        if left.is_null() {
            return Ok(right.is_null());
        }

        if right.is_null() {
            return Ok(false);
        }

        jvm.invoke_virtual(left, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (right.clone(),))
            .await
    }
}
