use alloc::{boxed::Box, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{File, RuntimeClassProto, RuntimeContext};

// class java.io.FileDescriptor
pub struct FileDescriptor;

impl FileDescriptor {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/FileDescriptor",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<clinit>", "()V", Self::cl_init, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("raw", "[B", Default::default()),
                JavaFieldProto::new("err", "Ljava/io/FileDescriptor;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("in", "Ljava/io/FileDescriptor;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("out", "Ljava/io/FileDescriptor;", FieldAccessFlags::STATIC),
            ],
            access_flags: Default::default(),
        }
    }

    async fn cl_init(jvm: &Jvm, runtime: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.io.FileDescriptor::<clinit>()");

        let stderr_file = runtime.stderr();
        if let Ok(stderr_file) = stderr_file {
            let mut stderr = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
            jvm.put_rust_object_field(&mut stderr, "raw", stderr_file).await?;

            jvm.put_static_field("java/io/FileDescriptor", "err", "Ljava/io/FileDescriptor;", stderr)
                .await?;
        }

        let stdin_file = runtime.stdin();
        if let Ok(stdin_file) = stdin_file {
            let mut stdin = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
            jvm.put_rust_object_field(&mut stdin, "raw", stdin_file).await?;

            jvm.put_static_field("java/io/FileDescriptor", "in", "Ljava/io/FileDescriptor;", stdin)
                .await?;
        }

        let stdout_file = runtime.stdout();
        if let Ok(stdout_file) = stdout_file {
            let mut stdout = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
            jvm.put_rust_object_field(&mut stdout, "raw", stdout_file).await?;

            jvm.put_static_field("java/io/FileDescriptor", "out", "Ljava/io/FileDescriptor;", stdout)
                .await?;
        }

        Ok(())
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FileDescriptor::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    pub async fn from_file(jvm: &Jvm, file: Box<dyn File>) -> Result<ClassInstanceRef<Self>> {
        let mut this = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
        jvm.put_rust_object_field(&mut this, "raw", file).await?;

        Ok(this.into())
    }

    pub async fn file(jvm: &Jvm, this: ClassInstanceRef<Self>) -> Result<Box<dyn File>> {
        jvm.get_rust_object_field(&this, "raw").await
    }
}
