use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.io.UTFDataFormatException
#[java_class(name = "java/io/UTFDataFormatException", extends = "java/io/IOException", public)]
pub struct UTFDataFormatException;

#[java_methods]
impl UTFDataFormatException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.UTFDataFormatException::<init>({this:?})");
        jvm.invoke_special(&this, "java/io/IOException", "<init>", "()V", ()).await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.UTFDataFormatException::<init>({this:?}, {message:?})");
        jvm.invoke_special(&this, "java/io/IOException", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
