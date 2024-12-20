use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.io.InputStream
pub struct InputStream;

impl InputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/InputStream",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("available", "()I", Default::default()),
                JavaMethodProto::new_abstract("read", "([BII)I", Default::default()),
                JavaMethodProto::new("read", "([B)I", Self::read, Default::default()),
                JavaMethodProto::new_abstract("read", "()I", Default::default()),
                JavaMethodProto::new_abstract("close", "()V", Default::default()),
                JavaMethodProto::new("skip", "(J)J", Self::skip, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::warn!("stub java.lang.InputStream::<init>({:?})", &this);

        Ok(())
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        tracing::debug!("java.lang.InputStream::read({:?}, {:?})", &this, &b);

        let array_length = jvm.array_length(&b).await? as i32;

        jvm.invoke_virtual(&this, "read", "([BII)I", (b, 0, array_length)).await
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i64) -> Result<i64> {
        tracing::debug!("java.lang.InputStream::skip({:?}, {:?})", &this, n);

        let scratch = jvm.instantiate_array("B", n as _).await?;
        let _: i32 = jvm.invoke_virtual(&this, "read", "([BII)I", (scratch.clone(), 0, n as i32)).await?;

        Ok(n)
    }
}
