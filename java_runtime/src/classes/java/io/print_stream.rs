use core::future::Future;

use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{File, OutputStream, OutputStreamWriter},
        lang::{Appendable, CharSequence, Object, String},
        util::{Formatter, Locale, Properties},
    },
};

// class java.io.PrintStream
pub struct PrintStream;

impl PrintStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/PrintStream",
            parent_class: Some("java/io/FilterOutputStream"),
            interfaces: vec!["java/lang/Appendable", "java/io/Closeable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;Z)V", Self::init_auto_flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/OutputStream;ZLjava/lang/String;)V",
                    Self::init_auto_flush_encoding,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_path, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init_path_encoding,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init_file, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/File;Ljava/lang/String;)V",
                    Self::init_file_encoding,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("checkError", "()Z", Self::check_error, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setError", "()V", Self::set_error, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(I)V", Self::write_byte, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([BII)V", Self::write_bytes, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(Ljava/lang/Object;)V", Self::print_object, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(Ljava/lang/String;)V", Self::print_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(I)V", Self::print_int, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(J)V", Self::print_long, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(C)V", Self::print_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "([C)V", Self::print_chars, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(Z)V", Self::print_bool, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(F)V", Self::print_float, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(D)V", Self::print_double, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "()V", Self::println, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(Ljava/lang/Object;)V", Self::println_object, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(I)V", Self::println_int, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(J)V", Self::println_long, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(C)V", Self::println_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "([C)V", Self::println_chars, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(Z)V", Self::println_bool, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(F)V", Self::println_float, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "(D)V", Self::println_double, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "printf",
                    "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
                    Self::printf,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::VARARGS,
                ),
                JavaMethodProto::new(
                    "printf",
                    "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
                    Self::printf_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::VARARGS,
                ),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
                    Self::format,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::VARARGS,
                ),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
                    Self::format_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::VARARGS,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;)Ljava/io/PrintStream;",
                    Self::append_char_sequence,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;II)Ljava/io/PrintStream;",
                    Self::append_char_sequence_range,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("append", "(C)Ljava/io/PrintStream;", Self::append_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
                    Self::append_char_sequence,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;",
                    Self::append_char_sequence_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(C)Ljava/lang/Appendable;",
                    Self::append_char,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("autoFlush", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("trouble", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("charOut", "Ljava/io/OutputStreamWriter;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("closing", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::<init>({this:?}, {out:?})");
        jvm.invoke_special(&this, "java/io/PrintStream", "<init>", "(Ljava/io/OutputStream;Z)V", (out, false))
            .await
    }

    async fn init_auto_flush(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<OutputStream>,
        auto_flush: bool,
    ) -> Result<()> {
        tracing::debug!("java.io.PrintStream::<init>({this:?}, {out:?}, {auto_flush})");

        let props: ClassInstanceRef<Properties> = jvm.get_static_field("java/lang/System", "props", "Ljava/util/Properties;").await?;
        let encoding: ClassInstanceRef<String> = if props.is_null() {
            JavaLangString::from_rust_string(jvm, "UTF-8").await?.into()
        } else {
            let key = JavaLangString::from_rust_string(jvm, "file.encoding").await?;
            let encoding: ClassInstanceRef<String> = jvm
                .invoke_virtual(
                    &props,
                    "java/util/Properties",
                    "getProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    (key,),
                )
                .await?;
            if encoding.is_null() {
                JavaLangString::from_rust_string(jvm, "UTF-8").await?.into()
            } else {
                encoding
            }
        };
        jvm.invoke_special(
            &this,
            "java/io/PrintStream",
            "<init>",
            "(Ljava/io/OutputStream;ZLjava/lang/String;)V",
            (out, auto_flush, encoding),
        )
        .await
    }

    async fn init_auto_flush_encoding(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<OutputStream>,
        auto_flush: bool,
        encoding: ClassInstanceRef<String>,
    ) -> Result<()> {
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output is null").await);
        }
        OutputStreamWriter::validate_encoding(jvm, &encoding).await?;

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterOutputStream", "<init>", "(Ljava/io/OutputStream;)V", (out.clone(),))
            .await?;
        let this_output: ClassInstanceRef<OutputStream> = this.instance.clone().into();
        let char_out = jvm
            .new_class(
                "java/io/OutputStreamWriter",
                "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                (this_output, encoding),
            )
            .await?;
        jvm.put_field(&mut this, "autoFlush", "Z", auto_flush).await?;
        jvm.put_field(&mut this, "trouble", "Z", false).await?;
        jvm.put_field(&mut this, "charOut", "Ljava/io/OutputStreamWriter;", char_out).await?;
        jvm.put_field(&mut this, "closing", "Z", false).await
    }

    async fn init_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, path: ClassInstanceRef<String>) -> Result<()> {
        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file name is null").await);
        }
        let file: ClassInstanceRef<File> = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?.into();
        jvm.invoke_special(&this, "java/io/PrintStream", "<init>", "(Ljava/io/File;)V", (file,))
            .await
    }

    async fn init_path_encoding(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        path: ClassInstanceRef<String>,
        encoding: ClassInstanceRef<String>,
    ) -> Result<()> {
        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file name is null").await);
        }
        OutputStreamWriter::validate_encoding(jvm, &encoding).await?;
        let file: ClassInstanceRef<File> = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?.into();
        jvm.invoke_special(
            &this,
            "java/io/PrintStream",
            "<init>",
            "(Ljava/io/File;Ljava/lang/String;)V",
            (file, encoding),
        )
        .await
    }

    async fn init_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file is null").await);
        }
        let output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?.into();
        jvm.invoke_special(&this, "java/io/PrintStream", "<init>", "(Ljava/io/OutputStream;)V", (output,))
            .await
    }

    async fn init_file_encoding(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        file: ClassInstanceRef<File>,
        encoding: ClassInstanceRef<String>,
    ) -> Result<()> {
        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file is null").await);
        }
        OutputStreamWriter::validate_encoding(jvm, &encoding).await?;
        let output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?.into();
        jvm.invoke_special(
            &this,
            "java/io/PrintStream",
            "<init>",
            "(Ljava/io/OutputStream;ZLjava/lang/String;)V",
            (output, false, encoding),
        )
        .await
    }

    async fn with_monitor<T, F>(jvm: &Jvm, this: &ClassInstanceRef<Self>, operation: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        jvm.monitor_enter(this).await?;
        match operation.await {
            Ok(value) => {
                jvm.monitor_exit(this).await?;
                Ok(value)
            }
            Err(error) => {
                if let Err(exit_error) = jvm.monitor_exit(this).await {
                    tracing::error!(?exit_error, "failed to release PrintStream monitor");
                }
                Err(error)
            }
        }
    }

    async fn check_error(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.PrintStream::checkError({this:?})");

        Self::with_monitor(jvm, &this, async {
            let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
            if !out.is_null() {
                let _: () = jvm.invoke_virtual(&this, "java/io/PrintStream", "flush", "()V", ()).await?;
                if jvm.is_instance(&**out, "java/io/PrintStream") {
                    return jvm.invoke_virtual(&out, "java/io/PrintStream", "checkError", "()Z", ()).await;
                }
            }
            jvm.get_field(&this, "trouble", "Z").await
        })
        .await
    }

    async fn set_error(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "trouble", "Z", true).await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::close({this:?})");

        Self::with_monitor(jvm, &this, async {
            let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
            if out.is_null() || jvm.get_field::<bool>(&this, "closing", "Z").await? {
                return Ok(());
            }

            let mut this = this.clone();
            jvm.put_field(&mut this, "closing", "Z", true).await?;
            let char_out: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(&this, "charOut", "Ljava/io/OutputStreamWriter;").await?;
            let char_out_result = jvm.invoke_virtual(&char_out, "java/io/OutputStreamWriter", "close", "()V", ()).await;
            let close_out = match char_out_result {
                Ok(()) => true,
                Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                    jvm.put_field(&mut this, "trouble", "Z", true).await?;
                    false
                }
                Err(error) => return Err(error),
            };

            if close_out {
                match jvm.invoke_virtual(&out, "java/io/OutputStream", "close", "()V", ()).await {
                    Ok(()) => {}
                    Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                        jvm.put_field(&mut this, "trouble", "Z", true).await?;
                    }
                    Err(error) => return Err(error),
                }
            }

            let closed_output: ClassInstanceRef<OutputStream> = None.into();
            let closed_writer: ClassInstanceRef<OutputStreamWriter> = None.into();
            jvm.put_field(&mut this, "charOut", "Ljava/io/OutputStreamWriter;", closed_writer).await?;
            jvm.put_field(&mut this, "out", "Ljava/io/OutputStream;", closed_output).await
        })
        .await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::flush({this:?})");

        Self::with_monitor(jvm, &this, async {
            let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm.invoke_virtual(&out, "java/io/OutputStream", "flush", "()V", ()).await;
            Self::suppress_io_exception(jvm, &this, result).await.map(|_| ())
        })
        .await
    }

    async fn write_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.PrintStream::write({this:?}, {value})");

        Self::with_monitor(jvm, &this, async {
            let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm.invoke_virtual(&out, "java/io/OutputStream", "write", "(I)V", (value,)).await;
            if Self::suppress_io_exception(jvm, &this, result).await?
                && value == b'\n' as i32
                && jvm.get_field::<bool>(&this, "autoFlush", "Z").await?
            {
                let result = jvm.invoke_virtual(&out, "java/io/OutputStream", "flush", "()V", ()).await;
                Self::suppress_io_exception(jvm, &this, result).await?;
            }
            Ok(())
        })
        .await
    }

    async fn write_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        bytes: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.PrintStream::write({this:?}, {bytes:?}, {off}, {len})");

        Self::with_monitor(jvm, &this, async {
            if bytes.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "bytes is null").await);
            }
            let array_length = jvm.array_length(&bytes).await? as i32;
            if off < 0 || len < 0 || off > array_length - len {
                return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid offset or length").await);
            }

            let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm
                .invoke_virtual(&out, "java/io/OutputStream", "write", "([BII)V", (bytes, off, len))
                .await;
            if Self::suppress_io_exception(jvm, &this, result).await? && jvm.get_field::<bool>(&this, "autoFlush", "Z").await? {
                let result = jvm.invoke_virtual(&out, "java/io/OutputStream", "flush", "()V", ()).await;
                Self::suppress_io_exception(jvm, &this, result).await?;
            }
            Ok(())
        })
        .await
    }

    async fn print_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::print({this:?}, {value:?})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(Ljava/lang/Object;)Ljava/lang/String;", (value,))
            .await?;
        Self::write_string(jvm, &this, value).await
    }

    async fn print_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::print({this:?}, {value:?})");
        let value = if value.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            value
        };
        Self::write_string(jvm, &this, value).await
    }

    async fn print_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(I)Ljava/lang/String;", (value,))
            .await?;
        Self::write_string(jvm, &this, value).await
    }

    async fn print_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(J)Ljava/lang/String;", (value,))
            .await?;
        Self::write_string(jvm, &this, value).await
    }

    async fn print_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        let mut chars = jvm.instantiate_array("C", 1).await?;
        jvm.store_array(&mut chars, 0, [value]).await?;
        Self::write_characters(jvm, &this, chars.into()).await
    }

    async fn print_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        Self::write_characters(jvm, &this, value).await
    }

    async fn print_bool(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(Z)Ljava/lang/String;", (value,))
            .await?;
        Self::write_string(jvm, &this, value).await
    }

    async fn print_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(F)Ljava/lang/String;", (value,))
            .await?;
        Self::write_string(jvm, &this, value).await
    }

    async fn print_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(D)Ljava/lang/String;", (value,))
            .await?;
        Self::write_string(jvm, &this, value).await
    }

    async fn println(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        Self::new_line(jvm, &this).await
    }

    async fn println_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<()> {
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(Ljava/lang/Object;)Ljava/lang/String;", (value,))
            .await?;
        Self::with_monitor(jvm, &this, async {
            let _: () = jvm
                .invoke_virtual(&this, "java/io/PrintStream", "print", "(Ljava/lang/String;)V", (value,))
                .await?;
            Self::new_line(jvm, &this).await
        })
        .await
    }

    async fn println_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        Self::with_monitor(jvm, &this, async {
            let _: () = jvm
                .invoke_virtual(&this, "java/io/PrintStream", "print", "(Ljava/lang/String;)V", (value,))
                .await?;
            Self::new_line(jvm, &this).await
        })
        .await
    }

    async fn println_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        Self::with_monitor(jvm, &this, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintStream", "print", "(I)V", (value,)).await?;
            Self::new_line(jvm, &this).await
        })
        .await
    }

    async fn println_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        Self::with_monitor(jvm, &this, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintStream", "print", "(J)V", (value,)).await?;
            Self::new_line(jvm, &this).await
        })
        .await
    }

    async fn println_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        Self::with_monitor(jvm, &this, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintStream", "print", "(C)V", (value,)).await?;
            Self::new_line(jvm, &this).await
        })
        .await
    }

    async fn println_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        Self::with_monitor(jvm, &this, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintStream", "print", "([C)V", (value,)).await?;
            Self::new_line(jvm, &this).await
        })
        .await
    }

    async fn println_bool(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        Self::with_monitor(jvm, &this, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintStream", "print", "(Z)V", (value,)).await?;
            Self::new_line(jvm, &this).await
        })
        .await
    }

    async fn println_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        Self::with_monitor(jvm, &this, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintStream", "print", "(F)V", (value,)).await?;
            Self::new_line(jvm, &this).await
        })
        .await
    }

    async fn println_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        Self::with_monitor(jvm, &this, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintStream", "print", "(D)V", (value,)).await?;
            Self::new_line(jvm, &this).await
        })
        .await
    }

    async fn printf(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        format: ClassInstanceRef<String>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        jvm.invoke_virtual(
            &this,
            "java/io/PrintStream",
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
            (format, arguments),
        )
        .await
    }

    async fn printf_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        locale: ClassInstanceRef<Locale>,
        format: ClassInstanceRef<String>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        jvm.invoke_virtual(
            &this,
            "java/io/PrintStream",
            "format",
            "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
            (locale, format, arguments),
        )
        .await
    }

    async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        format: ClassInstanceRef<String>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_virtual(
            &this,
            "java/io/PrintStream",
            "format",
            "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
            (locale, format, arguments),
        )
        .await
    }

    async fn format_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        locale: ClassInstanceRef<Locale>,
        format: ClassInstanceRef<String>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::with_monitor(jvm, &this, async {
            let appendable: ClassInstanceRef<Appendable> = this.instance.clone().into();
            let formatter: ClassInstanceRef<Formatter> = jvm
                .new_class("java/util/Formatter", "(Ljava/lang/Appendable;Ljava/util/Locale;)V", (appendable, locale))
                .await?
                .into();
            let _: ClassInstanceRef<Formatter> = jvm
                .invoke_virtual(
                    &formatter,
                    "java/util/Formatter",
                    "format",
                    "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
                    (format, arguments),
                )
                .await?;
            Ok(this.clone())
        })
        .await
    }

    async fn append_char_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Self>> {
        let string: ClassInstanceRef<String> = if sequence.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&sequence, &sequence.class_definition().name(), "toString", "()Ljava/lang/String;", ())
                .await?
        };
        let _: () = jvm
            .invoke_virtual(&this, "java/io/PrintStream", "print", "(Ljava/lang/String;)V", (string,))
            .await?;
        Ok(this)
    }

    async fn append_char_sequence_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let sequence: ClassInstanceRef<CharSequence> = if sequence.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            sequence
        };
        let subsequence: ClassInstanceRef<CharSequence> = jvm
            .invoke_virtual(
                &sequence,
                &sequence.class_definition().name(),
                "subSequence",
                "(II)Ljava/lang/CharSequence;",
                (start, end),
            )
            .await?;
        let string: ClassInstanceRef<String> = jvm
            .invoke_virtual(
                &subsequence,
                &subsequence.class_definition().name(),
                "toString",
                "()Ljava/lang/String;",
                (),
            )
            .await?;
        let _: () = jvm
            .invoke_virtual(&this, "java/io/PrintStream", "print", "(Ljava/lang/String;)V", (string,))
            .await?;
        Ok(this)
    }

    async fn append_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, character: JavaChar) -> Result<ClassInstanceRef<Self>> {
        let _: () = jvm.invoke_virtual(&this, "java/io/PrintStream", "print", "(C)V", (character,)).await?;
        Ok(this)
    }

    async fn write_string(jvm: &Jvm, this: &ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&value, "java/lang/String", "toCharArray", "()[C", ()).await?;
        Self::write_characters(jvm, this, chars).await
    }

    async fn write_characters(jvm: &Jvm, this: &ClassInstanceRef<Self>, chars: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        Self::with_monitor(jvm, this, async {
            if chars.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "chars is null").await);
            }

            let out: ClassInstanceRef<OutputStream> = jvm.get_field(this, "out", "Ljava/io/OutputStream;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                return Ok(());
            }

            let length = jvm.array_length(&chars).await?;
            let values: Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
            let char_out: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(this, "charOut", "Ljava/io/OutputStreamWriter;").await?;
            let result = jvm
                .invoke_virtual(&char_out, "java/io/OutputStreamWriter", "write", "([CII)V", (chars, 0, length as i32))
                .await;
            if !Self::suppress_io_exception(jvm, this, result).await? {
                return Ok(());
            }

            if jvm.get_field::<bool>(this, "autoFlush", "Z").await? && values.contains(&('\n' as JavaChar)) {
                let result = jvm.invoke_virtual(&out, "java/io/OutputStream", "flush", "()V", ()).await;
                Self::suppress_io_exception(jvm, this, result).await?;
            }
            Ok(())
        })
        .await
    }

    async fn new_line(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<()> {
        Self::with_monitor(jvm, this, async {
            let out: ClassInstanceRef<OutputStream> = jvm.get_field(this, "out", "Ljava/io/OutputStream;").await?;
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
            let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&separator, "java/lang/String", "toCharArray", "()[C", ()).await?;
            let length = jvm.array_length(&chars).await?;
            let char_out: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(this, "charOut", "Ljava/io/OutputStreamWriter;").await?;
            let result = jvm
                .invoke_virtual(&char_out, "java/io/OutputStreamWriter", "write", "([CII)V", (chars, 0, length as i32))
                .await;
            if !Self::suppress_io_exception(jvm, this, result).await? {
                return Ok(());
            }

            if jvm.get_field::<bool>(this, "autoFlush", "Z").await? {
                let result = jvm.invoke_virtual(&out, "java/io/OutputStream", "flush", "()V", ()).await;
                Self::suppress_io_exception(jvm, this, result).await?;
            }
            Ok(())
        })
        .await
    }

    async fn suppress_io_exception(jvm: &Jvm, this: &ClassInstanceRef<Self>, result: Result<()>) -> Result<bool> {
        match result {
            Ok(()) => Ok(true),
            Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                let mut this = this.clone();
                jvm.put_field(&mut this, "trouble", "Z", true).await?;
                Ok(false)
            }
            Err(error) => Err(error),
        }
    }
}
