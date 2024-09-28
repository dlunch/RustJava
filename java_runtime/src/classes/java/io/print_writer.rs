use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    classes::java::{io::Writer, lang::String},
    RuntimeClassProto, RuntimeContext,
};

// class java.io.PrintWriter
pub struct PrintWriter;

impl PrintWriter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/PrintWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/Writer;)V", Self::init, Default::default()),
                JavaMethodProto::new("write", "([CII)I", Self::write, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("out", "Ljava/io/Writer;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, out: ClassInstanceRef<Writer>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "out", "Ljava/io/Writer;", out).await?;

        Ok(())
    }

    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.PrintWriter::write({:?}, {:?}, {:?}, {:?})", &this, &chars, &off, &len);

        let out = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;

        let _: i32 = jvm.invoke_virtual(&out, "write", "([CII)I", (chars, off, len)).await?;

        Ok(len)
    }

    async fn println(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::println({:?}, {:?})", &this, &string);

        let string = format!("{}\n", JavaLangString::to_rust_string(jvm, &string).await?);
        let string = JavaLangString::from_rust_string(jvm, &string).await?;

        let _: () = jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;)V", (string,)).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

    use crate::{classes::java::lang::String, test::test_jvm};

    #[tokio::test]
    async fn test_print_writer() -> Result<()> {
        let jvm = test_jvm().await?;

        let sw = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
        let pw = jvm.new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (sw.clone(),)).await?;

        let hello = JavaLangString::from_rust_string(&jvm, "hello").await?;
        let world = JavaLangString::from_rust_string(&jvm, "world").await?;

        let _: () = jvm.invoke_virtual(&pw, "println", "(Ljava/lang/String;)V", (hello,)).await?;
        let _: () = jvm.invoke_virtual(&pw, "println", "(Ljava/lang/String;)V", (world,)).await?;

        let result: ClassInstanceRef<String> = jvm.invoke_virtual(&sw, "toString", "()Ljava/lang/String;", ()).await?;
        let result = JavaLangString::to_rust_string(&jvm, &result).await?;

        assert_eq!(result, "hello\nworld\n");

        Ok(())
    }
}
