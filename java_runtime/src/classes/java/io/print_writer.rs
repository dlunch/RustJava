use core::future::Future;

use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{OutputStream, Writer},
        lang::{Object, String},
    },
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
                JavaMethodProto::new("<init>", "(Ljava/io/Writer;)V", Self::init_writer, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/io/Writer;Z)V", Self::init_writer_auto_flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init_output_stream, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/OutputStream;Z)V",
                    Self::init_output_stream_auto_flush,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("write", "(I)V", Self::write_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([C)V", Self::write_chars, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([CII)V", Self::write_chars_range, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(Ljava/lang/String;)V", Self::write_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(Ljava/lang/String;II)V", Self::write_string_range, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(Z)V", Self::print_boolean, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(C)V", Self::print_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(I)V", Self::print_int, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(J)V", Self::print_long, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(F)V", Self::print_float, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(D)V", Self::print_double, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "([C)V", Self::print_chars, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(Ljava/lang/String;)V", Self::print_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(Ljava/lang/Object;)V", Self::print_object, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "()V", Self::println, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(Z)V", Self::println_boolean, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(C)V", Self::println_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(I)V", Self::println_int, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(J)V", Self::println_long, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(F)V", Self::println_float, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(D)V", Self::println_double, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "([C)V", Self::println_chars, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(Ljava/lang/Object;)V", Self::println_object, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("checkError", "()Z", Self::check_error, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("out", "Ljava/io/Writer;", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("autoFlush", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("trouble", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init_writer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<Writer>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::<init>({this:?}, {out:?})");
        jvm.invoke_special(&this, "java/io/PrintWriter", "<init>", "(Ljava/io/Writer;Z)V", (out, false))
            .await
    }

    async fn init_writer_auto_flush(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<Writer>,
        auto_flush: bool,
    ) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::<init>({this:?}, {out:?}, {auto_flush})");

        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output is null").await);
        }

        let _: () = jvm
            .invoke_special(&this, "java/io/Writer", "<init>", "(Ljava/lang/Object;)V", (out.clone(),))
            .await?;
        jvm.put_field(&mut this, "out", "Ljava/io/Writer;", out).await?;
        jvm.put_field(&mut this, "autoFlush", "Z", auto_flush).await?;
        jvm.put_field(&mut this, "trouble", "Z", false).await
    }

    async fn init_output_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::<init>({this:?}, {out:?})");
        jvm.invoke_special(&this, "java/io/PrintWriter", "<init>", "(Ljava/io/OutputStream;Z)V", (out, false))
            .await
    }

    async fn init_output_stream_auto_flush(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<OutputStream>,
        auto_flush: bool,
    ) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::<init>({this:?}, {out:?}, {auto_flush})");

        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output is null").await);
        }

        let writer: ClassInstanceRef<Writer> = jvm
            .new_class("java/io/OutputStreamWriter", "(Ljava/io/OutputStream;)V", (out,))
            .await?
            .into();
        jvm.invoke_special(&this, "java/io/PrintWriter", "<init>", "(Ljava/io/Writer;Z)V", (writer, auto_flush))
            .await
    }

    async fn with_lock<T, F>(jvm: &Jvm, lock: &ClassInstanceRef<Object>, operation: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        jvm.monitor_enter(lock).await?;
        match operation.await {
            Ok(value) => {
                jvm.monitor_exit(lock).await?;
                Ok(value)
            }
            Err(error) => {
                if let Err(exit_error) = jvm.monitor_exit(lock).await {
                    tracing::error!(?exit_error, "failed to release PrintWriter lock");
                }
                Err(error)
            }
        }
    }

    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {value})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm.invoke_virtual(&out, "write", "(I)V", (value,)).await;
            Self::suppress_io_exception(jvm, &this, result).await
        })
        .await
    }

    async fn write_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, chars: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {chars:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            if chars.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "chars is null").await);
            }
            let length = jvm.array_length(&chars).await? as i32;
            jvm.invoke_virtual(&this, "write", "([CII)V", (chars, 0, length)).await
        })
        .await
    }

    async fn write_chars_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {chars:?}, {offset}, {length})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            if chars.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "chars is null").await);
            }
            let array_length = jvm.array_length(&chars).await? as i32;
            if offset < 0 || length < 0 || offset > array_length - length {
                return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid offset or length").await);
            }

            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm.invoke_virtual(&out, "write", "([CII)V", (chars, offset, length)).await;
            Self::suppress_io_exception(jvm, &this, result).await
        })
        .await
    }

    async fn write_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {string:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            if string.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
            }
            let length: i32 = jvm.invoke_virtual(&string, "length", "()I", ()).await?;
            jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;II)V", (string, 0, length)).await
        })
        .await
    }

    async fn write_string_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {string:?}, {offset}, {length})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            if string.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
            }
            let string_length: i32 = jvm.invoke_virtual(&string, "length", "()I", ()).await?;
            if offset < 0 || length < 0 || offset > string_length - length {
                return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid offset or length").await);
            }

            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm
                .invoke_virtual(&out, "write", "(Ljava/lang/String;II)V", (string, offset, length))
                .await;
            Self::suppress_io_exception(jvm, &this, result).await
        })
        .await
    }

    async fn print_boolean(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(Z)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    async fn print_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        jvm.invoke_virtual(&this, "write", "(I)V", (value as i32,)).await
    }

    async fn print_int(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(I)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    async fn print_long(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(J)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    async fn print_float(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(F)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    async fn print_double(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(D)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    async fn print_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value:?})");
        jvm.invoke_virtual(&this, "write", "([C)V", (value,)).await
    }

    async fn print_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value:?})");
        let value = if value.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            value
        };
        jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;)V", (value,)).await
    }

    async fn print_object(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value:?})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(Ljava/lang/Object;)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    async fn println(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::println({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                return Ok(());
            }

            let key = JavaLangString::from_rust_string(jvm, "line.separator").await?;
            let separator: ClassInstanceRef<String> = jvm
                .invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
                .await?;
            let separator = if separator.is_null() {
                JavaLangString::from_rust_string(jvm, "\n").await?.into()
            } else {
                separator
            };
            match jvm.invoke_virtual(&out, "write", "(Ljava/lang/String;)V", (separator,)).await {
                Ok(()) => {}
                Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                    let mut this = this.clone();
                    jvm.put_field(&mut this, "trouble", "Z", true).await?;
                    return Ok(());
                }
                Err(error) => return Err(error),
            }

            if jvm.get_field::<bool>(&this, "autoFlush", "Z").await? {
                let result = jvm.invoke_virtual(&out, "flush", "()V", ()).await;
                Self::suppress_io_exception(jvm, &this, result).await
            } else {
                Ok(())
            }
        })
        .await
    }

    async fn println_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "print", "(Z)V", (value,)).await?;
            jvm.invoke_virtual(&this, "println", "()V", ()).await
        })
        .await
    }

    async fn println_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "print", "(C)V", (value,)).await?;
            jvm.invoke_virtual(&this, "println", "()V", ()).await
        })
        .await
    }

    async fn println_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "print", "(I)V", (value,)).await?;
            jvm.invoke_virtual(&this, "println", "()V", ()).await
        })
        .await
    }

    async fn println_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "print", "(J)V", (value,)).await?;
            jvm.invoke_virtual(&this, "println", "()V", ()).await
        })
        .await
    }

    async fn println_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "print", "(F)V", (value,)).await?;
            jvm.invoke_virtual(&this, "println", "()V", ()).await
        })
        .await
    }

    async fn println_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "print", "(D)V", (value,)).await?;
            jvm.invoke_virtual(&this, "println", "()V", ()).await
        })
        .await
    }

    async fn println_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "print", "([C)V", (value,)).await?;
            jvm.invoke_virtual(&this, "println", "()V", ()).await
        })
        .await
    }

    async fn println_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "print", "(Ljava/lang/String;)V", (value,)).await?;
            jvm.invoke_virtual(&this, "println", "()V", ()).await
        })
        .await
    }

    async fn println_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "print", "(Ljava/lang/Object;)V", (value,)).await?;
            jvm.invoke_virtual(&this, "println", "()V", ()).await
        })
        .await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::flush({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm.invoke_virtual(&out, "flush", "()V", ()).await;
            Self::suppress_io_exception(jvm, &this, result).await
        })
        .await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::close({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                return Ok(());
            }

            match jvm.invoke_virtual(&out, "close", "()V", ()).await {
                Ok(()) => {
                    let mut this = this.clone();
                    let closed: ClassInstanceRef<Writer> = None.into();
                    jvm.put_field(&mut this, "out", "Ljava/io/Writer;", closed).await
                }
                Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                    let mut this = this.clone();
                    jvm.put_field(&mut this, "trouble", "Z", true).await
                }
                Err(error) => Err(error),
            }
        })
        .await
    }

    async fn check_error(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.PrintWriter::checkError({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
            if !out.is_null() {
                let _: () = jvm.invoke_virtual(&this, "flush", "()V", ()).await?;
                if jvm.is_instance(&**out, "java/io/PrintWriter") {
                    return jvm.invoke_virtual(&out, "checkError", "()Z", ()).await;
                }
            }
            jvm.get_field(&this, "trouble", "Z").await
        })
        .await
    }

    async fn suppress_io_exception(jvm: &Jvm, this: &ClassInstanceRef<Self>, result: Result<()>) -> Result<()> {
        match result {
            Ok(()) => Ok(()),
            Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await
            }
            Err(error) => Err(error),
        }
    }
}
