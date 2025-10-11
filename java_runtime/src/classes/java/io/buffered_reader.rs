use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{io::Reader, lang::String},
};

// class java.io.BufferedReader

const BUF_SIZE: usize = 1024;
pub struct BufferedReader;

impl BufferedReader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/BufferedReader",
            parent_class: Some("java/io/Reader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/Reader;)V", Self::init, Default::default()),
                JavaMethodProto::new("readLine", "()Ljava/lang/String;", Self::read_line, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("in", "Ljava/io/Reader;", Default::default()),
                JavaFieldProto::new("buf", "[C", Default::default()),
                JavaFieldProto::new("bufSize", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<Reader>) -> Result<()> {
        tracing::debug!("java.io.BufferedReader::<init>({:?}, {:?})", &this, &r#in);

        let _: () = jvm.invoke_special(&this, "java/io/Reader", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "in", "Ljava/io/Reader;", r#in).await?;

        let buf = jvm.instantiate_array("C", BUF_SIZE).await?;
        jvm.put_field(&mut this, "buf", "[C", buf).await?;
        jvm.put_field(&mut this, "bufSize", "I", 0).await?;

        Ok(())
    }

    async fn read_line(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.BufferedReader::readLine({:?})", &this);

        let buf: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;

        // fill buf
        let mut buf_size: i32 = jvm.get_field(&this, "bufSize", "I").await?;
        let mut pos = None;
        while pos.is_none() {
            let r#in = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
            let read: i32 = jvm
                .invoke_virtual(&r#in, "read", "([CII)I", (buf.clone(), buf_size, (BUF_SIZE as i32) - buf_size))
                .await?;
            if read == -1 {
                break;
            }

            buf_size += read;

            jvm.put_field(&mut this, "bufSize", "I", buf_size).await?;

            let char_buf: Vec<JavaChar> = jvm.load_array(&buf, 0, buf_size as _).await?;
            pos = char_buf.iter().position(|&c| c == b'\n' as _);
        }

        // can't fill buffer
        if buf_size == 0 {
            return Ok(None.into());
        }

        Ok(if let Some(x) = pos {
            // found newline
            let result = jvm.new_class("java/lang/String", "([CII)V", (buf.clone(), 0, x as i32)).await?;

            // advance buffer
            let _: () = jvm
                .invoke_static(
                    "java/lang/System",
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    (buf.clone(), (x + 1) as i32, buf, 0, buf_size - x as i32),
                )
                .await?;
            jvm.put_field(&mut this, "bufSize", "I", buf_size - x as i32 - 1).await?;

            result.into()
        } else {
            // end of stream, and no newline
            jvm.put_field(&mut this, "bufSize", "I", 0).await?;

            let result = jvm.new_class("java/lang/String", "([CII)V", (buf, 0, buf_size)).await?;
            result.into()
        })
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.BufferedReader::close({this:?})");

        let r#in = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
        let _: () = jvm.invoke_virtual(&r#in, "close", "()V", ()).await?;

        Ok(())
    }
}
