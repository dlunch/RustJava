use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableSet
pub struct CollectionsUnmodifiableSet;

impl CollectionsUnmodifiableSet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableSet",
            parent_class: Some("java/util/Collections$UnmodifiableCollection"),
            interfaces: vec!["java/util/Set", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Set;)V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, set: ClassInstanceRef<Object>) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/Collections$UnmodifiableCollection",
            "<init>",
            "(Ljava/util/Collection;)V",
            (set,),
        )
        .await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if !other.is_null() && this.identity() == other.identity() {
            return Ok(true);
        }
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&set, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "c", "Ljava/util/Collection;").await?;
        jvm.invoke_virtual(&set, "java/lang/Object", "hashCode", "()I", ()).await
    }
}
