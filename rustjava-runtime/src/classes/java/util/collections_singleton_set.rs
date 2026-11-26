use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$SingletonSet
#[java_class(
    name = "java/util/Collections$SingletonSet",
    extends = "java/util/AbstractSet",
    implements("java/io/Serializable"),
    final
)]
pub struct CollectionsSingletonSet {
    #[field(private, final)]
    element: ClassInstanceRef<Object>,
}

#[java_methods]
impl CollectionsSingletonSet {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/Collections$SingletonSet", "element", "Ljava/lang/Object;", element)
            .await
    }

    #[method(public)]
    async fn size(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1)
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<bool> {
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$SingletonSet", "element", "Ljava/lang/Object;")
            .await?;
        if target.is_null() {
            return Ok(element.is_null());
        }
        if element.is_null() {
            return Ok(false);
        }
        jvm.invoke_virtual(&target, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (element,))
            .await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$SingletonSet", "element", "Ljava/lang/Object;")
            .await?;
        let list = jvm
            .new_class("java/util/Collections$CopiesList", "(ILjava/lang/Object;)V", (1, element))
            .await?;
        jvm.invoke_virtual(&list, "java/util/Collections$CopiesList", "iterator", "()Ljava/util/Iterator;", ())
            .await
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<bool> {
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$SingletonSet", "element", "Ljava/lang/Object;")
            .await?;
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
        Err(jvm.exception("java/lang/UnsupportedOperationException", "singleton set").await)
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }
        if this.identity() == other.identity() {
            return Ok(true);
        }
        if !jvm.is_instance(other.as_ref(), "java/util/Set")
            || jvm
                .invoke_virtual::<_, i32>(&other, &other.class_definition().name(), "size", "()I", ())
                .await?
                != 1
        {
            return Ok(false);
        }
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&other, &other.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        let other_element: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
            .await?;
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$SingletonSet", "element", "Ljava/lang/Object;")
            .await?;
        if other_element.is_null() {
            return Ok(element.is_null());
        }
        if element.is_null() {
            return Ok(false);
        }
        jvm.invoke_virtual(&other_element, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (element,))
            .await
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$SingletonSet", "element", "Ljava/lang/Object;")
            .await?;
        if element.is_null() {
            Ok(0)
        } else {
            jvm.invoke_virtual(&element, "java/lang/Object", "hashCode", "()I", ()).await
        }
    }
}
