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
                JavaMethodProto::new("mark", "(I)V", Self::mark, Default::default()),
                JavaMethodProto::new("reset", "()V", Self::reset, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InputStream::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        tracing::debug!("java.io.InputStream::read({:?}, {:?})", &this, &b);

        let array_length = jvm.array_length(&b).await? as i32;

        jvm.invoke_virtual(&this, "read", "([BII)I", (b, 0, array_length)).await
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i64) -> Result<i64> {
        tracing::debug!("java.io.InputStream::skip({:?}, {:?})", &this, n);

        let scratch = jvm.instantiate_array("B", n as _).await?;
        let _: i32 = jvm.invoke_virtual(&this, "read", "([BII)I", (scratch.clone(), 0, n as i32)).await?;

        Ok(n)
    }

    async fn mark(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, readlimit: i32) -> Result<()> {
        tracing::debug!("java.io.InputStream::mark({:?}, {:?})", &this, readlimit);

        Ok(())
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InputStream::reset({:?})", &this);

        Err(jvm.exception("java/io/IOException", "reset not supported").await)
    }
}
