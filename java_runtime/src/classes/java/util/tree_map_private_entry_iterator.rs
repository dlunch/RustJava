use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::{TreeMap, TreeMapEntry};

// abstract class java.util.TreeMap$PrivateEntryIterator
pub struct TreeMapPrivateEntryIterator;

impl TreeMapPrivateEntryIterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TreeMap$PrivateEntryIterator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Iterator"],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                    Self::init,
                    MethodAccessFlags::empty(),
                ),
                JavaMethodProto::new("hasNext", "()Z", Self::has_next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new_abstract("next", "()Ljava/lang/Object;", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new("remove", "()V", Self::remove, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("map", "Ljava/util/TreeMap;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("next", "Ljava/util/TreeMap$Entry;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("lastReturned", "Ljava/util/TreeMap$Entry;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("upper", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("toEnd", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

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
        jvm.put_field(&mut this, "map", "Ljava/util/TreeMap;", map).await?;
        jvm.put_field(&mut this, "next", "Ljava/util/TreeMap$Entry;", next).await?;
        jvm.put_field(&mut this, "upper", "Ljava/lang/Object;", upper).await?;
        jvm.put_field(&mut this, "toEnd", "Z", to_end).await
    }

    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let next: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&this, "next", "Ljava/util/TreeMap$Entry;").await?;
        if next.is_null() {
            return Ok(false);
        }
        if jvm.get_field::<bool>(&this, "toEnd", "Z").await? {
            return Ok(true);
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "map", "Ljava/util/TreeMap;").await?;
        let key: ClassInstanceRef<Object> = jvm.get_field(&next, "key", "Ljava/lang/Object;").await?;
        let upper: ClassInstanceRef<Object> = jvm.get_field(&this, "upper", "Ljava/lang/Object;").await?;
        Ok(TreeMap::compare(jvm, &map, &key, &upper).await? < 0)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let last_returned: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&this, "lastReturned", "Ljava/util/TreeMap$Entry;").await?;
        if last_returned.is_null() {
            return Err(jvm.exception("java/lang/IllegalStateException", "iterator state").await);
        }
        let left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&last_returned, "left", "Ljava/util/TreeMap$Entry;").await?;
        let right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&last_returned, "right", "Ljava/util/TreeMap$Entry;").await?;
        if !left.is_null() && !right.is_null() {
            jvm.put_field(&mut this, "next", "Ljava/util/TreeMap$Entry;", last_returned.clone())
                .await?;
        }
        let mut map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "map", "Ljava/util/TreeMap;").await?;
        TreeMap::delete_entry(jvm, &mut map, last_returned).await?;
        jvm.put_field(
            &mut this,
            "lastReturned",
            "Ljava/util/TreeMap$Entry;",
            ClassInstanceRef::<TreeMapEntry>::from(None),
        )
        .await
    }

    pub(super) async fn next_entry<T>(jvm: &Jvm, mut this: ClassInstanceRef<T>) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let next: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&this, "next", "Ljava/util/TreeMap$Entry;").await?;
        if next.is_null() {
            return Err(jvm.exception("java/util/NoSuchElementException", "TreeMap iterator exhausted").await);
        }
        if !jvm.get_field::<bool>(&this, "toEnd", "Z").await? {
            let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "map", "Ljava/util/TreeMap;").await?;
            let key: ClassInstanceRef<Object> = jvm.get_field(&next, "key", "Ljava/lang/Object;").await?;
            let upper: ClassInstanceRef<Object> = jvm.get_field(&this, "upper", "Ljava/lang/Object;").await?;
            if TreeMap::compare(jvm, &map, &key, &upper).await? >= 0 {
                return Err(jvm.exception("java/util/NoSuchElementException", "TreeMap iterator exhausted").await);
            }
        }
        let successor = TreeMap::successor(jvm, next.clone()).await?;
        jvm.put_field(&mut this, "next", "Ljava/util/TreeMap$Entry;", successor).await?;
        jvm.put_field(&mut this, "lastReturned", "Ljava/util/TreeMap$Entry;", next.clone())
            .await?;
        Ok(next)
    }
}
