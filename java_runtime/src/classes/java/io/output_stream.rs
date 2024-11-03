use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.io.OutputStream
pub struct OutputStream;

impl OutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/OutputStream",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("write", "([B)V", Self::write_bytes, Default::default()),
                JavaMethodProto::new("write", "([BII)V", Self::write_bytes_offset, Default::default()),
                JavaMethodProto::new_abstract("write", "(I)V", Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.OutputStream::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn write_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buffer: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.io.OutputStream::write({:?}, {:?})", &this, &buffer);

        let length = jvm.array_length(&buffer).await?;

        let _: () = jvm.invoke_virtual(&this, "write", "([BII)V", (buffer, 0, length as i32)).await?;

        Ok(())
    }

    async fn write_bytes_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.OutputStream::write({:?}, {:?}, {:?}, {:?})", &this, &buffer, &offset, &length);

        let mut bytes = vec![0; length as usize];
        jvm.array_raw_buffer(&buffer).await?.read(offset as _, &mut bytes)?;
        for byte in bytes {
            let _: () = jvm.invoke_virtual(&this, "write", "(I)V", (byte as i32,)).await?;
        }

        Ok(())
    }
}
