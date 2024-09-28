use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.io.Reader
pub struct Reader;

impl Reader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/Reader",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("read", "([C)I", Self::read, Default::default()),
                JavaMethodProto::new_abstract("read", "([CII)I", Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.Reader::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buf: ClassInstanceRef<JavaChar>) -> Result<i32> {
        tracing::debug!("java.io.Reader::read({:?}, {:?})", &this, &buf);

        let len = jvm.array_length(&buf).await? as i32;
        let result = jvm.invoke_virtual(&this, "read", "([CII)I", (buf, 0, len)).await?;

        Ok(result)
    }
}
