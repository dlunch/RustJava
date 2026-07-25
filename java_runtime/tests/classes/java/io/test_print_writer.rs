use alloc::{boxed::Box, collections::BTreeMap, format, vec, vec::Vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{OutputStream, PrintWriter, Writer},
        lang::{Object, String, StringBuffer},
    },
};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct ProbeWriter;

impl ProbeWriter {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ProbeWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(III)V", Self::init, Default::default()),
                JavaMethodProto::new("write", "([CII)V", Self::write, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("content", "Ljava/lang/StringBuffer;", Default::default()),
                JavaFieldProto::new("writeMode", "I", Default::default()),
                JavaFieldProto::new("flushMode", "I", Default::default()),
                JavaFieldProto::new("closeMode", "I", Default::default()),
                JavaFieldProto::new("writeCount", "I", Default::default()),
                JavaFieldProto::new("flushCount", "I", Default::default()),
                JavaFieldProto::new("closeCount", "I", Default::default()),
                JavaFieldProto::new("blockFirstWrite", "Z", Default::default()),
                JavaFieldProto::new("firstWriteEntered", "Z", Default::default()),
                JavaFieldProto::new("releaseFirstWrite", "Z", Default::default()),
                JavaFieldProto::new("blockFirstClose", "Z", Default::default()),
                JavaFieldProto::new("firstCloseEntered", "Z", Default::default()),
                JavaFieldProto::new("releaseFirstClose", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        write_mode: i32,
        flush_mode: i32,
        close_mode: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;
        let content = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
        jvm.put_field(&mut this, "content", "Ljava/lang/StringBuffer;", content).await?;
        jvm.put_field(&mut this, "writeMode", "I", write_mode).await?;
        jvm.put_field(&mut this, "flushMode", "I", flush_mode).await?;
        jvm.put_field(&mut this, "closeMode", "I", close_mode).await?;
        jvm.put_field(&mut this, "writeCount", "I", 0).await?;
        jvm.put_field(&mut this, "flushCount", "I", 0).await?;
        jvm.put_field(&mut this, "closeCount", "I", 0).await?;
        jvm.put_field(&mut this, "blockFirstWrite", "Z", false).await?;
        jvm.put_field(&mut this, "firstWriteEntered", "Z", false).await?;
        jvm.put_field(&mut this, "releaseFirstWrite", "Z", false).await?;
        jvm.put_field(&mut this, "blockFirstClose", "Z", false).await?;
        jvm.put_field(&mut this, "firstCloseEntered", "Z", false).await?;
        jvm.put_field(&mut this, "releaseFirstClose", "Z", false).await
    }

    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        let write_count: i32 = jvm.get_field(&this, "writeCount", "I").await?;
        jvm.put_field(&mut this, "writeCount", "I", write_count + 1).await?;
        match jvm.get_field::<i32>(&this, "writeMode", "I").await? {
            1 => return Err(jvm.exception("java/io/IOException", "write failed").await),
            2 => return Err(jvm.exception("java/lang/IllegalStateException", "write failed").await),
            _ => {}
        }
        if write_count == 0 && jvm.get_field::<bool>(&this, "blockFirstWrite", "Z").await? {
            jvm.put_field(&mut this, "firstWriteEntered", "Z", true).await?;
            for _ in 0..1000 {
                if jvm.get_field::<bool>(&this, "releaseFirstWrite", "Z").await? {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
            if !jvm.get_field::<bool>(&this, "releaseFirstWrite", "Z").await? {
                return Err(jvm
                    .exception("java/lang/IllegalStateException", "timed out waiting for write release")
                    .await);
            }
        }

        let content: ClassInstanceRef<StringBuffer> = jvm.get_field(&this, "content", "Ljava/lang/StringBuffer;").await?;
        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(&content, "append", "([CII)Ljava/lang/StringBuffer;", (chars, offset, length))
            .await?;
        Ok(())
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let flush_count: i32 = jvm.get_field(&this, "flushCount", "I").await?;
        jvm.put_field(&mut this, "flushCount", "I", flush_count + 1).await?;
        match jvm.get_field::<i32>(&this, "flushMode", "I").await? {
            1 => Err(jvm.exception("java/io/IOException", "flush failed").await),
            2 => Err(jvm.exception("java/lang/IllegalStateException", "flush failed").await),
            _ => Ok(()),
        }
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let close_count: i32 = jvm.get_field(&this, "closeCount", "I").await?;
        jvm.put_field(&mut this, "closeCount", "I", close_count + 1).await?;
        if close_count == 0 && jvm.get_field::<bool>(&this, "blockFirstClose", "Z").await? {
            jvm.put_field(&mut this, "firstCloseEntered", "Z", true).await?;
            for _ in 0..1000 {
                if jvm.get_field::<bool>(&this, "releaseFirstClose", "Z").await? {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
            if !jvm.get_field::<bool>(&this, "releaseFirstClose", "Z").await? {
                return Err(jvm
                    .exception("java/lang/IllegalStateException", "timed out waiting for close release")
                    .await);
            }
        }
        match jvm.get_field::<i32>(&this, "closeMode", "I").await? {
            1 => Err(jvm.exception("java/io/IOException", "close failed").await),
            2 => Err(jvm.exception("java/lang/IllegalStateException", "close failed").await),
            _ => Ok(()),
        }
    }
}

struct OverridePrintWriter;

impl OverridePrintWriter {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "OverridePrintWriter",
            parent_class: Some("java/io/PrintWriter"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/Writer;)V", Self::init, Default::default()),
                JavaMethodProto::new("print", "(I)V", Self::print_int, Default::default()),
                JavaMethodProto::new("println", "()V", Self::println, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, writer: ClassInstanceRef<Writer>) -> Result<()> {
        jvm.invoke_special(&this, "java/io/PrintWriter", "<init>", "(Ljava/io/Writer;)V", (writer,))
            .await
    }

    async fn print_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = JavaLangString::from_rust_string(jvm, &format!("<{value}>")).await?;
        jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;)V", (value,)).await
    }

    async fn println(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let value = JavaLangString::from_rust_string(jvm, "<newline>").await?;
        jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;)V", (value,)).await
    }
}

struct PrintWriterRunner;

impl PrintWriterRunner {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "PrintWriterRunner",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/PrintWriter;I)V", Self::init, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("writer", "Ljava/io/PrintWriter;", Default::default()),
                JavaFieldProto::new("value", "I", Default::default()),
                JavaFieldProto::new("started", "Z", Default::default()),
                JavaFieldProto::new("done", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        writer: ClassInstanceRef<PrintWriter>,
        value: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "writer", "Ljava/io/PrintWriter;", writer).await?;
        jvm.put_field(&mut this, "value", "I", value).await?;
        jvm.put_field(&mut this, "started", "Z", false).await?;
        jvm.put_field(&mut this, "done", "Z", false).await
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "started", "Z", true).await?;
        let writer: ClassInstanceRef<PrintWriter> = jvm.get_field(&this, "writer", "Ljava/io/PrintWriter;").await?;
        let value: i32 = jvm.get_field(&this, "value", "I").await?;
        let _: () = jvm.invoke_virtual(&writer, "println", "(I)V", (value,)).await?;
        jvm.put_field(&mut this, "done", "Z", true).await
    }
}

struct PrintWriterCloseRunner;

impl PrintWriterCloseRunner {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "PrintWriterCloseRunner",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/PrintWriter;)V", Self::init, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("writer", "Ljava/io/PrintWriter;", Default::default()),
                JavaFieldProto::new("started", "Z", Default::default()),
                JavaFieldProto::new("done", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, writer: ClassInstanceRef<PrintWriter>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "writer", "Ljava/io/PrintWriter;", writer).await?;
        jvm.put_field(&mut this, "started", "Z", false).await?;
        jvm.put_field(&mut this, "done", "Z", false).await
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "started", "Z", true).await?;
        let writer: ClassInstanceRef<PrintWriter> = jvm.get_field(&this, "writer", "Ljava/io/PrintWriter;").await?;
        let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;
        jvm.put_field(&mut this, "done", "Z", true).await
    }
}

async fn probe_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            ProbeWriter::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            OverridePrintWriter::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            PrintWriterRunner::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            PrintWriterCloseRunner::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;
    Ok(jvm)
}

async fn assert_monitor_released(jvm: &Jvm, writer: &ClassInstanceRef<Writer>) -> Result<()> {
    jvm.monitor_enter(writer).await?;
    jvm.monitor_exit(writer).await?;
    let result = jvm.monitor_exit(writer).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("PrintWriter leaked a reentrant Writer.lock acquisition");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalMonitorStateException"));
    Ok(())
}

#[tokio::test]
async fn pw_01_constructors_fields_descriptors_and_access_flags() -> Result<()> {
    let proto = PrintWriter::as_proto();
    assert_eq!(proto.parent_class, Some("java/io/Writer"));
    assert!(proto.access_flags.contains(ClassAccessFlags::PUBLIC));

    let expected_methods = [
        ("<init>", "(Ljava/io/Writer;)V"),
        ("<init>", "(Ljava/io/Writer;Z)V"),
        ("<init>", "(Ljava/io/OutputStream;)V"),
        ("<init>", "(Ljava/io/OutputStream;Z)V"),
        ("write", "(I)V"),
        ("write", "([C)V"),
        ("write", "([CII)V"),
        ("write", "(Ljava/lang/String;)V"),
        ("write", "(Ljava/lang/String;II)V"),
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
        ("flush", "()V"),
        ("close", "()V"),
        ("checkError", "()Z"),
    ];
    assert_eq!(proto.methods.len(), expected_methods.len());
    for (name, descriptor) in expected_methods {
        let methods = proto
            .methods
            .iter()
            .filter(|method| method.name == name && method.descriptor == descriptor)
            .collect::<Vec<_>>();
        assert_eq!(methods.len(), 1, "missing or duplicated {name}{descriptor}");
        assert!(methods[0].access_flags.contains(MethodAccessFlags::PUBLIC));
    }

    assert_eq!(proto.fields.len(), 3);
    let out = proto.fields.iter().find(|field| field.name == "out").expect("out field");
    assert_eq!(out.descriptor, "Ljava/io/Writer;");
    assert_eq!(out.access_flags, FieldAccessFlags::PROTECTED);
    let auto_flush = proto.fields.iter().find(|field| field.name == "autoFlush").expect("autoFlush field");
    assert_eq!(auto_flush.descriptor, "Z");
    assert_eq!(auto_flush.access_flags, FieldAccessFlags::PRIVATE);
    let trouble = proto.fields.iter().find(|field| field.name == "trouble").expect("trouble field");
    assert_eq!(trouble.descriptor, "Z");
    assert_eq!(trouble.access_flags, FieldAccessFlags::PRIVATE);

    let jvm = probe_jvm().await?;
    let writer: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 0, 0)).await?.into();
    let default_writer = jvm.new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (writer.clone(),)).await?;
    assert!(!jvm.get_field::<bool>(&default_writer, "autoFlush", "Z").await?);
    assert!(!jvm.get_field::<bool>(&default_writer, "trouble", "Z").await?);
    let lock: ClassInstanceRef<Object> = jvm.get_field(&default_writer, "lock", "Ljava/lang/Object;").await?;
    assert_eq!(lock.identity(), writer.identity());
    let stored_out: ClassInstanceRef<Writer> = jvm.get_field(&default_writer, "out", "Ljava/io/Writer;").await?;
    assert_eq!(stored_out.identity(), writer.identity());
    let configured_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;Z)V", (writer.clone(), true))
        .await?;
    assert!(jvm.get_field::<bool>(&configured_writer, "autoFlush", "Z").await?);

    let output: ClassInstanceRef<OutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let output_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/OutputStream;Z)V", (output.clone(), true))
        .await?;
    assert!(jvm.get_field::<bool>(&output_writer, "autoFlush", "Z").await?);
    let default_output_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    assert!(!jvm.get_field::<bool>(&default_output_writer, "autoFlush", "Z").await?);
    let output_text = JavaLangString::from_rust_string(&jvm, "os").await?;
    let _: () = jvm
        .invoke_virtual(&default_output_writer, "write", "(Ljava/lang/String;)V", (output_text,))
        .await?;
    let _: () = jvm.invoke_virtual(&default_output_writer, "flush", "()V", ()).await?;
    let output_bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    assert_eq!(jvm.load_array::<i8>(&output_bytes, 0, 2).await?, [b'o' as i8, b's' as i8]);

    let null_writer: ClassInstanceRef<Writer> = None.into();
    let null_writer_result = jvm.new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (null_writer,)).await;
    let Err(JavaError::JavaException(exception)) = null_writer_result else {
        panic!("null Writer must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null_output: ClassInstanceRef<OutputStream> = None.into();
    let null_output_result = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/OutputStream;Z)V", (null_output, false))
        .await;
    let Err(JavaError::JavaException(exception)) = null_output_result else {
        panic!("null OutputStream must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn pw_02_write_overloads_suppress_only_ioexception() -> Result<()> {
    let jvm = probe_jvm().await?;
    let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
        .await?;

    let _: () = jvm.invoke_virtual(&writer, "write", "(I)V", ('A' as i32,)).await?;
    let mut chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars, 0, ['B' as JavaChar, 'C' as JavaChar, 'D' as JavaChar])
        .await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "([C)V", (chars.clone(),)).await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "([CII)V", (chars.clone(), 1, 1)).await?;
    let text = JavaLangString::from_rust_string(&jvm, "EFG").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (text.clone(),)).await?;
    let _: () = jvm
        .invoke_virtual(&writer, "write", "(Ljava/lang/String;II)V", (text.clone(), 1, 1))
        .await?;
    let result: ClassInstanceRef<String> = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "ABCDCEFGF");

    let invalid_chars: Result<()> = jvm.invoke_virtual(&writer, "write", "([CII)V", (chars.clone(), -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = invalid_chars else {
        panic!("invalid char range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    let invalid_string: Result<()> = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;II)V", (text, 2, 2)).await;
    let Err(JavaError::JavaException(exception)) = invalid_string else {
        panic!("invalid String range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    assert!(!jvm.get_field::<bool>(&writer, "trouble", "Z").await?);

    let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
    let null_chars_result: Result<()> = jvm.invoke_virtual(&writer, "write", "([C)V", (null_chars,)).await;
    let Err(JavaError::JavaException(exception)) = null_chars_result else {
        panic!("null char array must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    let null_string: ClassInstanceRef<String> = None.into();
    let null_string_result: Result<()> = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (null_string,)).await;
    let Err(JavaError::JavaException(exception)) = null_string_result else {
        panic!("null String must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let failing_writer: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (1, 0, 0)).await?.into();
    let suppressing = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (failing_writer.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&suppressing, "write", "(I)V", ('x' as i32,)).await?;
    assert!(jvm.get_field::<bool>(&suppressing, "trouble", "Z").await?);
    assert_monitor_released(&jvm, &failing_writer).await?;

    let runtime_writer: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (2, 0, 0)).await?.into();
    let propagating = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (runtime_writer.clone(),))
        .await?;
    let runtime_result: Result<()> = jvm.invoke_virtual(&propagating, "write", "(I)V", ('x' as i32,)).await;
    let Err(JavaError::JavaException(exception)) = runtime_result else {
        panic!("user RuntimeException must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert!(!jvm.get_field::<bool>(&propagating, "trouble", "Z").await?);
    assert_monitor_released(&jvm, &runtime_writer).await?;

    Ok(())
}

#[tokio::test]
async fn pw_03_print_overloads_follow_string_value_of_and_null_rules() -> Result<()> {
    let jvm = test_jvm().await?;
    let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
        .await?;

    let _: () = jvm.invoke_virtual(&writer, "print", "(Z)V", (true,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "print", "(C)V", ('|' as JavaChar,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "print", "(I)V", (-2,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "print", "(J)V", (3i64,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "print", "(F)V", (1.5f32,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "print", "(D)V", (f64::INFINITY,)).await?;
    let mut chars = jvm.instantiate_array("C", 2).await?;
    jvm.store_array(&mut chars, 0, ['X' as JavaChar, 'Y' as JavaChar]).await?;
    let _: () = jvm.invoke_virtual(&writer, "print", "([C)V", (chars,)).await?;

    let null_string: ClassInstanceRef<String> = None.into();
    let _: () = jvm.invoke_virtual(&writer, "print", "(Ljava/lang/String;)V", (null_string,)).await?;
    let object_string = JavaLangString::from_rust_string(&jvm, "obj").await?;
    let object: ClassInstanceRef<Object> = object_string.into();
    let _: () = jvm.invoke_virtual(&writer, "print", "(Ljava/lang/Object;)V", (object,)).await?;
    let null_object: ClassInstanceRef<Object> = None.into();
    let _: () = jvm.invoke_virtual(&writer, "print", "(Ljava/lang/Object;)V", (null_object,)).await?;

    let result: ClassInstanceRef<String> = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "true|-231.5InfinityXYnullobjnull");

    Ok(())
}

#[tokio::test]
async fn pw_04_println_uses_line_separator_and_is_the_only_autoflush_path() -> Result<()> {
    let jvm = probe_jvm().await?;
    let key = JavaLangString::from_rust_string(&jvm, "line.separator").await?;
    let separator = JavaLangString::from_rust_string(&jvm, "\r\n").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key, separator),
        )
        .await?;

    let probe: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 0, 0)).await?.into();
    let writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;Z)V", (probe.clone(), true))
        .await?;
    let prefix = JavaLangString::from_rust_string(&jvm, "p").await?;
    let _: () = jvm.invoke_virtual(&writer, "print", "(Ljava/lang/String;)V", (prefix,)).await?;
    assert_eq!(jvm.get_field::<i32>(&probe, "flushCount", "I").await?, 0);
    let newline = JavaLangString::from_rust_string(&jvm, "\n").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (newline,)).await?;
    assert_eq!(jvm.get_field::<i32>(&probe, "flushCount", "I").await?, 0);
    let _: () = jvm.invoke_virtual(&writer, "println", "(I)V", (7,)).await?;
    assert_eq!(jvm.get_field::<i32>(&probe, "flushCount", "I").await?, 1);
    let _: () = jvm.invoke_virtual(&writer, "println", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&probe, "flushCount", "I").await?, 2);

    let content: ClassInstanceRef<StringBuffer> = jvm.get_field(&probe, "content", "Ljava/lang/StringBuffer;").await?;
    let content: ClassInstanceRef<String> = jvm.invoke_virtual(&content, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &content).await?, "p\n7\r\n\r\n");

    let no_flush_probe: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 0, 0)).await?.into();
    let no_flush_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;Z)V", (no_flush_probe.clone(), false))
        .await?;
    let _: () = jvm.invoke_virtual(&no_flush_writer, "println", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&no_flush_probe, "flushCount", "I").await?, 0);

    let newline_failure: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (1, 0, 0)).await?.into();
    let newline_failure_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;Z)V", (newline_failure.clone(), true))
        .await?;
    let _: () = jvm.invoke_virtual(&newline_failure_writer, "println", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&newline_failure, "writeCount", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&newline_failure, "flushCount", "I").await?, 0);
    assert!(jvm.get_field::<bool>(&newline_failure_writer, "trouble", "Z").await?);
    assert_monitor_released(&jvm, &newline_failure).await?;

    let fallback_jvm = test_jvm().await?;
    let fallback_output = fallback_jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let fallback_writer = fallback_jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (fallback_output.clone(),))
        .await?;
    let _: () = fallback_jvm.invoke_virtual(&fallback_writer, "println", "()V", ()).await?;
    let fallback: ClassInstanceRef<String> = fallback_jvm
        .invoke_virtual(&fallback_output, "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&fallback_jvm, &fallback).await?, "\n");

    Ok(())
}

#[tokio::test]
async fn pw_04_println_is_atomic_and_uses_java_virtual_dispatch() -> Result<()> {
    let jvm = probe_jvm().await?;

    let dispatch_probe: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 0, 0)).await?.into();
    let overriding_writer = jvm
        .new_class("OverridePrintWriter", "(Ljava/io/Writer;)V", (dispatch_probe.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&overriding_writer, "println", "(I)V", (7,)).await?;
    let dispatch_content: ClassInstanceRef<StringBuffer> = jvm.get_field(&dispatch_probe, "content", "Ljava/lang/StringBuffer;").await?;
    let dispatch_content: ClassInstanceRef<String> = jvm.invoke_virtual(&dispatch_content, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &dispatch_content).await?, "<7><newline>");

    let mut blocking_probe: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 0, 0)).await?.into();
    jvm.put_field(&mut blocking_probe, "blockFirstWrite", "Z", true).await?;
    let writer: ClassInstanceRef<PrintWriter> = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (blocking_probe.clone(),))
        .await?
        .into();
    let first_runner = jvm
        .new_class("PrintWriterRunner", "(Ljava/io/PrintWriter;I)V", (writer.clone(), 1))
        .await?;
    let second_runner = jvm.new_class("PrintWriterRunner", "(Ljava/io/PrintWriter;I)V", (writer, 2)).await?;
    let first_thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (first_runner.clone(),))
        .await?;
    let second_thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (second_runner.clone(),))
        .await?;

    let _: () = jvm.invoke_virtual(&first_thread, "start", "()V", ()).await?;
    let mut first_write_entered = false;
    for _ in 0..100 {
        first_write_entered = jvm.get_field::<bool>(&blocking_probe, "firstWriteEntered", "Z").await?;
        if first_write_entered {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    assert!(first_write_entered, "first println did not reach its backing write");

    let _: () = jvm.invoke_virtual(&second_thread, "start", "()V", ()).await?;
    let mut second_started = false;
    for _ in 0..100 {
        second_started = jvm.get_field::<bool>(&second_runner, "started", "Z").await?;
        if second_started {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(second_started, "second println worker did not start");
    assert_eq!(
        jvm.get_field::<i32>(&blocking_probe, "writeCount", "I").await?,
        1,
        "second println entered the backing writer before the first line completed"
    );
    assert!(!jvm.get_field::<bool>(&second_runner, "done", "Z").await?);

    jvm.put_field(&mut blocking_probe, "releaseFirstWrite", "Z", true).await?;
    let _: () = jvm.invoke_virtual(&first_thread, "join", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&second_thread, "join", "()V", ()).await?;
    assert!(jvm.get_field::<bool>(&first_runner, "done", "Z").await?);
    assert!(jvm.get_field::<bool>(&second_runner, "done", "Z").await?);

    let content: ClassInstanceRef<StringBuffer> = jvm.get_field(&blocking_probe, "content", "Ljava/lang/StringBuffer;").await?;
    let content: ClassInstanceRef<String> = jvm.invoke_virtual(&content, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &content).await?, "1\n2\n");

    let mut close_probe: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 0, 0)).await?.into();
    jvm.put_field(&mut close_probe, "blockFirstClose", "Z", true).await?;
    let close_writer: ClassInstanceRef<PrintWriter> = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (close_probe.clone(),))
        .await?
        .into();
    let first_close_runner = jvm
        .new_class("PrintWriterCloseRunner", "(Ljava/io/PrintWriter;)V", (close_writer.clone(),))
        .await?;
    let second_close_runner = jvm
        .new_class("PrintWriterCloseRunner", "(Ljava/io/PrintWriter;)V", (close_writer.clone(),))
        .await?;
    let first_close_thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (first_close_runner.clone(),))
        .await?;
    let second_close_thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (second_close_runner.clone(),))
        .await?;

    let _: () = jvm.invoke_virtual(&first_close_thread, "start", "()V", ()).await?;
    let mut first_close_entered = false;
    for _ in 0..100 {
        first_close_entered = jvm.get_field::<bool>(&close_probe, "firstCloseEntered", "Z").await?;
        if first_close_entered {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    assert!(first_close_entered, "first close did not reach the backing writer");

    let _: () = jvm.invoke_virtual(&second_close_thread, "start", "()V", ()).await?;
    let mut second_close_started = false;
    for _ in 0..100 {
        second_close_started = jvm.get_field::<bool>(&second_close_runner, "started", "Z").await?;
        if second_close_started {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(second_close_started, "second close worker did not start");
    assert_eq!(jvm.get_field::<i32>(&close_probe, "closeCount", "I").await?, 1);
    assert!(!jvm.get_field::<bool>(&second_close_runner, "done", "Z").await?);

    jvm.put_field(&mut close_probe, "releaseFirstClose", "Z", true).await?;
    let _: () = jvm.invoke_virtual(&first_close_thread, "join", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&second_close_thread, "join", "()V", ()).await?;
    assert!(jvm.get_field::<bool>(&first_close_runner, "done", "Z").await?);
    assert!(jvm.get_field::<bool>(&second_close_runner, "done", "Z").await?);
    assert_eq!(jvm.get_field::<i32>(&close_probe, "closeCount", "I").await?, 1);
    let closed_out: ClassInstanceRef<Writer> = jvm.get_field(&close_writer, "out", "Ljava/io/Writer;").await?;
    assert!(closed_out.is_null());

    Ok(())
}

#[tokio::test]
async fn pw_05_flush_close_and_check_error_preserve_trouble_state() -> Result<()> {
    let jvm = probe_jvm().await?;

    let mut flush_failure: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 1, 0)).await?.into();
    let flush_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (flush_failure.clone(),))
        .await?;
    assert!(jvm.invoke_virtual::<_, bool>(&flush_writer, "checkError", "()Z", ()).await?);
    assert_eq!(jvm.get_field::<i32>(&flush_failure, "flushCount", "I").await?, 1);
    assert_monitor_released(&jvm, &flush_failure).await?;
    jvm.put_field(&mut flush_failure, "flushMode", "I", 0).await?;
    let _: () = jvm.invoke_virtual(&flush_writer, "flush", "()V", ()).await?;
    assert!(jvm.get_field::<bool>(&flush_writer, "trouble", "Z").await?);

    let nested_failure: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 1, 0)).await?.into();
    let inner_writer: ClassInstanceRef<Writer> = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (nested_failure,))
        .await?
        .into();
    let outer_writer = jvm.new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (inner_writer,)).await?;
    assert!(jvm.invoke_virtual::<_, bool>(&outer_writer, "checkError", "()Z", ()).await?);

    let mut close_failure: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 0, 1)).await?.into();
    let close_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (close_failure.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&close_writer, "close", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&close_failure, "closeCount", "I").await?, 1);
    let open_after_failure: ClassInstanceRef<Writer> = jvm.get_field(&close_writer, "out", "Ljava/io/Writer;").await?;
    assert!(!open_after_failure.is_null());
    assert!(jvm.get_field::<bool>(&close_writer, "trouble", "Z").await?);
    assert_monitor_released(&jvm, &close_failure).await?;

    jvm.put_field(&mut close_failure, "closeMode", "I", 0).await?;
    let _: () = jvm.invoke_virtual(&close_writer, "close", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&close_writer, "close", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&close_failure, "closeCount", "I").await?, 2);
    assert!(jvm.get_field::<bool>(&close_writer, "trouble", "Z").await?);
    let closed_out: ClassInstanceRef<Writer> = jvm.get_field(&close_writer, "out", "Ljava/io/Writer;").await?;
    assert!(closed_out.is_null());

    let closed_probe: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 0, 0)).await?.into();
    let closed_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (closed_probe.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&closed_writer, "close", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&closed_writer, "write", "(I)V", ('x' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&closed_writer, "flush", "()V", ()).await?;
    assert!(jvm.invoke_virtual::<_, bool>(&closed_writer, "checkError", "()Z", ()).await?);
    assert_eq!(jvm.get_field::<i32>(&closed_probe, "writeCount", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&closed_probe, "flushCount", "I").await?, 0);

    let runtime_flush: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 2, 0)).await?.into();
    let runtime_flush_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (runtime_flush.clone(),))
        .await?;
    let runtime_flush_result: Result<()> = jvm.invoke_virtual(&runtime_flush_writer, "flush", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = runtime_flush_result else {
        panic!("flush RuntimeException must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert_monitor_released(&jvm, &runtime_flush).await?;

    let mut runtime_close: ClassInstanceRef<Writer> = jvm.new_class("ProbeWriter", "(III)V", (0, 0, 2)).await?.into();
    let runtime_close_writer = jvm
        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (runtime_close.clone(),))
        .await?;
    let runtime_close_result: Result<()> = jvm.invoke_virtual(&runtime_close_writer, "close", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = runtime_close_result else {
        panic!("close RuntimeException must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    let open_after_runtime: ClassInstanceRef<Writer> = jvm.get_field(&runtime_close_writer, "out", "Ljava/io/Writer;").await?;
    assert!(!open_after_runtime.is_null());
    assert_monitor_released(&jvm, &runtime_close).await?;
    jvm.put_field(&mut runtime_close, "closeMode", "I", 0).await?;
    let _: () = jvm.invoke_virtual(&runtime_close_writer, "close", "()V", ()).await?;
    let closed_after_retry: ClassInstanceRef<Writer> = jvm.get_field(&runtime_close_writer, "out", "Ljava/io/Writer;").await?;
    assert!(closed_after_retry.is_null());

    Ok(())
}
