use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$EmptySet
#[java_class(
    name = "java/util/Collections$EmptySet",
    extends = "java/util/AbstractSet",
    implements("java/io/Serializable"),
    final
)]
pub struct CollectionsEmptySet;

#[java_methods]
impl CollectionsEmptySet {
    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await
    }

    #[method(public)]
    async fn size(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }

    #[method(public)]
    async fn contains(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "EMPTY_LIST", "Ljava/util/List;").await?;
        jvm.invoke_virtual(&list, &list.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await
    }

    #[method(public)]
    async fn clear(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    #[method(public)]
    async fn remove(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(other.as_ref(), "java/util/Set") {
            return Ok(false);
        }
        jvm.invoke_virtual(&other, &other.class_definition().name(), "isEmpty", "()Z", ()).await
    }

    #[method(public)]
    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }
}
