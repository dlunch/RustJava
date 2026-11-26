use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$EmptyList
#[java_class(
    name = "java/util/Collections$EmptyList",
    extends = "java/util/AbstractList",
    implements("java/io/Serializable"),
    final
)]
pub struct CollectionsEmptyList;

#[java_methods]
impl CollectionsEmptyList {
    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await
    }

    #[method(public)]
    async fn size(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }

    #[method(public)]
    async fn contains(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/IndexOutOfBoundsException", "empty list").await)
    }

    #[method(public)]
    async fn clear(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    #[method(name = "remove", public)]
    async fn remove_object(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }

    #[method(name = "add", public)]
    async fn add_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "empty list").await)
    }

    #[method(public)]
    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _: ClassInstanceRef<Self>,
        _: i32,
        _: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "empty list").await)
    }

    #[method(name = "remove", public)]
    async fn remove_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "empty list").await)
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(other.as_ref(), "java/util/List") {
            return Ok(false);
        }
        jvm.invoke_virtual(&other, &other.class_definition().name(), "isEmpty", "()Z", ()).await
    }

    #[method(public)]
    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1)
    }
}
