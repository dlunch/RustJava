use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::{TreeMap, TreeMapEntry, TreeMapPrivateEntryIterator};

// class java.util.TreeMap$KeyIterator
#[java_class(name = "java/util/TreeMap$KeyIterator", extends = "java/util/TreeMap$PrivateEntryIterator", final)]
pub struct TreeMapKeyIterator;

#[java_methods]
impl TreeMapKeyIterator {
    #[method(constructor)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        map: ClassInstanceRef<TreeMap>,
        next: ClassInstanceRef<TreeMapEntry>,
        upper: ClassInstanceRef<Object>,
        to_end: bool,
    ) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/TreeMap$PrivateEntryIterator",
            "<init>",
            "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
            (map, next, upper, to_end),
        )
        .await
    }

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry = TreeMapPrivateEntryIterator::next_entry(jvm, this).await?;
        jvm.get_field(&entry, "java/util/TreeMap$Entry", "key", "Ljava/lang/Object;").await
    }
}
