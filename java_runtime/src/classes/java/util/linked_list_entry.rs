use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.LinkedList$Entry
pub struct LinkedListEntry;

impl LinkedListEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/LinkedList$Entry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "<init>",
                "(Ljava/lang/Object;Ljava/util/LinkedList$Entry;Ljava/util/LinkedList$Entry;)V",
                Self::init,
                Default::default(),
            )],
            fields: vec![
                JavaFieldProto::new("element", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("next", "Ljava/util/LinkedList$Entry;", Default::default()),
                JavaFieldProto::new("previous", "Ljava/util/LinkedList$Entry;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        next: ClassInstanceRef<Self>,
        previous: ClassInstanceRef<Self>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "element", "Ljava/lang/Object;", element).await?;
        jvm.put_field(&mut this, "next", "Ljava/util/LinkedList$Entry;", next).await?;
        jvm.put_field(&mut this, "previous", "Ljava/util/LinkedList$Entry;", previous).await
    }
}
