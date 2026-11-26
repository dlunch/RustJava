use jvm::{Array, AsClassInstance, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// private static class java.util.Arrays$ArrayList
#[java_class(
    name = "java/util/Arrays$ArrayList",
    extends = "java/util/AbstractList",
    implements("java/io/Serializable")
)]
pub struct ArraysArrayList {
    #[field(private, final)]
    a: ClassInstanceRef<Array<Object>>,
}

#[java_methods]
impl ArraysArrayList {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, array: ClassInstanceRef<Array<Object>>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/Arrays$ArrayList", "a", "[Ljava/lang/Object;", array)
            .await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let array: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "java/util/Arrays$ArrayList", "a", "[Ljava/lang/Object;").await?;
        Ok(jvm.array_length(&array).await? as i32)
    }

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let array: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "java/util/Arrays$ArrayList", "a", "[Ljava/lang/Object;").await?;
        let length = jvm.array_length(&array).await?;
        if index < 0 || index as usize >= length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "index").await);
        }
        Ok(jvm.load_array(&array, index as usize, 1).await?.remove(0))
    }

    #[method(public)]
    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let mut array: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "java/util/Arrays$ArrayList", "a", "[Ljava/lang/Object;").await?;
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

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }

    #[method(name = "add", public)]
    async fn add_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }

    #[method(name = "remove", public)]
    async fn remove_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "fixed-size list").await)
    }
}
