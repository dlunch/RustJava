use alloc::{boxed::Box, format, vec::Vec};

use jvm::{Array, ClassInstance, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.util.ArrayList
#[java_class(
    name = "java/util/ArrayList",
    extends = "java/util/AbstractList",
    implements("java/lang/Cloneable", "java/io/Serializable"),
    public
)]
pub struct ArrayList {
    #[field(transient)]
    elementData: ClassInstanceRef<Array<Object>>,
    #[field(private)]
    size: i32,
}

#[java_methods]
impl ArrayList {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.ArrayList::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/ArrayList", "<init>", "(I)V", (10,)).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.util.ArrayList::<init>({this:?}, {capacity:?})");

        if capacity < 0 {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", &format!("Illegal Capacity: {capacity}"))
                .await);
        }

        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;

        let element_data = jvm.instantiate_array("Ljava/lang/Object;", capacity as usize).await?;
        jvm.put_field(&mut this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;", element_data)
            .await?;
        jvm.put_field(&mut this, "java/util/ArrayList", "size", "I", 0).await?;

        Ok(())
    }

    #[method(constructor, descriptor = "(Ljava/util/Collection;)V", public)]
    async fn init_from_collection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        collection: ClassInstanceRef<Object>,
    ) -> Result<()> {
        tracing::debug!("java.util.ArrayList::<init>({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        let size: i32 = jvm
            .invoke_virtual(&collection, &collection.class_definition().name(), "size", "()I", ())
            .await?;
        let _: () = jvm.invoke_special(&this, "java/util/ArrayList", "<init>", "(I)V", (size,)).await?;
        let _: bool = jvm
            .invoke_virtual(&this, "java/util/ArrayList", "addAll", "(Ljava/util/Collection;)Z", (collection,))
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.ArrayList::add({this:?}, {element:?})");

        let size: i32 = jvm.get_field(&this, "java/util/ArrayList", "size", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (size + 1) as usize).await?;

        let mut element_data = jvm.get_field(&this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;").await?;
        jvm.store_array(&mut element_data, size as usize, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "java/util/ArrayList", "size", "I", size + 1).await?;

        Ok(true)
    }

    #[method(name = "add", public)]
    async fn add_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<()> {
        tracing::debug!("java.util.ArrayList::add({this:?}, {index:?}, {element:?})");

        let size: i32 = jvm.get_field(&this, "java/util/ArrayList", "size", "I").await?;
        if index < 0 || index > size {
            return Err(Self::index_out_of_bounds(jvm, index, size).await);
        }

        Self::ensure_capacity(jvm, &mut this, (size + 1) as usize).await?;

        let mut element_data = jvm.get_field(&this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;").await?;
        let num_to_move = size - index;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, index as usize, num_to_move as usize).await?;
            jvm.store_array(&mut element_data, (index + 1) as usize, to_shift).await?;
        }

        jvm.store_array(&mut element_data, index as usize, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "java/util/ArrayList", "size", "I", size + 1).await?;

        Ok(())
    }

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.ArrayList::get({this:?}, {index:?})");

        let size: i32 = jvm.get_field(&this, "java/util/ArrayList", "size", "I").await?;
        if index < 0 || index >= size {
            return Err(Self::index_out_of_bounds(jvm, index, size).await);
        }

        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;").await?;
        let element: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as usize, 1).await?.into_iter().next().unwrap();

        Ok(element)
    }

    #[method(public)]
    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.ArrayList::set({this:?}, {index:?}, {element:?})");

        let size: i32 = jvm.get_field(&this, "java/util/ArrayList", "size", "I").await?;
        if index < 0 || index >= size {
            return Err(Self::index_out_of_bounds(jvm, index, size).await);
        }

        let mut element_data = jvm.get_field(&this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;").await?;
        let old_element: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as usize, 1).await?.into_iter().next().unwrap();
        jvm.store_array(&mut element_data, index as usize, core::iter::once(element)).await?;

        Ok(old_element)
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.ArrayList::remove({this:?}, {index:?})");

        let size: i32 = jvm.get_field(&this, "java/util/ArrayList", "size", "I").await?;
        if index < 0 || index >= size {
            return Err(Self::index_out_of_bounds(jvm, index, size).await);
        }

        let mut element_data = jvm.get_field(&this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;").await?;
        let removed: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as usize, 1).await?.into_iter().next().unwrap();

        let num_to_move = size - index - 1;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, (index + 1) as usize, num_to_move as usize).await?;
            jvm.store_array(&mut element_data, index as usize, to_shift).await?;
        }

        let null_ref: ClassInstanceRef<Object> = None.into();
        jvm.store_array(&mut element_data, (size - 1) as usize, core::iter::once(null_ref))
            .await?;
        jvm.put_field(&mut this, "java/util/ArrayList", "size", "I", size - 1).await?;

        Ok(removed)
    }

    #[method(name = "remove", public)]
    async fn remove_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.ArrayList::remove({this:?}, {element:?})");

        let index: i32 = jvm
            .invoke_virtual(&this, "java/util/ArrayList", "indexOf", "(Ljava/lang/Object;)I", (element,))
            .await?;
        if index < 0 {
            return Ok(false);
        }

        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "java/util/ArrayList", "remove", "(I)Ljava/lang/Object;", (index,))
            .await?;

        Ok(true)
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.ArrayList::size({this:?})");

        jvm.get_field(&this, "java/util/ArrayList", "size", "I").await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.ArrayList::isEmpty({this:?})");

        let size: i32 = jvm.get_field(&this, "java/util/ArrayList", "size", "I").await?;

        Ok(size == 0)
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.ArrayList::contains({this:?}, {element:?})");

        let index: i32 = jvm
            .invoke_virtual(&this, "java/util/ArrayList", "indexOf", "(Ljava/lang/Object;)I", (element,))
            .await?;

        Ok(index >= 0)
    }

    #[method(public)]
    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.ArrayList::indexOf({this:?}, {element:?})");

        let size: i32 = jvm.get_field(&this, "java/util/ArrayList", "size", "I").await?;
        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;").await?;

        for index in 0..size {
            let item: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as usize, 1).await?.into_iter().next().unwrap();
            if Self::object_equals(jvm, &element, &item).await? {
                return Ok(index);
            }
        }

        Ok(-1)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.ArrayList::clear({this:?})");

        let size: i32 = jvm.get_field(&this, "java/util/ArrayList", "size", "I").await?;
        if size > 0 {
            let mut element_data = jvm.get_field(&this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;").await?;
            let nulls: Vec<ClassInstanceRef<Object>> = (0..size).map(|_| None.into()).collect();
            jvm.store_array(&mut element_data, 0, nulls).await?;
        }

        jvm.put_field(&mut this, "java/util/ArrayList", "size", "I", 0).await?;

        Ok(())
    }

    #[method(public)]
    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        tracing::debug!("java.util.ArrayList::toArray({this:?})");

        let size: i32 = jvm.get_field(&this, "java/util/ArrayList", "size", "I").await?;
        let element_data: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;").await?;

        Self::copy_to_array(jvm, &element_data, size).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.ArrayList::iterator({this:?})");

        let iterator = jvm.new_class("java/util/ArrayList$Itr", "(Ljava/util/List;I)V", (this, 0)).await?;

        Ok(iterator.into())
    }

    #[method(descriptor = "()Ljava/util/ListIterator;", public)]
    async fn list_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm.new_class("java/util/ArrayList$ListItr", "(Ljava/util/List;I)V", (this, 0)).await?;
        Ok(iterator.into())
    }

    #[method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", public)]
    async fn list_iterator_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm
            .new_class("java/util/ArrayList$ListItr", "(Ljava/util/List;I)V", (this, index))
            .await?;
        Ok(iterator.into())
    }

    async fn ensure_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, min_capacity: usize) -> Result<()> {
        let element_data: Box<dyn ClassInstance> = jvm.get_field(this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;").await?;
        let current_capacity = jvm.array_length(&element_data).await?;

        if min_capacity <= current_capacity {
            return Ok(());
        }

        let new_capacity = if current_capacity == 0 { 1 } else { current_capacity * 2 }.max(min_capacity);
        let size: i32 = jvm.get_field(this, "java/util/ArrayList", "size", "I").await?;
        let old_elements: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, 0, size as usize).await?;

        let mut new_element_data = jvm.instantiate_array("Ljava/lang/Object;", new_capacity).await?;
        if !old_elements.is_empty() {
            jvm.store_array(&mut new_element_data, 0, old_elements).await?;
        }
        jvm.put_field(this, "java/util/ArrayList", "elementData", "[Ljava/lang/Object;", new_element_data)
            .await?;

        Ok(())
    }

    async fn index_out_of_bounds(jvm: &Jvm, index: i32, size: i32) -> jvm::JavaError {
        jvm.exception("java/lang/IndexOutOfBoundsException", &format!("Index: {index}, Size: {size}"))
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

    async fn copy_to_array(jvm: &Jvm, source: &ClassInstanceRef<Array<Object>>, len: i32) -> Result<ClassInstanceRef<Array<Object>>> {
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "source").await);
        }

        if len < 0 {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "negative length").await);
        }

        let len = len as usize;
        let elements: Vec<ClassInstanceRef<Object>> = if len == 0 { Vec::new() } else { jvm.load_array(source, 0, len).await? };
        let mut copy: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", len).await?.into();
        if !elements.is_empty() {
            jvm.store_array(&mut copy, 0, elements).await?;
        }

        Ok(copy)
    }
}
