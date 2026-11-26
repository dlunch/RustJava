use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.util.Hashtable$Enumerator
#[java_class(name = "java/util/Hashtable$Enumerator", implements("java/util/Enumeration", "java/util/Iterator"))]
pub struct HashtableEnumerator {
    #[field(private)]
    elements: ClassInstanceRef<Array<Object>>,
    index: i32,
}

#[java_methods]
impl HashtableEnumerator {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, elements: ClassInstanceRef<Array<Object>>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$Enumerator::<init>({this:?}, {elements:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/util/Hashtable$Enumerator", "elements", "[Ljava/lang/Object;", elements)
            .await?;
        jvm.put_field(&mut this, "java/util/Hashtable$Enumerator", "index", "I", 0).await?;

        Ok(())
    }

    #[method(public)]
    async fn has_more_elements(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Self::has_next(jvm, context, this).await
    }

    #[method(public)]
    async fn next_element(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Self::next(jvm, context, this).await
    }

    #[method(public)]
    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$Enumerator::hasNext({this:?})");

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .get_field(&this, "java/util/Hashtable$Enumerator", "elements", "[Ljava/lang/Object;")
            .await?;
        let index: i32 = jvm.get_field(&this, "java/util/Hashtable$Enumerator", "index", "I").await?;
        if index < 0 {
            return Ok(false);
        }

        Ok((index as usize) < jvm.array_length(&elements).await?)
    }

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable$Enumerator::next({this:?})");

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .get_field(&this, "java/util/Hashtable$Enumerator", "elements", "[Ljava/lang/Object;")
            .await?;
        let index: i32 = jvm.get_field(&this, "java/util/Hashtable$Enumerator", "index", "I").await?;
        if index < 0 || index as usize >= jvm.array_length(&elements).await? {
            return Err(jvm.exception("java/util/NoSuchElementException", "Hashtable iterator exhausted").await);
        }

        let mut values = jvm.load_array(&elements, index as usize, 1).await?;
        let Some(element) = values.pop() else {
            return Err(jvm.exception("java/util/NoSuchElementException", "Hashtable iterator exhausted").await);
        };
        jvm.put_field(&mut this, "java/util/Hashtable$Enumerator", "index", "I", index + 1)
            .await?;

        Ok(element)
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$Enumerator::remove({this:?})");

        Err(jvm.exception("java/lang/UnsupportedOperationException", "Iterator.remove").await)
    }
}
