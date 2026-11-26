use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::{LinkedHashMap, LinkedHashMapEntry};

// class java.util.LinkedHashMap$EntryIterator
#[java_class(
    name = "java/util/LinkedHashMap$EntryIterator",
    extends = "java/util/LinkedHashMap$LinkedHashIterator",
    final
)]
pub struct LinkedHashMapEntryIterator;

#[java_methods]
impl LinkedHashMapEntryIterator {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, map: ClassInstanceRef<LinkedHashMap>) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/LinkedHashMap$LinkedHashIterator",
            "<init>",
            "(Ljava/util/LinkedHashMap;)V",
            (map,),
        )
        .await
    }

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry: ClassInstanceRef<LinkedHashMapEntry> = jvm
            .invoke_special(
                &this,
                "java/util/LinkedHashMap$LinkedHashIterator",
                "nextEntry",
                "()Ljava/util/LinkedHashMap$Entry;",
                (),
            )
            .await?;

        Ok(ClassInstanceRef::new(entry.instance))
    }
}
