use core::mem;

use alloc::{sync::Arc, vec, vec::Vec};

use async_lock::RwLock;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::Object, RuntimeClassProto, RuntimeContext};

// I'm too lazy to implement vector in java, so i'm leveraging rust vector here...
type RustVector = Arc<RwLock<Vec<ClassInstanceRef<Object>>>>;

// class java.util.Vector
pub struct Vector {}

impl Vector {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_capacity, Default::default()),
                JavaMethodProto::new("<init>", "(II)V", Self::init_with_capacity_increment, Default::default()),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, Default::default()),
                JavaMethodProto::new("addElement", "(Ljava/lang/Object;)V", Self::add_element, Default::default()),
                JavaMethodProto::new("elementAt", "(I)Ljava/lang/Object;", Self::element_at, Default::default()),
                JavaMethodProto::new("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Self::set, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("raw", "[B", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({:?})", &this);

        jvm.invoke_special(&this, "java/util/Vector", "<init>", "(I)V", (0,)).await?;

        Ok(())
    }

    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.util.Vector::<init>({:?}, {:?})", &this, capacity);

        jvm.invoke_special(&this, "java/util/Vector", "<init>", "(II)V", (capacity, 0)).await?;

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

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let rust_vector: RustVector = Arc::new(RwLock::new(Vec::with_capacity(capacity as _)));

        jvm.put_rust_object_field(&mut this, "raw", rust_vector).await?;

        Ok(())
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Vector::add({:?}, {:?})", &this, &element);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        rust_vector.write().await.push(element);

        Ok(true)
    }

    async fn add_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.util.Vector::addElement({:?}, {:?})", &this, &element);

        // do we need to call add() instead?
        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        rust_vector.write().await.push(element);

        Ok(())
    }

    async fn element_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Vector::elementAt({:?}, {:?})", &this, index);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let element = rust_vector.read().await.get(index as usize).unwrap().clone();

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

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let old_element = mem::replace(&mut rust_vector.write().await[index as usize], element);

        Ok(old_element)
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Vector::size({:?})", &this);

        let rust_vector = Self::get_rust_vector(jvm, &this).await?;
        let size = rust_vector.read().await.len();

        Ok(size as _)
    }

    async fn get_rust_vector(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<RustVector> {
        jvm.get_rust_object_field(this, "raw").await
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

    use crate::{classes::java::lang::Object, test::test_jvm};

    #[futures_test::test]
    async fn test_vector() -> Result<()> {
        let jvm = test_jvm().await?;

        let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

        let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
        let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;

        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element1.clone(),)).await?;
        let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element2.clone(),)).await?;

        let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
        assert_eq!(size, 2);

        let element_at1: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "elementAt", "(I)Ljava/lang/Object;", (0,)).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &element_at1).await?, "testValue1");

        Ok(())
    }

    #[futures_test::test]
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
}
