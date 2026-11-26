use alloc::boxed::Box;

use jvm::{Array, ClassInstance, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::HashMap;

// class java.util.HashMap$ValueIterator
#[java_class(name = "java/util/HashMap$ValueIterator", extends = "java/util/HashMap$HashIterator", final)]
pub struct HashMapValueIterator;

#[java_methods]
impl HashMapValueIterator {
    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry: Box<dyn ClassInstance> = jvm
            .invoke_special(&this, "java/util/HashMap$HashIterator", "next", "()Ljava/lang/Object;", ())
            .await?;
        jvm.get_field(&entry, "java/util/HashMap$Entry", "value", "Ljava/lang/Object;").await
    }

    #[method(constructor)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        map: ClassInstanceRef<HashMap>,
        elements: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        tracing::debug!("java.util.HashMap$ValueIterator::<init>({this:?}, {elements:?})");

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
