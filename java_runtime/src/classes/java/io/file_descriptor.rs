use alloc::{boxed::Box, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

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
                JavaFieldProto::new("fd", "I", Default::default()),
                JavaFieldProto::new("err", "Ljava/io/FileDescriptor;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("in", "Ljava/io/FileDescriptor;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("out", "Ljava/io/FileDescriptor;", FieldAccessFlags::STATIC),
            ],
            access_flags: Default::default(),
        }
    }

    async fn cl_init(jvm: &Jvm, runtime: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.io.FileDescriptor::<clinit>()");

        let stderr_fd = runtime.stderr();
        if let Ok(stderr_fd) = stderr_fd {
            let mut stderr = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
            jvm.put_field(&mut stderr, "fd", "I", stderr_fd.id() as i32).await?;

            jvm.put_static_field("java/io/FileDescriptor", "err", "Ljava/io/FileDescriptor;", stderr)
                .await?;
        }

        let stdin_fd = runtime.stdin();
        if let Ok(stdin_fd) = stdin_fd {
            let mut stdin = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
            jvm.put_field(&mut stdin, "fd", "I", stdin_fd.id() as i32).await?;

            jvm.put_static_field("java/io/FileDescriptor", "in", "Ljava/io/FileDescriptor;", stdin)
                .await?;
        }

        let stdout_fd = runtime.stdout();
        if let Ok(stdout_fd) = stdout_fd {
            let mut stdout = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
            jvm.put_field(&mut stdout, "fd", "I", stdout_fd.id() as i32).await?;

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

    pub async fn from_fd(jvm: &Jvm, fd: crate::FileDescriptorId) -> Result<ClassInstanceRef<Self>> {
        let mut this = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
        jvm.put_field(&mut this, "fd", "I", fd.id() as i32).await?;

        Ok(this.into())
    }

    pub async fn file(jvm: &Jvm, runtime: &RuntimeContext, this: ClassInstanceRef<Self>) -> Result<Box<dyn crate::File>> {
        let fd: i32 = jvm.get_field(&this, "fd", "I").await?;
        if fd <= 0 {
            return Err(jvm.exception("java/io/IOException", "Invalid file descriptor").await);
        }
        let fd = crate::FileDescriptorId::new(fd as u32);
        match runtime.get_file(fd) {
            Ok(file) => Ok(file),
            Err(_) => Err(jvm.exception("java/io/IOException", "Invalid file descriptor").await),
        }
    }

    pub async fn close(jvm: &Jvm, runtime: &RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let fd: i32 = jvm.get_field(&this, "fd", "I").await?;
        if fd > 0 {
            runtime.close_file(crate::FileDescriptorId::new(fd as u32));
        }
        Ok(())
    }
}
