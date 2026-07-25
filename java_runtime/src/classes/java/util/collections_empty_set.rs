use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$EmptySet
pub struct CollectionsEmptySet;

impl CollectionsEmptySet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$EmptySet",
            parent_class: Some("java/util/AbstractSet"),
            interfaces: vec!["java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PRIVATE),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::FINAL,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await
    }

    async fn size(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }

    async fn contains(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "EMPTY_LIST", "Ljava/util/List;").await?;
        jvm.invoke_virtual(&list, "iterator", "()Ljava/util/Iterator;", ()).await
    }

    async fn clear(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn remove(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(other.as_ref(), "java/util/Set") {
            return Ok(false);
        }
        jvm.invoke_virtual(&other, "isEmpty", "()Z", ()).await
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }
}
