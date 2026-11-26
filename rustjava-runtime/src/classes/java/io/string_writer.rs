use alloc::boxed::Box;

use jvm::{Array, ClassInstance, ClassInstanceRef, JavaChar, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{String, StringBuffer},
};

// class java.io.StringWriter
#[java_class(name = "java/io/StringWriter", extends = "java/io/Writer", public)]
pub struct StringWriter {
    #[field(private, final)]
    buf: ClassInstanceRef<StringBuffer>,
}

#[java_methods]
impl StringWriter {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.StringWriter::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;

        let buf = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
        jvm.put_field(&mut this, "java/io/StringWriter", "buf", "Ljava/lang/StringBuffer;", buf)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.StringWriter::write({this:?}, {chars:?}, {off:?}, {len:?})");

        let buf: Box<dyn ClassInstance> = jvm.get_field(&this, "java/io/StringWriter", "buf", "Ljava/lang/StringBuffer;").await?;

        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &buf,
                "java/lang/StringBuffer",
                "append",
                "([CII)Ljava/lang/StringBuffer;",
                (chars, off, len),
            )
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn flush(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.StringWriter::flush({this:?})");
        Ok(())
    }

    #[method(public)]
    async fn close(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.StringWriter::close({this:?})");
        Ok(())
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.StringWriter::toString({this:?})");

        let buf: Box<dyn ClassInstance> = jvm.get_field(&this, "java/io/StringWriter", "buf", "Ljava/lang/StringBuffer;").await?;

        let string = jvm
            .invoke_virtual(&buf, "java/lang/StringBuffer", "toString", "()Ljava/lang/String;", ())
            .await?;

        Ok(string)
    }
}
