use alloc::boxed::Box;

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// class java.io.FileDescriptor
#[java_class(name = "java/io/FileDescriptor", extends = "java/lang/Object", public, final)]
pub struct FileDescriptor {
    #[field(private)]
    fd: i32,
    #[field(static)]
    err: ClassInstanceRef<Self>,
    #[field(static)]
    r#in: ClassInstanceRef<Self>,
    #[field(static)]
    out: ClassInstanceRef<Self>,
}

#[java_methods]
impl FileDescriptor {
    #[method(static_initializer)]
    async fn cl_init(jvm: &Jvm, runtime: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.io.FileDescriptor::<clinit>()");

        for (name, host_descriptor) in [("err", runtime.stderr()), ("in", runtime.stdin()), ("out", runtime.stdout())] {
            let mut descriptor = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
            if let Ok(host_descriptor) = host_descriptor {
                jvm.put_field(&mut descriptor, "java/io/FileDescriptor", "fd", "I", host_descriptor.id() as i32)
                    .await?;
            }
            jvm.put_static_field("java/io/FileDescriptor", name, "Ljava/io/FileDescriptor;", descriptor)
                .await?;
        }

        Ok(())
    }

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FileDescriptor::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    pub async fn from_fd(jvm: &Jvm, fd: crate::FileDescriptorId) -> Result<ClassInstanceRef<Self>> {
        let mut this = jvm.new_class("java/io/FileDescriptor", "()V", []).await?;
        jvm.put_field(&mut this, "java/io/FileDescriptor", "fd", "I", fd.id() as i32).await?;

        Ok(this.into())
    }

    pub async fn file(jvm: &Jvm, runtime: &RuntimeContext, this: ClassInstanceRef<Self>) -> Result<Box<dyn crate::File>> {
        let fd: i32 = jvm.get_field(&this, "java/io/FileDescriptor", "fd", "I").await?;
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
        let fd: i32 = jvm.get_field(&this, "java/io/FileDescriptor", "fd", "I").await?;
        if fd > 0 {
            runtime.close_file(crate::FileDescriptorId::new(fd as u32));
        }
        Ok(())
    }
}
