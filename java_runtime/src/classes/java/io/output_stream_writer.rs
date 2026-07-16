use alloc::{string::String as RustString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::OutputStream,
        lang::{String, System},
    },
};

// class java.io.OutputStreamWriter
pub struct OutputStreamWriter;

impl OutputStreamWriter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/OutputStreamWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                    Self::init_with_encoding,
                    Default::default(),
                ),
                JavaMethodProto::new("write", "([CII)V", Self::write, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("out", "Ljava/io/OutputStream;", Default::default()),
                JavaFieldProto::new("encoding", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("pendingHighSurrogate", "C", Default::default()),
                JavaFieldProto::new("hasPendingHighSurrogate", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.OutputStreamWriter::<init>({this:?}, {out:?})");

        let encoding = System::get_charset(jvm).await?;
        let encoding = JavaLangString::from_rust_string(jvm, &encoding).await?;
        jvm.invoke_special(
            &this,
            "java/io/OutputStreamWriter",
            "<init>",
            "(Ljava/io/OutputStream;Ljava/lang/String;)V",
            (out, encoding),
        )
        .await
    }

    async fn init_with_encoding(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<OutputStream>,
        encoding: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.io.OutputStreamWriter::<init>({this:?}, {out:?}, {encoding:?})");

        if out.is_null() || encoding.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output or encoding is null").await);
        }

        let encoding_name = JavaLangString::to_rust_string(jvm, &encoding).await?.to_ascii_uppercase();
        if !matches!(
            encoding_name.as_str(),
            "UTF-8" | "UTF8" | "EUC-KR" | "EUCKR" | "KS-C-5601-1987" | "MS949" | "CP949"
        ) {
            return Err(jvm.exception("java/io/UnsupportedEncodingException", &encoding_name).await);
        }

        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "out", "Ljava/io/OutputStream;", out).await?;
        jvm.put_field(&mut this, "encoding", "Ljava/lang/String;", encoding).await?;
        jvm.put_field(&mut this, "hasPendingHighSurrogate", "Z", false).await?;

        Ok(())
    }

    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.OutputStreamWriter::write({this:?}, {chars:?}, {off}, {len})");

        let array_length = jvm.array_length(&chars).await? as i32;
        if off < 0 || len < 0 || off > array_length - len {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }

        let mut utf16: Vec<JavaChar> = jvm.load_array(&chars, off as usize, len as usize).await?;
        let has_pending: bool = jvm.get_field(&this, "hasPendingHighSurrogate", "Z").await?;
        if has_pending {
            let pending: JavaChar = jvm.get_field(&this, "pendingHighSurrogate", "C").await?;
            utf16.insert(0, pending);
            jvm.put_field(&mut this, "hasPendingHighSurrogate", "Z", false).await?;
        }
        if utf16.last().is_some_and(|value| (0xd800..=0xdbff).contains(value)) {
            let Some(pending) = utf16.pop() else {
                return Ok(());
            };
            jvm.put_field(&mut this, "pendingHighSurrogate", "C", pending).await?;
            jvm.put_field(&mut this, "hasPendingHighSurrogate", "Z", true).await?;
        }
        if utf16.is_empty() {
            return Ok(());
        }

        let value: RustString = char::decode_utf16(utf16).map(|value| value.unwrap_or('?')).collect();
        let encoding: ClassInstanceRef<String> = jvm.get_field(&this, "encoding", "Ljava/lang/String;").await?;
        let encoding = JavaLangString::to_rust_string(jvm, &encoding).await?.to_ascii_uppercase();
        let bytes = if matches!(encoding.as_str(), "UTF-8" | "UTF8") {
            value.into_bytes()
        } else {
            encoding_rs::EUC_KR.encode(&value).0.into_owned()
        };

        let mut java_bytes = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_array(&mut java_bytes, 0, bytes.into_iter().map(|value| value as i8)).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        jvm.invoke_virtual(&out, "write", "([B)V", (java_bytes,)).await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.OutputStreamWriter::flush({this:?})");
        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        jvm.invoke_virtual(&out, "flush", "()V", ()).await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.OutputStreamWriter::close({this:?})");

        let has_pending: bool = jvm.get_field(&this, "hasPendingHighSurrogate", "Z").await?;
        if has_pending {
            jvm.put_field(&mut this, "hasPendingHighSurrogate", "Z", false).await?;
            let mut replacement = jvm.instantiate_array("C", 1).await?;
            jvm.store_array(&mut replacement, 0, ['?' as JavaChar]).await?;
            let _: () = jvm.invoke_virtual(&this, "write", "([CII)V", (replacement, 0, 1)).await?;
        }

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        jvm.invoke_virtual(&out, "close", "()V", ()).await
    }
}
