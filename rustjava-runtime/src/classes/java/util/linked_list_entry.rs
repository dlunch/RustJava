use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.util.LinkedList$Entry
#[java_class(name = "java/util/LinkedList$Entry")]
pub struct LinkedListEntry {
    element: ClassInstanceRef<Object>,
    next: ClassInstanceRef<Self>,
    previous: ClassInstanceRef<Self>,
}

#[java_methods]
impl LinkedListEntry {
    #[method(constructor)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        next: ClassInstanceRef<Self>,
        previous: ClassInstanceRef<Self>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/LinkedList$Entry", "element", "Ljava/lang/Object;", element)
            .await?;
        jvm.put_field(&mut this, "java/util/LinkedList$Entry", "next", "Ljava/util/LinkedList$Entry;", next)
            .await?;
        jvm.put_field(
            &mut this,
            "java/util/LinkedList$Entry",
            "previous",
            "Ljava/util/LinkedList$Entry;",
            previous,
        )
        .await
    }
}
