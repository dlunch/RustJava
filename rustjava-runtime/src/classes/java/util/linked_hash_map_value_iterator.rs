use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::{LinkedHashMap, LinkedHashMapEntry};

// class java.util.LinkedHashMap$ValueIterator
#[java_class(
    name = "java/util/LinkedHashMap$ValueIterator",
    extends = "java/util/LinkedHashMap$LinkedHashIterator",
    final
)]
pub struct LinkedHashMapValueIterator;

#[java_methods]
impl LinkedHashMapValueIterator {
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

        jvm.get_field(&entry, "java/util/LinkedHashMap$Entry", "value", "Ljava/lang/Object;")
            .await
    }
}
