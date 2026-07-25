use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, AsClassInstance, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// private static class java.util.Arrays$ArrayList
pub struct ArraysArrayList;

impl ArraysArrayList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Arrays$ArrayList",
            parent_class: Some("java/util/AbstractList"),
            interfaces: vec!["java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "([Ljava/lang/Object;)V", Self::init, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(I)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Self::set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(ILjava/lang/Object;)V", Self::add_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(I)Ljava/lang/Object;", Self::remove_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new(
                "a",
                "[Ljava/lang/Object;",
                FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
            )],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, array: ClassInstanceRef<Array<Object>>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "a", "[Ljava/lang/Object;", array).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let array: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "a", "[Ljava/lang/Object;").await?;
        Ok(jvm.array_length(&array).await? as i32)
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let array: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "a", "[Ljava/lang/Object;").await?;
        let length = jvm.array_length(&array).await?;
        if index < 0 || index as usize >= length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "index").await);
        }
        Ok(jvm.load_array(&array, index as usize, 1).await?.remove(0))
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let mut array: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "a", "[Ljava/lang/Object;").await?;
        let length = jvm.array_length(&array).await?;
        if index < 0 || index as usize >= length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "index").await);
        }
        if !element.is_null() && !jvm.array_store_allowed(array.as_class_instance(), element.as_class_instance()) {
            return Err(jvm.exception("java/lang/ArrayStoreException", &element.class_definition().name()).await);
        }
        let previous = jvm.load_array(&array, index as usize, 1).await?.remove(0);
        jvm.store_array(&mut array, index as usize, core::iter::once(element)).await?;
        Ok(previous)
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }

    async fn add_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }

    async fn remove_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }
}
