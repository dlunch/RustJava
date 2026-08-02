use alloc::{boxed::Box, collections::BTreeMap, format, vec, vec::Vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::classes::java::{
    io::{ByteArrayOutputStream, OutputStream, OutputStreamWriter, PrintStream},
    lang::{Object, String},
};
use java_runtime::{RuntimeClassProto, RuntimeContext};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, JavaValue, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct ProbeOutputStream;

impl ProbeOutputStream {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ProbeOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(III)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(I)V", Self::write, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("content", "Ljava/io/ByteArrayOutputStream;", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("writeMode", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("flushMode", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("closeMode", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("writeCount", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("flushCount", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("closeCount", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("blockFirstWrite", "Z", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("firstWriteEntered", "Z", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("releaseFirstWrite", "Z", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("blockFirstClose", "Z", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("firstCloseEntered", "Z", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("releaseFirstClose", "Z", FieldAccessFlags::PUBLIC),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
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
        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;
        let content = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
        jvm.put_field(&mut this, "content", "Ljava/io/ByteArrayOutputStream;", content).await?;
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

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "writeCount", "I").await?;
        jvm.put_field(&mut this, "writeCount", "I", count + 1).await?;
        match jvm.get_field::<i32>(&this, "writeMode", "I").await? {
            1 => return Err(jvm.exception("java/io/IOException", "write failed").await),
            2 => return Err(jvm.exception("java/lang/IllegalStateException", "write failed").await),
            3 if count == 0 => return Err(jvm.exception("java/io/IOException", "first write failed").await),
            4 if count == 1 => return Err(jvm.exception("java/io/IOException", "second write failed").await),
            _ => {}
        }
        if count == 0 && jvm.get_field::<bool>(&this, "blockFirstWrite", "Z").await? {
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

        let content: ClassInstanceRef<ByteArrayOutputStream> = jvm.get_field(&this, "content", "Ljava/io/ByteArrayOutputStream;").await?;
        jvm.invoke_virtual(&content, "write", "(I)V", (value,)).await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "flushCount", "I").await?;
        jvm.put_field(&mut this, "flushCount", "I", count + 1).await?;
        match jvm.get_field::<i32>(&this, "flushMode", "I").await? {
            1 => Err(jvm.exception("java/io/IOException", "flush failed").await),
            2 => Err(jvm.exception("java/lang/IllegalStateException", "flush failed").await),
            3 if count == 2 => Err(jvm.exception("java/io/IOException", "third flush failed").await),
            _ => Ok(()),
        }
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "closeCount", "I").await?;
        jvm.put_field(&mut this, "closeCount", "I", count + 1).await?;
        if count == 0 && jvm.get_field::<bool>(&this, "blockFirstClose", "Z").await? {
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

struct OverridePrintStream;

impl OverridePrintStream {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "OverridePrintStream",
            parent_class: Some("java/io/PrintStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("print", "(I)V", Self::print_int, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("println", "()V", Self::println, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([BII)V", Self::write, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("printCount", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("printlnCount", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("writeCount", "I", FieldAccessFlags::PUBLIC),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "java/io/PrintStream", "<init>", "(Ljava/io/OutputStream;)V", (out,))
            .await?;
        jvm.put_field(&mut this, "printCount", "I", 0).await?;
        jvm.put_field(&mut this, "printlnCount", "I", 0).await?;
        jvm.put_field(&mut this, "writeCount", "I", 0).await
    }

    async fn print_int(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "printCount", "I").await?;
        jvm.put_field(&mut this, "printCount", "I", count + 1).await?;
        let value = JavaLangString::from_rust_string(jvm, &format!("<{value}>")).await?;
        jvm.invoke_special(&this, "java/io/PrintStream", "print", "(Ljava/lang/String;)V", (value,))
            .await
    }

    async fn println(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "printlnCount", "I").await?;
        jvm.put_field(&mut this, "printlnCount", "I", count + 1).await
    }

    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        bytes: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "writeCount", "I").await?;
        jvm.put_field(&mut this, "writeCount", "I", count + 1).await?;
        jvm.invoke_special(&this, "java/io/PrintStream", "write", "([BII)V", (bytes, offset, length))
            .await
    }
}

struct PrintStreamRunner;

impl PrintStreamRunner {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "PrintStreamRunner",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/PrintStream;I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("run", "()V", Self::run, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("stream", "Ljava/io/PrintStream;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("value", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("started", "Z", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("done", "Z", FieldAccessFlags::PUBLIC),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        stream: ClassInstanceRef<PrintStream>,
        value: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "stream", "Ljava/io/PrintStream;", stream).await?;
        jvm.put_field(&mut this, "value", "I", value).await?;
        jvm.put_field(&mut this, "started", "Z", false).await?;
        jvm.put_field(&mut this, "done", "Z", false).await
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "started", "Z", true).await?;
        let stream: ClassInstanceRef<PrintStream> = jvm.get_field(&this, "stream", "Ljava/io/PrintStream;").await?;
        let value: i32 = jvm.get_field(&this, "value", "I").await?;
        let _: () = jvm.invoke_virtual(&stream, "println", "(I)V", (value,)).await?;
        jvm.put_field(&mut this, "done", "Z", true).await
    }
}

struct PrintStreamCloseRunner;

impl PrintStreamCloseRunner {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "PrintStreamCloseRunner",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/PrintStream;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("run", "()V", Self::run, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("stream", "Ljava/io/PrintStream;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("started", "Z", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("done", "Z", FieldAccessFlags::PUBLIC),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, stream: ClassInstanceRef<PrintStream>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "stream", "Ljava/io/PrintStream;", stream).await?;
        jvm.put_field(&mut this, "started", "Z", false).await?;
        jvm.put_field(&mut this, "done", "Z", false).await
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "started", "Z", true).await?;
        let stream: ClassInstanceRef<PrintStream> = jvm.get_field(&this, "stream", "Ljava/io/PrintStream;").await?;
        let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
        jvm.put_field(&mut this, "done", "Z", true).await
    }
}

async fn probe_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            ProbeOutputStream::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            OverridePrintStream::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            PrintStreamRunner::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            PrintStreamCloseRunner::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;
    Ok(jvm)
}

async fn assert_monitor_released(jvm: &Jvm, stream: &ClassInstanceRef<PrintStream>) -> Result<()> {
    jvm.monitor_enter(stream).await?;
    jvm.monitor_exit(stream).await?;
    let result = jvm.monitor_exit(stream).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("PrintStream leaked a reentrant monitor acquisition");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalMonitorStateException"));
    Ok(())
}

#[tokio::test]
async fn ps_01_constructor_state_descriptors_and_access_flags() -> Result<()> {
    let proto = PrintStream::as_proto();
    assert_eq!(proto.parent_class, Some("java/io/FilterOutputStream"));
    assert_eq!(proto.interfaces, vec!["java/lang/Appendable", "java/io/Closeable"]);
    assert!(proto.access_flags.contains(ClassAccessFlags::PUBLIC));

    for descriptor in [
        "(Ljava/io/OutputStream;)V",
        "(Ljava/io/OutputStream;Z)V",
        "(Ljava/io/OutputStream;ZLjava/lang/String;)V",
        "(Ljava/lang/String;)V",
        "(Ljava/lang/String;Ljava/lang/String;)V",
        "(Ljava/io/File;)V",
        "(Ljava/io/File;Ljava/lang/String;)V",
    ] {
        let constructors = proto
            .methods
            .iter()
            .filter(|method| method.name == "<init>" && method.descriptor == descriptor)
            .collect::<Vec<_>>();
        assert_eq!(constructors.len(), 1, "missing or duplicate constructor {descriptor}");
        assert!(constructors[0].access_flags.contains(MethodAccessFlags::PUBLIC));
    }
    let set_error = proto
        .methods
        .iter()
        .find(|method| method.name == "setError" && method.descriptor == "()V")
        .expect("missing setError()V");
    assert_eq!(set_error.access_flags, MethodAccessFlags::PROTECTED);
    assert!(!proto.methods.iter().any(|method| method.name == "clearError"));
    assert!(
        !proto
            .methods
            .iter()
            .any(|method| method.name == "println" && matches!(method.descriptor.as_str(), "(B)V" | "(S)V"))
    );

    let auto_flush = proto.fields.iter().find(|field| field.name == "autoFlush").expect("autoFlush field");
    assert_eq!(auto_flush.descriptor, "Z");
    assert_eq!(auto_flush.access_flags, FieldAccessFlags::PRIVATE);
    let trouble = proto.fields.iter().find(|field| field.name == "trouble").expect("trouble field");
    assert_eq!(trouble.descriptor, "Z");
    assert_eq!(trouble.access_flags, FieldAccessFlags::PRIVATE);
    let char_out = proto.fields.iter().find(|field| field.name == "charOut").expect("charOut field");
    assert_eq!(char_out.descriptor, "Ljava/io/OutputStreamWriter;");
    assert_eq!(char_out.access_flags, FieldAccessFlags::PRIVATE);
    let closing = proto.fields.iter().find(|field| field.name == "closing").expect("closing field");
    assert_eq!(closing.descriptor, "Z");
    assert_eq!(closing.access_flags, FieldAccessFlags::PRIVATE);

    let jvm = probe_jvm().await?;
    let output: ClassInstanceRef<OutputStream> = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 0)).await?.into();
    let default_stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    assert!(!jvm.get_field::<bool>(&default_stream, "autoFlush", "Z").await?);
    assert!(!jvm.get_field::<bool>(&default_stream, "trouble", "Z").await?);
    assert!(!jvm.get_field::<bool>(&default_stream, "closing", "Z").await?);
    let char_out: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(&default_stream, "charOut", "Ljava/io/OutputStreamWriter;").await?;
    assert!(!char_out.is_null());
    let configured_stream = jvm.new_class("java/io/PrintStream", "(Ljava/io/OutputStream;Z)V", (output, true)).await?;
    assert!(jvm.get_field::<bool>(&configured_stream, "autoFlush", "Z").await?);

    for descriptor in ["(Ljava/io/OutputStream;)V", "(Ljava/io/OutputStream;Z)V"] {
        let null_output: ClassInstanceRef<OutputStream> = None.into();
        let result = if descriptor.ends_with(";Z)V") {
            jvm.new_class("java/io/PrintStream", descriptor, (null_output, true)).await
        } else {
            jvm.new_class("java/io/PrintStream", descriptor, (null_output,)).await
        };
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{descriptor} null output must throw NullPointerException");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }

    Ok(())
}

#[tokio::test]
async fn ps_02_auto_flush_matches_public_write_and_println_contracts() -> Result<()> {
    let jvm = probe_jvm().await?;
    let output = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 0)).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;Z)V", (output.clone(), true))
        .await?;

    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", ('x' as i32,)).await?;
    assert_eq!(jvm.get_field::<i32>(&output, "flushCount", "I").await?, 0);

    let formatted_output = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 0)).await?;
    let formatted_stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;Z)V", (formatted_output.clone(), true))
        .await?;
    let format = JavaLangString::from_rust_string(&jvm, "%s").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;
    let mut arguments: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    jvm.store_array(&mut arguments, 0, [JavaValue::Object(Some(value))]).await?;
    let _: ClassInstanceRef<PrintStream> = jvm
        .invoke_virtual(
            &formatted_stream,
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
            (format, arguments),
        )
        .await?;
    let format_writes: i32 = jvm.get_field(&formatted_output, "writeCount", "I").await?;
    assert!(format_writes > 0);
    assert_eq!(jvm.get_field::<i32>(&formatted_output, "flushCount", "I").await?, 1);

    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", ('\n' as i32,)).await?;
    assert_eq!(jvm.get_field::<i32>(&output, "flushCount", "I").await?, 1);

    let mut bytes = jvm.instantiate_array("B", 2).await?;
    jvm.store_array(&mut bytes, 0, [b'a' as i8, b'b' as i8]).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "([BII)V", (bytes, 0, 2)).await?;
    assert_eq!(jvm.get_field::<i32>(&output, "flushCount", "I").await?, 2);

    let text = JavaLangString::from_rust_string(&jvm, "s").await?;
    let object: ClassInstanceRef<Object> = text.clone().into();
    let mut chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut chars, 0, ['c' as JavaChar]).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Ljava/lang/Object;)V", (object,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Ljava/lang/String;)V", (text,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(I)V", (1,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(J)V", (2i64,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(C)V", ('d' as JavaChar,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "([C)V", (chars,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(I)V", (3,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(I)V", (4,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Z)V", (true,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(F)V", (1.5f32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(D)V", (2.5f64,)).await?;
    assert!(jvm.get_field::<i32>(&output, "flushCount", "I").await? >= 14);

    let output = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 0)).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let mut bytes = jvm.instantiate_array("B", 1).await?;
    jvm.store_array(&mut bytes, 0, [b'\n' as i8]).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", ('\n' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "([BII)V", (bytes, 0, 1)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&output, "flushCount", "I").await?, 0);
    assert!(!jvm.invoke_virtual::<_, bool>(&stream, "checkError", "()Z", ()).await?);
    assert_eq!(jvm.get_field::<i32>(&output, "flushCount", "I").await?, 1);

    Ok(())
}

#[tokio::test]
async fn ps_02_uses_default_encoding_split_surrogate_state_and_line_separator() -> Result<()> {
    let jvm = test_jvm().await?;

    let encoding_key = JavaLangString::from_rust_string(&jvm, "file.encoding").await?;
    let euc_kr = JavaLangString::from_rust_string(&jvm, "EUC-KR").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (encoding_key, euc_kr),
        )
        .await?;
    let separator_key = JavaLangString::from_rust_string(&jvm, "line.separator").await?;
    let separator = JavaLangString::from_rust_string(&jvm, "\r\n").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (separator_key, separator),
        )
        .await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let value = JavaLangString::from_rust_string(&jvm, "가").await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(Ljava/lang/String;)V", (value,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "()V", ()).await?;
    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    assert_eq!(
        jvm.load_array::<i8>(&bytes, 0, jvm.array_length(&bytes).await?).await?,
        [0xb0u8 as i8, 0xa1u8 as i8, b'\r' as i8, b'\n' as i8]
    );

    let encoding_key = JavaLangString::from_rust_string(&jvm, "file.encoding").await?;
    let utf8 = JavaLangString::from_rust_string(&jvm, "UTF-8").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (encoding_key, utf8),
        )
        .await?;
    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(C)V", (0xd83d as JavaChar,)).await?;
    let pending: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    assert_eq!(jvm.array_length(&pending).await?, 0);
    let _: () = jvm.invoke_virtual(&stream, "print", "(C)V", (0xde00 as JavaChar,)).await?;
    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    assert_eq!(
        jvm.load_array::<i8>(&bytes, 0, jvm.array_length(&bytes).await?).await?,
        [0xf0u8 as i8, 0x9fu8 as i8, 0x98u8 as i8, 0x80u8 as i8]
    );

    Ok(())
}

#[tokio::test]
async fn ps_02_typed_println_is_atomic_and_preserves_virtual_dispatch() -> Result<()> {
    let jvm = probe_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let stream = jvm
        .new_class("OverridePrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(I)V", (7,)).await?;
    assert_eq!(jvm.get_field::<i32>(&stream, "printCount", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&stream, "printlnCount", "I").await?, 0);
    assert!(jvm.get_field::<i32>(&stream, "writeCount", "I").await? >= 2);
    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    assert_eq!(
        jvm.load_array::<i8>(&bytes, 0, jvm.array_length(&bytes).await?).await?,
        b"<7>\n".iter().map(|value| *value as i8).collect::<Vec<_>>()
    );

    let mut blocking_output = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 0)).await?;
    jvm.put_field(&mut blocking_output, "blockFirstWrite", "Z", true).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (blocking_output.clone(),))
        .await?
        .into();
    let first_runner = jvm
        .new_class("PrintStreamRunner", "(Ljava/io/PrintStream;I)V", (stream.clone(), 1))
        .await?;
    let second_runner = jvm
        .new_class("PrintStreamRunner", "(Ljava/io/PrintStream;I)V", (stream.clone(), 2))
        .await?;
    let first_thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (first_runner.clone(),))
        .await?;
    let second_thread = jvm
        .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (second_runner.clone(),))
        .await?;

    let _: () = jvm.invoke_virtual(&first_thread, "start", "()V", ()).await?;
    let mut first_write_entered = false;
    for _ in 0..1000 {
        first_write_entered = jvm.get_field::<bool>(&blocking_output, "firstWriteEntered", "Z").await?;
        if first_write_entered {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    assert!(first_write_entered, "first println did not reach its backing write");

    let _: () = jvm.invoke_virtual(&second_thread, "start", "()V", ()).await?;
    let mut second_started = false;
    for _ in 0..1000 {
        second_started = jvm.get_field::<bool>(&second_runner, "started", "Z").await?;
        if second_started {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(second_started, "second println worker did not start");
    assert_eq!(jvm.get_field::<i32>(&blocking_output, "writeCount", "I").await?, 1);
    assert!(!jvm.get_field::<bool>(&second_runner, "done", "Z").await?);

    jvm.put_field(&mut blocking_output, "releaseFirstWrite", "Z", true).await?;
    let _: () = jvm.invoke_virtual(&first_thread, "join", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&second_thread, "join", "()V", ()).await?;
    let content: ClassInstanceRef<ByteArrayOutputStream> = jvm.get_field(&blocking_output, "content", "Ljava/io/ByteArrayOutputStream;").await?;
    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&content, "toByteArray", "()[B", ()).await?;
    assert_eq!(
        jvm.load_array::<i8>(&bytes, 0, jvm.array_length(&bytes).await?).await?,
        b"1\n2\n".iter().map(|value| *value as i8).collect::<Vec<_>>()
    );

    Ok(())
}

#[tokio::test]
async fn ps_02_write_and_close_are_serialized_on_the_stream_monitor() -> Result<()> {
    let jvm = probe_jvm().await?;
    let mut output = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 0)).await?;
    jvm.put_field(&mut output, "blockFirstWrite", "Z", true).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?
        .into();
    let writer = jvm
        .new_class("PrintStreamRunner", "(Ljava/io/PrintStream;I)V", (stream.clone(), 1))
        .await?;
    let closer = jvm
        .new_class("PrintStreamCloseRunner", "(Ljava/io/PrintStream;)V", (stream.clone(),))
        .await?;
    let writer_thread = jvm.new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (writer,)).await?;
    let close_thread = jvm.new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (closer.clone(),)).await?;

    let _: () = jvm.invoke_virtual(&writer_thread, "start", "()V", ()).await?;
    let mut first_write_entered = false;
    for _ in 0..1000 {
        first_write_entered = jvm.get_field::<bool>(&output, "firstWriteEntered", "Z").await?;
        if first_write_entered {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    assert!(first_write_entered);

    let _: () = jvm.invoke_virtual(&close_thread, "start", "()V", ()).await?;
    let mut close_started = false;
    for _ in 0..1000 {
        close_started = jvm.get_field::<bool>(&closer, "started", "Z").await?;
        if close_started {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(close_started);
    assert_eq!(jvm.get_field::<i32>(&output, "closeCount", "I").await?, 0);
    assert!(!jvm.get_field::<bool>(&closer, "done", "Z").await?);

    jvm.put_field(&mut output, "releaseFirstWrite", "Z", true).await?;
    let _: () = jvm.invoke_virtual(&writer_thread, "join", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&close_thread, "join", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&output, "closeCount", "I").await?, 1);
    let content: ClassInstanceRef<ByteArrayOutputStream> = jvm.get_field(&output, "content", "Ljava/io/ByteArrayOutputStream;").await?;
    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&content, "toByteArray", "()[B", ()).await?;
    assert_eq!(
        jvm.load_array::<i8>(&bytes, 0, jvm.array_length(&bytes).await?).await?,
        b"1\n".iter().map(|value| *value as i8).collect::<Vec<_>>()
    );

    Ok(())
}

#[tokio::test]
async fn ps_02_failure_phases_continue_like_jdk_and_nested_error_is_visible() -> Result<()> {
    let jvm = probe_jvm().await?;

    let value_failure = jvm.new_class("ProbeOutputStream", "(III)V", (3, 0, 0)).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (value_failure.clone(),))
        .await?
        .into();
    let value = JavaLangString::from_rust_string(&jvm, "V").await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Ljava/lang/String;)V", (value,)).await?;
    assert_eq!(jvm.get_field::<i32>(&value_failure, "writeCount", "I").await?, 2);
    let content: ClassInstanceRef<ByteArrayOutputStream> = jvm.get_field(&value_failure, "content", "Ljava/io/ByteArrayOutputStream;").await?;
    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&content, "toByteArray", "()[B", ()).await?;
    assert_eq!(jvm.load_array::<i8>(&bytes, 0, 1).await?, [b'\n' as i8]);
    assert!(jvm.invoke_virtual::<_, bool>(&stream, "checkError", "()Z", ()).await?);
    assert_monitor_released(&jvm, &stream).await?;

    let newline_failure = jvm.new_class("ProbeOutputStream", "(III)V", (4, 0, 0)).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;Z)V", (newline_failure.clone(), true))
        .await?
        .into();
    let value = JavaLangString::from_rust_string(&jvm, "V").await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Ljava/lang/String;)V", (value,)).await?;
    assert_eq!(jvm.get_field::<i32>(&newline_failure, "writeCount", "I").await?, 2);
    assert_eq!(jvm.get_field::<i32>(&newline_failure, "flushCount", "I").await?, 2);
    assert!(jvm.invoke_virtual::<_, bool>(&stream, "checkError", "()Z", ()).await?);

    let final_flush_failure = jvm.new_class("ProbeOutputStream", "(III)V", (0, 3, 0)).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;Z)V", (final_flush_failure.clone(), true))
        .await?
        .into();
    let value = JavaLangString::from_rust_string(&jvm, "V").await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Ljava/lang/String;)V", (value,)).await?;
    assert_eq!(jvm.get_field::<i32>(&final_flush_failure, "flushCount", "I").await?, 3);
    assert!(jvm.get_field::<bool>(&stream, "trouble", "Z").await?);

    let nested_failure = jvm.new_class("ProbeOutputStream", "(III)V", (0, 1, 0)).await?;
    let inner: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (nested_failure,))
        .await?
        .into();
    let outer = jvm.new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (inner,)).await?;
    assert!(jvm.invoke_virtual::<_, bool>(&outer, "checkError", "()Z", ()).await?);

    Ok(())
}

#[tokio::test]
async fn ps_02_suppresses_only_ioexception_and_closes_once() -> Result<()> {
    let jvm = probe_jvm().await?;

    let io_failure = jvm.new_class("ProbeOutputStream", "(III)V", (1, 0, 0)).await?;
    let stream = jvm.new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (io_failure,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (1,)).await?;
    assert!(jvm.get_field::<bool>(&stream, "trouble", "Z").await?);
    let stream: ClassInstanceRef<PrintStream> = stream.into();
    assert_monitor_released(&jvm, &stream).await?;

    let runtime_failure = jvm.new_class("ProbeOutputStream", "(III)V", (2, 0, 0)).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (runtime_failure,))
        .await?
        .into();
    let result: Result<()> = jvm.invoke_virtual(&stream, "write", "(I)V", (1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("non-IOException from write must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert!(!jvm.get_field::<bool>(&stream, "trouble", "Z").await?);
    assert_monitor_released(&jvm, &stream).await?;

    let clean_output = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 0)).await?;
    let stream = jvm.new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (clean_output,)).await?;
    let null_bytes: ClassInstanceRef<Array<i8>> = None.into();
    let result: Result<()> = jvm.invoke_virtual(&stream, "write", "([BII)V", (null_bytes, 0, 0)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null byte array must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    let bytes = jvm.instantiate_array("B", 1).await?;
    let result: Result<()> = jvm.invoke_virtual(&stream, "write", "([BII)V", (bytes, 1, 1)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("invalid byte range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    assert!(!jvm.get_field::<bool>(&stream, "trouble", "Z").await?);

    let flush_failure = jvm.new_class("ProbeOutputStream", "(III)V", (0, 1, 0)).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (flush_failure,))
        .await?;
    let _: () = jvm.invoke_virtual(&stream, "flush", "()V", ()).await?;
    assert!(jvm.get_field::<bool>(&stream, "trouble", "Z").await?);

    let runtime_flush_failure = jvm.new_class("ProbeOutputStream", "(III)V", (0, 2, 0)).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (runtime_flush_failure,))
        .await?
        .into();
    let result: Result<()> = jvm.invoke_virtual(&stream, "flush", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("non-IOException from flush must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert_monitor_released(&jvm, &stream).await?;
    let result: Result<bool> = jvm.invoke_virtual(&stream, "checkError", "()Z", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("checkError must propagate a non-IOException from flush");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert_monitor_released(&jvm, &stream).await?;

    let close_failure = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 1)).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (close_failure.clone(),))
        .await?
        .into();
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&close_failure, "closeCount", "I").await?, 1);
    assert!(jvm.get_field::<bool>(&stream, "trouble", "Z").await?);
    assert!(jvm.get_field::<bool>(&stream, "closing", "Z").await?);
    let closed_output: ClassInstanceRef<OutputStream> = jvm.get_field(&stream, "out", "Ljava/io/OutputStream;").await?;
    assert!(closed_output.is_null());
    let closed_writer: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(&stream, "charOut", "Ljava/io/OutputStreamWriter;").await?;
    assert!(closed_writer.is_null());
    assert_monitor_released(&jvm, &stream).await?;

    let mut runtime_close_failure = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 2)).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (runtime_close_failure.clone(),))
        .await?
        .into();
    let result: Result<()> = jvm.invoke_virtual(&stream, "close", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("non-IOException from close must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&runtime_close_failure, "closeCount", "I").await?, 1);
    let stored_output: ClassInstanceRef<OutputStream> = jvm.get_field(&stream, "out", "Ljava/io/OutputStream;").await?;
    assert_eq!(stored_output.identity(), runtime_close_failure.identity());
    let stored_writer: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(&stream, "charOut", "Ljava/io/OutputStreamWriter;").await?;
    assert!(!stored_writer.is_null());
    assert!(jvm.get_field::<bool>(&stream, "closing", "Z").await?);
    assert!(!jvm.get_field::<bool>(&stream, "trouble", "Z").await?);
    assert_monitor_released(&jvm, &stream).await?;
    jvm.put_field(&mut runtime_close_failure, "closeMode", "I", 0).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&runtime_close_failure, "closeCount", "I").await?, 1);
    let stored_output: ClassInstanceRef<OutputStream> = jvm.get_field(&stream, "out", "Ljava/io/OutputStream;").await?;
    assert_eq!(stored_output.identity(), runtime_close_failure.identity());
    assert_monitor_released(&jvm, &stream).await?;

    let mut encoder_close_failure = jvm.new_class("ProbeOutputStream", "(III)V", (2, 0, 0)).await?;
    let stream: ClassInstanceRef<PrintStream> = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (encoder_close_failure.clone(),))
        .await?
        .into();
    let _: () = jvm.invoke_virtual(&stream, "print", "(C)V", (0xd83d as JavaChar,)).await?;
    let result: Result<()> = jvm.invoke_virtual(&stream, "close", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("non-IOException from encoder close must propagate");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&encoder_close_failure, "writeCount", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&encoder_close_failure, "closeCount", "I").await?, 0);
    assert!(jvm.get_field::<bool>(&stream, "closing", "Z").await?);
    assert!(!jvm.get_field::<bool>(&stream, "trouble", "Z").await?);
    assert_monitor_released(&jvm, &stream).await?;
    jvm.put_field(&mut encoder_close_failure, "writeMode", "I", 0).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&encoder_close_failure, "writeCount", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&encoder_close_failure, "closeCount", "I").await?, 0);
    assert_monitor_released(&jvm, &stream).await?;

    let closed_output = jvm.new_class("ProbeOutputStream", "(III)V", (0, 0, 0)).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (closed_output.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&stream, "checkError", "()Z", ()).await?);
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (1,)).await?;
    assert!(jvm.invoke_virtual::<_, bool>(&stream, "checkError", "()Z", ()).await?);
    assert_eq!(jvm.get_field::<i32>(&closed_output, "writeCount", "I").await?, 0);

    Ok(())
}

#[tokio::test]
async fn test_print_stream_cldc_api() -> Result<()> {
    let jvm = test_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;

    let prefix = JavaLangString::from_rust_string(&jvm, "v=").await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(Ljava/lang/String;)V", (prefix,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(I)V", (7,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(C)V", (' ' as u16,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(Z)V", (true,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "()V", ()).await?;

    let mut chars = jvm.instantiate_array("C", 2).await?;
    jvm.store_array(&mut chars, 0, ['O' as JavaChar, 'K' as JavaChar]).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "([C)V", (chars,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(D)V", (1.5f64,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(F)V", (1.0f32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(D)V", (f64::INFINITY,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "flush", "()V", ()).await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&stream, "checkError", "()Z", ()).await?);

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    let values: Vec<i8> = jvm.load_array(&bytes, 0, jvm.array_length(&bytes).await?).await?;
    let values = values.into_iter().map(|value| value as u8).collect::<Vec<_>>();
    assert_eq!(values, b"v=7 true\nOK1.5\n1.0Infinity\n");

    Ok(())
}

#[tokio::test]
async fn test_print_stream_remaining_overloads_and_close() -> Result<()> {
    let jvm = test_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let stream = jvm
        .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let text = JavaLangString::from_rust_string(&jvm, "obj").await?;
    let object: ClassInstanceRef<Object> = text.clone().into();
    let null_object: ClassInstanceRef<Object> = None.into();
    let null_string: ClassInstanceRef<String> = None.into();

    let _: () = jvm.invoke_virtual(&stream, "print", "(Ljava/lang/Object;)V", (object.clone(),)).await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(Ljava/lang/Object;)V", (null_object,)).await?;
    let _: () = jvm
        .invoke_virtual(&stream, "print", "(Ljava/lang/String;)V", (null_string.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&stream, "print", "(J)V", (9i64,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", ('|' as i32,)).await?;

    let _: () = jvm.invoke_virtual(&stream, "println", "(Ljava/lang/Object;)V", (object,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Ljava/lang/String;)V", (null_string,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(I)V", (-1,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(J)V", (2i64,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(C)V", ('A' as JavaChar,)).await?;

    let mut chars = jvm.instantiate_array("C", 2).await?;
    jvm.store_array(&mut chars, 0, ['B' as JavaChar, 'C' as JavaChar]).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "([C)V", (chars,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(I)V", (-3,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(I)V", (4,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(Z)V", (false,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "println", "(F)V", (2.5f32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&stream, "checkError", "()Z", ()).await?);

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    let values: Vec<i8> = jvm.load_array(&bytes, 0, jvm.array_length(&bytes).await?).await?;
    assert_eq!(
        values.into_iter().map(|value| value as u8).collect::<Vec<_>>(),
        b"objnullnull9|obj\nnull\n-1\n2\nA\nBC\n-3\n4\nfalse\n2.5\n"
    );

    let null_output: ClassInstanceRef<OutputStream> = None.into();
    let result = jvm.new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (null_output,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null output must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
