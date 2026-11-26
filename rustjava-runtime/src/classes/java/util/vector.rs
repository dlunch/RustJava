use alloc::{
    boxed::Box,
    format,
    string::{String as RustString, ToString},
    vec::Vec,
};

use jvm::{Array, ClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String},
};

// class java.util.Vector
#[java_class(
    name = "java/util/Vector",
    extends = "java/util/AbstractList",
    implements("java/util/List", "java/lang/Cloneable", "java/io/Serializable"),
    public
)]
pub struct Vector {
    #[field(protected)]
    elementData: ClassInstanceRef<Array<Object>>,
    #[field(protected)]
    elementCount: i32,
    #[field(protected)]
    capacityIncrement: i32,
}

#[java_methods]
impl Vector {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/Vector", "<init>", "(I)V", (10,)).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({this:?}, {capacity:?})");

        let _: () = jvm.invoke_special(&this, "java/util/Vector", "<init>", "(II)V", (capacity, 0)).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_capacity_increment(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        capacity: i32,
        capacity_increment: i32,
    ) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({this:?}, {capacity:?}, {capacity_increment:?})");

        if capacity < 0 {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", &format!("Illegal Capacity: {capacity}"))
                .await);
        }

        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;

        let element_data = jvm.instantiate_array("Ljava/lang/Object;", capacity as _).await?;
        jvm.put_field(&mut this, "java/util/Vector", "elementData", "[Ljava/lang/Object;", element_data)
            .await?;
        jvm.put_field(&mut this, "java/util/Vector", "elementCount", "I", 0).await?;
        jvm.put_field(&mut this, "java/util/Vector", "capacityIncrement", "I", capacity_increment)
            .await?;

        Ok(())
    }

    #[method(constructor, descriptor = "(Ljava/util/Collection;)V", public)]
    async fn init_from_collection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        collection: ClassInstanceRef<Object>,
    ) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        let size: i32 = jvm
            .invoke_virtual(&collection, &collection.class_definition().name(), "size", "()I", ())
            .await?;
        let _: () = jvm.invoke_special(&this, "java/util/Vector", "<init>", "(I)V", (size,)).await?;
        let _: bool = jvm
            .invoke_virtual(&this, "java/util/Vector", "addAll", "(Ljava/util/Collection;)Z", (collection,))
            .await?;

        Ok(())
    }

    #[method(public, synchronized)]
    async fn capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        Ok(jvm.array_length(&element_data).await? as i32)
    }

    #[method(public, synchronized)]
    async fn copy_into(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut destination: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        tracing::debug!("java.util.Vector::copyInto({this:?}, {destination:?})");

        if destination.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "destination").await);
        }

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        let elements: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, 0, element_count as usize).await?;
        jvm.store_array(&mut destination, 0, elements).await
    }

    #[method(descriptor = "()Ljava/util/Enumeration;", public)]
    async fn elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::elements({this:?})");

        let snapshot: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&this, "java/util/Vector", "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        Ok(jvm
            .new_class("java/util/Hashtable$Enumerator", "([Ljava/lang/Object;)V", (snapshot,))
            .await?
            .into())
    }

    #[method(name = "ensureCapacity", public, synchronized)]
    async fn ensure_capacity_api(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, min_capacity: i32) -> Result<()> {
        tracing::debug!("java.util.Vector::ensureCapacity({this:?}, {min_capacity:?})");

        if min_capacity > 0 {
            Self::ensure_capacity(jvm, &mut this, min_capacity as usize).await?;
        }

        Ok(())
    }

    #[method(public, synchronized)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::add({this:?}, {element:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        let mut element_data = Self::ensure_capacity(jvm, &mut this, (element_count + 1) as _).await?;
        jvm.store_array(&mut element_data, element_count as _, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "java/util/Vector", "elementCount", "I", element_count + 1)
            .await?;

        Ok(true)
    }

    #[method(name = "add", public, synchronized)]
    async fn add_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32, element: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.util.Vector::add({this:?}, {index:?}, {element:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        if index < 0 || index > element_count {
            return Err(jvm
                .exception("java/lang/IndexOutOfBoundsException", &format!("Index: {index}, Size: {element_count}"))
                .await);
        }

        jvm.invoke_virtual(&this, "java/util/Vector", "insertElementAt", "(Ljava/lang/Object;I)V", (element, index))
            .await
    }

    #[method(public, synchronized)]
    async fn add_element(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.util.Vector::addElement({this:?}, {element:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        let mut element_data = Self::ensure_capacity(jvm, &mut this, (element_count + 1) as _).await?;
        jvm.store_array(&mut element_data, element_count as _, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "java/util/Vector", "elementCount", "I", element_count + 1)
            .await?;

        Ok(())
    }

    #[method(public, synchronized)]
    async fn insert_element_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        index: i32,
    ) -> Result<()> {
        tracing::debug!("java.util.Vector::insertElementAt({this:?}, {element:?}, {index:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        if index < 0 || index > element_count {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} > {element_count}"))
                .await);
        }
        let mut element_data = Self::ensure_capacity(jvm, &mut this, (element_count + 1) as _).await?;

        let num_to_move = element_count - index;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, index as _, num_to_move as _).await?;
            jvm.store_array(&mut element_data, (index + 1) as _, to_shift).await?;
        }

        jvm.store_array(&mut element_data, index as _, core::iter::once(element)).await?;
        jvm.put_field(&mut this, "java/util/Vector", "elementCount", "I", element_count + 1)
            .await?;

        Ok(())
    }

    #[method(public, synchronized)]
    async fn element_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::elementAt({this:?}, {index:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        if index < 0 || index >= element_count {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {element_count}"))
                .await);
        }

        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        let element: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as _, 1).await?.into_iter().next().unwrap();

        Ok(element)
    }

    #[method(public, synchronized)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::get({this:?}, {index:?})");

        jvm.invoke_virtual(&this, "java/util/Vector", "elementAt", "(I)Ljava/lang/Object;", (index,))
            .await
    }

    #[method(public, synchronized)]
    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::set({this:?}, {index:?}, {element:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        if index < 0 || index >= element_count {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {element_count}"))
                .await);
        }

        let mut element_data = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        let old_element: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as _, 1).await?.into_iter().next().unwrap();
        jvm.store_array(&mut element_data, index as _, core::iter::once(element)).await?;

        Ok(old_element)
    }

    #[method(public, synchronized)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Vector::size({this:?})");

        jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await
    }

    #[method(public, synchronized)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Vector::isEmpty({this:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;

        Ok(element_count == 0)
    }

    #[method(public, synchronized)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::remove({this:?}, {index:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        if index < 0 || index >= element_count {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {element_count}"))
                .await);
        }

        let mut element_data = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        let removed: ClassInstanceRef<Object> = jvm.load_array(&element_data, index as _, 1).await?.into_iter().next().unwrap();

        let num_to_move = element_count - index - 1;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, (index + 1) as _, num_to_move as _).await?;
            jvm.store_array(&mut element_data, index as _, to_shift).await?;
        }

        let null_ref: ClassInstanceRef<Object> = None.into();
        jvm.store_array(&mut element_data, (element_count - 1) as _, core::iter::once(null_ref))
            .await?;
        jvm.put_field(&mut this, "java/util/Vector", "elementCount", "I", element_count - 1)
            .await?;

        Ok(removed)
    }

    #[method(name = "remove", public)]
    async fn remove_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::remove({this:?}, {element:?})");

        jvm.invoke_virtual(&this, "java/util/Vector", "removeElement", "(Ljava/lang/Object;)Z", (element,))
            .await
    }

    #[method(public, synchronized)]
    async fn remove_all_elements(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::removeAllElements({this:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        let mut element_data = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;

        let nulls: Vec<ClassInstanceRef<Object>> = (0..element_count).map(|_| None.into()).collect();
        if !nulls.is_empty() {
            jvm.store_array(&mut element_data, 0, nulls).await?;
        }

        jvm.put_field(&mut this, "java/util/Vector", "elementCount", "I", 0).await?;

        Ok(())
    }

    #[method(public, synchronized)]
    async fn remove_element_at(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        tracing::debug!("java.util.Vector::removeElementAt({this:?}, {index:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        if index < 0 || index >= element_count {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {element_count}"))
                .await);
        }

        let mut element_data = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;

        let num_to_move = element_count - index - 1;
        if num_to_move > 0 {
            let to_shift: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, (index + 1) as _, num_to_move as _).await?;
            jvm.store_array(&mut element_data, index as _, to_shift).await?;
        }

        let null_ref: ClassInstanceRef<Object> = None.into();
        jvm.store_array(&mut element_data, (element_count - 1) as _, core::iter::once(null_ref))
            .await?;
        jvm.put_field(&mut this, "java/util/Vector", "elementCount", "I", element_count - 1)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.Vector::indexOf({this:?}, {element:?})");

        jvm.invoke_virtual(&this, "java/util/Vector", "indexOf", "(Ljava/lang/Object;I)I", (element, 0))
            .await
    }

    #[method(name = "indexOf", public, synchronized)]
    async fn index_of_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        index: i32,
    ) -> Result<i32> {
        tracing::debug!("java.util.Vector::indexOf({this:?}, {element:?}, {index:?})");

        if index < 0 {
            return Err(jvm.exception("java/lang/ArrayIndexOutOfBoundsException", &index.to_string()).await);
        }

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;

        for i in index..element_count {
            let item: ClassInstanceRef<Object> = jvm.load_array(&element_data, i as _, 1).await?.into_iter().next().unwrap();

            if item.is_null() && element.is_null() {
                return Ok(i);
            }

            if item.is_null() || element.is_null() {
                continue;
            }

            if Self::object_equals(jvm, &element, &item).await? {
                return Ok(i);
            }
        }

        Ok(-1)
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::contains({this:?}, {element:?})");

        let index: i32 = jvm
            .invoke_virtual(&this, "java/util/Vector", "indexOf", "(Ljava/lang/Object;)I", (element,))
            .await?;

        Ok(index >= 0)
    }

    #[method(public, synchronized)]
    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.Vector::lastIndexOf({this:?}, {element:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;

        let index: i32 = jvm
            .invoke_virtual(
                &this,
                "java/util/Vector",
                "lastIndexOf",
                "(Ljava/lang/Object;I)I",
                (element, element_count - 1),
            )
            .await?;

        Ok(index)
    }

    #[method(name = "lastIndexOf", public, synchronized)]
    async fn last_index_of_index(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        index: i32,
    ) -> Result<i32> {
        tracing::debug!("java.util.Vector::lastIndexOf({this:?}, {element:?}, {index:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;

        if index >= element_count {
            return Err(jvm
                .exception("java/lang/IndexOutOfBoundsException", &format!("{index} >= {element_count}"))
                .await);
        }

        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;

        for i in (0..=index).rev() {
            let item: ClassInstanceRef<Object> = jvm.load_array(&element_data, i as _, 1).await?.into_iter().next().unwrap();

            if item.is_null() && element.is_null() {
                return Ok(i);
            }

            if item.is_null() || element.is_null() {
                continue;
            }

            if Self::object_equals(jvm, &element, &item).await? {
                return Ok(i);
            }
        }

        Ok(-1)
    }

    #[method(public, synchronized)]
    async fn first_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::firstElement({this:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;

        if element_count == 0 {
            return Err(jvm.exception("java/util/NoSuchElementException", "Vector is empty").await);
        }

        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        let element: ClassInstanceRef<Object> = jvm.load_array(&element_data, 0, 1).await?.into_iter().next().unwrap();

        Ok(element)
    }

    #[method(public, synchronized)]
    async fn last_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::lastElement({this:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        if element_count == 0 {
            return Err(jvm.exception("java/util/NoSuchElementException", "Vector is empty").await);
        }

        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        Ok(jvm
            .load_array::<ClassInstanceRef<Object>>(&element_data, (element_count - 1) as usize, 1)
            .await?
            .pop()
            .unwrap())
    }

    #[method(public, synchronized)]
    async fn set_element_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        index: i32,
    ) -> Result<()> {
        tracing::debug!("java.util.Vector::setElementAt({this:?}, {element:?}, {index:?})");
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &this,
                "java/util/Vector",
                "set",
                "(ILjava/lang/Object;)Ljava/lang/Object;",
                (index, element),
            )
            .await?;
        Ok(())
    }

    #[method(public, synchronized)]
    async fn set_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, new_size: i32) -> Result<()> {
        tracing::debug!("java.util.Vector::setSize({this:?}, {new_size:?})");

        if new_size < 0 {
            return Err(jvm.exception("java/lang/ArrayIndexOutOfBoundsException", &new_size.to_string()).await);
        }

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        if new_size > element_count {
            Self::ensure_capacity(jvm, &mut this, new_size as usize).await?;
        } else if new_size < element_count {
            let mut element_data = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
            let nulls: Vec<ClassInstanceRef<Object>> = (new_size..element_count).map(|_| None.into()).collect();
            jvm.store_array(&mut element_data, new_size as usize, nulls).await?;
        }
        jvm.put_field(&mut this, "java/util/Vector", "elementCount", "I", new_size).await
    }

    #[method(public, synchronized)]
    async fn remove_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::removeElement({this:?}, {element:?})");

        let index: i32 = jvm
            .invoke_virtual(&this, "java/util/Vector", "indexOf", "(Ljava/lang/Object;)I", (element,))
            .await?;

        if index >= 0 {
            let _: () = jvm.invoke_virtual(&this, "java/util/Vector", "removeElementAt", "(I)V", (index,)).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::clear({this:?})");

        jvm.invoke_virtual(&this, "java/util/Vector", "removeAllElements", "()V", ()).await
    }

    #[method(public, synchronized)]
    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        tracing::debug!("java.util.Vector::toArray({this:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        let element_data: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;

        Self::copy_to_array(jvm, &element_data, element_count).await
    }

    #[method(name = "toArray", public, synchronized)]
    async fn to_typed_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        destination: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Array<Object>>> {
        tracing::debug!("java.util.Vector::toArray({this:?}, {destination:?})");

        jvm.invoke_special(
            &this,
            "java/util/AbstractCollection",
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (destination,),
        )
        .await
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public, synchronized)]
    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::addAll({this:?}, {collection:?})");

        jvm.invoke_special(
            &this,
            "java/util/AbstractCollection",
            "addAll",
            "(Ljava/util/Collection;)Z",
            (collection,),
        )
        .await
    }

    #[method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", public, synchronized)]
    async fn add_all_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        collection: ClassInstanceRef<Object>,
    ) -> Result<bool> {
        tracing::debug!("java.util.Vector::addAll({this:?}, {index:?}, {collection:?})");

        jvm.invoke_special(
            &this,
            "java/util/AbstractList",
            "addAll",
            "(ILjava/util/Collection;)Z",
            (index, collection),
        )
        .await
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public, synchronized)]
    async fn contains_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::containsAll({this:?}, {collection:?})");

        jvm.invoke_special(
            &this,
            "java/util/AbstractCollection",
            "containsAll",
            "(Ljava/util/Collection;)Z",
            (collection,),
        )
        .await
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public, synchronized)]
    async fn remove_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::removeAll({this:?}, {collection:?})");

        jvm.invoke_special(
            &this,
            "java/util/AbstractCollection",
            "removeAll",
            "(Ljava/util/Collection;)Z",
            (collection,),
        )
        .await
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public, synchronized)]
    async fn retain_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::retainAll({this:?}, {collection:?})");

        jvm.invoke_special(
            &this,
            "java/util/AbstractCollection",
            "retainAll",
            "(Ljava/util/Collection;)Z",
            (collection,),
        )
        .await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public, synchronized)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::iterator({this:?})");

        let iterator = jvm.new_class("java/util/Vector$Itr", "(Ljava/util/Vector;I)V", (this, 0)).await?;

        Ok(iterator.into())
    }

    #[method(descriptor = "()Ljava/util/ListIterator;", public, synchronized)]
    async fn list_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm.new_class("java/util/Vector$ListItr", "(Ljava/util/Vector;I)V", (this, 0)).await?;
        Ok(iterator.into())
    }

    #[method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", public, synchronized)]
    async fn list_iterator_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm.new_class("java/util/Vector$ListItr", "(Ljava/util/Vector;I)V", (this, index)).await?;
        Ok(iterator.into())
    }

    #[method(public, synchronized)]
    async fn trim_to_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::trimToSize({this:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        let current_capacity = jvm.array_length(&element_data).await?;

        if (element_count as usize) < current_capacity {
            let elements: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, 0, element_count as _).await?;
            let mut new_element_data = jvm.instantiate_array("Ljava/lang/Object;", element_count as _).await?;
            jvm.store_array(&mut new_element_data, 0, elements).await?;
            jvm.put_field(&mut this, "java/util/Vector", "elementData", "[Ljava/lang/Object;", new_element_data)
                .await?;
        }

        Ok(())
    }

    #[method(public, synchronized)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Vector::toString({this:?})");

        let element_count: i32 = jvm.get_field(&this, "java/util/Vector", "elementCount", "I").await?;
        let element_data: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        let elements: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, 0, element_count as usize).await?;
        let mut result = RustString::from("[");
        for (index, element) in elements.into_iter().enumerate() {
            if index > 0 {
                result.push_str(", ");
            }

            if element.is_null() {
                result.push_str("null");
            } else if element.instance.as_ref().unwrap().equals(&**this)? {
                result.push_str("(this Collection)");
            } else {
                let text: ClassInstanceRef<String> = jvm
                    .invoke_virtual(&element, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                    .await?;
                result.push_str(&JavaLangString::to_rust_string(jvm, &text).await?);
            }
        }
        result.push(']');

        Ok(JavaLangString::from_rust_string(jvm, &result).await?.into())
    }

    async fn ensure_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, min_capacity: usize) -> Result<ClassInstanceRef<Array<Object>>> {
        let element_data: ClassInstanceRef<Array<Object>> = jvm.get_field(this, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
        let current_capacity = jvm.array_length(&element_data).await?;

        if min_capacity > current_capacity {
            let capacity_increment: i32 = jvm.get_field(this, "java/util/Vector", "capacityIncrement", "I").await?;
            let new_capacity = if capacity_increment > 0 {
                current_capacity + capacity_increment as usize
            } else {
                current_capacity * 2
            };
            let new_capacity = new_capacity.max(min_capacity);

            let element_count: i32 = jvm.get_field(this, "java/util/Vector", "elementCount", "I").await?;
            let old_elements: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, 0, element_count as _).await?;

            let mut new_element_data = jvm.instantiate_array("Ljava/lang/Object;", new_capacity).await?;
            jvm.store_array(&mut new_element_data, 0, old_elements).await?;
            jvm.put_field(this, "java/util/Vector", "elementData", "[Ljava/lang/Object;", new_element_data.clone())
                .await?;
            return Ok(new_element_data.into());
        }

        Ok(element_data)
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
