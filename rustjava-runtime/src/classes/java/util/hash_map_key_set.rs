use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::HashMap;

// class java.util.HashMap$KeySet
#[java_class(name = "java/util/HashMap$KeySet", extends = "java/util/AbstractSet", final)]
pub struct HashMapKeySet {
    #[field(private)]
    map: ClassInstanceRef<HashMap>,
}

#[java_methods]
impl HashMapKeySet {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<HashMap>) -> Result<()> {
        tracing::debug!("java.util.HashMap$KeySet::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/HashMap$KeySet", "map", "Ljava/util/HashMap;", map)
            .await?;

        Ok(())
    }

    #[method(public, final)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.HashMap$KeySet::size({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$KeySet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "size", "()I", ()).await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$KeySet::isEmpty({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$KeySet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "isEmpty", "()Z", ()).await
    }

    #[method(public, final)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$KeySet::contains({this:?}, {key:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$KeySet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "containsKey", "(Ljava/lang/Object;)Z", (key,))
            .await
    }

    #[method(public, final)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$KeySet::remove({this:?}, {key:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$KeySet", "map", "Ljava/util/HashMap;").await?;
        let contains: bool = jvm
            .invoke_virtual(&map, "java/util/HashMap", "containsKey", "(Ljava/lang/Object;)Z", (key.clone(),))
            .await?;
        if !contains {
            return Ok(false);
        }

        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "java/util/HashMap", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await?;

        Ok(true)
    }

    #[method(public, final)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashMap$KeySet::clear({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$KeySet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public, final)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap$KeySet::iterator({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$KeySet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "keyIterator", "()Ljava/util/Iterator;", ())
            .await
    }
}
