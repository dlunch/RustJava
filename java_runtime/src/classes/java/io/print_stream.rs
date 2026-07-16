use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};
use core::char;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::OutputStream,
        lang::{Object, String},
    },
};

// class java.io.PrintStream
pub struct PrintStream;

impl PrintStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/PrintStream",
            parent_class: Some("java/io/FilterOutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("checkError", "()Z", Self::check_error, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write_byte, Default::default()),
                JavaMethodProto::new("write", "([BII)V", Self::write_bytes, Default::default()),
                JavaMethodProto::new("print", "(Ljava/lang/Object;)V", Self::print_object, Default::default()),
                JavaMethodProto::new("print", "(Ljava/lang/String;)V", Self::print_string, Default::default()),
                JavaMethodProto::new("print", "(I)V", Self::print_int, Default::default()),
                JavaMethodProto::new("print", "(J)V", Self::print_long, Default::default()),
                JavaMethodProto::new("print", "(C)V", Self::print_char, Default::default()),
                JavaMethodProto::new("print", "([C)V", Self::print_chars, Default::default()),
                JavaMethodProto::new("print", "(Z)V", Self::print_bool, Default::default()),
                JavaMethodProto::new("print", "(F)V", Self::print_float, Default::default()),
                JavaMethodProto::new("print", "(D)V", Self::print_double, Default::default()),
                JavaMethodProto::new("println", "()V", Self::println, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/Object;)V", Self::println_object, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println_string, Default::default()),
                JavaMethodProto::new("println", "(I)V", Self::println_int, Default::default()),
                JavaMethodProto::new("println", "(J)V", Self::println_long, Default::default()),
                JavaMethodProto::new("println", "(C)V", Self::println_char, Default::default()),
                JavaMethodProto::new("println", "([C)V", Self::println_chars, Default::default()),
                JavaMethodProto::new("println", "(B)V", Self::println_byte, Default::default()),
                JavaMethodProto::new("println", "(S)V", Self::println_short, Default::default()),
                JavaMethodProto::new("println", "(Z)V", Self::println_bool, Default::default()),
                JavaMethodProto::new("println", "(F)V", Self::println_float, Default::default()),
                JavaMethodProto::new("println", "(D)V", Self::println_double, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("trouble", "Z", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::<init>({this:?}, {out:?})");

        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output is null").await);
        }

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterOutputStream", "<init>", "(Ljava/io/OutputStream;)V", (out,))
            .await?;
        jvm.put_field(&mut this, "trouble", "Z", false).await
    }

    async fn check_error(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.PrintStream::checkError({this:?})");
        let _: () = jvm.invoke_virtual(&this, "flush", "()V", ()).await?;
        jvm.get_field(&this, "trouble", "Z").await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::close({this:?})");
        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        if jvm.invoke_virtual::<_, ()>(&out, "close", "()V", ()).await.is_err() {
            jvm.put_field(&mut this, "trouble", "Z", true).await?;
        }
        Ok(())
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::flush({this:?})");
        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        if jvm.invoke_virtual::<_, ()>(&out, "flush", "()V", ()).await.is_err() {
            jvm.put_field(&mut this, "trouble", "Z", true).await?;
        }
        Ok(())
    }

    async fn write_byte(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.PrintStream::write({this:?}, {value})");
        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        if jvm.invoke_virtual::<_, ()>(&out, "write", "(I)V", (value,)).await.is_err() {
            jvm.put_field(&mut this, "trouble", "Z", true).await?;
        }
        Ok(())
    }

    async fn write_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        bytes: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.PrintStream::write({this:?}, {bytes:?}, {off}, {len})");
        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        if jvm.invoke_virtual::<_, ()>(&out, "write", "([BII)V", (bytes, off, len)).await.is_err() {
            jvm.put_field(&mut this, "trouble", "Z", true).await?;
        }
        Ok(())
    }

    async fn print_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::print({this:?}, {value:?})");
        if value.is_null() {
            return Self::write_text(jvm, &this, "null").await;
        }

        let value: ClassInstanceRef<String> = jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?;
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        Self::write_text(jvm, &this, &value).await
    }

    async fn print_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::print({this:?}, {value:?})");
        if value.is_null() {
            Self::write_text(jvm, &this, "null").await
        } else {
            let value = JavaLangString::to_rust_string(jvm, &value).await?;
            Self::write_text(jvm, &this, &value).await
        }
    }

    async fn print_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        Self::write_text(jvm, &this, &format!("{value}")).await
    }

    async fn print_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        Self::write_text(jvm, &this, &format!("{value}")).await
    }

    async fn print_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        let value = char::from_u32(value as u32).unwrap_or('?');
        Self::write_text(jvm, &this, &value.to_string()).await
    }

    async fn print_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        let length = jvm.array_length(&value).await?;
        let value: Vec<JavaChar> = jvm.load_array(&value, 0, length).await?;
        let value: RustString = char::decode_utf16(value).map(|value| value.unwrap_or('?')).collect();
        Self::write_text(jvm, &this, &value).await
    }

    async fn print_bool(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        Self::write_text(jvm, &this, if value { "true" } else { "false" }).await
    }

    async fn print_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,))
            .await?;
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        Self::write_text(jvm, &this, &value).await
    }

    async fn print_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,))
            .await?;
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        Self::write_text(jvm, &this, &value).await
    }

    async fn println(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        Self::write_text(jvm, &this, "\n").await
    }

    async fn println_object(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<()> {
        Self::print_object(jvm, context, this.clone(), value).await?;
        Self::write_text(jvm, &this, "\n").await
    }

    async fn println_string(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        Self::print_string(jvm, context, this.clone(), value).await?;
        Self::write_text(jvm, &this, "\n").await
    }

    async fn println_int(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        Self::print_int(jvm, context, this.clone(), value).await?;
        Self::write_text(jvm, &this, "\n").await
    }

    async fn println_long(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        Self::print_long(jvm, context, this.clone(), value).await?;
        Self::write_text(jvm, &this, "\n").await
    }

    async fn println_char(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        Self::print_char(jvm, context, this.clone(), value).await?;
        Self::write_text(jvm, &this, "\n").await
    }

    async fn println_chars(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<()> {
        Self::print_chars(jvm, context, this.clone(), value).await?;
        Self::write_text(jvm, &this, "\n").await
    }

    async fn println_byte(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i8) -> Result<()> {
        Self::println_int(jvm, context, this, value as i32).await
    }

    async fn println_short(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i16) -> Result<()> {
        Self::println_int(jvm, context, this, value as i32).await
    }

    async fn println_bool(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        Self::print_bool(jvm, context, this.clone(), value).await?;
        Self::write_text(jvm, &this, "\n").await
    }

    async fn println_float(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        Self::print_float(jvm, context, this.clone(), value).await?;
        Self::write_text(jvm, &this, "\n").await
    }

    async fn println_double(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        Self::print_double(jvm, context, this.clone(), value).await?;
        Self::write_text(jvm, &this, "\n").await
    }

    async fn write_text(jvm: &Jvm, this: &ClassInstanceRef<Self>, value: &str) -> Result<()> {
        let bytes = value.as_bytes();
        let mut java_bytes = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_array(&mut java_bytes, 0, bytes.iter().map(|value| *value as i8)).await?;
        jvm.invoke_virtual(this, "write", "([BII)V", (java_bytes, 0, bytes.len() as i32)).await
    }
}
