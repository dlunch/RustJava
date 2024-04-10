use alloc::{boxed::Box, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::FieldAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{File, RuntimeClassProto, RuntimeContext};

// class java.io.FileDescriptor
pub struct FileDescriptor {}

impl FileDescriptor {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<clinit>", "()V", Self::cl_init, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("raw", "[B", Default::default()),
                JavaFieldProto::new("err", "Ljava/io/FileDescriptor;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("in", "Ljava/io/FileDescriptor;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("out", "Ljava/io/FileDescriptor;", FieldAccessFlags::STATIC),
            ],
        }
    }

    async fn cl_init(jvm: &Jvm, runtime: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.io.FileDescriptor::<clinit>()");

        let mut stderr = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
        jvm.put_rust_object_field(&mut stderr, "raw", runtime.stderr().unwrap()).await?;

        let mut stdin = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
        jvm.put_rust_object_field(&mut stdin, "raw", runtime.stdin().unwrap()).await?;

        let mut stdout = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
        jvm.put_rust_object_field(&mut stdout, "raw", runtime.stdout().unwrap()).await?;

        jvm.put_static_field("java/io/FileDescriptor", "err", "Ljava/io/FileDescriptor;", stderr)
            .await?;
        jvm.put_static_field("java/io/FileDescriptor", "in", "Ljava/io/FileDescriptor;", stdin)
            .await?;
        jvm.put_static_field("java/io/FileDescriptor", "out", "Ljava/io/FileDescriptor;", stdout)
            .await?;

        Ok(())
    }

    async fn init(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FileDescriptor::<init>({:?})", &this);

        Ok(())
    }

    pub async fn file(jvm: &Jvm, this: ClassInstanceRef<Self>) -> Result<Box<dyn File>> {
        jvm.get_rust_object_field(&this, "raw").await
    }
}
