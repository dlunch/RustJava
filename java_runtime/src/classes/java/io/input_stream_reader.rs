use core::cmp::min;

use alloc::{vec, vec::Vec};

use bytemuck::{cast_slice, cast_vec};
use encoding_rs::{EUC_KR, UTF_8};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

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
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/InputStream;Ljava/lang/String;)V",
                    Self::init_with_charset,
                    Default::default(),
                ),
                JavaMethodProto::new("read", "([CII)I", Self::read, Default::default()),
                JavaMethodProto::new("ready", "()Z", Self::ready, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("in", "Ljava/io/InputStream;", Default::default()),
                JavaFieldProto::new("readBuf", "[B", Default::default()),
                JavaFieldProto::new("readBufSize", "I", Default::default()),
                JavaFieldProto::new("writeBuf", "[C", Default::default()),
                JavaFieldProto::new("writeBufSize", "I", Default::default()),
                JavaFieldProto::new("charset", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("endOfInput", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.InputStreamReader::<init>({this:?}, {:?})", &r#in);

        let charset = System::get_charset(jvm).await?;
        let charset_java = JavaLangString::from_rust_string(jvm, &charset).await?;
        jvm.invoke_special(
            &this,
            "java/io/InputStreamReader",
            "<init>",
            "(Ljava/io/InputStream;Ljava/lang/String;)V",
            (r#in, charset_java),
        )
        .await
    }

    async fn init_with_charset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        r#in: ClassInstanceRef<InputStream>,
        charset: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<()> {
        tracing::debug!("java.io.InputStreamReader::<init>({this:?}, {in:?}, {charset:?})", in = &r#in);

        if r#in.is_null() || charset.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "input or encoding is null").await);
        }

        let charset_name = JavaLangString::to_rust_string(jvm, &charset).await?.to_ascii_uppercase();
        let charset_name = match charset_name.as_str() {
            "UTF-8" | "UTF8" => "UTF-8",
            "EUC-KR" | "EUCKR" | "KS-C-5601-1987" | "MS949" | "CP949" => "EUC-KR",
            _ => return Err(jvm.exception("java/io/UnsupportedEncodingException", &charset_name).await),
        };

        let _: () = jvm.invoke_special(&this, "java/io/Reader", "<init>", "()V", ()).await?;
        let charset = JavaLangString::from_rust_string(jvm, charset_name).await?;
        jvm.put_field(&mut this, "charset", "Ljava/lang/String;", charset).await?;

        let read_buf = jvm.instantiate_array("B", BUF_SIZE).await?;
        jvm.put_field(&mut this, "readBuf", "[B", read_buf).await?;
        jvm.put_field(&mut this, "readBufSize", "I", 0).await?;

        let write_buf = jvm.instantiate_array("C", BUF_SIZE * 3).await?;
        jvm.put_field(&mut this, "writeBuf", "[C", write_buf).await?;
        jvm.put_field(&mut this, "writeBufSize", "I", 0).await?;
        jvm.put_field(&mut this, "endOfInput", "Z", false).await?;

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
        tracing::debug!("java.io.InputStreamReader::read({this:?}, {buf:?}, {offset:?}, {length:?})");

        let destination_length = jvm.array_length(&buf).await? as i32;
        if offset < 0 || length < 0 || offset > destination_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }
        if length == 0 {
            return Ok(0);
        }

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
                } else {
                    jvm.put_field(&mut this, "endOfInput", "Z", true).await?;
                    if read_buf_size == 0 && write_buf_size == 0 {
                        return Ok(-1);
                    }
                }
            }

            let read_buf_size: i32 = jvm.get_field(&this, "readBufSize", "I").await?;
            let mut read_buf_data = vec![0; read_buf_size as _];
            jvm.array_raw_buffer(&read_buf).await?.read(0, &mut read_buf_data)?;

            let charset_ref = jvm.get_field(&this, "charset", "Ljava/lang/String;").await?;
            let charset = JavaLangString::to_rust_string(jvm, &charset_ref).await?;
            let mut decoder = if charset == "UTF-8" {
                UTF_8.new_decoder_without_bom_handling()
            } else if charset == "EUC-KR" {
                EUC_KR.new_decoder_without_bom_handling()
            } else {
                return Err(jvm.exception("java/io/UnsupportedEncodingException", &charset).await);
            };

            let read_buf_data: Vec<u8> = cast_vec(read_buf_data);
            let end_of_input: bool = jvm.get_field(&this, "endOfInput", "Z").await?;
            let mut decode_length = read_buf_data.len();
            if !end_of_input && charset == "UTF-8" && decode_length > 0 {
                let mut lead_index = decode_length - 1;
                while lead_index > 0 && read_buf_data[lead_index] & 0xc0 == 0x80 {
                    lead_index -= 1;
                }
                let expected_length = match read_buf_data[lead_index] {
                    0xc0..=0xdf => 2,
                    0xe0..=0xef => 3,
                    0xf0..=0xf7 => 4,
                    _ => 1,
                };
                if decode_length - lead_index < expected_length {
                    decode_length = lead_index;
                }
            } else if !end_of_input && charset == "EUC-KR" && read_buf_data.last().is_some_and(|value| *value >= 0x81) {
                decode_length -= 1;
            }

            let mut decoded = vec![0; BUF_SIZE * 3];
            let (_, read, wrote, _) = decoder.decode_to_utf16(&read_buf_data[..decode_length], &mut decoded, end_of_input);

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
        tracing::debug!("java.io.InputStreamReader::close({this:?})");

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let _: () = jvm.invoke_virtual(&r#in, "close", "()V", ()).await?;

        Ok(())
    }

    async fn ready(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.InputStreamReader::ready({this:?})");

        let write_buf_size: i32 = jvm.get_field(&this, "writeBufSize", "I").await?;
        if write_buf_size > 0 {
            return Ok(true);
        }

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let available: i32 = jvm.invoke_virtual(&r#in, "available", "()I", ()).await?;
        Ok(available > 0)
    }
}
