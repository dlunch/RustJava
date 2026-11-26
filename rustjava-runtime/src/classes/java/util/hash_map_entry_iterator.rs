use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::HashMap;

// class java.util.HashMap$EntryIterator
#[java_class(name = "java/util/HashMap$EntryIterator", extends = "java/util/HashMap$HashIterator", final)]
pub struct HashMapEntryIterator;

#[java_methods]
impl HashMapEntryIterator {
    #[method(constructor)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        map: ClassInstanceRef<HashMap>,
        elements: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        tracing::debug!("java.util.HashMap$EntryIterator::<init>({this:?}, {elements:?})");

        jvm.invoke_special(
            &this,
            "java/util/HashMap$HashIterator",
            "<init>",
            "(Ljava/util/HashMap;[Ljava/lang/Object;)V",
            (map, elements),
        )
        .await
    }
}
