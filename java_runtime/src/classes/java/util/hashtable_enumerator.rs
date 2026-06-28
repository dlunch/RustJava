use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Hashtable$Enumerator
pub struct HashtableEnumerator;

impl HashtableEnumerator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Hashtable$Enumerator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Enumeration", "java/util/Iterator"],
            methods: vec![
                JavaMethodProto::new("<init>", "([Ljava/lang/Object;)V", Self::init, Default::default()),
                JavaMethodProto::new("hasMoreElements", "()Z", Self::has_more_elements, Default::default()),
                JavaMethodProto::new("nextElement", "()Ljava/lang/Object;", Self::next_element, Default::default()),
                JavaMethodProto::new("hasNext", "()Z", Self::has_next, Default::default()),
                JavaMethodProto::new("next", "()Ljava/lang/Object;", Self::next, Default::default()),
                JavaMethodProto::new("remove", "()V", Self::remove, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("elements", "[Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("index", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, elements: ClassInstanceRef<Array<Object>>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$Enumerator::<init>({this:?}, {elements:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "elements", "[Ljava/lang/Object;", elements).await?;
        jvm.put_field(&mut this, "index", "I", 0).await?;

        Ok(())
    }

    async fn has_more_elements(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Self::has_next(jvm, context, this).await
    }

    async fn next_element(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Self::next(jvm, context, this).await
    }

    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$Enumerator::hasNext({this:?})");

        let elements: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elements", "[Ljava/lang/Object;").await?;
        let index: i32 = jvm.get_field(&this, "index", "I").await?;
        if index < 0 {
            return Ok(false);
        }

        Ok((index as usize) < jvm.array_length(&elements).await?)
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable$Enumerator::next({this:?})");

        let elements: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elements", "[Ljava/lang/Object;").await?;
        let index: i32 = jvm.get_field(&this, "index", "I").await?;
        if index < 0 || index as usize >= jvm.array_length(&elements).await? {
            return Err(jvm.exception("java/util/NoSuchElementException", "Hashtable iterator exhausted").await);
        }

        let mut values = jvm.load_array(&elements, index as usize, 1).await?;
        let Some(element) = values.pop() else {
            return Err(jvm.exception("java/util/NoSuchElementException", "Hashtable iterator exhausted").await);
        };
        jvm.put_field(&mut this, "index", "I", index + 1).await?;

        Ok(element)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$Enumerator::remove({this:?})");

        Err(jvm.exception("java/lang/UnsupportedOperationException", "Iterator.remove").await)
    }
}
