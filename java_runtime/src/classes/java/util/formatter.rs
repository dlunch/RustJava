use alloc::{format, string::String as RustString, string::ToString, vec, vec::Vec};
use core::fmt::Write;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{File, IOException, OutputStream, OutputStreamWriter, PrintStream, Writer},
        lang::{Appendable, CharSequence, Object, String},
        util::Locale,
    },
};

struct FormatSpecifier {
    text: RustString,
    argument_index: Option<usize>,
    reuse_previous: bool,
    flags: Vec<char>,
    width: Option<usize>,
    precision: Option<usize>,
    conversion: char,
}

enum NumericKind {
    Finite,
    Nan,
    Infinite,
}

// public final class java.util.Formatter
pub struct Formatter;

impl Formatter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Formatter",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Closeable", "java/io/Flushable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/Appendable;)V",
                    Self::init_with_appendable,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("<init>", "(Ljava/util/Locale;)V", Self::init_with_locale, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/Appendable;Ljava/util/Locale;)V",
                    Self::init_with_appendable_locale,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_path, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init_with_path_encoding,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V",
                    Self::init_with_path_encoding_locale,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init_with_file, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/File;Ljava/lang/String;)V",
                    Self::init_with_file_encoding,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V",
                    Self::init_with_file_encoding_locale,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/PrintStream;)V",
                    Self::init_with_print_stream,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/OutputStream;)V",
                    Self::init_with_output_stream,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                    Self::init_with_output_stream_encoding,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/OutputStream;Ljava/lang/String;Ljava/util/Locale;)V",
                    Self::init_with_output_stream_encoding_locale,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("locale", "()Ljava/util/Locale;", Self::locale, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("out", "()Ljava/lang/Appendable;", Self::out, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("ioException", "()Ljava/io/IOException;", Self::io_exception, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
                    Self::format,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::VARARGS,
                ),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
                    Self::format_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::VARARGS,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("a", "Ljava/lang/Appendable;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("l", "Ljava/util/Locale;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("lastException", "Ljava/io/IOException;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/lang/Appendable;Ljava/util/Locale;)V",
            (ClassInstanceRef::<Appendable>::new(None), locale),
        )
        .await
    }

    async fn init_with_appendable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        appendable: ClassInstanceRef<Appendable>,
    ) -> Result<()> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/lang/Appendable;Ljava/util/Locale;)V",
            (appendable, locale),
        )
        .await
    }

    async fn init_with_locale(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, locale: ClassInstanceRef<Locale>) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/lang/Appendable;Ljava/util/Locale;)V",
            (ClassInstanceRef::<Appendable>::new(None), locale),
        )
        .await
    }

    async fn init_with_appendable_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        appendable: ClassInstanceRef<Appendable>,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let appendable: ClassInstanceRef<Appendable> = if appendable.is_null() {
            jvm.new_class("java/lang/StringBuilder", "()V", ()).await?.into()
        } else {
            appendable
        };
        jvm.put_field(&mut this, "a", "Ljava/lang/Appendable;", appendable).await?;
        jvm.put_field(&mut this, "l", "Ljava/util/Locale;", locale).await?;
        jvm.put_field(
            &mut this,
            "lastException",
            "Ljava/io/IOException;",
            ClassInstanceRef::<IOException>::new(None),
        )
        .await
    }

    async fn init_with_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, path: ClassInstanceRef<String>) -> Result<()> {
        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file name is null").await);
        }
        let file: ClassInstanceRef<File> = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?.into();
        jvm.invoke_special(&this, "java/util/Formatter", "<init>", "(Ljava/io/File;)V", (file,))
            .await
    }

    async fn init_with_path_encoding(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        path: ClassInstanceRef<String>,
        encoding: ClassInstanceRef<String>,
    ) -> Result<()> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V",
            (path, encoding, locale),
        )
        .await
    }

    async fn init_with_path_encoding_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        path: ClassInstanceRef<String>,
        encoding: ClassInstanceRef<String>,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<()> {
        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file name is null").await);
        }
        OutputStreamWriter::validate_encoding(jvm, &encoding).await?;
        let file: ClassInstanceRef<File> = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?.into();
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V",
            (file, encoding, locale),
        )
        .await
    }

    async fn init_with_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file is null").await);
        }
        let stream: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?.into();
        let writer: ClassInstanceRef<Writer> = jvm
            .new_class("java/io/OutputStreamWriter", "(Ljava/io/OutputStream;)V", (stream,))
            .await?
            .into();
        let appendable: ClassInstanceRef<Appendable> = jvm.new_class("java/io/BufferedWriter", "(Ljava/io/Writer;)V", (writer,)).await?.into();
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/lang/Appendable;Ljava/util/Locale;)V",
            (appendable, locale),
        )
        .await
    }

    async fn init_with_file_encoding(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        file: ClassInstanceRef<File>,
        encoding: ClassInstanceRef<String>,
    ) -> Result<()> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V",
            (file, encoding, locale),
        )
        .await
    }

    async fn init_with_file_encoding_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        file: ClassInstanceRef<File>,
        encoding: ClassInstanceRef<String>,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<()> {
        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file is null").await);
        }
        OutputStreamWriter::validate_encoding(jvm, &encoding).await?;
        let stream: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?.into();
        let writer: ClassInstanceRef<Writer> = jvm
            .new_class(
                "java/io/OutputStreamWriter",
                "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                (stream, encoding),
            )
            .await?
            .into();
        let appendable: ClassInstanceRef<Appendable> = jvm.new_class("java/io/BufferedWriter", "(Ljava/io/Writer;)V", (writer,)).await?.into();
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/lang/Appendable;Ljava/util/Locale;)V",
            (appendable, locale),
        )
        .await
    }

    async fn init_with_print_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        stream: ClassInstanceRef<PrintStream>,
    ) -> Result<()> {
        if stream.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "print stream is null").await);
        }
        let appendable: ClassInstanceRef<Appendable> = stream.instance.into();
        jvm.invoke_special(&this, "java/util/Formatter", "<init>", "(Ljava/lang/Appendable;)V", (appendable,))
            .await
    }

    async fn init_with_output_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        stream: ClassInstanceRef<OutputStream>,
    ) -> Result<()> {
        if stream.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output stream is null").await);
        }
        let writer: ClassInstanceRef<Writer> = jvm
            .new_class("java/io/OutputStreamWriter", "(Ljava/io/OutputStream;)V", (stream,))
            .await?
            .into();
        let appendable: ClassInstanceRef<Appendable> = jvm.new_class("java/io/BufferedWriter", "(Ljava/io/Writer;)V", (writer,)).await?.into();
        jvm.invoke_special(&this, "java/util/Formatter", "<init>", "(Ljava/lang/Appendable;)V", (appendable,))
            .await
    }

    async fn init_with_output_stream_encoding(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        stream: ClassInstanceRef<OutputStream>,
        encoding: ClassInstanceRef<String>,
    ) -> Result<()> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/io/OutputStream;Ljava/lang/String;Ljava/util/Locale;)V",
            (stream, encoding, locale),
        )
        .await
    }

    async fn init_with_output_stream_encoding_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        stream: ClassInstanceRef<OutputStream>,
        encoding: ClassInstanceRef<String>,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<()> {
        if stream.is_null() || encoding.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output stream or encoding is null").await);
        }
        let writer: ClassInstanceRef<Writer> = jvm
            .new_class(
                "java/io/OutputStreamWriter",
                "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                (stream, encoding),
            )
            .await?
            .into();
        let appendable: ClassInstanceRef<Appendable> = jvm.new_class("java/io/BufferedWriter", "(Ljava/io/Writer;)V", (writer,)).await?.into();
        jvm.invoke_special(
            &this,
            "java/util/Formatter",
            "<init>",
            "(Ljava/lang/Appendable;Ljava/util/Locale;)V",
            (appendable, locale),
        )
        .await
    }

    async fn locale(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Locale>> {
        let _: ClassInstanceRef<Appendable> = jvm
            .invoke_virtual(&this, "java/util/Formatter", "out", "()Ljava/lang/Appendable;", ())
            .await?;
        jvm.get_field(&this, "l", "Ljava/util/Locale;").await
    }

    async fn out(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Appendable>> {
        let appendable: ClassInstanceRef<Appendable> = jvm.get_field(&this, "a", "Ljava/lang/Appendable;").await?;
        if appendable.is_null() {
            return Err(JavaError::JavaException(
                jvm.new_class("java/util/FormatterClosedException", "()V", ()).await?,
            ));
        }
        Ok(appendable)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let appendable: ClassInstanceRef<Appendable> = jvm
            .invoke_virtual(&this, "java/util/Formatter", "out", "()Ljava/lang/Appendable;", ())
            .await?;
        jvm.invoke_virtual(&appendable, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let appendable: ClassInstanceRef<Appendable> = jvm
            .invoke_virtual(&this, "java/util/Formatter", "out", "()Ljava/lang/Appendable;", ())
            .await?;
        if !jvm.is_instance(&**appendable, "java/io/Flushable") {
            return Ok(());
        }
        match jvm
            .invoke_virtual(&appendable, &appendable.class_definition().name(), "flush", "()V", ())
            .await
        {
            Ok(()) => Ok(()),
            Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                jvm.put_field(
                    &mut this,
                    "lastException",
                    "Ljava/io/IOException;",
                    ClassInstanceRef::<IOException>::from(exception),
                )
                .await
            }
            Err(error) => Err(error),
        }
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let appendable: ClassInstanceRef<Appendable> = jvm.get_field(&this, "a", "Ljava/lang/Appendable;").await?;
        if appendable.is_null() {
            return Ok(());
        }
        let result = if jvm.is_instance(&**appendable, "java/io/Closeable") {
            jvm.invoke_virtual(&appendable, &appendable.class_definition().name(), "close", "()V", ())
                .await
        } else {
            Ok(())
        };
        jvm.put_field(&mut this, "a", "Ljava/lang/Appendable;", ClassInstanceRef::<Appendable>::new(None))
            .await?;
        match result {
            Ok(()) => Ok(()),
            Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                jvm.put_field(
                    &mut this,
                    "lastException",
                    "Ljava/io/IOException;",
                    ClassInstanceRef::<IOException>::from(exception),
                )
                .await
            }
            Err(error) => Err(error),
        }
    }

    async fn io_exception(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<IOException>> {
        jvm.get_field(&this, "lastException", "Ljava/io/IOException;").await
    }

    async fn append_output(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, characters: Vec<JavaChar>) -> Result<()> {
        if characters.is_empty() {
            return Ok(());
        }
        let appendable: ClassInstanceRef<Appendable> = jvm
            .invoke_virtual(this, "java/util/Formatter", "out", "()Ljava/lang/Appendable;", ())
            .await?;
        let text: ClassInstanceRef<CharSequence> = JavaLangString::from_utf16(jvm, characters).await?.into();
        match jvm
            .invoke_virtual::<_, ClassInstanceRef<Appendable>>(
                &appendable,
                &appendable.class_definition().name(),
                "append",
                "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
                (text,),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {
                jvm.put_field(
                    this,
                    "lastException",
                    "Ljava/io/IOException;",
                    ClassInstanceRef::<IOException>::from(exception),
                )
                .await
            }
            Err(error) => Err(error),
        }
    }

    async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        format: ClassInstanceRef<String>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.get_field(&this, "l", "Ljava/util/Locale;").await?;
        jvm.invoke_virtual(
            &this,
            "java/util/Formatter",
            "format",
            "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (locale, format, arguments),
        )
        .await
    }

    async fn format_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        locale: ClassInstanceRef<Locale>,
        format: ClassInstanceRef<String>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<Appendable> = jvm
            .invoke_virtual(&this, "java/util/Formatter", "out", "()Ljava/lang/Appendable;", ())
            .await?;
        if format.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "format is null").await);
        }

        let characters = JavaLangString::to_utf16(jvm, &format).await?;
        let argument_values: Option<Vec<ClassInstanceRef<Object>>> = if arguments.is_null() {
            None
        } else {
            Some(jvm.load_array(&arguments, 0, jvm.array_length(&arguments).await?).await?)
        };
        for validation_only in [true, false] {
            let mut cursor = 0;
            let mut literal_start = 0;
            let mut ordinary_index = 0;
            let mut previous_index = None;

            while cursor < characters.len() {
                if characters[cursor] != '%' as JavaChar {
                    cursor += 1;
                    continue;
                }
                if !validation_only {
                    Self::append_output(jvm, &mut this, characters[literal_start..cursor].to_vec()).await?;
                }
                let specifier_start = cursor;
                cursor += 1;
                if cursor == characters.len() {
                    let conversion = JavaLangString::from_rust_string(jvm, "%").await?;
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/UnknownFormatConversionException", "(Ljava/lang/String;)V", (conversion,))
                            .await?,
                    ));
                }

                let mut argument_index = None;
                let number_start = cursor;
                while cursor < characters.len() && char::from_u32(characters[cursor] as u32).is_some_and(|value| value.is_ascii_digit()) {
                    cursor += 1;
                }
                if cursor > number_start && cursor < characters.len() && characters[cursor] == '$' as JavaChar {
                    let value = RustString::from_utf16_lossy(&characters[number_start..cursor])
                        .parse::<usize>()
                        .unwrap_or(0);
                    argument_index = value.checked_sub(1);
                    cursor += 1;
                } else {
                    cursor = number_start;
                }

                let mut flags = Vec::new();
                while cursor < characters.len() {
                    let flag = char::from_u32(characters[cursor] as u32).unwrap_or('\u{fffd}');
                    if !matches!(flag, '-' | '#' | '+' | ' ' | '0' | ',' | '(' | '<') {
                        break;
                    }
                    if flags.contains(&flag) {
                        let duplicate = JavaLangString::from_rust_string(jvm, &flag.to_string()).await?;
                        return Err(JavaError::JavaException(
                            jvm.new_class("java/util/DuplicateFormatFlagsException", "(Ljava/lang/String;)V", (duplicate,))
                                .await?,
                        ));
                    }
                    flags.push(flag);
                    cursor += 1;
                }

                let width_start = cursor;
                while cursor < characters.len() && char::from_u32(characters[cursor] as u32).is_some_and(|value| value.is_ascii_digit()) {
                    cursor += 1;
                }
                let width = if cursor > width_start {
                    match RustString::from_utf16_lossy(&characters[width_start..cursor]).parse::<i32>() {
                        Ok(width) => Some(width as usize),
                        Err(_) => {
                            return Err(JavaError::JavaException(
                                jvm.new_class("java/util/IllegalFormatWidthException", "(I)V", (-1,)).await?,
                            ));
                        }
                    }
                } else {
                    None
                };

                let precision = if cursor < characters.len() && characters[cursor] == '.' as JavaChar {
                    cursor += 1;
                    let precision_start = cursor;
                    while cursor < characters.len() && char::from_u32(characters[cursor] as u32).is_some_and(|value| value.is_ascii_digit()) {
                        cursor += 1;
                    }
                    if precision_start == cursor {
                        let conversion = JavaLangString::from_rust_string(jvm, ".").await?;
                        return Err(JavaError::JavaException(
                            jvm.new_class("java/util/UnknownFormatConversionException", "(Ljava/lang/String;)V", (conversion,))
                                .await?,
                        ));
                    }
                    match RustString::from_utf16_lossy(&characters[precision_start..cursor]).parse::<i32>() {
                        Ok(precision) => Some(precision as usize),
                        Err(_) => {
                            return Err(JavaError::JavaException(
                                jvm.new_class("java/util/IllegalFormatPrecisionException", "(I)V", (-1,)).await?,
                            ));
                        }
                    }
                } else {
                    None
                };

                if cursor == characters.len() {
                    let conversion = JavaLangString::from_rust_string(jvm, "%").await?;
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/UnknownFormatConversionException", "(Ljava/lang/String;)V", (conversion,))
                            .await?,
                    ));
                }
                let conversion = char::from_u32(characters[cursor] as u32).unwrap_or('\u{fffd}');
                cursor += 1;
                let text = RustString::from_utf16_lossy(&characters[specifier_start..cursor]);
                let specifier = FormatSpecifier {
                    text,
                    argument_index,
                    reuse_previous: flags.contains(&'<'),
                    flags,
                    width,
                    precision,
                    conversion,
                };

                if !matches!(
                    specifier.conversion,
                    's' | 'S' | 'b' | 'B' | 'h' | 'H' | 'c' | 'C' | 'd' | 'o' | 'x' | 'X' | 'e' | 'E' | 'f' | 'g' | 'G' | '%' | 'n'
                ) {
                    let conversion = JavaLangString::from_rust_string(jvm, &specifier.conversion.to_string()).await?;
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/UnknownFormatConversionException", "(Ljava/lang/String;)V", (conversion,))
                            .await?,
                    ));
                }

                if specifier.conversion == '%' {
                    if let Some(precision) = specifier.precision {
                        return Err(JavaError::JavaException(
                            jvm.new_class("java/util/IllegalFormatPrecisionException", "(I)V", (precision as i32,))
                                .await?,
                        ));
                    }
                    let illegal_flags: RustString = specifier.flags.iter().filter(|flag| **flag != '-').collect();
                    if !illegal_flags.is_empty() {
                        let flags = JavaLangString::from_rust_string(jvm, &specifier.flags.iter().collect::<RustString>()).await?;
                        return Err(JavaError::JavaException(
                            jvm.new_class("java/util/IllegalFormatFlagsException", "(Ljava/lang/String;)V", (flags,))
                                .await?,
                        ));
                    }
                    if specifier.flags.contains(&'-') && specifier.width.is_none() {
                        let text = JavaLangString::from_rust_string(jvm, &specifier.text).await?;
                        return Err(JavaError::JavaException(
                            jvm.new_class("java/util/MissingFormatWidthException", "(Ljava/lang/String;)V", (text,))
                                .await?,
                        ));
                    }
                    if !validation_only {
                        let output = Self::apply_width(jvm, vec!['%' as JavaChar], &specifier, ' ' as JavaChar).await?;
                        Self::append_output(jvm, &mut this, output).await?;
                    }
                    literal_start = cursor;
                    continue;
                }
                if specifier.conversion == 'n' {
                    if let Some(precision) = specifier.precision {
                        return Err(JavaError::JavaException(
                            jvm.new_class("java/util/IllegalFormatPrecisionException", "(I)V", (precision as i32,))
                                .await?,
                        ));
                    }
                    if let Some(width) = specifier.width {
                        return Err(JavaError::JavaException(
                            jvm.new_class("java/util/IllegalFormatWidthException", "(I)V", (width as i32,)).await?,
                        ));
                    }
                    if !specifier.flags.is_empty() {
                        let flags = JavaLangString::from_rust_string(jvm, &specifier.flags.iter().collect::<RustString>()).await?;
                        return Err(JavaError::JavaException(
                            jvm.new_class("java/util/IllegalFormatFlagsException", "(Ljava/lang/String;)V", (flags,))
                                .await?,
                        ));
                    }
                    if !validation_only {
                        let key = JavaLangString::from_rust_string(jvm, "line.separator").await?;
                        let separator: ClassInstanceRef<String> = jvm
                            .invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
                            .await?;
                        let line = if separator.is_null() {
                            vec!['\n' as JavaChar]
                        } else {
                            JavaLangString::to_utf16(jvm, &separator).await?
                        };
                        Self::append_output(jvm, &mut this, line).await?;
                    }
                    literal_start = cursor;
                    continue;
                }

                if specifier.flags.contains(&'-') && specifier.width.is_none() {
                    let text = JavaLangString::from_rust_string(jvm, &specifier.text).await?;
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/MissingFormatWidthException", "(Ljava/lang/String;)V", (text,))
                            .await?,
                    ));
                }
                Self::validate_specifier(jvm, &specifier).await?;
                if validation_only {
                    literal_start = cursor;
                    continue;
                }

                let selected_index = if specifier.reuse_previous {
                    previous_index
                } else if let Some(index) = specifier.argument_index {
                    Some(index)
                } else {
                    let index = ordinary_index;
                    ordinary_index += 1;
                    Some(index)
                };
                let Some(selected_index) = selected_index else {
                    let text = JavaLangString::from_rust_string(jvm, &specifier.text).await?;
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/MissingFormatArgumentException", "(Ljava/lang/String;)V", (text,))
                            .await?,
                    ));
                };
                previous_index = Some(selected_index);
                let argument = match &argument_values {
                    None => ClassInstanceRef::<Object>::new(None),
                    Some(values) if selected_index < values.len() => values[selected_index].clone(),
                    Some(_) => {
                        let text = JavaLangString::from_rust_string(jvm, &specifier.text).await?;
                        return Err(JavaError::JavaException(
                            jvm.new_class("java/util/MissingFormatArgumentException", "(Ljava/lang/String;)V", (text,))
                                .await?,
                        ));
                    }
                };
                if let Some(output) = Self::format_argument(jvm, &this, &locale, &specifier, argument).await? {
                    Self::append_output(jvm, &mut this, output).await?;
                }
                literal_start = cursor;
            }
            if !validation_only {
                Self::append_output(jvm, &mut this, characters[literal_start..].to_vec()).await?;
            }
        }
        Ok(this)
    }

    async fn validate_flags(jvm: &Jvm, specifier: &FormatSpecifier, allowed: &str) -> Result<()> {
        let invalid: RustString = specifier.flags.iter().filter(|flag| **flag != '<' && !allowed.contains(**flag)).collect();
        if invalid.is_empty() {
            return Ok(());
        }
        let flags = JavaLangString::from_rust_string(jvm, &invalid).await?;
        Err(JavaError::JavaException(
            jvm.new_class(
                "java/util/FormatFlagsConversionMismatchException",
                "(Ljava/lang/String;C)V",
                (flags, specifier.conversion as JavaChar),
            )
            .await?,
        ))
    }

    async fn validate_specifier(jvm: &Jvm, specifier: &FormatSpecifier) -> Result<()> {
        let conversion = specifier.conversion.to_ascii_lowercase();
        match conversion {
            's' => Self::validate_flags(jvm, specifier, "-#").await,
            'b' | 'h' => Self::validate_flags(jvm, specifier, "-").await,
            'c' => {
                if let Some(precision) = specifier.precision {
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/IllegalFormatPrecisionException", "(I)V", (precision as i32,))
                            .await?,
                    ));
                }
                Self::validate_flags(jvm, specifier, "-").await
            }
            'd' | 'o' | 'x' => {
                if specifier.flags.contains(&'0') && specifier.width.is_none() {
                    let text = JavaLangString::from_rust_string(jvm, &specifier.text).await?;
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/MissingFormatWidthException", "(Ljava/lang/String;)V", (text,))
                            .await?,
                    ));
                }
                Self::validate_numeric_flags(jvm, specifier).await?;
                if let Some(precision) = specifier.precision {
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/IllegalFormatPrecisionException", "(I)V", (precision as i32,))
                            .await?,
                    ));
                }
                Self::validate_flags(jvm, specifier, if conversion == 'd' { "-+ 0,(" } else { "-#+ 0(" }).await
            }
            'e' | 'f' | 'g' => {
                if specifier.flags.contains(&'0') && specifier.width.is_none() {
                    let text = JavaLangString::from_rust_string(jvm, &specifier.text).await?;
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/MissingFormatWidthException", "(Ljava/lang/String;)V", (text,))
                            .await?,
                    ));
                }
                Self::validate_numeric_flags(jvm, specifier).await?;
                Self::validate_flags(
                    jvm,
                    specifier,
                    match conversion {
                        'e' => "-+ 0(#",
                        'f' => "-+ 0,(#",
                        'g' => "-+ 0,(",
                        _ => "",
                    },
                )
                .await
            }
            _ => Ok(()),
        }
    }

    async fn illegal_conversion(jvm: &Jvm, specifier: &FormatSpecifier, argument: &ClassInstanceRef<Object>) -> Result<JavaError> {
        let class_name = argument.class_definition().name();
        let argument_class = jvm.resolve_class(&class_name).await?.java_class();
        Ok(JavaError::JavaException(
            jvm.new_class(
                "java/util/IllegalFormatConversionException",
                "(CLjava/lang/Class;)V",
                (specifier.conversion as JavaChar, argument_class),
            )
            .await?,
        ))
    }

    async fn uppercase_utf16(jvm: &Jvm, text: Vec<JavaChar>) -> Result<Vec<JavaChar>> {
        let Some(capacity) = text.len().checked_mul(6) else {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        };
        let mut output = Vec::new();
        if output.try_reserve_exact(capacity).is_err() {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        }
        for character in char::decode_utf16(text) {
            match character {
                Ok(character) => {
                    for uppercase in character.to_uppercase() {
                        let mut encoded = [0; 2];
                        output.extend_from_slice(uppercase.encode_utf16(&mut encoded));
                    }
                }
                Err(error) => output.push(error.unpaired_surrogate()),
            }
        }
        Ok(output)
    }

    async fn format_argument(
        jvm: &Jvm,
        formatter: &ClassInstanceRef<Self>,
        locale: &ClassInstanceRef<Locale>,
        specifier: &FormatSpecifier,
        argument: ClassInstanceRef<Object>,
    ) -> Result<Option<Vec<JavaChar>>> {
        let conversion = specifier.conversion.to_ascii_lowercase();
        let uppercase = specifier.conversion.is_ascii_uppercase();

        match conversion {
            's' => {
                if !argument.is_null() && jvm.is_instance(&**argument, "java/util/Formattable") {
                    let mut flags = 0;
                    if specifier.flags.contains(&'-') {
                        flags |= 1;
                    }
                    if uppercase {
                        flags |= 2;
                    }
                    if specifier.flags.contains(&'#') {
                        flags |= 4;
                    }
                    let formatter_locale: ClassInstanceRef<Locale> = jvm.get_field(formatter, "l", "Ljava/util/Locale;").await?;
                    let callback_formatter = if (formatter_locale.is_null() && locale.is_null())
                        || (!formatter_locale.is_null() && !locale.is_null() && formatter_locale.identity() == locale.identity())
                    {
                        formatter.clone()
                    } else {
                        let appendable: ClassInstanceRef<Appendable> = jvm
                            .invoke_virtual(formatter, "java/util/Formatter", "out", "()Ljava/lang/Appendable;", ())
                            .await?;
                        jvm.new_class(
                            "java/util/Formatter",
                            "(Ljava/lang/Appendable;Ljava/util/Locale;)V",
                            (appendable, locale.clone()),
                        )
                        .await?
                        .into()
                    };
                    let _: () = jvm
                        .invoke_virtual(
                            &argument,
                            &argument.class_definition().name(),
                            "formatTo",
                            "(Ljava/util/Formatter;III)V",
                            (
                                callback_formatter,
                                flags,
                                specifier.width.map_or(-1, |width| width as i32),
                                specifier.precision.map_or(-1, |precision| precision as i32),
                            ),
                        )
                        .await?;
                    return Ok(None);
                }
                if specifier.flags.contains(&'#') {
                    let flags = JavaLangString::from_rust_string(jvm, "#").await?;
                    return Err(JavaError::JavaException(
                        jvm.new_class(
                            "java/util/FormatFlagsConversionMismatchException",
                            "(Ljava/lang/String;C)V",
                            (flags, specifier.conversion as JavaChar),
                        )
                        .await?,
                    ));
                }
                let mut text = if argument.is_null() {
                    "null".encode_utf16().collect()
                } else {
                    let value: ClassInstanceRef<String> = jvm
                        .invoke_virtual(&argument, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                        .await?;
                    JavaLangString::to_utf16(jvm, &value).await?
                };
                if let Some(precision) = specifier.precision {
                    text.truncate(precision);
                }
                if uppercase {
                    text = Self::uppercase_utf16(jvm, text).await?;
                }
                Ok(Some(Self::apply_width(jvm, text, specifier, ' ' as JavaChar).await?))
            }
            'b' => {
                let value = if argument.is_null() {
                    false
                } else if jvm.is_instance(&**argument, "java/lang/Boolean") {
                    jvm.invoke_virtual(&argument, &argument.class_definition().name(), "booleanValue", "()Z", ())
                        .await?
                } else {
                    true
                };
                let mut text: Vec<JavaChar> = if value { "true" } else { "false" }.encode_utf16().collect();
                if let Some(precision) = specifier.precision {
                    text.truncate(precision);
                }
                if uppercase {
                    text = Self::uppercase_utf16(jvm, text).await?;
                }
                Ok(Some(Self::apply_width(jvm, text, specifier, ' ' as JavaChar).await?))
            }
            'h' => {
                let mut text: Vec<JavaChar> = if argument.is_null() {
                    "null".encode_utf16().collect()
                } else {
                    let hash: i32 = jvm.invoke_virtual(&argument, "java/lang/Object", "hashCode", "()I", ()).await?;
                    format!("{:x}", hash as u32).encode_utf16().collect()
                };
                if let Some(precision) = specifier.precision {
                    text.truncate(precision);
                }
                if uppercase {
                    text = Self::uppercase_utf16(jvm, text).await?;
                }
                Ok(Some(Self::apply_width(jvm, text, specifier, ' ' as JavaChar).await?))
            }
            'c' => {
                if argument.is_null() {
                    let text = if uppercase { "NULL" } else { "null" }.encode_utf16().collect();
                    return Ok(Some(Self::apply_width(jvm, text, specifier, ' ' as JavaChar).await?));
                }
                let code_point = if jvm.is_instance(&**argument, "java/lang/Character") {
                    jvm.invoke_virtual::<_, JavaChar>(&argument, &argument.class_definition().name(), "charValue", "()C", ())
                        .await? as i32
                } else if jvm.is_instance(&**argument, "java/lang/Byte")
                    || jvm.is_instance(&**argument, "java/lang/Short")
                    || jvm.is_instance(&**argument, "java/lang/Integer")
                {
                    jvm.invoke_virtual(&argument, &argument.class_definition().name(), "intValue", "()I", ())
                        .await?
                } else {
                    return Err(Self::illegal_conversion(jvm, specifier, &argument).await?);
                };
                if !(0..=0x10ffff).contains(&code_point) {
                    return Err(JavaError::JavaException(
                        jvm.new_class("java/util/IllegalFormatCodePointException", "(I)V", (code_point,)).await?,
                    ));
                }
                let mut text = if code_point < 0x10000 {
                    vec![code_point as JavaChar]
                } else {
                    let value = code_point - 0x10000;
                    vec![(0xd800 + (value >> 10)) as JavaChar, (0xdc00 + (value & 0x3ff)) as JavaChar]
                };
                if uppercase {
                    text = Self::uppercase_utf16(jvm, text).await?;
                }
                Ok(Some(Self::apply_width(jvm, text, specifier, ' ' as JavaChar).await?))
            }
            'd' | 'o' | 'x' => Self::format_integral(jvm, specifier, argument, uppercase).await.map(Some),
            'e' | 'f' | 'g' => Self::format_floating(jvm, specifier, argument, uppercase).await.map(Some),
            _ => Ok(Some(Vec::new())),
        }
    }

    async fn apply_width(jvm: &Jvm, mut text: Vec<JavaChar>, specifier: &FormatSpecifier, padding: JavaChar) -> Result<Vec<JavaChar>> {
        let width = specifier.width.unwrap_or(0);
        if text.len() >= width {
            return Ok(text);
        }
        let padding_length = width - text.len();
        if specifier.flags.contains(&'-') {
            if text.try_reserve_exact(padding_length).is_err() {
                return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
            }
            text.resize(width, padding);
            Ok(text)
        } else {
            let mut output = Vec::new();
            if output.try_reserve_exact(width).is_err() {
                return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
            }
            output.resize(padding_length, padding);
            output.extend(text);
            Ok(output)
        }
    }

    async fn validate_numeric_flags(jvm: &Jvm, specifier: &FormatSpecifier) -> Result<()> {
        let conflicting =
            (specifier.flags.contains(&'+') && specifier.flags.contains(&' ')) || (specifier.flags.contains(&'-') && specifier.flags.contains(&'0'));
        if !conflicting {
            return Ok(());
        }
        let flags = JavaLangString::from_rust_string(jvm, &specifier.flags.iter().collect::<RustString>()).await?;
        Err(JavaError::JavaException(
            jvm.new_class("java/util/IllegalFormatFlagsException", "(Ljava/lang/String;)V", (flags,))
                .await?,
        ))
    }

    async fn format_integral(jvm: &Jvm, specifier: &FormatSpecifier, argument: ClassInstanceRef<Object>, uppercase: bool) -> Result<Vec<JavaChar>> {
        let conversion = specifier.conversion.to_ascii_lowercase();
        if argument.is_null() {
            return Self::apply_width(
                jvm,
                if uppercase { "NULL" } else { "null" }.encode_utf16().collect(),
                specifier,
                ' ' as JavaChar,
            )
            .await;
        }

        let class_name = argument.class_definition().name();
        let bits = match class_name.as_str() {
            "java/lang/Byte" => 8,
            "java/lang/Short" => 16,
            "java/lang/Integer" => 32,
            "java/lang/Long" => 64,
            _ => return Err(Self::illegal_conversion(jvm, specifier, &argument).await?),
        };
        if conversion != 'd' {
            Self::validate_flags(jvm, specifier, "-#0").await?;
        }
        let value: i64 = jvm
            .invoke_virtual(&argument, &argument.class_definition().name(), "longValue", "()J", ())
            .await?;
        let (digits, negative, prefix) = if conversion == 'd' {
            (value.unsigned_abs().to_string(), value < 0, RustString::new())
        } else {
            let unsigned = match bits {
                8 => value as i8 as u8 as u64,
                16 => value as i16 as u16 as u64,
                32 => value as i32 as u32 as u64,
                _ => value as u64,
            };
            let digits = if conversion == 'o' {
                format!("{unsigned:o}")
            } else if uppercase {
                format!("{unsigned:X}")
            } else {
                format!("{unsigned:x}")
            };
            let prefix = if specifier.flags.contains(&'#') {
                if conversion == 'o' {
                    "0".into()
                } else if uppercase {
                    "0X".into()
                } else {
                    "0x".into()
                }
            } else {
                RustString::new()
            };
            (digits, false, prefix)
        };
        Self::finish_numeric(
            jvm,
            digits,
            negative,
            prefix,
            specifier,
            conversion == 'd' && specifier.flags.contains(&','),
            NumericKind::Finite,
        )
        .await
    }

    async fn format_floating(jvm: &Jvm, specifier: &FormatSpecifier, argument: ClassInstanceRef<Object>, uppercase: bool) -> Result<Vec<JavaChar>> {
        let conversion = specifier.conversion.to_ascii_lowercase();
        if argument.is_null() {
            return Self::apply_width(
                jvm,
                if uppercase { "NULL" } else { "null" }.encode_utf16().collect(),
                specifier,
                ' ' as JavaChar,
            )
            .await;
        }
        if !jvm.is_instance(&**argument, "java/lang/Float") && !jvm.is_instance(&**argument, "java/lang/Double") {
            return Err(Self::illegal_conversion(jvm, specifier, &argument).await?);
        }
        let value: f64 = jvm
            .invoke_virtual(&argument, &argument.class_definition().name(), "doubleValue", "()D", ())
            .await?;
        let negative = value.is_sign_negative() && !value.is_nan();
        let magnitude = libm::fabs(value);
        let precision = specifier.precision.unwrap_or(6);
        let mut digits = if value.is_nan() {
            "NaN".into()
        } else if value.is_infinite() {
            "Infinity".into()
        } else {
            match conversion {
                'f' => Self::decimal(jvm, magnitude, precision).await?,
                'e' => Self::scientific(jvm, magnitude, precision).await?,
                'g' => {
                    let precision = precision.max(1);
                    let exponent = if magnitude == 0.0 {
                        0
                    } else {
                        libm::floor(libm::log10(magnitude)) as i32
                    };
                    if exponent < -4 || exponent >= precision as i32 {
                        Self::scientific(jvm, magnitude, precision - 1).await?
                    } else {
                        let fractional = (precision as i32 - exponent - 1).max(0) as usize;
                        let rounded = Self::round_fraction(magnitude, fractional);
                        let rounded_exponent = if rounded == 0.0 { 0 } else { libm::floor(libm::log10(rounded)) as i32 };
                        if rounded_exponent >= precision as i32 {
                            Self::scientific(jvm, magnitude, precision - 1).await?
                        } else {
                            let fractional = (precision as i32 - rounded_exponent - 1).max(0) as usize;
                            Self::decimal(jvm, magnitude, fractional).await?
                        }
                    }
                }
                _ => RustString::new(),
            }
        };
        if value.is_finite() && specifier.flags.contains(&'#') && !digits.contains('.') {
            if let Some(exponent) = digits.find('e') {
                digits.insert(exponent, '.');
            } else {
                digits.push('.');
            }
        }
        if uppercase {
            digits = digits.to_uppercase();
        }
        let kind = if value.is_nan() {
            NumericKind::Nan
        } else if value.is_infinite() {
            NumericKind::Infinite
        } else {
            NumericKind::Finite
        };
        Self::finish_numeric(
            jvm,
            digits,
            negative,
            RustString::new(),
            specifier,
            specifier.flags.contains(&',') && conversion != 'e',
            kind,
        )
        .await
    }

    async fn decimal(jvm: &Jvm, value: f64, precision: usize) -> Result<RustString> {
        let Some(capacity) = precision.checked_add(400) else {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        };
        let mut output = RustString::new();
        if output.try_reserve_exact(capacity).is_err() {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        }
        let value = Self::round_fraction(value, precision);
        if write!(&mut output, "{value:.precision$}").is_err() {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        }
        Ok(output)
    }

    fn round_fraction(value: f64, precision: usize) -> f64 {
        if precision > 308 {
            return value;
        }
        let factor = libm::pow(10.0, precision as f64);
        let scaled = value * factor;
        if !scaled.is_finite() {
            return value;
        }
        if value == 0.0 {
            return value;
        }
        let exponent = libm::floor(libm::log10(value)) as i32;
        let discarded_index = exponent + precision as i32 + 1;
        let canonical = format!("{value:e}");
        let round_up = discarded_index >= 0
            && canonical
                .bytes()
                .take_while(|byte| *byte != b'e')
                .filter(|byte| byte.is_ascii_digit())
                .nth(discarded_index as usize)
                .is_some_and(|digit| digit >= b'5');
        (libm::floor(scaled) + if round_up { 1.0 } else { 0.0 }) / factor
    }

    async fn scientific(jvm: &Jvm, value: f64, precision: usize) -> Result<RustString> {
        let mut exponent = if value == 0.0 { 0 } else { libm::floor(libm::log10(value)) as i32 };
        let scale = libm::pow(10.0, exponent as f64);
        let mut mantissa = if value == 0.0 {
            0.0
        } else if value.to_bits() == 1 {
            // Java's canonical decimal representation of Double.MIN_VALUE is 4.9E-324.
            4.9
        } else if scale == 0.0 {
            value * 1.0e308 / libm::pow(10.0, (exponent + 308) as f64)
        } else {
            value / scale
        };
        mantissa = Self::round_fraction(mantissa, precision);
        if mantissa >= 10.0 {
            mantissa /= 10.0;
            exponent += 1;
        }
        let mantissa = Self::decimal(jvm, mantissa, precision).await?;
        let exponent_magnitude = exponent.unsigned_abs().to_string();
        let Some(capacity) = mantissa
            .len()
            .checked_add(exponent_magnitude.len())
            .and_then(|length| length.checked_add(3))
        else {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        };
        let mut output = RustString::new();
        if output.try_reserve_exact(capacity).is_err() {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        }
        output.push_str(&mantissa);
        output.push('e');
        output.push(if exponent < 0 { '-' } else { '+' });
        if exponent_magnitude.len() < 2 {
            output.push('0');
        }
        output.push_str(&exponent_magnitude);
        Ok(output)
    }

    async fn finish_numeric(
        jvm: &Jvm,
        mut digits: RustString,
        negative: bool,
        prefix: RustString,
        specifier: &FormatSpecifier,
        grouping: bool,
        kind: NumericKind,
    ) -> Result<Vec<JavaChar>> {
        if grouping && matches!(kind, NumericKind::Finite) {
            let separator = digits.find(['.', 'e', 'E']).unwrap_or(digits.len());
            let integer = &digits[..separator];
            let suffix = &digits[separator..];
            let Some(capacity) = digits.len().checked_add(integer.len().saturating_sub(1) / 3) else {
                return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
            };
            let mut grouped = RustString::new();
            if grouped.try_reserve_exact(capacity).is_err() {
                return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
            }
            for (index, character) in integer.chars().enumerate() {
                if index > 0 && (integer.len() - index).is_multiple_of(3) {
                    grouped.push(',');
                }
                grouped.push(character);
            }
            grouped.push_str(suffix);
            digits = grouped;
        }

        let parentheses = negative && specifier.flags.contains(&'(');
        let sign = if parentheses {
            ""
        } else if negative {
            "-"
        } else if !matches!(kind, NumericKind::Nan) && specifier.flags.contains(&'+') {
            "+"
        } else if !matches!(kind, NumericKind::Nan) && specifier.flags.contains(&' ') {
            " "
        } else {
            ""
        };
        let Some(fixed) = sign
            .len()
            .checked_add(prefix.len())
            .and_then(|length| length.checked_add(usize::from(parentheses) * 2))
        else {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        };
        if specifier.flags.contains(&'0') && !specifier.flags.contains(&'-') && matches!(kind, NumericKind::Finite) {
            let width = specifier.width.unwrap_or(0);
            let Some(current) = fixed.checked_add(digits.len()) else {
                return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
            };
            if current < width {
                let padding_length = width - current;
                let Some(capacity) = padding_length.checked_add(digits.len()) else {
                    return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
                };
                let mut padded = RustString::new();
                if padded.try_reserve_exact(capacity).is_err() {
                    return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
                }
                for _ in 0..padding_length {
                    padded.push('0');
                }
                padded.push_str(&digits);
                digits = padded;
            }
        }

        let Some(capacity) = fixed.checked_add(digits.len()) else {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        };
        let mut result = RustString::new();
        if result.try_reserve_exact(capacity).is_err() {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        }
        if parentheses {
            result.push('(');
        } else {
            result.push_str(sign);
        }
        result.push_str(&prefix);
        result.push_str(&digits);
        if parentheses {
            result.push(')');
        }
        let mut characters = Vec::new();
        if characters.try_reserve_exact(result.len()).is_err() {
            return Err(jvm.exception("java/lang/OutOfMemoryError", "formatted output is too large").await);
        }
        characters.extend(result.encode_utf16());
        Self::apply_width(jvm, characters, specifier, ' ' as JavaChar).await
    }
}
