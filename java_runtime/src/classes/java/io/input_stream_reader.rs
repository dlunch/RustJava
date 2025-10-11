use core::cmp::min;

use alloc::{sync::Arc, vec};

use bytemuck::{cast_slice, cast_vec};
use encoding_rs::{Decoder, EUC_KR, UTF_8};
use parking_lot::Mutex;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{io::InputStream, lang::System},
};

const BUF_SIZE: usize = 10;

// class java.io.InputStreamReader
pub struct InputStreamReader;

impl InputStreamReader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/InputStreamReader",
            parent_class: Some("java/io/Reader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("read", "([CII)I", Self::read, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("in", "Ljava/io/InputStream;", Default::default()),
                JavaFieldProto::new("readBuf", "[B", Default::default()),
                JavaFieldProto::new("readBufSize", "I", Default::default()),
                JavaFieldProto::new("writeBuf", "[C", Default::default()),
                JavaFieldProto::new("writeBufSize", "I", Default::default()),
                JavaFieldProto::new("decoder", "[B", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.InputStreamReader::<init>({:?}, {:?})", &this, &r#in);

        let _: () = jvm.invoke_special(&this, "java/io/Reader", "<init>", "()V", ()).await?;

        let charset = System::get_charset(jvm).await?;

        let decoder = if charset == "UTF-8" {
            UTF_8.new_decoder()
        } else if charset == "EUC-KR" {
            EUC_KR.new_decoder()
        } else {
            unimplemented!("unsupported charset: {}", charset)
        };

        jvm.put_rust_object_field(&mut this, "decoder", Arc::new(Mutex::new(decoder))).await?;

        let read_buf = jvm.instantiate_array("B", BUF_SIZE).await?;
        jvm.put_field(&mut this, "readBuf", "[B", read_buf).await?;
        jvm.put_field(&mut this, "readBufSize", "I", 0).await?;

        let write_buf = jvm.instantiate_array("C", BUF_SIZE).await?;
        jvm.put_field(&mut this, "writeBuf", "[C", write_buf).await?;
        jvm.put_field(&mut this, "writeBufSize", "I", 0).await?;

        jvm.put_field(&mut this, "in", "Ljava/io/InputStream;", r#in).await?;

        Ok(())
    }

    async fn read(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.InputStreamReader::read({:?}, {:?}, {:?}, {:?})", &this, &buf, &offset, &length);

        let write_buf_size: i32 = jvm.get_field(&this, "writeBufSize", "I").await?;

        if write_buf_size < length {
            let read_buf: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "readBuf", "[B").await?;
            let read_buf_size: i32 = jvm.get_field(&this, "readBufSize", "I").await?;

            if read_buf_size < (BUF_SIZE / 2) as _ {
                let bytes_to_read = BUF_SIZE as i32 - read_buf_size;

                let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

                let temp = jvm.instantiate_array("B", bytes_to_read as _).await?;
                let read: i32 = jvm.invoke_virtual(&r#in, "read", "([BII)I", (temp.clone(), 0, bytes_to_read)).await?;
                if read != -1 {
                    let _: () = jvm
                        .invoke_static(
                            "java/lang/System",
                            "arraycopy",
                            "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                            (temp, 0, read_buf.clone(), read_buf_size, read),
                        )
                        .await?;
                    jvm.put_field(&mut this, "readBufSize", "I", read_buf_size + read).await?;
                } else if read_buf_size == 0 {
                    return Ok(-1);
                }
            }

            let read_buf_size: i32 = jvm.get_field(&this, "readBufSize", "I").await?;
            let mut read_buf_data = vec![0; read_buf_size as _];
            jvm.array_raw_buffer(&read_buf).await?.read(0, &mut read_buf_data).unwrap();

            let decoder: Arc<Mutex<Decoder>> = jvm.get_rust_object_field(&this, "decoder").await?;

            let mut decoded = vec![0; BUF_SIZE * 3];
            let (_, read, wrote, _) = decoder.lock().decode_to_utf16(&cast_vec(read_buf_data), &mut decoded, false);

            // advance readBuf
            let _: () = jvm
                .invoke_static(
                    "java/lang/System",
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    (read_buf.clone(), read as i32, read_buf, 0, (read_buf_size - read as i32)),
                )
                .await?;
            jvm.put_field(&mut this, "readBufSize", "I", read_buf_size - read as i32).await?;

            // add to writeBuf
            let mut write_buf = jvm.get_field(&this, "writeBuf", "[C").await?;
            let write_buf_size: i32 = jvm.get_field(&this, "writeBufSize", "I").await?;
            jvm.store_array(
                &mut write_buf,
                write_buf_size as _,
                cast_slice::<u16, JavaChar>(&decoded[..wrote]).to_vec(),
            )
            .await?;
            jvm.put_field(&mut this, "writeBufSize", "I", write_buf_size + wrote as i32).await?;
        }

        let write_buf: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "writeBuf", "[C").await?;
        let write_buf_size: i32 = jvm.get_field(&this, "writeBufSize", "I").await?;

        let to_copy = min(length, write_buf_size);

        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (write_buf.clone(), 0, buf, offset, to_copy),
            )
            .await?;

        // advance writeBuf
        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (write_buf.clone(), to_copy, write_buf, 0, write_buf_size - to_copy),
            )
            .await?;
        jvm.put_field(&mut this, "writeBufSize", "I", write_buf_size - to_copy).await?;

        Ok(to_copy)
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InputStreamReader::close({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let _: () = jvm.invoke_virtual(&r#in, "close", "()V", ()).await?;

        Ok(())
    }
}
