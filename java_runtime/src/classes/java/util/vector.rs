use core::mem;

use alloc::vec;
use alloc::{boxed::Box, format, sync::Arc, vec::Vec};

use async_lock::Mutex;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstance, ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::Object, RuntimeClassProto, RuntimeContext};

// I'm too lazy to implement vector in java, so i'm leveraging rust vector here...
type RustVector = Arc<Mutex<Vec<Option<Box<dyn ClassInstance>>>>>;

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
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/Object;)I", Self::last_index_of, Default::default()),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/Object;I)I", Self::last_index_of_index, Default::default()),
                JavaMethodProto::new("firstElement", "()Ljava/lang/Object;", Self::first_element, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("raw", "[B", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/util/Vector", "<init>", "(I)V", (0,)).await?;

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

        let rust_vector: RustVector = Arc::new(Mutex::new(Vec::with_capacity(capacity as _)));

        jvm.put_rust_object_field(&mut this, "raw", rust_vector).await?;

        Ok(())
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::add({:?}, {:?})", &this, &element);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        rust_vector.lock().await.push(element.into());

        Ok(true)
    }

    async fn add_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.util.Vector::addElement({:?}, {:?})", &this, &element);

        // do we need to call add() instead?
        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        rust_vector.lock().await.push(element.into());

        Ok(())
    }

    async fn insert_element_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        index: i32,
    ) -> Result<()> {
        tracing::debug!("java.util.Vector::insertElementAt({:?}, {:?}, {:?})", &this, &element, index);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        rust_vector.lock().await.insert(index as usize, element.into());

        Ok(())
    }

    async fn element_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::elementAt({:?}, {:?})", &this, index);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let element = rust_vector.lock().await.get(index as usize).unwrap().clone();

        Ok(element.into())
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::set({:?}, {:?}, {:?})", &this, index, &element);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let old_element = mem::replace(&mut rust_vector.lock().await[index as usize], element.into());

        Ok(old_element.into())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Vector::size({:?})", &this);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let size = rust_vector.lock().await.len();

        Ok(size as _)
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Vector::isEmpty({:?})", &this);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let is_empty = rust_vector.lock().await.is_empty();

        Ok(is_empty)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::remove({:?}, {:?})", &this, index);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let removed = rust_vector.lock().await.remove(index as usize);

        Ok(removed.into())
    }

    async fn remove_all_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::removeAllElements({:?})", &this);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        rust_vector.lock().await.clear();

        Ok(())
    }

    async fn remove_element_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        tracing::debug!("java.util.Vector::removeElementAt({:?}, {:?})", &this, index);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        rust_vector.lock().await.remove(index as usize);

        Ok(())
    }

    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.Vector::lastIndexOf({:?}, {:?})", &this, &element);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let index = rust_vector.lock().await.len() - 1;

        let index: i32 = jvm
            .invoke_virtual(&this, "lastIndexOf", "(Ljava/lang/Object;I)I", (element, index as i32))
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

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let vector = rust_vector.lock().await;
        let size = vector.len();

        if index as usize >= size {
            return Err(jvm
                .exception("java/lang/IndexOutOfBoundsException", &format!("{} >= {}", index, size))
                .await);
        }

        for (i, item) in vector[..=index as usize].iter().enumerate().rev() {
            if item.is_none() {
                if element.is_null() {
                    return Ok(i as i32);
                }
                continue;
            }

            let value: Box<dyn ClassInstance> = element.clone().into();
            if item.as_ref().unwrap().equals(&*value)? {
                return Ok(i as i32);
            }
        }

        Ok(-1)
    }

    async fn first_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::firstElement({:?})", &this);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;

        if rust_vector.lock().await.len() == 0 {
            return Ok(None.into());
        }

        let element = rust_vector.lock().await.first().cloned().unwrap();

        Ok(element.into())
    }

    async fn get_rust_vector(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<RustVector> {
        jvm.get_rust_object_field(this, "raw").await
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

    use crate::{classes::java::lang::Object, test::test_jvm};

    #[tokio::test]
    async fn test_vector() -> Result<()> {
        let jvm = test_jvm().await?;

        let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

        let is_empty: bool = jvm.invoke_virtual(&vector, "isEmpty", "()Z", ()).await?;
        assert!(is_empty);

        let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
        let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;

        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element1.clone(),)).await?;
        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element2.clone(),)).await?;

        let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
        assert_eq!(size, 2);

        let element_at1: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "elementAt", "(I)Ljava/lang/Object;", (0,)).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &element_at1).await?, "testValue1");

        let is_empty: bool = jvm.invoke_virtual(&vector, "isEmpty", "()Z", ()).await?;
        assert!(!is_empty);

        let removed: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "remove", "(I)Ljava/lang/Object;", (0,)).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "testValue1");

        let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
        assert_eq!(size, 1);

        let _: () = jvm.invoke_virtual(&vector, "removeElementAt", "(I)V", (0,)).await?;

        let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
        assert_eq!(size, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_vector_null() -> Result<()> {
        let jvm = test_jvm().await?;

        let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (None,)).await?;

        let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
        assert_eq!(size, 1);

        let element_at: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "elementAt", "(I)Ljava/lang/Object;", (0,)).await?;
        assert!(element_at.is_null());

        Ok(())
    }

    #[tokio::test]
    async fn test_vector_last_index_of() -> Result<()> {
        let jvm = test_jvm().await?;

        let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

        let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
        let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;
        let element3 = JavaLangString::from_rust_string(&jvm, "testValue3").await?;

        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element1.clone(),)).await?;
        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element2.clone(),)).await?;
        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element3.clone(),)).await?;
        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element2.clone(),)).await?;

        let index: i32 = jvm
            .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (element2.clone(),))
            .await?;
        assert_eq!(index, 3);

        let index: i32 = jvm
            .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (element1.clone(),))
            .await?;
        assert_eq!(index, 0);

        let non_existing_element = JavaLangString::from_rust_string(&jvm, "nonExisting").await?;
        let index: i32 = jvm
            .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (non_existing_element,))
            .await?;
        assert_eq!(index, -1);

        let index: i32 = jvm
            .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;I)I", (element2.clone(), 2))
            .await?;
        assert_eq!(index, 1);

        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (None,)).await?;
        let index: i32 = jvm.invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (None,)).await?;
        assert_eq!(index, 4);

        Ok(())
    }
}
