use alloc::{boxed::Box, collections::BTreeMap, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{RuntimeClassProto, RuntimeContext, classes::java::io::OutputStream};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result};
use jvm_rust::ClassDefinitionImpl;
use test_utils::{TestRuntime, create_test_jvm};

struct CloseProbeOutputStream;

impl CloseProbeOutputStream {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CloseProbeOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(II)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(I)V", Self::write, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("flushMode", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("closeMode", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("bytesWritten", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("flushCalls", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("closeCalls", "I", FieldAccessFlags::PUBLIC),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, flush_mode: i32, close_mode: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "flushMode", "I", flush_mode).await?;
        jvm.put_field(&mut this, "closeMode", "I", close_mode).await?;
        jvm.put_field(&mut this, "bytesWritten", "I", 0).await?;
        jvm.put_field(&mut this, "flushCalls", "I", 0).await?;
        jvm.put_field(&mut this, "closeCalls", "I", 0).await
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, _: i32) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "bytesWritten", "I").await?;
        jvm.put_field(&mut this, "bytesWritten", "I", count + 1).await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let calls: i32 = jvm.get_field(&this, "flushCalls", "I").await?;
        jvm.put_field(&mut this, "flushCalls", "I", calls + 1).await?;
        match jvm.get_field::<i32>(&this, "flushMode", "I").await? {
            1 => Err(jvm.exception("java/io/IOException", "flush failed").await),
            2 => Err(jvm.exception("java/lang/IllegalStateException", "flush failed").await),
            _ => Ok(()),
        }
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let calls: i32 = jvm.get_field(&this, "closeCalls", "I").await?;
        jvm.put_field(&mut this, "closeCalls", "I", calls + 1).await?;
        match jvm.get_field::<i32>(&this, "closeMode", "I").await? {
            1 => Err(jvm.exception("java/io/IOException", "close failed").await),
            2 => Err(jvm.exception("java/lang/IllegalStateException", "close failed").await),
            _ => Ok(()),
        }
    }
}

struct FlushOverrideBufferedOutputStream;

impl FlushOverrideBufferedOutputStream {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "FlushOverrideBufferedOutputStream",
            parent_class: Some("java/io/BufferedOutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;II)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("flushMode", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("flushCalls", "I", FieldAccessFlags::PUBLIC),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<OutputStream>,
        size: i32,
        flush_mode: i32,
    ) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "java/io/BufferedOutputStream", "<init>", "(Ljava/io/OutputStream;I)V", (out, size))
            .await?;
        jvm.put_field(&mut this, "flushMode", "I", flush_mode).await?;
        jvm.put_field(&mut this, "flushCalls", "I", 0).await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let calls: i32 = jvm.get_field(&this, "flushCalls", "I").await?;
        jvm.put_field(&mut this, "flushCalls", "I", calls + 1).await?;
        match jvm.get_field::<i32>(&this, "flushMode", "I").await? {
            1 => Err(jvm.exception("java/io/IOException", "override flush failed").await),
            2 => Err(jvm.exception("java/lang/IllegalStateException", "override flush failed").await),
            _ => jvm.invoke_special(&this, "java/io/BufferedOutputStream", "flush", "()V", ()).await,
        }
    }
}

async fn output_stream_close_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            CloseProbeOutputStream::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            FlushOverrideBufferedOutputStream::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;
    Ok(jvm)
}

#[tokio::test]
async fn buffered_output_stream_inherited_close_dispatches_virtual_flush() -> Result<()> {
    let jvm = output_stream_close_jvm().await?;
    let backing: ClassInstanceRef<CloseProbeOutputStream> = jvm.new_class("CloseProbeOutputStream", "(II)V", (0, 0)).await?.into();
    let output: ClassInstanceRef<OutputStream> = backing.instance.clone().into();
    let stream: ClassInstanceRef<FlushOverrideBufferedOutputStream> = jvm
        .new_class("FlushOverrideBufferedOutputStream", "(Ljava/io/OutputStream;II)V", (output, 4, 0))
        .await?
        .into();
    let buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&stream, "buf", "[B").await?;

    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (7,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;

    assert_eq!(jvm.get_field::<i32>(&stream, "flushCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing, "bytesWritten", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing, "flushCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing, "closeCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&stream, "count", "I").await?, 0);
    let retained_buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&stream, "buf", "[B").await?;
    assert_eq!(retained_buffer.identity(), buffer.identity());
    let closed_output: ClassInstanceRef<OutputStream> = jvm.get_field(&stream, "out", "Ljava/io/OutputStream;").await?;
    assert!(closed_output.is_null());

    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&stream, "flushCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing, "closeCalls", "I").await?, 1);
    Ok(())
}

#[tokio::test]
async fn filter_output_stream_close_preserves_failure_sequence_and_state() -> Result<()> {
    let jvm = output_stream_close_jvm().await?;

    let backing: ClassInstanceRef<CloseProbeOutputStream> = jvm.new_class("CloseProbeOutputStream", "(II)V", (0, 0)).await?.into();
    let output: ClassInstanceRef<OutputStream> = backing.instance.clone().into();
    let stream: ClassInstanceRef<FlushOverrideBufferedOutputStream> = jvm
        .new_class("FlushOverrideBufferedOutputStream", "(Ljava/io/OutputStream;II)V", (output, 4, 1))
        .await?
        .into();
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (1,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "close", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&stream, "flushCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&stream, "count", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing, "bytesWritten", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&backing, "closeCalls", "I").await?, 1);
    let closed_output: ClassInstanceRef<OutputStream> = jvm.get_field(&stream, "out", "Ljava/io/OutputStream;").await?;
    assert!(closed_output.is_null());

    let backing: ClassInstanceRef<CloseProbeOutputStream> = jvm.new_class("CloseProbeOutputStream", "(II)V", (0, 0)).await?.into();
    let output: ClassInstanceRef<OutputStream> = backing.instance.clone().into();
    let stream: ClassInstanceRef<FlushOverrideBufferedOutputStream> = jvm
        .new_class("FlushOverrideBufferedOutputStream", "(Ljava/io/OutputStream;II)V", (output, 4, 2))
        .await?
        .into();
    let result: Result<()> = jvm.invoke_virtual(&stream, "close", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unchecked flush failure must escape inherited close");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&backing, "closeCalls", "I").await?, 0);
    let closed_output: ClassInstanceRef<OutputStream> = jvm.get_field(&stream, "out", "Ljava/io/OutputStream;").await?;
    assert!(closed_output.is_null());

    let backing: ClassInstanceRef<CloseProbeOutputStream> = jvm.new_class("CloseProbeOutputStream", "(II)V", (0, 1)).await?.into();
    let output: ClassInstanceRef<OutputStream> = backing.instance.clone().into();
    let stream: ClassInstanceRef<FlushOverrideBufferedOutputStream> = jvm
        .new_class("FlushOverrideBufferedOutputStream", "(Ljava/io/OutputStream;II)V", (output, 4, 0))
        .await?
        .into();
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (1,)).await?;
    let result: Result<()> = jvm.invoke_virtual(&stream, "close", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("backing close failure must escape inherited close");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    assert_eq!(jvm.get_field::<i32>(&stream, "flushCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&stream, "count", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&backing, "bytesWritten", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing, "flushCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing, "closeCalls", "I").await?, 1);
    let closed_output: ClassInstanceRef<OutputStream> = jvm.get_field(&stream, "out", "Ljava/io/OutputStream;").await?;
    assert!(closed_output.is_null());

    Ok(())
}
