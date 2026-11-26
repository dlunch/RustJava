use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::Hashtable;

// class java.util.Hashtable$KeySet
#[java_class(name = "java/util/Hashtable$KeySet", extends = "java/util/AbstractSet")]
pub struct HashtableKeySet {
    #[field(private)]
    map: ClassInstanceRef<Hashtable>,
}

#[java_methods]
impl HashtableKeySet {
    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Hashtable>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$KeySet::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/Hashtable$KeySet", "map", "Ljava/util/Hashtable;", map)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Hashtable$KeySet::size({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$KeySet", "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "size", "()I", ()).await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$KeySet::isEmpty({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$KeySet", "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "isEmpty", "()Z", ()).await
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$KeySet::contains({this:?}, {key:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$KeySet", "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "containsKey", "(Ljava/lang/Object;)Z", (key,))
            .await
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$KeySet::remove({this:?}, {key:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$KeySet", "map", "Ljava/util/Hashtable;").await?;
        let contains: bool = jvm
            .invoke_virtual(&map, "java/util/Hashtable", "containsKey", "(Ljava/lang/Object;)Z", (key.clone(),))
            .await?;
        if !contains {
            return Ok(false);
        }

        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "java/util/Hashtable", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await?;

        Ok(true)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$KeySet::clear({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$KeySet", "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable$KeySet::iterator({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$KeySet", "map", "Ljava/util/Hashtable;").await?;
        let snapshot = Hashtable::keys_snapshot(jvm, &map).await?;
        let iterator = jvm
            .new_class("java/util/Hashtable$Enumerator", "([Ljava/lang/Object;)V", (snapshot,))
            .await?;

        Ok(iterator.into())
    }
}
