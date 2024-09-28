use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    classes::java::lang::{String, StringBuffer},
    RuntimeClassProto, RuntimeContext,
};

// class java.io.StringWriter
pub struct StringWriter;

impl StringWriter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/StringWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("write", "([CII)I", Self::write, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("buf", "Ljava/lang/StringBuffer;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.StringWriter::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;

        let buf = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
        jvm.put_field(&mut this, "buf", "Ljava/lang/StringBuffer;", buf).await?;

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
        tracing::debug!("java.io.StringWriter::write({:?}, {:?}, {:?}, {:?})", &this, &chars, &off, &len);

        let buf = jvm.get_field(&this, "buf", "Ljava/lang/StringBuffer;").await?;

        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(&buf, "append", "([CII)Ljava/lang/StringBuffer;", (chars, off, len))
            .await?;

        Ok(len)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.StringWriter::to_string({:?})", &this);

        let buf = jvm.get_field(&this, "buf", "Ljava/lang/StringBuffer;").await?;

        let string = jvm.invoke_virtual(&buf, "toString", "()Ljava/lang/String;", ()).await?;

        Ok(string)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use jvm::{runtime::JavaLangString, JavaChar, Result};

    use crate::test::test_jvm;

    #[tokio::test]
    async fn test_string_writer() -> Result<()> {
        let jvm = test_jvm().await?;

        let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await.unwrap();

        let mut buf = jvm.instantiate_array("C", 3).await?;

        jvm.store_array(&mut buf, 0, vec![b'a' as JavaChar, b'b' as JavaChar, b'c' as JavaChar])
            .await?;

        let _: i32 = jvm.invoke_virtual(&string_writer, "write", "([CII)I", (buf.clone(), 0, 3)).await.unwrap();

        let _: i32 = jvm.invoke_virtual(&string_writer, "write", "([CII)I", (buf, 1, 2)).await.unwrap();

        let string = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", ()).await.unwrap();

        let string = JavaLangString::to_rust_string(&jvm, &string).await?;

        assert_eq!(string, "abcbc"); // cspell: disable-line

        Ok(())
    }
}
