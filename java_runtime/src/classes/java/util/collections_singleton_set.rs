use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$SingletonSet
pub struct CollectionsSingletonSet;

impl CollectionsSingletonSet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$SingletonSet",
            parent_class: Some("java/util/AbstractSet"),
            interfaces: vec!["java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/Object;)V", Self::init, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new(
                "element",
                "Ljava/lang/Object;",
                FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
            )],
            access_flags: ClassAccessFlags::FINAL,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "element", "Ljava/lang/Object;", element).await
    }

    async fn size(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1)
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<bool> {
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        if target.is_null() {
            return Ok(element.is_null());
        }
        if element.is_null() {
            return Ok(false);
        }
        jvm.invoke_virtual(&target, "equals", "(Ljava/lang/Object;)Z", (element,)).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        let list = jvm
            .new_class("java/util/Collections$CopiesList", "(ILjava/lang/Object;)V", (1, element))
            .await?;
        jvm.invoke_virtual(&list, "iterator", "()Ljava/util/Iterator;", ()).await
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<bool> {
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        let equal = if target.is_null() {
            element.is_null()
        } else if element.is_null() {
            false
        } else {
            jvm.invoke_virtual::<_, bool>(&target, "equals", "(Ljava/lang/Object;)Z", (element,))
                .await?
        };
        if !equal {
            return Ok(false);
        }
        Err(jvm.exception("java/lang/UnsupportedOperationException", "singleton set").await)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }
        if this.identity() == other.identity() {
            return Ok(true);
        }
        if !jvm.is_instance(other.as_ref(), "java/util/Set") || jvm.invoke_virtual::<_, i32>(&other, "size", "()I", ()).await? != 1 {
            return Ok(false);
        }
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&other, "iterator", "()Ljava/util/Iterator;", ()).await?;
        let other_element: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        if other_element.is_null() {
            return Ok(element.is_null());
        }
        if element.is_null() {
            return Ok(false);
        }
        jvm.invoke_virtual(&other_element, "equals", "(Ljava/lang/Object;)Z", (element,)).await
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        if element.is_null() {
            Ok(0)
        } else {
            jvm.invoke_virtual(&element, "hashCode", "()I", ()).await
        }
    }
}
