use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.io.InterruptedIOException
#[java_class(name = "java/io/InterruptedIOException", extends = "java/io/IOException", public)]
pub struct InterruptedIOException {
    #[field(public)]
    bytesTransferred: i32,
}

#[java_methods]
impl InterruptedIOException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InterruptedIOException::<init>({this:?})");
        let _: () = jvm.invoke_special(&this, "java/io/IOException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/io/InterruptedIOException", "bytesTransferred", "I", 0)
            .await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.InterruptedIOException::<init>({this:?}, {message:?})");
        let _: () = jvm
            .invoke_special(&this, "java/io/IOException", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;
        jvm.put_field(&mut this, "java/io/InterruptedIOException", "bytesTransferred", "I", 0)
            .await
    }
}
