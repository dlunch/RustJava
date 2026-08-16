use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    classes::java::{
        io::{File, OutputStream, PrintStream},
        lang::{Appendable, Class as JavaClass, Object, String as JavaString},
        util::{FormatterBigDecimalLayoutForm, Locale},
    },
    get_runtime_class_proto,
};
use jvm::{Array, ClassInstanceRef, JavaError, JavaValue, Result, runtime::JavaLangString};
use test_utils::test_jvm;

#[test]
fn java_5_formatter_public_api_is_registered() {
    let formatter = get_runtime_class_proto("java/util/Formatter").expect("Formatter must be registered");
    assert_eq!(formatter.parent_class, Some("java/lang/Object"));
    assert_eq!(formatter.interfaces, vec!["java/io/Closeable", "java/io/Flushable"]);
    assert_eq!(formatter.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL);
    assert_eq!(formatter.methods.len(), 22);
    assert!(!formatter.fields.iter().any(|field| field.access_flags.contains(FieldAccessFlags::PUBLIC)));

    for descriptor in [
        "()V",
        "(Ljava/lang/Appendable;)V",
        "(Ljava/util/Locale;)V",
        "(Ljava/lang/Appendable;Ljava/util/Locale;)V",
        "(Ljava/lang/String;)V",
        "(Ljava/lang/String;Ljava/lang/String;)V",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V",
        "(Ljava/io/File;)V",
        "(Ljava/io/File;Ljava/lang/String;)V",
        "(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V",
        "(Ljava/io/PrintStream;)V",
        "(Ljava/io/OutputStream;)V",
        "(Ljava/io/OutputStream;Ljava/lang/String;)V",
        "(Ljava/io/OutputStream;Ljava/lang/String;Ljava/util/Locale;)V",
    ] {
        let constructor = formatter
            .methods
            .iter()
            .find(|method| method.name == "<init>" && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Formatter{descriptor}"));
        assert_eq!(constructor.access_flags, MethodAccessFlags::PUBLIC);
    }

    for (name, descriptor) in [
        ("locale", "()Ljava/util/Locale;"),
        ("out", "()Ljava/lang/Appendable;"),
        ("toString", "()Ljava/lang/String;"),
        ("flush", "()V"),
        ("close", "()V"),
        ("ioException", "()Ljava/io/IOException;"),
        ("format", "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;"),
        ("format", "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;"),
    ] {
        let method = formatter
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Formatter.{name}{descriptor}"));
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC));
        if name == "format" {
            assert!(method.access_flags.contains(MethodAccessFlags::VARARGS));
        }
    }

    let formattable = get_runtime_class_proto("java/util/Formattable").expect("Formattable must be registered");
    assert_eq!(formattable.parent_class, None);
    assert_eq!(
        formattable.access_flags,
        ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT
    );
    assert!(formattable.interfaces.is_empty());
    assert!(formattable.fields.is_empty());
    assert_eq!(formattable.methods.len(), 1);
    let format_to = formattable
        .methods
        .iter()
        .find(|method| method.name == "formatTo" && method.descriptor == "(Ljava/util/Formatter;III)V")
        .expect("Formattable.formatTo must be registered");
    assert_eq!(format_to.access_flags, MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT);

    let flags = get_runtime_class_proto("java/util/FormattableFlags").expect("FormattableFlags must be registered");
    assert_eq!(flags.parent_class, Some("java/lang/Object"));
    assert!(flags.interfaces.is_empty());
    assert_eq!(flags.access_flags, ClassAccessFlags::PUBLIC);
    assert_eq!(flags.fields.len(), 3);
    assert!(!flags.methods.iter().any(|method| method.access_flags.contains(MethodAccessFlags::PUBLIC)));
    for name in ["LEFT_JUSTIFY", "UPPERCASE", "ALTERNATE"] {
        let field = flags.fields.iter().find(|field| field.name == name && field.descriptor == "I").unwrap();
        assert_eq!(
            field.access_flags,
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL
        );
    }

    let enum_base = get_runtime_class_proto("java/lang/Enum").expect("Enum must be registered");
    assert_eq!(enum_base.parent_class, Some("java/lang/Object"));
    assert_eq!(enum_base.interfaces, vec!["java/lang/Comparable", "java/io/Serializable"]);
    assert_eq!(enum_base.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT);
    assert_eq!(enum_base.methods.len(), 12);
    assert_eq!(enum_base.fields.len(), 2);
    for (name, descriptor, flags) in [
        ("<init>", "(Ljava/lang/String;I)V", MethodAccessFlags::PROTECTED),
        ("name", "()Ljava/lang/String;", MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
        ("ordinal", "()I", MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
        ("toString", "()Ljava/lang/String;", MethodAccessFlags::PUBLIC),
        ("equals", "(Ljava/lang/Object;)Z", MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
        ("hashCode", "()I", MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
        ("clone", "()Ljava/lang/Object;", MethodAccessFlags::PROTECTED | MethodAccessFlags::FINAL),
        ("compareTo", "(Ljava/lang/Enum;)I", MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
        (
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
        ),
        (
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        ),
        ("finalize", "()V", MethodAccessFlags::PROTECTED | MethodAccessFlags::FINAL),
    ] {
        let method = enum_base
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Enum.{name}{descriptor}"));
        assert_eq!(method.access_flags, flags);
    }
    let compare_bridge = enum_base
        .methods
        .iter()
        .find(|method| method.name == "compareTo" && method.descriptor == "(Ljava/lang/Object;)I")
        .expect("Enum.compareTo bridge must be registered");
    assert_eq!(
        compare_bridge.access_flags,
        MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC
    );

    let layout = get_runtime_class_proto("java/util/Formatter$BigDecimalLayoutForm").expect("Formatter.BigDecimalLayoutForm must be registered");
    assert_eq!(layout.parent_class, Some("java/lang/Enum"));
    assert!(layout.interfaces.is_empty());
    assert_eq!(
        layout.access_flags,
        ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL | ClassAccessFlags::ENUM
    );
    assert_eq!(layout.methods.len(), 4);
    assert_eq!(layout.fields.len(), 3);
    for name in ["SCIENTIFIC", "DECIMAL_FLOAT"] {
        let field = layout
            .fields
            .iter()
            .find(|field| field.name == name && field.descriptor == "Ljava/util/Formatter$BigDecimalLayoutForm;")
            .unwrap_or_else(|| panic!("missing Formatter.BigDecimalLayoutForm.{name}"));
        assert_eq!(
            field.access_flags,
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL | FieldAccessFlags::ENUM
        );
    }
    for (name, descriptor) in [
        ("values", "()[Ljava/util/Formatter$BigDecimalLayoutForm;"),
        ("valueOf", "(Ljava/lang/String;)Ljava/util/Formatter$BigDecimalLayoutForm;"),
    ] {
        let method = layout
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Formatter.BigDecimalLayoutForm.{name}{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC);
    }

    for (class, return_type) in [("java/io/PrintStream", "PrintStream"), ("java/io/PrintWriter", "PrintWriter")] {
        let proto = get_runtime_class_proto(class).unwrap_or_else(|| panic!("{class} must be registered"));
        for name in ["printf", "format"] {
            for descriptor in [
                format!("(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/{return_type};"),
                format!("(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/{return_type};"),
            ] {
                let method = proto
                    .methods
                    .iter()
                    .find(|method| method.name == name && method.descriptor == descriptor)
                    .unwrap_or_else(|| panic!("missing {class}.{name}{descriptor}"));
                assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC | MethodAccessFlags::VARARGS);
            }
        }
    }

    let string = get_runtime_class_proto("java/lang/String").expect("String must be registered");
    for descriptor in [
        "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
        "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
    ] {
        let method = string
            .methods
            .iter()
            .find(|method| method.name == "format" && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing String.format{descriptor}"));
        assert_eq!(
            method.access_flags,
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::VARARGS
        );
    }
}

#[test]
fn java_5_formatter_io_types_use_covariant_methods_and_compiler_bridges() {
    let appendable = get_runtime_class_proto("java/lang/Appendable").expect("Appendable must be registered");
    assert_eq!(appendable.methods.len(), 3);
    assert!(appendable.fields.is_empty());

    for class in ["java/io/Closeable", "java/io/Flushable"] {
        let proto = get_runtime_class_proto(class).unwrap_or_else(|| panic!("{class} must be registered"));
        assert_eq!(proto.parent_class, None);
        assert_eq!(
            proto.access_flags,
            ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT
        );
        assert_eq!(proto.methods.len(), 1);
        assert_eq!(proto.methods[0].name, if class == "java/io/Closeable" { "close" } else { "flush" });
        assert_eq!(proto.methods[0].descriptor, "()V");
        assert_eq!(proto.methods[0].access_flags, MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT);
    }

    let output_stream = get_runtime_class_proto("java/io/OutputStream").expect("OutputStream must be registered");
    assert_eq!(output_stream.interfaces, vec!["java/io/Closeable", "java/io/Flushable"]);
    assert_eq!(output_stream.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT);
    assert_eq!(output_stream.methods.len(), 6);
    assert!(output_stream.fields.is_empty());

    let writer = get_runtime_class_proto("java/io/Writer").expect("Writer must be registered");
    assert_eq!(writer.interfaces, vec!["java/lang/Appendable", "java/io/Closeable", "java/io/Flushable"]);
    assert_eq!(writer.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT);
    assert_eq!(writer.methods.len(), 15);
    for descriptor in [
        "(Ljava/lang/CharSequence;)Ljava/io/Writer;",
        "(Ljava/lang/CharSequence;II)Ljava/io/Writer;",
        "(C)Ljava/io/Writer;",
    ] {
        let method = writer
            .methods
            .iter()
            .find(|method| method.name == "append" && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Writer.append{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }
    for descriptor in [
        "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
        "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;",
        "(C)Ljava/lang/Appendable;",
    ] {
        let method = writer
            .methods
            .iter()
            .find(|method| method.name == "append" && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Writer.append bridge {descriptor}"));
        assert_eq!(
            method.access_flags,
            MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC
        );
    }

    let print_stream = get_runtime_class_proto("java/io/PrintStream").expect("PrintStream must be registered");
    assert_eq!(print_stream.methods.len(), 42);
    assert_eq!(print_stream.interfaces, vec!["java/lang/Appendable", "java/io/Closeable"]);
    for (name, descriptor) in [
        ("<init>", "(Ljava/io/OutputStream;)V"),
        ("<init>", "(Ljava/io/OutputStream;Z)V"),
        ("<init>", "(Ljava/io/OutputStream;ZLjava/lang/String;)V"),
        ("<init>", "(Ljava/lang/String;)V"),
        ("<init>", "(Ljava/lang/String;Ljava/lang/String;)V"),
        ("<init>", "(Ljava/io/File;)V"),
        ("<init>", "(Ljava/io/File;Ljava/lang/String;)V"),
        ("flush", "()V"),
        ("close", "()V"),
        ("checkError", "()Z"),
        ("write", "(I)V"),
        ("write", "([BII)V"),
        ("print", "(Z)V"),
        ("print", "(C)V"),
        ("print", "(I)V"),
        ("print", "(J)V"),
        ("print", "(F)V"),
        ("print", "(D)V"),
        ("print", "([C)V"),
        ("print", "(Ljava/lang/String;)V"),
        ("print", "(Ljava/lang/Object;)V"),
        ("println", "()V"),
        ("println", "(Z)V"),
        ("println", "(C)V"),
        ("println", "(I)V"),
        ("println", "(J)V"),
        ("println", "(F)V"),
        ("println", "(D)V"),
        ("println", "([C)V"),
        ("println", "(Ljava/lang/String;)V"),
        ("println", "(Ljava/lang/Object;)V"),
        ("printf", "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;"),
        ("printf", "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;"),
        ("format", "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;"),
        ("format", "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;"),
        ("append", "(Ljava/lang/CharSequence;)Ljava/io/PrintStream;"),
        ("append", "(Ljava/lang/CharSequence;II)Ljava/io/PrintStream;"),
        ("append", "(C)Ljava/io/PrintStream;"),
        ("append", "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;"),
        ("append", "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;"),
        ("append", "(C)Ljava/lang/Appendable;"),
    ] {
        assert_eq!(
            print_stream
                .methods
                .iter()
                .filter(|method| method.name == name && method.descriptor == descriptor)
                .count(),
            1,
            "missing or duplicated PrintStream.{name}{descriptor}"
        );
    }
    let print_writer = get_runtime_class_proto("java/io/PrintWriter").expect("PrintWriter must be registered");
    assert_eq!(print_writer.methods.len(), 49);
    for proto in [&print_stream, &print_writer] {
        let set_error = proto
            .methods
            .iter()
            .find(|method| method.name == "setError" && method.descriptor == "()V")
            .expect("setError must be registered");
        assert_eq!(set_error.access_flags, MethodAccessFlags::PROTECTED);
        assert!(!proto.methods.iter().any(|method| method.name == "clearError"));
    }
    for descriptor in [
        "(Ljava/lang/CharSequence;)Ljava/io/Writer;",
        "(Ljava/lang/CharSequence;II)Ljava/io/Writer;",
        "(C)Ljava/io/Writer;",
        "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
        "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;",
        "(C)Ljava/lang/Appendable;",
    ] {
        let method = print_writer
            .methods
            .iter()
            .find(|method| method.name == "append" && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing PrintWriter.append bridge {descriptor}"));
        assert_eq!(
            method.access_flags,
            MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC
        );
    }
}

#[test]
fn java_5_format_exception_hierarchy_and_public_methods_are_registered() {
    for (class, parent, methods) in [
        (
            "java/util/DuplicateFormatFlagsException",
            "java/util/IllegalFormatException",
            vec![
                ("<init>", "(Ljava/lang/String;)V"),
                ("getFlags", "()Ljava/lang/String;"),
                ("getMessage", "()Ljava/lang/String;"),
            ],
        ),
        (
            "java/util/FormatFlagsConversionMismatchException",
            "java/util/IllegalFormatException",
            vec![
                ("<init>", "(Ljava/lang/String;C)V"),
                ("getFlags", "()Ljava/lang/String;"),
                ("getConversion", "()C"),
                ("getMessage", "()Ljava/lang/String;"),
            ],
        ),
        (
            "java/util/IllegalFormatCodePointException",
            "java/util/IllegalFormatException",
            vec![("<init>", "(I)V"), ("getCodePoint", "()I"), ("getMessage", "()Ljava/lang/String;")],
        ),
        (
            "java/util/IllegalFormatConversionException",
            "java/util/IllegalFormatException",
            vec![
                ("<init>", "(CLjava/lang/Class;)V"),
                ("getConversion", "()C"),
                ("getArgumentClass", "()Ljava/lang/Class;"),
                ("getMessage", "()Ljava/lang/String;"),
            ],
        ),
        (
            "java/util/IllegalFormatFlagsException",
            "java/util/IllegalFormatException",
            vec![
                ("<init>", "(Ljava/lang/String;)V"),
                ("getFlags", "()Ljava/lang/String;"),
                ("getMessage", "()Ljava/lang/String;"),
            ],
        ),
        (
            "java/util/IllegalFormatPrecisionException",
            "java/util/IllegalFormatException",
            vec![("<init>", "(I)V"), ("getPrecision", "()I"), ("getMessage", "()Ljava/lang/String;")],
        ),
        (
            "java/util/IllegalFormatWidthException",
            "java/util/IllegalFormatException",
            vec![("<init>", "(I)V"), ("getWidth", "()I"), ("getMessage", "()Ljava/lang/String;")],
        ),
        (
            "java/util/MissingFormatArgumentException",
            "java/util/IllegalFormatException",
            vec![
                ("<init>", "(Ljava/lang/String;)V"),
                ("getFormatSpecifier", "()Ljava/lang/String;"),
                ("getMessage", "()Ljava/lang/String;"),
            ],
        ),
        (
            "java/util/MissingFormatWidthException",
            "java/util/IllegalFormatException",
            vec![
                ("<init>", "(Ljava/lang/String;)V"),
                ("getFormatSpecifier", "()Ljava/lang/String;"),
                ("getMessage", "()Ljava/lang/String;"),
            ],
        ),
        (
            "java/util/UnknownFormatConversionException",
            "java/util/IllegalFormatException",
            vec![
                ("<init>", "(Ljava/lang/String;)V"),
                ("getConversion", "()Ljava/lang/String;"),
                ("getMessage", "()Ljava/lang/String;"),
            ],
        ),
        (
            "java/util/UnknownFormatFlagsException",
            "java/util/IllegalFormatException",
            vec![
                ("<init>", "(Ljava/lang/String;)V"),
                ("getFlags", "()Ljava/lang/String;"),
                ("getMessage", "()Ljava/lang/String;"),
            ],
        ),
    ] {
        let proto = get_runtime_class_proto(class).unwrap_or_else(|| panic!("{class} must be registered"));
        assert_eq!(proto.parent_class, Some(parent));
        assert_eq!(proto.access_flags, ClassAccessFlags::PUBLIC);
        assert_eq!(proto.methods.len(), methods.len());
        for (name, descriptor) in methods {
            let method = proto
                .methods
                .iter()
                .find(|method| method.name == name && method.descriptor == descriptor)
                .unwrap_or_else(|| panic!("missing {class}.{name}{descriptor}"));
            assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
        }
    }

    let illegal_format = get_runtime_class_proto("java/util/IllegalFormatException").expect("IllegalFormatException must be registered");
    assert_eq!(illegal_format.parent_class, Some("java/lang/IllegalArgumentException"));
    assert_eq!(illegal_format.access_flags, ClassAccessFlags::PUBLIC);
    assert_eq!(illegal_format.methods.len(), 1);
    assert_eq!(illegal_format.methods[0].name, "<init>");
    assert_eq!(illegal_format.methods[0].descriptor, "()V");
    assert_eq!(illegal_format.methods[0].access_flags, MethodAccessFlags::empty());

    let closed = get_runtime_class_proto("java/util/FormatterClosedException").expect("FormatterClosedException must be registered");
    assert_eq!(closed.parent_class, Some("java/lang/IllegalStateException"));
    assert_eq!(closed.access_flags, ClassAccessFlags::PUBLIC);
    assert_eq!(closed.methods.len(), 1);
    assert_eq!(closed.methods[0].name, "<init>");
    assert_eq!(closed.methods[0].descriptor, "()V");
    assert_eq!(closed.methods[0].access_flags, MethodAccessFlags::PUBLIC);
}

#[test]
fn java_5_autoboxing_value_of_overloads_are_registered() {
    for (class, descriptor) in [
        ("java/lang/Boolean", "(Z)Ljava/lang/Boolean;"),
        ("java/lang/Byte", "(B)Ljava/lang/Byte;"),
        ("java/lang/Short", "(S)Ljava/lang/Short;"),
        ("java/lang/Integer", "(I)Ljava/lang/Integer;"),
        ("java/lang/Long", "(J)Ljava/lang/Long;"),
        ("java/lang/Float", "(F)Ljava/lang/Float;"),
        ("java/lang/Double", "(D)Ljava/lang/Double;"),
        ("java/lang/Character", "(C)Ljava/lang/Character;"),
    ] {
        let proto = get_runtime_class_proto(class).unwrap_or_else(|| panic!("{class} must be registered"));
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == "valueOf" && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing {class}.valueOf{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC);
    }
}

#[tokio::test]
async fn format_exception_state_messages_and_null_contracts_match_java_5() -> Result<()> {
    let jvm = test_jvm().await?;

    for (class, getter, value, expected_message) in [
        ("java/util/DuplicateFormatFlagsException", "getFlags", "-", "Flags = '-'"),
        ("java/util/IllegalFormatFlagsException", "getFlags", "+ ", "Flags = '+ '"),
        (
            "java/util/MissingFormatArgumentException",
            "getFormatSpecifier",
            "%2$s",
            "Format specifier '%2$s'",
        ),
        ("java/util/MissingFormatWidthException", "getFormatSpecifier", "%-s", "%-s"),
        ("java/util/UnknownFormatConversionException", "getConversion", "q", "Conversion = 'q'"),
        ("java/util/UnknownFormatFlagsException", "getFlags", "!", "Flags = !"),
    ] {
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let exception = jvm.new_class(class, "(Ljava/lang/String;)V", (value.clone(),)).await?;
        let stored: ClassInstanceRef<JavaString> = jvm
            .invoke_virtual(&exception, &exception.class_definition().name(), getter, "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(
            JavaLangString::to_rust_string(&jvm, &stored).await?,
            JavaLangString::to_rust_string(&jvm, &value).await?
        );
        let message: ClassInstanceRef<JavaString> = jvm
            .invoke_virtual(&exception, &exception.class_definition().name(), "getMessage", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, expected_message);

        let null = ClassInstanceRef::<JavaString>::new(None);
        let result = jvm.new_class(class, "(Ljava/lang/String;)V", (null,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{class} must reject a null constructor argument");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }

    let flags = JavaLangString::from_rust_string(&jvm, "#").await?;
    let mismatch = jvm
        .new_class(
            "java/util/FormatFlagsConversionMismatchException",
            "(Ljava/lang/String;C)V",
            (flags, 'b' as u16),
        )
        .await?;
    let stored_flags: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(&mismatch, &mismatch.class_definition().name(), "getFlags", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &stored_flags).await?, "#");
    assert_eq!(
        jvm.invoke_virtual::<_, u16>(&mismatch, &mismatch.class_definition().name(), "getConversion", "()C", ())
            .await?,
        'b' as u16
    );
    let message: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(&mismatch, &mismatch.class_definition().name(), "getMessage", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, "Conversion = b, Flags = #");
    let null_flags = ClassInstanceRef::<JavaString>::new(None);
    let result = jvm
        .new_class(
            "java/util/FormatFlagsConversionMismatchException",
            "(Ljava/lang/String;C)V",
            (null_flags, 'b' as u16),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("FormatFlagsConversionMismatchException must reject null flags");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let code_point = jvm.new_class("java/util/IllegalFormatCodePointException", "(I)V", (-1,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&code_point, &code_point.class_definition().name(), "getCodePoint", "()I", ())
            .await?,
        -1
    );
    let message: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(
            &code_point,
            &code_point.class_definition().name(),
            "getMessage",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, "Code point = 0xffffffff");

    let string_class: ClassInstanceRef<JavaClass> = jvm.resolve_class("java/lang/String").await?.java_class().into();
    let conversion = jvm
        .new_class(
            "java/util/IllegalFormatConversionException",
            "(CLjava/lang/Class;)V",
            ('d' as u16, string_class.clone()),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, u16>(&conversion, &conversion.class_definition().name(), "getConversion", "()C", ())
            .await?,
        'd' as u16
    );
    let stored_class: ClassInstanceRef<JavaClass> = jvm
        .invoke_virtual(
            &conversion,
            &conversion.class_definition().name(),
            "getArgumentClass",
            "()Ljava/lang/Class;",
            (),
        )
        .await?;
    assert_eq!(stored_class.identity(), string_class.identity());
    let message: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(
            &conversion,
            &conversion.class_definition().name(),
            "getMessage",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, "d != java.lang.String");
    let null_class = ClassInstanceRef::<JavaClass>::new(None);
    let result = jvm
        .new_class(
            "java/util/IllegalFormatConversionException",
            "(CLjava/lang/Class;)V",
            ('d' as u16, null_class),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("IllegalFormatConversionException must reject a null argument class");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (class, getter, value) in [
        ("java/util/IllegalFormatPrecisionException", "getPrecision", -2),
        ("java/util/IllegalFormatWidthException", "getWidth", -3),
    ] {
        let exception = jvm.new_class(class, "(I)V", (value,)).await?;
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&exception, &exception.class_definition().name(), getter, "()I", ())
                .await?,
            value
        );
        let message: ClassInstanceRef<JavaString> = jvm
            .invoke_virtual(&exception, &exception.class_definition().name(), "getMessage", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, value.to_string());
    }

    Ok(())
}

#[tokio::test]
async fn formatter_big_decimal_layout_form_has_java_5_enum_identity_and_order() -> Result<()> {
    let jvm = test_jvm().await?;
    let scientific: ClassInstanceRef<FormatterBigDecimalLayoutForm> = jvm
        .get_static_field(
            "java/util/Formatter$BigDecimalLayoutForm",
            "SCIENTIFIC",
            "Ljava/util/Formatter$BigDecimalLayoutForm;",
        )
        .await?;
    let name = jvm
        .invoke_virtual(
            &scientific,
            "java/util/Formatter$BigDecimalLayoutForm",
            "name",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "SCIENTIFIC");
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&scientific, "java/util/Formatter$BigDecimalLayoutForm", "ordinal", "()I", ())
            .await?,
        0
    );

    let values: ClassInstanceRef<Array<FormatterBigDecimalLayoutForm>> = jvm
        .invoke_static(
            "java/util/Formatter$BigDecimalLayoutForm",
            "values",
            "()[Ljava/util/Formatter$BigDecimalLayoutForm;",
            (),
        )
        .await?;
    assert_eq!(jvm.array_length(&values).await?, 2);
    let requested = JavaLangString::from_rust_string(&jvm, "DECIMAL_FLOAT").await?;
    let decimal: ClassInstanceRef<FormatterBigDecimalLayoutForm> = jvm
        .invoke_static(
            "java/util/Formatter$BigDecimalLayoutForm",
            "valueOf",
            "(Ljava/lang/String;)Ljava/util/Formatter$BigDecimalLayoutForm;",
            (requested,),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&decimal, "java/util/Formatter$BigDecimalLayoutForm", "ordinal", "()I", ())
            .await?,
        1
    );

    Ok(())
}

#[tokio::test]
async fn formatter_formats_common_java_5_conversions() -> Result<()> {
    let jvm = test_jvm().await?;
    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let format = JavaLangString::from_rust_string(&jvm, "%2$-6s %1$04d %3$.2f %% %n").await?;
    let integer = jvm.new_class("java/lang/Integer", "(I)V", (7,)).await?;
    let text = JavaLangString::from_rust_string(&jvm, "ok").await?;
    let double = jvm.new_class("java/lang/Double", "(D)V", (12.5,)).await?;
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 3).await?.into();
    jvm.store_array(
        &mut arguments,
        0,
        vec![JavaValue::from(integer), JavaValue::Object(Some(text)), JavaValue::from(double)],
    )
    .await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await?;
    let result = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "ok     0007 12.50 % \n");

    Ok(())
}

#[tokio::test]
async fn formatter_tracks_explicit_previous_and_ordinary_arguments_independently() -> Result<()> {
    let jvm = test_jvm().await?;
    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let format = JavaLangString::from_rust_string(&jvm, "%2$s|%1$d|%<x|%s").await?;
    let number = jvm.new_class("java/lang/Integer", "(I)V", (26,)).await?;
    let text = JavaLangString::from_rust_string(&jvm, "two").await?;
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut arguments, 0, [JavaValue::Object(Some(number)), JavaValue::Object(Some(text))])
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await?;
    let result = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "two|26|1a|26");

    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let format = JavaLangString::from_rust_string(&jvm, "%2$s|%<b").await?;
    let arguments = ClassInstanceRef::<Array<Object>>::new(None);
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await?;
    let result = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "null|false");

    let format = JavaLangString::from_rust_string(&jvm, "|%+x").await?;
    let arguments = ClassInstanceRef::<Array<Object>>::new(None);
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await?;
    let result = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "null|false|null");

    Ok(())
}

#[tokio::test]
async fn formatter_uppercase_preserves_unpaired_utf16_surrogates() -> Result<()> {
    let jvm = test_jvm().await?;
    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let format = JavaLangString::from_rust_string(&jvm, "%S|%C").await?;
    let string = JavaLangString::from_utf16(&jvm, vec!['a' as u16, 0xd800, 'b' as u16]).await?;
    let character = jvm.new_class("java/lang/Character", "(C)V", (0xd800u16,)).await?;
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut arguments, 0, [JavaValue::Object(Some(string)), JavaValue::Object(Some(character))])
        .await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await?;
    let result = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(
        JavaLangString::to_utf16(&jvm, &result).await?,
        vec!['A' as u16, 0xd800, 'B' as u16, '|' as u16, 0xd800]
    );

    Ok(())
}

#[tokio::test]
async fn print_stream_and_print_writer_use_formatter_for_varargs_output() -> Result<()> {
    let jvm = test_jvm().await?;
    let format = JavaLangString::from_rust_string(&jvm, "%s=%03d").await?;
    let label = JavaLangString::from_rust_string(&jvm, "n").await?;
    let number = jvm.new_class("java/lang/Integer", "(I)V", (7,)).await?;
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut arguments, 0, [JavaValue::Object(Some(label)), JavaValue::Object(Some(number))])
        .await?;

    let bytes = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (bytes.clone(),))
        .await?;
    let returned: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &stream,
            &stream.class_definition().name(),
            "printf",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
            (format.clone(), arguments.clone()),
        )
        .await?;
    assert_eq!(returned.identity(), stream.identity());
    let stream_text = jvm
        .invoke_virtual(&bytes, &bytes.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &stream_text).await?, "n=007");

    let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
        .await?;
    let returned: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &writer,
            &writer.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintWriter;",
            (format, arguments),
        )
        .await?;
    assert_eq!(returned.identity(), writer.identity());
    let writer_text = jvm
        .invoke_virtual(
            &string_writer,
            &string_writer.class_definition().name(),
            "toString",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &writer_text).await?, "n=007");

    Ok(())
}

#[tokio::test]
async fn charset_file_constructors_validate_encoding_before_opening_the_file() -> Result<()> {
    let jvm = test_jvm().await?;
    let path = JavaLangString::from_rust_string(&jvm, "existing.txt").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
    let encoding = JavaLangString::from_rust_string(&jvm, "not-a-charset").await?;
    let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;

    let formatter = jvm
        .new_class(
            "java/util/Formatter",
            "(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V",
            (file.clone(), encoding.clone(), locale),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = formatter else {
        panic!("Formatter must reject the charset before opening the file");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));

    let print_stream = jvm
        .new_class(
            "java/io/PrintStream",
            "(Ljava/io/File;Ljava/lang/String;)V",
            (file.clone(), encoding.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = print_stream else {
        panic!("PrintStream must reject the charset before opening the file");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));

    let print_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/File;Ljava/lang/String;)V", (file, encoding))
        .await;
    let Err(JavaError::JavaException(exception)) = print_writer else {
        panic!("PrintWriter must reject the charset before opening the file");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));

    Ok(())
}

#[tokio::test]
async fn formatter_and_print_file_constructors_enforce_java_5_null_contracts() -> Result<()> {
    let jvm = test_jvm().await?;

    let null_appendable = ClassInstanceRef::<Appendable>::new(None);
    let formatter = jvm
        .new_class("java/util/Formatter", "(Ljava/lang/Appendable;)V", (null_appendable,))
        .await?;
    let out: ClassInstanceRef<Appendable> = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "out", "()Ljava/lang/Appendable;", ())
        .await?;
    assert!(jvm.is_instance(&**out, "java/lang/StringBuilder"));

    let null_locale = ClassInstanceRef::<Locale>::new(None);
    let formatter = jvm.new_class("java/util/Formatter", "(Ljava/util/Locale;)V", (null_locale,)).await?;
    let locale: ClassInstanceRef<Locale> = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "locale", "()Ljava/util/Locale;", ())
        .await?;
    assert!(locale.is_null());

    let null_path = ClassInstanceRef::<JavaString>::new(None);
    let result = jvm.new_class("java/util/Formatter", "(Ljava/lang/String;)V", (null_path.clone(),)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Formatter(String) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null_file = ClassInstanceRef::<File>::new(None);
    let result = jvm.new_class("java/util/Formatter", "(Ljava/io/File;)V", (null_file.clone(),)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Formatter(File) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null_stream = ClassInstanceRef::<OutputStream>::new(None);
    let result = jvm
        .new_class("java/util/Formatter", "(Ljava/io/OutputStream;)V", (null_stream.clone(),))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Formatter(OutputStream) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null_print_stream = ClassInstanceRef::<PrintStream>::new(None);
    let result = jvm
        .new_class("java/util/Formatter", "(Ljava/io/PrintStream;)V", (null_print_stream,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Formatter(PrintStream) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let null_encoding = ClassInstanceRef::<JavaString>::new(None);
    let result = jvm
        .new_class(
            "java/util/Formatter",
            "(Ljava/io/OutputStream;Ljava/lang/String;)V",
            (output, null_encoding.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Formatter(OutputStream, String) must reject a null encoding");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (class, descriptor) in [
        ("java/io/PrintStream", "(Ljava/lang/String;)V"),
        ("java/io/PrintWriter", "(Ljava/lang/String;)V"),
    ] {
        let result = jvm.new_class(class, descriptor, (null_path.clone(),)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{class}{descriptor} must reject null");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }

    for (class, descriptor) in [("java/io/PrintStream", "(Ljava/io/File;)V"), ("java/io/PrintWriter", "(Ljava/io/File;)V")] {
        let result = jvm.new_class(class, descriptor, (null_file.clone(),)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{class}{descriptor} must reject null");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }

    Ok(())
}

#[tokio::test]
async fn formatter_handles_numeric_radix_grouping_and_uppercase_conversions() -> Result<()> {
    let jvm = test_jvm().await?;
    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let format = JavaLangString::from_rust_string(&jvm, "%1$,(d|%2$x|%2$o|%3$.2e|%3$.4g|%4$C|%5$B").await?;
    let negative = jvm.new_class("java/lang/Integer", "(I)V", (-1234,)).await?;
    let minus_one = jvm.new_class("java/lang/Integer", "(I)V", (-1,)).await?;
    let floating = jvm.new_class("java/lang/Double", "(D)V", (1234.0,)).await?;
    let character = jvm.new_class("java/lang/Character", "(C)V", ('a' as u16,)).await?;
    let boolean = jvm.new_class("java/lang/Boolean", "(Z)V", (true,)).await?;
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 5).await?.into();
    jvm.store_array(
        &mut arguments,
        0,
        vec![
            JavaValue::from(negative),
            JavaValue::from(minus_one),
            JavaValue::from(floating),
            JavaValue::from(character),
            JavaValue::from(boolean),
        ],
    )
    .await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await?;
    let result = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(
        JavaLangString::to_rust_string(&jvm, &result).await?,
        "(1,234)|ffffffff|37777777777|1.23e+03|1234|A|TRUE"
    );

    Ok(())
}

#[tokio::test]
async fn formatter_matches_java_5_radix_and_non_finite_number_rules() -> Result<()> {
    let jvm = test_jvm().await?;
    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let format = JavaLangString::from_rust_string(&jvm, "%1$#o|%2$#05x|%3$#.0e|%4$+f|%5$010f").await?;
    let zero = jvm.new_class("java/lang/Integer", "(I)V", (0,)).await?;
    let fifteen = jvm.new_class("java/lang/Integer", "(I)V", (15,)).await?;
    let one = jvm.new_class("java/lang/Double", "(D)V", (1.0,)).await?;
    let nan = jvm.new_class("java/lang/Double", "(D)V", (f64::NAN,)).await?;
    let infinity = jvm.new_class("java/lang/Double", "(D)V", (f64::INFINITY,)).await?;
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 5).await?.into();
    jvm.store_array(
        &mut arguments,
        0,
        [
            JavaValue::Object(Some(zero)),
            JavaValue::Object(Some(fifteen)),
            JavaValue::Object(Some(one)),
            JavaValue::Object(Some(nan)),
            JavaValue::Object(Some(infinity)),
        ],
    )
    .await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await?;
    let result = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "00|0x00f|1.e+00|NaN|  Infinity");

    for (format, expected) in [
        ("%+x", "java/util/FormatFlagsConversionMismatchException"),
        ("%+ x", "java/util/IllegalFormatFlagsException"),
    ] {
        let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
        let format = JavaLangString::from_rust_string(&jvm, format).await?;
        let value = jvm.new_class("java/lang/Integer", "(I)V", (15,)).await?;
        let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
        jvm.store_array(&mut arguments, 0, [JavaValue::Object(Some(value))]).await?;
        let result: Result<ClassInstanceRef<Object>> = jvm
            .invoke_virtual(
                &formatter,
                &formatter.class_definition().name(),
                "format",
                "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
                (format, arguments),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{expected} must be thrown");
        };
        assert!(jvm.is_instance(&*exception, expected));
    }

    Ok(())
}

#[tokio::test]
async fn formatter_rounds_fixed_scientific_and_general_values_like_java_5() -> Result<()> {
    let jvm = test_jvm().await?;
    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let format = JavaLangString::from_rust_string(&jvm, "%.2f|%.3f|%.0f|%.2e|%.6g|%.6g|%.6g|%.6g|%.4g|%.4g|%g|%g|%.2e|%.3g|%.2e|%.0f|%.2f").await?;
    let values = [
        2.675,
        9.9995,
        0.5,
        9.995,
        0.0,
        0.0000999999,
        0.00001,
        999999.9,
        9999.5,
        9.9995,
        -0.0,
        1.2,
        f64::MIN_POSITIVE * f64::EPSILON,
        f64::MIN_POSITIVE * f64::EPSILON,
        f64::MAX,
        2.4999999999999996,
        1.005,
    ];
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", values.len()).await?.into();
    for (index, value) in values.into_iter().enumerate() {
        let value = jvm.new_class("java/lang/Double", "(D)V", (value,)).await?;
        jvm.store_array(&mut arguments, index, [JavaValue::Object(Some(value))]).await?;
    }

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await?;
    let result = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(
        JavaLangString::to_rust_string(&jvm, &result).await?,
        "2.68|10.000|1|1.00e+01|0.00000|9.99999e-05|1.00000e-05|1.00000e+06|1.000e+04|10.00|-0.00000|1.20000|4.90e-324|4.90e-324|1.80e+308|2|1.01"
    );

    Ok(())
}

#[tokio::test]
async fn formatter_throws_java_5_format_exceptions_and_rejects_use_after_close() -> Result<()> {
    let jvm = test_jvm().await?;

    for (format, expected) in [
        ("%2$s", "java/util/MissingFormatArgumentException"),
        ("%q", "java/util/UnknownFormatConversionException"),
        ("%D", "java/util/UnknownFormatConversionException"),
        ("%O", "java/util/UnknownFormatConversionException"),
        ("%F", "java/util/UnknownFormatConversionException"),
        ("%-0d", "java/util/MissingFormatWidthException"),
        ("%-n", "java/util/IllegalFormatFlagsException"),
        ("%2$#b", "java/util/FormatFlagsConversionMismatchException"),
        ("%2$.2c", "java/util/IllegalFormatPrecisionException"),
        ("%2$+x", "java/util/MissingFormatArgumentException"),
        ("%2147483648s", "java/util/IllegalFormatWidthException"),
        ("%.2147483648s", "java/util/IllegalFormatPrecisionException"),
    ] {
        let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
        let format = JavaLangString::from_rust_string(&jvm, format).await?;
        let arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
        let result: Result<ClassInstanceRef<Object>> = jvm
            .invoke_virtual(
                &formatter,
                &formatter.class_definition().name(),
                "format",
                "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
                (format, arguments),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{expected} must be thrown");
        };
        assert!(jvm.is_instance(&*exception, expected));
    }

    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "close", "()V", ())
        .await?;
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "out", "()Ljava/lang/Appendable;", ())
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("FormatterClosedException must be thrown");
    };
    assert!(jvm.is_instance(&*exception, "java/util/FormatterClosedException"));

    Ok(())
}

#[tokio::test]
async fn formatter_validates_all_syntax_before_writing_but_preserves_runtime_partial_output() -> Result<()> {
    let jvm = test_jvm().await?;

    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let format = JavaLangString::from_rust_string(&jvm, "before %s after %q").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    jvm.store_array(&mut arguments, 0, [JavaValue::Object(Some(value))]).await?;
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("the trailing unknown conversion must fail");
    };
    assert!(jvm.is_instance(&*exception, "java/util/UnknownFormatConversionException"));
    let text = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "");

    let formatter = jvm.new_class("java/util/Formatter", "()V", ()).await?;
    let format = JavaLangString::from_rust_string(&jvm, "before %d").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    jvm.store_array(&mut arguments, 0, [JavaValue::Object(Some(value))]).await?;
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &formatter,
            &formatter.class_definition().name(),
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("the incompatible runtime argument must fail");
    };
    assert!(jvm.is_instance(&*exception, "java/util/IllegalFormatConversionException"));
    let text = jvm
        .invoke_virtual(&formatter, &formatter.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "before ");

    Ok(())
}
