use core::future::Future;

use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        io::{File, OutputStream, OutputStreamWriter, Writer},
        lang::{Appendable, CharSequence, Object, String},
        util::{Formatter, Locale},
    },
};

// class java.io.PrintWriter
#[java_class(name = "java/io/PrintWriter", extends = "java/io/Writer", public)]
pub struct PrintWriter {
    #[field(protected)]
    out: ClassInstanceRef<Writer>,
    #[field(private)]
    autoFlush: bool,
    #[field(private)]
    trouble: bool,
}

#[java_methods]
impl PrintWriter {
    #[method(constructor, public)]
    async fn init_writer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<Writer>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::<init>({this:?}, {out:?})");
        jvm.invoke_special(&this, "java/io/PrintWriter", "<init>", "(Ljava/io/Writer;Z)V", (out, false))
            .await
    }

    #[method(constructor, public)]
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
        jvm.put_field(&mut this, "java/io/PrintWriter", "out", "Ljava/io/Writer;", out).await?;
        jvm.put_field(&mut this, "java/io/PrintWriter", "autoFlush", "Z", auto_flush).await?;
        jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", false).await
    }

    #[method(constructor, public)]
    async fn init_output_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::<init>({this:?}, {out:?})");
        jvm.invoke_special(&this, "java/io/PrintWriter", "<init>", "(Ljava/io/OutputStream;Z)V", (out, false))
            .await
    }

    #[method(constructor, public)]
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

    #[method(constructor, public)]
    async fn init_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, path: ClassInstanceRef<String>) -> Result<()> {
        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file name is null").await);
        }
        let file: ClassInstanceRef<File> = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?.into();
        jvm.invoke_special(&this, "java/io/PrintWriter", "<init>", "(Ljava/io/File;)V", (file,))
            .await
    }

    #[method(constructor, public)]
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
            "java/io/PrintWriter",
            "<init>",
            "(Ljava/io/File;Ljava/lang/String;)V",
            (file, encoding),
        )
        .await
    }

    #[method(constructor, public)]
    async fn init_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file is null").await);
        }
        let output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?.into();
        let writer: ClassInstanceRef<Writer> = jvm
            .new_class("java/io/OutputStreamWriter", "(Ljava/io/OutputStream;)V", (output,))
            .await?
            .into();
        let writer: ClassInstanceRef<Writer> = jvm.new_class("java/io/BufferedWriter", "(Ljava/io/Writer;)V", (writer,)).await?.into();
        jvm.invoke_special(&this, "java/io/PrintWriter", "<init>", "(Ljava/io/Writer;Z)V", (writer, false))
            .await
    }

    #[method(constructor, public)]
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
        let writer: ClassInstanceRef<Writer> = jvm
            .new_class(
                "java/io/OutputStreamWriter",
                "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                (output, encoding),
            )
            .await?
            .into();
        let writer: ClassInstanceRef<Writer> = jvm.new_class("java/io/BufferedWriter", "(Ljava/io/Writer;)V", (writer,)).await?.into();
        jvm.invoke_special(&this, "java/io/PrintWriter", "<init>", "(Ljava/io/Writer;Z)V", (writer, false))
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

    #[method(name = "write", public)]
    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {value})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "java/io/PrintWriter", "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm.invoke_virtual(&out, "java/io/Writer", "write", "(I)V", (value,)).await;
            Self::suppress_io_exception(jvm, &this, result).await
        })
        .await
    }

    #[method(name = "write", public)]
    async fn write_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, chars: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {chars:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            if chars.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "chars is null").await);
            }
            let length = jvm.array_length(&chars).await? as i32;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "write", "([CII)V", (chars, 0, length))
                .await
        })
        .await
    }

    #[method(name = "write", public)]
    async fn write_chars_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {chars:?}, {offset}, {length})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            if chars.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "chars is null").await);
            }
            let array_length = jvm.array_length(&chars).await? as i32;
            if offset < 0 || length < 0 || offset > array_length - length {
                return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid offset or length").await);
            }

            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "java/io/PrintWriter", "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm
                .invoke_virtual(&out, "java/io/Writer", "write", "([CII)V", (chars, offset, length))
                .await;
            Self::suppress_io_exception(jvm, &this, result).await
        })
        .await
    }

    #[method(name = "write", public)]
    async fn write_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {string:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            if string.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
            }
            let length: i32 = jvm.invoke_virtual(&string, "java/lang/String", "length", "()I", ()).await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "write", "(Ljava/lang/String;II)V", (string, 0, length))
                .await
        })
        .await
    }

    #[method(name = "write", public)]
    async fn write_string_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::write({this:?}, {string:?}, {offset}, {length})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            if string.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
            }
            let string_length: i32 = jvm.invoke_virtual(&string, "java/lang/String", "length", "()I", ()).await?;
            if offset < 0 || length < 0 || offset > string_length - length {
                return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid offset or length").await);
            }

            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "java/io/PrintWriter", "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm
                .invoke_virtual(&out, "java/io/Writer", "write", "(Ljava/lang/String;II)V", (string, offset, length))
                .await;
            Self::suppress_io_exception(jvm, &this, result).await
        })
        .await
    }

    #[method(name = "print", public)]
    async fn print_boolean(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(Z)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    #[method(name = "print", public)]
    async fn print_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        jvm.invoke_virtual(&this, "java/io/PrintWriter", "write", "(I)V", (value as i32,)).await
    }

    #[method(name = "print", public)]
    async fn print_int(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(I)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    #[method(name = "print", public)]
    async fn print_long(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(J)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    #[method(name = "print", public)]
    async fn print_float(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(F)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    #[method(name = "print", public)]
    async fn print_double(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(D)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    #[method(name = "print", public)]
    async fn print_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value:?})");
        jvm.invoke_virtual(&this, "java/io/PrintWriter", "write", "([C)V", (value,)).await
    }

    #[method(name = "print", public)]
    async fn print_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value:?})");
        let value = if value.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            value
        };
        jvm.invoke_virtual(&this, "java/io/PrintWriter", "write", "(Ljava/lang/String;)V", (value,))
            .await
    }

    #[method(name = "print", public)]
    async fn print_object(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::print({this:?}, {value:?})");
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/String", "valueOf", "(Ljava/lang/Object;)Ljava/lang/String;", (value,))
            .await?;
        Self::print_string(jvm, context, this, value).await
    }

    #[method(public)]
    async fn println(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::println({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "java/io/PrintWriter", "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", true).await?;
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
            match jvm
                .invoke_virtual(&out, "java/io/Writer", "write", "(Ljava/lang/String;)V", (separator,))
                .await
            {
                Ok(()) => {}
                Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                    let mut this = this.clone();
                    jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", true).await?;
                    return Ok(());
                }
                Err(error) => return Err(error),
            }

            if jvm.get_field::<bool>(&this, "java/io/PrintWriter", "autoFlush", "Z").await? {
                let result = jvm.invoke_virtual(&out, "java/io/Writer", "flush", "()V", ()).await;
                Self::suppress_io_exception(jvm, &this, result).await
            } else {
                Ok(())
            }
        })
        .await
    }

    #[method(name = "println", public)]
    async fn println_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintWriter", "print", "(Z)V", (value,)).await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "println", "()V", ()).await
        })
        .await
    }

    #[method(name = "println", public)]
    async fn println_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintWriter", "print", "(C)V", (value,)).await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "println", "()V", ()).await
        })
        .await
    }

    #[method(name = "println", public)]
    async fn println_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintWriter", "print", "(I)V", (value,)).await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "println", "()V", ()).await
        })
        .await
    }

    #[method(name = "println", public)]
    async fn println_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintWriter", "print", "(J)V", (value,)).await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "println", "()V", ()).await
        })
        .await
    }

    #[method(name = "println", public)]
    async fn println_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintWriter", "print", "(F)V", (value,)).await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "println", "()V", ()).await
        })
        .await
    }

    #[method(name = "println", public)]
    async fn println_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintWriter", "print", "(D)V", (value,)).await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "println", "()V", ()).await
        })
        .await
    }

    #[method(name = "println", public)]
    async fn println_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm.invoke_virtual(&this, "java/io/PrintWriter", "print", "([C)V", (value,)).await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "println", "()V", ()).await
        })
        .await
    }

    #[method(name = "println", public)]
    async fn println_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm
                .invoke_virtual(&this, "java/io/PrintWriter", "print", "(Ljava/lang/String;)V", (value,))
                .await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "println", "()V", ()).await
        })
        .await
    }

    #[method(name = "println", public)]
    async fn println_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let _: () = jvm
                .invoke_virtual(&this, "java/io/PrintWriter", "print", "(Ljava/lang/Object;)V", (value,))
                .await?;
            jvm.invoke_virtual(&this, "java/io/PrintWriter", "println", "()V", ()).await
        })
        .await
    }

    #[method(public)]
    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::flush({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "java/io/PrintWriter", "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                let mut this = this.clone();
                jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", true).await?;
                return Ok(());
            }

            let result = jvm.invoke_virtual(&out, "java/io/Writer", "flush", "()V", ()).await;
            Self::suppress_io_exception(jvm, &this, result).await
        })
        .await
    }

    #[method(public)]
    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::close({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "java/io/PrintWriter", "out", "Ljava/io/Writer;").await?;
            if out.is_null() {
                return Ok(());
            }

            match jvm.invoke_virtual(&out, "java/io/Writer", "close", "()V", ()).await {
                Ok(()) => {
                    let mut this = this.clone();
                    let closed: ClassInstanceRef<Writer> = None.into();
                    jvm.put_field(&mut this, "java/io/PrintWriter", "out", "Ljava/io/Writer;", closed).await
                }
                Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                    let mut this = this.clone();
                    jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", true).await
                }
                Err(error) => Err(error),
            }
        })
        .await
    }

    #[method(public)]
    async fn check_error(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.PrintWriter::checkError({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
            let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "java/io/PrintWriter", "out", "Ljava/io/Writer;").await?;
            if !out.is_null() {
                let _: () = jvm.invoke_virtual(&this, "java/io/PrintWriter", "flush", "()V", ()).await?;
                if jvm.is_instance(&**out, "java/io/PrintWriter") {
                    return jvm.invoke_virtual(&out, "java/io/PrintWriter", "checkError", "()Z", ()).await;
                }
            }
            jvm.get_field(&this, "java/io/PrintWriter", "trouble", "Z").await
        })
        .await
    }

    #[method(protected)]
    async fn set_error(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", true).await
    }

    #[method(public, varargs)]
    async fn printf(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        format: ClassInstanceRef<String>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        jvm.invoke_virtual(
            &this,
            "java/io/PrintWriter",
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintWriter;",
            (format, arguments),
        )
        .await
    }

    #[method(name = "printf", public, varargs)]
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
            "java/io/PrintWriter",
            "format",
            "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintWriter;",
            (locale, format, arguments),
        )
        .await
    }

    #[method(public, varargs)]
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
            "java/io/PrintWriter",
            "format",
            "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintWriter;",
            (locale, format, arguments),
        )
        .await
    }

    #[method(name = "format", public, varargs)]
    async fn format_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        locale: ClassInstanceRef<Locale>,
        format: ClassInstanceRef<String>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/io/PrintWriter", "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, async {
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
            if jvm.get_field::<bool>(&this, "java/io/PrintWriter", "autoFlush", "Z").await? {
                let _: () = jvm.invoke_virtual(&this, "java/io/PrintWriter", "flush", "()V", ()).await?;
            }
            Ok(this.clone())
        })
        .await
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/Writer;", public, bridge, synthetic)]
    #[method(
        name = "append",
        descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
        public,
        bridge,
        synthetic
    )]
    async fn append_char_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<Writer> = jvm
            .invoke_special(
                &this,
                "java/io/Writer",
                "append",
                "(Ljava/lang/CharSequence;)Ljava/io/Writer;",
                (sequence,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/Writer;", public, bridge, synthetic)]
    #[method(
        name = "append",
        descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;",
        public,
        bridge,
        synthetic
    )]
    async fn append_char_sequence_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<Writer> = jvm
            .invoke_special(
                &this,
                "java/io/Writer",
                "append",
                "(Ljava/lang/CharSequence;II)Ljava/io/Writer;",
                (sequence, start, end),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(C)Ljava/io/Writer;", public, bridge, synthetic)]
    #[method(name = "append", descriptor = "(C)Ljava/lang/Appendable;", public, bridge, synthetic)]
    async fn append_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, character: JavaChar) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<Writer> = jvm
            .invoke_special(&this, "java/io/Writer", "append", "(C)Ljava/io/Writer;", (character,))
            .await?;
        Ok(this)
    }

    async fn suppress_io_exception(jvm: &Jvm, this: &ClassInstanceRef<Self>, result: Result<()>) -> Result<()> {
        match result {
            Ok(()) => Ok(()),
            Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                let mut this = this.clone();
                jvm.put_field(&mut this, "java/io/PrintWriter", "trouble", "Z", true).await
            }
            Err(error) => Err(error),
        }
    }
}
