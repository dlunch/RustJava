use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$CopiesList
#[java_class(
    name = "java/util/Collections$CopiesList",
    extends = "java/util/AbstractList",
    implements("java/io/Serializable"),
    final
)]
pub struct CollectionsCopiesList {
    #[field(final)]
    n: i32,
    #[field(final)]
    element: ClassInstanceRef<Object>,
}

#[java_methods]
impl CollectionsCopiesList {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, count: i32, element: ClassInstanceRef<Object>) -> Result<()> {
        if count < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "List length = negative").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/Collections$CopiesList", "n", "I", count).await?;
        jvm.put_field(&mut this, "java/util/Collections$CopiesList", "element", "Ljava/lang/Object;", element)
            .await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/util/Collections$CopiesList", "n", "I").await
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<bool> {
        if jvm.get_field::<i32>(&this, "java/util/Collections$CopiesList", "n", "I").await? == 0 {
            return Ok(false);
        }
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$CopiesList", "element", "Ljava/lang/Object;")
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

    #[method(public)]
    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<i32> {
        if jvm.get_field::<i32>(&this, "java/util/Collections$CopiesList", "n", "I").await? == 0 {
            return Ok(-1);
        }
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$CopiesList", "element", "Ljava/lang/Object;")
            .await?;
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

    #[method(public)]
    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<i32> {
        let count: i32 = jvm.get_field(&this, "java/util/Collections$CopiesList", "n", "I").await?;
        if count == 0 {
            return Ok(-1);
        }
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$CopiesList", "element", "Ljava/lang/Object;")
            .await?;
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

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let count: i32 = jvm.get_field(&this, "java/util/Collections$CopiesList", "n", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "index").await);
        }
        jvm.get_field(&this, "java/util/Collections$CopiesList", "element", "Ljava/lang/Object;")
            .await
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        if jvm.get_field::<i32>(&this, "java/util/Collections$CopiesList", "n", "I").await? == 0 {
            return Ok(());
        }
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    #[method(name = "remove", public)]
    async fn remove_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<bool> {
        let count: i32 = jvm.get_field(&this, "java/util/Collections$CopiesList", "n", "I").await?;
        if count == 0 {
            return Ok(false);
        }
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$CopiesList", "element", "Ljava/lang/Object;")
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
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    #[method(name = "add", public)]
    async fn add_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    #[method(public)]
    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _: ClassInstanceRef<Self>,
        _: i32,
        _: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    #[method(name = "remove", public)]
    async fn remove_at(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "copies list").await)
    }

    #[method(public)]
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

        let count: i32 = jvm.get_field(&this, "java/util/Collections$CopiesList", "n", "I").await?;
        if jvm
            .invoke_virtual::<_, i32>(&other, &other.class_definition().name(), "size", "()I", ())
            .await?
            != count
        {
            return Ok(false);
        }
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$CopiesList", "element", "Ljava/lang/Object;")
            .await?;
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

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let count: i32 = jvm.get_field(&this, "java/util/Collections$CopiesList", "n", "I").await?;
        let element: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$CopiesList", "element", "Ljava/lang/Object;")
            .await?;
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
