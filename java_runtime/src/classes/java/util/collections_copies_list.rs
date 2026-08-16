use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$CopiesList
pub struct CollectionsCopiesList;

impl CollectionsCopiesList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$CopiesList",
            parent_class: Some("java/util/AbstractList"),
            interfaces: vec!["java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(ILjava/lang/Object;)V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(Ljava/lang/Object;)I", Self::index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/Object;)I", Self::last_index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(I)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove_object, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(ILjava/lang/Object;)V", Self::add_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Self::set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(I)Ljava/lang/Object;", Self::remove_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("n", "I", FieldAccessFlags::FINAL),
                JavaFieldProto::new("element", "Ljava/lang/Object;", FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::FINAL,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, count: i32, element: ClassInstanceRef<Object>) -> Result<()> {
        if count < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "List length = negative").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "n", "I", count).await?;
        jvm.put_field(&mut this, "element", "Ljava/lang/Object;", element).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "n", "I").await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<bool> {
        if jvm.get_field::<i32>(&this, "n", "I").await? == 0 {
            return Ok(false);
        }
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        if target.is_null() {
            return Ok(element.is_null());
        }
        if element.is_null() {
            return Ok(false);
        }
        jvm.invoke_virtual(&target, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (element,))
            .await
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<i32> {
        if jvm.get_field::<i32>(&this, "n", "I").await? == 0 {
            return Ok(-1);
        }
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        let equal = if target.is_null() {
            element.is_null()
        } else if element.is_null() {
            false
        } else {
            jvm.invoke_virtual::<_, bool>(&target, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (element,))
                .await?
        };
        Ok(if equal { 0 } else { -1 })
    }

    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<i32> {
        let count: i32 = jvm.get_field(&this, "n", "I").await?;
        if count == 0 {
            return Ok(-1);
        }
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        let equal = if target.is_null() {
            element.is_null()
        } else if element.is_null() {
            false
        } else {
            jvm.invoke_virtual::<_, bool>(&target, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (element,))
                .await?
        };
        Ok(if equal { count - 1 } else { -1 })
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let count: i32 = jvm.get_field(&this, "n", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "index").await);
        }
        jvm.get_field(&this, "element", "Ljava/lang/Object;").await
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        if jvm.get_field::<i32>(&this, "n", "I").await? == 0 {
            return Ok(());
        }
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    async fn remove_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<bool> {
        let count: i32 = jvm.get_field(&this, "n", "I").await?;
        if count == 0 {
            return Ok(false);
        }
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        let equal = if target.is_null() {
            element.is_null()
        } else if element.is_null() {
            false
        } else {
            jvm.invoke_virtual::<_, bool>(&target, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (element,))
                .await?
        };
        if !equal {
            return Ok(false);
        }
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    async fn add_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _: ClassInstanceRef<Self>,
        _: i32,
        _: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    async fn remove_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }
        if this.identity() == other.identity() {
            return Ok(true);
        }
        if !jvm.is_instance(other.as_ref(), "java/util/List") {
            return Ok(false);
        }

        let count: i32 = jvm.get_field(&this, "n", "I").await?;
        if jvm
            .invoke_virtual::<_, i32>(&other, &other.class_definition().name(), "size", "()I", ())
            .await?
            != count
        {
            return Ok(false);
        }
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&other, &other.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let current: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let equal = if element.is_null() {
                current.is_null()
            } else if current.is_null() {
                false
            } else {
                jvm.invoke_virtual::<_, bool>(&element, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (current,))
                    .await?
            };
            if !equal {
                return Ok(false);
            }
        }
        Ok(true)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let count: i32 = jvm.get_field(&this, "n", "I").await?;
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        let element_hash = if element.is_null() {
            0
        } else {
            jvm.invoke_virtual(&element, "java/lang/Object", "hashCode", "()I", ()).await?
        };
        let mut hash = 1i32;
        for _ in 0..count {
            hash = hash.wrapping_mul(31).wrapping_add(element_hash);
        }
        Ok(hash)
    }
}
