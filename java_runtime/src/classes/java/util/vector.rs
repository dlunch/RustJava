use alloc::{boxed::Box, format, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstance, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Vector
pub struct Vector;

impl Vector {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Vector",
            parent_class: Some("java/util/AbstractList"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_capacity, Default::default()),
                JavaMethodProto::new("<init>", "(II)V", Self::init_with_capacity_increment, Default::default()),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, Default::default()),
                JavaMethodProto::new("addElement", "(Ljava/lang/Object;)V", Self::add_element, Default::default()),
                JavaMethodProto::new("insertElementAt", "(Ljava/lang/Object;I)V", Self::insert_element_at, Default::default()),
                JavaMethodProto::new("elementAt", "(I)Ljava/lang/Object;", Self::element_at, Default::default()),
                JavaMethodProto::new("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Self::set, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("remove", "(I)Ljava/lang/Object;", Self::remove, Default::default()),
                JavaMethodProto::new("removeAllElements", "()V", Self::remove_all_elements, Default::default()),
                JavaMethodProto::new("removeElementAt", "(I)V", Self::remove_element_at, Default::default()),
                JavaMethodProto::new("indexOf", "(Ljava/lang/Object;)I", Self::index_of, Default::default()),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/Object;)I", Self::last_index_of, Default::default()),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/Object;I)I", Self::last_index_of_index, Default::default()),
                JavaMethodProto::new("firstElement", "()Ljava/lang/Object;", Self::first_element, Default::default()),
                JavaMethodProto::new("removeElement", "(Ljava/lang/Object;)Z", Self::remove_element, Default::default()),
                JavaMethodProto::new("trimToSize", "()V", Self::trim_to_size, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("elementData", "[Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("elementCount", "I", Default::default()),
                JavaFieldProto::new("capacityIncrement", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/util/Vector", "<init>", "(I)V", (10,)).await?;

        Ok(())
    }

    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({:?}, {:?})", &this, capacity);

        let _: () = jvm.invoke_special(&this, "java/util/Vector", "<init>", "(II)V", (capacity, 0)).await?;

        Ok(())
    }

    async fn init_with_capacity_increment(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        capacity: i32,
        capacity_increment: i32,
    ) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({:?}, {:?}, {:?})", &this, capacity, capacity_increment);

        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;

        let element_data = jvm.instantiate_array("Ljava/lang/Object;", capacity as _).await?;
        jvm.put_field(&mut this, "elementData", "[Ljava/lang/Object;", element_data).await?;
        jvm.put_field(&mut this, "elementCount", "I", 0).await?;
        jvm.put_field(&mut this, "capacityIncrement", "I", capacity_increment).await?;

        Ok(())
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::add({:?}, {:?})", &this, &element);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (element_count + 1) as _).await?;

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        jvm.store_array(&mut element_data, element_count as _, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "elementCount", "I", element_count + 1).await?;

        Ok(true)
    }

    async fn add_element(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.util.Vector::addElement({:?}, {:?})", &this, &element);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (element_count + 1) as _).await?;

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        jvm.store_array(&mut element_data, element_count as _, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "elementCount", "I", element_count + 1).await?;

        Ok(())
    }

    async fn insert_element_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        index: i32,
    ) -> Result<()> {
        tracing::debug!("java.util.Vector::insertElementAt({:?}, {:?}, {:?})", &this, &element, index);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (element_count + 1) as _).await?;

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;

        let num_to_move = element_count - index;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, index as _, num_to_move as _).await?;
            jvm.store_array(&mut element_data, (index + 1) as _, to_shift).await?;
        }

        jvm.store_array(&mut element_data, index as _, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "elementCount", "I", element_count + 1).await?;

        Ok(())
    }

    async fn element_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::elementAt({:?}, {:?})", &this, index);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        if index < 0 || index >= element_count {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {element_count}"))
                .await);
        }

        let element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        let element: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as _, 1).await?.into_iter().next().unwrap();

        Ok(element)
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::set({:?}, {:?}, {:?})", &this, index, &element);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        if index < 0 || index >= element_count {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {element_count}"))
                .await);
        }

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        let old_element: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as _, 1).await?.into_iter().next().unwrap();
        jvm.store_array(&mut element_data, index as _, core::iter::once(element)).await?;

        Ok(old_element)
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Vector::size({:?})", &this);

        jvm.get_field(&this, "elementCount", "I").await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Vector::isEmpty({:?})", &this);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;

        Ok(element_count == 0)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::remove({:?}, {:?})", &this, index);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        if index < 0 || index >= element_count {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {element_count}"))
                .await);
        }

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        let removed: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as _, 1).await?.into_iter().next().unwrap();

        let num_to_move = element_count - index - 1;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, (index + 1) as _, num_to_move as _).await?;
            jvm.store_array(&mut element_data, index as _, to_shift).await?;
        }

        let null_ref: ClassInstanceRef<Object> = None.into();
        jvm.store_array(&mut element_data, (element_count - 1) as _, core::iter::once(null_ref)).await?;
        jvm.put_field(&mut this, "elementCount", "I", element_count - 1).await?;

        Ok(removed)
    }

    async fn remove_all_elements(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::removeAllElements({:?})", &this);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;

        let nulls: Vec<ClassInstanceRef<Object>> = (0..element_count).map(|_| None.into()).collect();
        if !nulls.is_empty() {
            jvm.store_array(&mut element_data, 0, nulls).await?;
        }

        jvm.put_field(&mut this, "elementCount", "I", 0).await?;

        Ok(())
    }

    async fn remove_element_at(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        tracing::debug!("java.util.Vector::removeElementAt({:?}, {:?})", &this, index);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        if index < 0 || index >= element_count {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {element_count}"))
                .await);
        }

        let mut element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;

        let num_to_move = element_count - index - 1;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, (index + 1) as _, num_to_move as _).await?;
            jvm.store_array(&mut element_data, index as _, to_shift).await?;
        }

        let null_ref: ClassInstanceRef<Object> = None.into();
        jvm.store_array(&mut element_data, (element_count - 1) as _, core::iter::once(null_ref)).await?;
        jvm.put_field(&mut this, "elementCount", "I", element_count - 1).await?;

        Ok(())
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.Vector::indexOf({:?}, {:?})", &this, &element);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        let element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;

        for i in 0..element_count {
            let item: ClassInstanceRef<Object> = jvm.load_array(&element_data, i as _, 1).await?.into_iter().next().unwrap();

            if item.is_null() && element.is_null() {
                return Ok(i);
            }

            if item.is_null() || element.is_null() {
                continue;
            }

            let item_instance: Box<dyn ClassInstance> = item.into();
            let element_instance: Box<dyn ClassInstance> = element.clone().into();
            if item_instance.equals(&*element_instance)? {
                return Ok(i);
            }
        }

        Ok(-1)
    }

    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.Vector::lastIndexOf({:?}, {:?})", &this, &element);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;

        let index: i32 = jvm
            .invoke_virtual(&this, "lastIndexOf", "(Ljava/lang/Object;I)I", (element, element_count - 1))
            .await?;

        Ok(index)
    }

    async fn last_index_of_index(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        index: i32,
    ) -> Result<i32> {
        tracing::debug!("java.util.Vector::lastIndexOf({:?}, {:?}, {:?})", &this, &element, index);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;

        if index >= element_count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", &format!("{index} >= {element_count}")).await);
        }

        let element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;

        for i in (0..=index).rev() {
            let item: ClassInstanceRef<Object> = jvm.load_array(&element_data, i as _, 1).await?.into_iter().next().unwrap();

            if item.is_null() && element.is_null() {
                return Ok(i);
            }

            if item.is_null() || element.is_null() {
                continue;
            }

            let item_instance: Box<dyn ClassInstance> = item.into();
            let element_instance: Box<dyn ClassInstance> = element.clone().into();
            if item_instance.equals(&*element_instance)? {
                return Ok(i);
            }
        }

        Ok(-1)
    }

    async fn first_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::firstElement({:?})", &this);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;

        if element_count == 0 {
            return Ok(None.into());
        }

        let element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        let element: ClassInstanceRef<Object> = jvm.load_array(&element_data, 0, 1).await?.into_iter().next().unwrap();

        Ok(element)
    }

    async fn remove_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::removeElement({:?}, {:?})", &this, &element);

        let index: i32 = jvm.invoke_virtual(&this, "indexOf", "(Ljava/lang/Object;)I", (element,)).await?;

        if index >= 0 {
            let _: () = jvm.invoke_virtual(&this, "removeElementAt", "(I)V", (index,)).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn trim_to_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::trimToSize({:?})", &this);

        let element_count: i32 = jvm.get_field(&this, "elementCount", "I").await?;
        let element_data = jvm.get_field(&this, "elementData", "[Ljava/lang/Object;").await?;
        let current_capacity = jvm.array_length(&element_data).await?;

        if (element_count as usize) < current_capacity {
            let elements: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, 0, element_count as _).await?;
            let mut new_element_data = jvm.instantiate_array("Ljava/lang/Object;", element_count as _).await?;
            jvm.store_array(&mut new_element_data, 0, elements).await?;
            jvm.put_field(&mut this, "elementData", "[Ljava/lang/Object;", new_element_data).await?;
        }

        Ok(())
    }

    async fn ensure_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, min_capacity: usize) -> Result<()> {
        let element_data = jvm.get_field(this, "elementData", "[Ljava/lang/Object;").await?;
        let current_capacity = jvm.array_length(&element_data).await?;

        if min_capacity > current_capacity {
            let capacity_increment: i32 = jvm.get_field(this, "capacityIncrement", "I").await?;
            let new_capacity = if capacity_increment > 0 {
                current_capacity + capacity_increment as usize
            } else {
                current_capacity * 2
            };
            let new_capacity = new_capacity.max(min_capacity);

            let element_count: i32 = jvm.get_field(this, "elementCount", "I").await?;
            let old_elements: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, 0, element_count as _).await?;

            let mut new_element_data = jvm.instantiate_array("Ljava/lang/Object;", new_capacity).await?;
            jvm.store_array(&mut new_element_data, 0, old_elements).await?;
            jvm.put_field(this, "elementData", "[Ljava/lang/Object;", new_element_data).await?;
        }

        Ok(())
    }
}
