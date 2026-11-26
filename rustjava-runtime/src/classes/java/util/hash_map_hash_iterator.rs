use alloc::{boxed::Box, vec::Vec};

use jvm::{Array, ClassInstance, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::{HashMap, HashMapEntry};

// abstract class java.util.HashMap$HashIterator
#[java_class(name = "java/util/HashMap$HashIterator", implements("java/util/Iterator"), abstract)]
pub struct HashMapHashIterator {
    #[field(private)]
    map: ClassInstanceRef<HashMap>,
    #[field(private)]
    lastReturned: ClassInstanceRef<HashMapEntry>,
    #[field(private)]
    elements: ClassInstanceRef<Array<Object>>,
    index: i32,
}

#[java_methods]
impl HashMapHashIterator {
    #[method(constructor)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        map: ClassInstanceRef<HashMap>,
        elements: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        tracing::debug!("java.util.HashMap$HashIterator::<init>({this:?}, {elements:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/util/HashMap$HashIterator", "map", "Ljava/util/HashMap;", map)
            .await?;
        jvm.put_field(&mut this, "java/util/HashMap$HashIterator", "elements", "[Ljava/lang/Object;", elements)
            .await?;
        jvm.put_field(&mut this, "java/util/HashMap$HashIterator", "index", "I", 0).await?;

        Ok(())
    }

    #[method(public, final)]
    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$HashIterator::hasNext({this:?})");

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .get_field(&this, "java/util/HashMap$HashIterator", "elements", "[Ljava/lang/Object;")
            .await?;
        let index: i32 = jvm.get_field(&this, "java/util/HashMap$HashIterator", "index", "I").await?;
        if index < 0 {
            return Ok(false);
        }

        Ok((index as usize) < jvm.array_length(&elements).await?)
    }

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap$HashIterator::next({this:?})");

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .get_field(&this, "java/util/HashMap$HashIterator", "elements", "[Ljava/lang/Object;")
            .await?;
        let index: i32 = jvm.get_field(&this, "java/util/HashMap$HashIterator", "index", "I").await?;
        if index < 0 || index as usize >= jvm.array_length(&elements).await? {
            return Err(jvm.exception("java/util/NoSuchElementException", "HashMap iterator exhausted").await);
        }

        let mut values: Vec<ClassInstanceRef<Object>> = jvm.load_array(&elements, index as usize, 1).await?;
        let Some(element) = values.pop() else {
            return Err(jvm.exception("java/util/NoSuchElementException", "HashMap iterator exhausted").await);
        };
        jvm.put_field(&mut this, "java/util/HashMap$HashIterator", "index", "I", index + 1)
            .await?;
        jvm.put_field(
            &mut this,
            "java/util/HashMap$HashIterator",
            "lastReturned",
            "Ljava/util/HashMap$Entry;",
            element.clone(),
        )
        .await?;

        Ok(element)
    }

    #[method(public, final)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashMap$HashIterator::remove({this:?})");

        let entry: ClassInstanceRef<HashMapEntry> = jvm
            .get_field(&this, "java/util/HashMap$HashIterator", "lastReturned", "Ljava/util/HashMap$Entry;")
            .await?;
        if entry.is_null() {
            return Err(jvm.exception("java/lang/IllegalStateException", "No entry to remove").await);
        }
        let map: Box<dyn ClassInstance> = jvm
            .get_field(&this, "java/util/HashMap$HashIterator", "map", "Ljava/util/HashMap;")
            .await?;
        let key: ClassInstanceRef<Object> = jvm.get_field(&entry, "java/util/HashMap$Entry", "key", "Ljava/lang/Object;").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "java/util/HashMap", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await?;
        let null_entry: ClassInstanceRef<HashMapEntry> = None.into();
        jvm.put_field(
            &mut this,
            "java/util/HashMap$HashIterator",
            "lastReturned",
            "Ljava/util/HashMap$Entry;",
            null_entry,
        )
        .await
    }
}
