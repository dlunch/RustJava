use jvm::{ClassInstanceRef, JavaError, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// abstract class java.util.AbstractSet
#[java_class(
    name = "java/util/AbstractSet",
    extends = "java/util/AbstractCollection",
    implements("java/util/Set"),
    public,
    abstract
)]
pub struct AbstractSet;

#[java_methods]
impl AbstractSet {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractSet::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }
        if this.identity() == other.identity() {
            return Ok(true);
        }
        if !jvm.is_instance(other.as_ref(), "java/util/Set") {
            return Ok(false);
        }
        let size: i32 = jvm.invoke_virtual(&this, "java/util/AbstractSet", "size", "()I", ()).await?;
        if jvm
            .invoke_virtual::<_, i32>(&other, &other.class_definition().name(), "size", "()I", ())
            .await?
            != size
        {
            return Ok(false);
        }
        match jvm
            .invoke_virtual(&this, "java/util/AbstractSet", "containsAll", "(Ljava/util/Collection;)Z", (other,))
            .await
        {
            Ok(equal) => Ok(equal),
            Err(JavaError::JavaException(exception))
                if jvm.is_instance(&*exception, "java/lang/ClassCastException") || jvm.is_instance(&*exception, "java/lang/NullPointerException") =>
            {
                Ok(false)
            }
            Err(error) => Err(error),
        }
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "java/util/AbstractSet", "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        let mut hash = 0i32;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let element: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            if !element.is_null() {
                hash = hash.wrapping_add(jvm.invoke_virtual::<_, i32>(&element, "java/lang/Object", "hashCode", "()I", ()).await?);
            }
        }
        Ok(hash)
    }
}
