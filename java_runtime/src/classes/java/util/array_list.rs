use alloc::{format, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.ArrayList
pub struct ArrayList;

impl ArrayList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/ArrayList",
            parent_class: Some("java/util/AbstractList"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_capacity, Default::default()),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, Default::default()),
                JavaMethodProto::new("add", "(ILjava/lang/Object;)V", Self::add_at, Default::default()),
                JavaMethodProto::new("get", "(I)Ljava/lang/Object;", Self::get, Default::default()),
                JavaMethodProto::new("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Self::set, Default::default()),
                JavaMethodProto::new("remove", "(I)Ljava/lang/Object;", Self::remove, Default::default()),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove_object, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, Default::default()),
                JavaMethodProto::new("indexOf", "(Ljava/lang/Object;)I", Self::index_of, Default::default()),
                JavaMethodProto::new("clear", "()V", Self::clear, Default::default()),
                JavaMethodProto::new("toArray", "()[Ljava/lang/Object;", Self::to_array, Default::default()),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("elementData", "[Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("size", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.ArrayList::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/ArrayList", "<init>", "(I)V", (10,)).await?;

        Ok(())
    }

    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.util.ArrayList::<init>({this:?}, {capacity:?})");

        if capacity < 0 {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", &format!("Illegal Capacity: {capacity}"))
                .await);
        }

        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;

        let element_data = jvm.instantiate_array("Ljava/lang/Object;", capacity as usize).await?;
        jvm.put_field(&mut this, "elementData", "[Ljava/lang/Object;", element_data).await?;
        jvm.put_field(&mut this, "size", "I", 0).await?;

        Ok(())
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.ArrayList::add({this:?}, {element:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (size + 1) as usize).await?;

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        jvm.store_array(&mut element_data, size as usize, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "size", "I", size + 1).await?;

        Ok(true)
    }

    async fn add_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<()> {
        tracing::debug!("java.util.ArrayList::add({this:?}, {index:?}, {element:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index > size {
            return Err(Self::index_out_of_bounds(jvm, index, size).await);
        }

        Self::ensure_capacity(jvm, &mut this, (size + 1) as usize).await?;

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        let num_to_move = size - index;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, index as usize, num_to_move as usize).await?;
            jvm.store_array(&mut element_data, (index + 1) as usize, to_shift).await?;
        }

        jvm.store_array(&mut element_data, index as usize, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "size", "I", size + 1).await?;

        Ok(())
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.ArrayList::get({this:?}, {index:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index >= size {
            return Err(Self::index_out_of_bounds(jvm, index, size).await);
        }

        let element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        let element: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as usize, 1).await?.into_iter().next().unwrap();

        Ok(element)
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.ArrayList::set({this:?}, {index:?}, {element:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index >= size {
            return Err(Self::index_out_of_bounds(jvm, index, size).await);
        }

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        let old_element: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as usize, 1).await?.into_iter().next().unwrap();
        jvm.store_array(&mut element_data, index as usize, core::iter::once(element)).await?;

        Ok(old_element)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.ArrayList::remove({this:?}, {index:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index >= size {
            return Err(Self::index_out_of_bounds(jvm, index, size).await);
        }

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        let removed: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as usize, 1).await?.into_iter().next().unwrap();

        let num_to_move = size - index - 1;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, (index + 1) as usize, num_to_move as usize).await?;
            jvm.store_array(&mut element_data, index as usize, to_shift).await?;
        }

        let null_ref: ClassInstanceRef<Object> = None.into();
        jvm.store_array(&mut element_data, (size - 1) as usize, core::iter::once(null_ref))
            .await?;
        jvm.put_field(&mut this, "size", "I", size - 1).await?;

        Ok(removed)
    }

    async fn remove_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.ArrayList::remove({this:?}, {element:?})");

        let index: i32 = jvm.invoke_virtual(&this, "indexOf", "(Ljava/lang/Object;)I", (element,)).await?;
        if index < 0 {
            return Ok(false);
        }

        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "remove", "(I)Ljava/lang/Object;", (index,)).await?;

        Ok(true)
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.ArrayList::size({this:?})");

        jvm.get_field(&this, "size", "I").await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.ArrayList::isEmpty({this:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;

        Ok(size == 0)
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.ArrayList::contains({this:?}, {element:?})");

        let index: i32 = jvm.invoke_virtual(&this, "indexOf", "(Ljava/lang/Object;)I", (element,)).await?;

        Ok(index >= 0)
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.ArrayList::indexOf({this:?}, {element:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        let element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;

        for index in 0..size {
            let item: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as usize, 1).await?.into_iter().next().unwrap();
            if Self::object_equals(jvm, &element, &item).await? {
                return Ok(index);
            }
        }

        Ok(-1)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.ArrayList::clear({this:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if size > 0 {
            let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
            let nulls: Vec<ClassInstanceRef<Object>> = (0..size).map(|_| None.into()).collect();
            jvm.store_array(&mut element_data, 0, nulls).await?;
        }

        jvm.put_field(&mut this, "size", "I", 0).await?;

        Ok(())
    }

    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        tracing::debug!("java.util.ArrayList::toArray({this:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        let element_data: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;

        Self::copy_to_array(jvm, &element_data, size).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.ArrayList::iterator({this:?})");

        let snapshot: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&this, "toArray", "()[Ljava/lang/Object;", ()).await?;
        let iterator = jvm.new_class("java/util/ArrayList$Itr", "([Ljava/lang/Object;)V", (snapshot,)).await?;

        Ok(iterator.into())
    }

    async fn ensure_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, min_capacity: usize) -> Result<()> {
        let element_data = jvm.get_field(this, "elementData", "[Ljava/lang/Object;").await?;
        let current_capacity = jvm.array_length(&element_data).await?;

        if min_capacity <= current_capacity {
            return Ok(());
        }

        let new_capacity = if current_capacity == 0 { 1 } else { current_capacity * 2 }.max(min_capacity);
        let size: i32 = jvm.get_field(this, "size", "I").await?;
        let old_elements: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, 0, size as usize).await?;

        let mut new_element_data = jvm.instantiate_array("Ljava/lang/Object;", new_capacity).await?;
        if !old_elements.is_empty() {
            jvm.store_array(&mut new_element_data, 0, old_elements).await?;
        }
        jvm.put_field(this, "elementData", "[Ljava/lang/Object;", new_element_data).await?;

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

        jvm.invoke_virtual(left, "equals", "(Ljava/lang/Object;)Z", (right.clone(),)).await
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
