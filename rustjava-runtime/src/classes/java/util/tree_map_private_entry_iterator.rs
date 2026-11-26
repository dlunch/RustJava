use jvm::{AsClassInstance, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::{TreeMap, TreeMapEntry};

// abstract class java.util.TreeMap$PrivateEntryIterator
#[java_class(name = "java/util/TreeMap$PrivateEntryIterator", implements("java/util/Iterator"), abstract)]
pub struct TreeMapPrivateEntryIterator {
    #[field(private)]
    map: ClassInstanceRef<TreeMap>,
    #[field(private)]
    next: ClassInstanceRef<TreeMapEntry>,
    #[field(private)]
    lastReturned: ClassInstanceRef<TreeMapEntry>,
    #[field(private)]
    upper: ClassInstanceRef<Object>,
    #[field(private)]
    toEnd: bool,
}

#[java_methods]
impl TreeMapPrivateEntryIterator {
    #[method(constructor)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        map: ClassInstanceRef<TreeMap>,
        next: ClassInstanceRef<TreeMapEntry>,
        upper: ClassInstanceRef<Object>,
        to_end: bool,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/TreeMap$PrivateEntryIterator", "map", "Ljava/util/TreeMap;", map)
            .await?;
        jvm.put_field(
            &mut this,
            "java/util/TreeMap$PrivateEntryIterator",
            "next",
            "Ljava/util/TreeMap$Entry;",
            next,
        )
        .await?;
        jvm.put_field(&mut this, "java/util/TreeMap$PrivateEntryIterator", "upper", "Ljava/lang/Object;", upper)
            .await?;
        jvm.put_field(&mut this, "java/util/TreeMap$PrivateEntryIterator", "toEnd", "Z", to_end)
            .await
    }

    #[method(public)]
    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let next: ClassInstanceRef<TreeMapEntry> = jvm
            .get_field(&this, "java/util/TreeMap$PrivateEntryIterator", "next", "Ljava/util/TreeMap$Entry;")
            .await?;
        if next.is_null() {
            return Ok(false);
        }
        if jvm
            .get_field::<bool>(&this, "java/util/TreeMap$PrivateEntryIterator", "toEnd", "Z")
            .await?
        {
            return Ok(true);
        }
        let map: ClassInstanceRef<TreeMap> = jvm
            .get_field(&this, "java/util/TreeMap$PrivateEntryIterator", "map", "Ljava/util/TreeMap;")
            .await?;
        let key: ClassInstanceRef<Object> = jvm.get_field(&next, "java/util/TreeMap$Entry", "key", "Ljava/lang/Object;").await?;
        let upper: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/TreeMap$PrivateEntryIterator", "upper", "Ljava/lang/Object;")
            .await?;
        Ok(TreeMap::compare(jvm, &map, &key, &upper).await? < 0)
    }

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>>;

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let last_returned: ClassInstanceRef<TreeMapEntry> = jvm
            .get_field(
                &this,
                "java/util/TreeMap$PrivateEntryIterator",
                "lastReturned",
                "Ljava/util/TreeMap$Entry;",
            )
            .await?;
        if last_returned.is_null() {
            return Err(jvm.exception("java/lang/IllegalStateException", "iterator state").await);
        }
        let left: ClassInstanceRef<TreeMapEntry> = jvm
            .get_field(&last_returned, "java/util/TreeMap$Entry", "left", "Ljava/util/TreeMap$Entry;")
            .await?;
        let right: ClassInstanceRef<TreeMapEntry> = jvm
            .get_field(&last_returned, "java/util/TreeMap$Entry", "right", "Ljava/util/TreeMap$Entry;")
            .await?;
        if !left.is_null() && !right.is_null() {
            jvm.put_field(
                &mut this,
                "java/util/TreeMap$PrivateEntryIterator",
                "next",
                "Ljava/util/TreeMap$Entry;",
                last_returned.clone(),
            )
            .await?;
        }
        let mut map: ClassInstanceRef<TreeMap> = jvm
            .get_field(&this, "java/util/TreeMap$PrivateEntryIterator", "map", "Ljava/util/TreeMap;")
            .await?;
        TreeMap::delete_entry(jvm, &mut map, last_returned).await?;
        jvm.put_field(
            &mut this,
            "java/util/TreeMap$PrivateEntryIterator",
            "lastReturned",
            "Ljava/util/TreeMap$Entry;",
            ClassInstanceRef::<TreeMapEntry>::from(None),
        )
        .await
    }

    pub(super) async fn next_entry<T>(jvm: &Jvm, mut this: ClassInstanceRef<T>) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let next: ClassInstanceRef<TreeMapEntry> = jvm
            .get_field(
                this.as_class_instance(),
                "java/util/TreeMap$PrivateEntryIterator",
                "next",
                "Ljava/util/TreeMap$Entry;",
            )
            .await?;
        if next.is_null() {
            return Err(jvm.exception("java/util/NoSuchElementException", "TreeMap iterator exhausted").await);
        }
        if !jvm
            .get_field::<bool>(this.as_class_instance(), "java/util/TreeMap$PrivateEntryIterator", "toEnd", "Z")
            .await?
        {
            let map: ClassInstanceRef<TreeMap> = jvm
                .get_field(
                    this.as_class_instance(),
                    "java/util/TreeMap$PrivateEntryIterator",
                    "map",
                    "Ljava/util/TreeMap;",
                )
                .await?;
            let key: ClassInstanceRef<Object> = jvm.get_field(&next, "java/util/TreeMap$Entry", "key", "Ljava/lang/Object;").await?;
            let upper: ClassInstanceRef<Object> = jvm
                .get_field(
                    this.as_class_instance(),
                    "java/util/TreeMap$PrivateEntryIterator",
                    "upper",
                    "Ljava/lang/Object;",
                )
                .await?;
            if TreeMap::compare(jvm, &map, &key, &upper).await? >= 0 {
                return Err(jvm.exception("java/util/NoSuchElementException", "TreeMap iterator exhausted").await);
            }
        }
        let successor = TreeMap::successor(jvm, next.clone()).await?;
        jvm.put_field(
            &mut this,
            "java/util/TreeMap$PrivateEntryIterator",
            "next",
            "Ljava/util/TreeMap$Entry;",
            successor,
        )
        .await?;
        jvm.put_field(
            &mut this,
            "java/util/TreeMap$PrivateEntryIterator",
            "lastReturned",
            "Ljava/util/TreeMap$Entry;",
            next.clone(),
        )
        .await?;
        Ok(next)
    }
}
