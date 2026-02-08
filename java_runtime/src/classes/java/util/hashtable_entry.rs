use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Hashtable$Entry
pub struct HashtableEntry;

impl HashtableEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Hashtable$Entry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "<init>",
                "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/Hashtable$Entry;)V",
                Self::init,
                Default::default(),
            )],
            fields: vec![
                JavaFieldProto::new("hash", "I", Default::default()),
                JavaFieldProto::new("key", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("value", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("next", "Ljava/util/Hashtable$Entry;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        hash: i32,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
        next: ClassInstanceRef<HashtableEntry>,
    ) -> Result<()> {
        tracing::debug!("java.util.Hashtable$Entry::<init>({this:?}, {hash:?}, {key:?}, {value:?}, {next:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "hash", "I", hash).await?;
        jvm.put_field(&mut this, "key", "Ljava/lang/Object;", key).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        jvm.put_field(&mut this, "next", "Ljava/util/Hashtable$Entry;", next).await?;

        Ok(())
    }
}
