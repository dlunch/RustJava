use alloc::{boxed::Box, collections::BTreeMap, vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{BufferedWriter, Writer},
        lang::Object,
    },
};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;
use test_utils::{TestRuntime, create_test_jvm};

struct LockCheckingWriter;

impl LockCheckingWriter {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "LockCheckingWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(IZ)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([CII)V", Self::write, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("failWrites", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("failClose", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("writeCalls", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("flushCalls", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("closeCalls", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("written", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, fail_writes: i32, fail_close: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "failWrites", "I", fail_writes).await?;
        jvm.put_field(&mut this, "failClose", "Z", fail_close).await?;
        jvm.put_field(&mut this, "writeCalls", "I", 0).await?;
        jvm.put_field(&mut this, "flushCalls", "I", 0).await?;
        jvm.put_field(&mut this, "closeCalls", "I", 0).await?;
        jvm.put_field(&mut this, "written", "I", 0).await
    }

    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        _: ClassInstanceRef<Array<JavaChar>>,
        _: i32,
        length: i32,
    ) -> Result<()> {
        jvm.object_notify(&this, 1).await?;
        let calls: i32 = jvm.get_field(&this, "writeCalls", "I").await?;
        jvm.put_field(&mut this, "writeCalls", "I", calls + 1).await?;
        let failures: i32 = jvm.get_field(&this, "failWrites", "I").await?;
        if failures > 0 {
            jvm.put_field(&mut this, "failWrites", "I", failures - 1).await?;
            return Err(jvm.exception("java/io/IOException", "write failed").await);
        }
        let written: i32 = jvm.get_field(&this, "written", "I").await?;
        jvm.put_field(&mut this, "written", "I", written + length).await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.object_notify(&this, 1).await?;
        let calls: i32 = jvm.get_field(&this, "flushCalls", "I").await?;
        jvm.put_field(&mut this, "flushCalls", "I", calls + 1).await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.object_notify(&this, 1).await?;
        let calls: i32 = jvm.get_field(&this, "closeCalls", "I").await?;
        jvm.put_field(&mut this, "closeCalls", "I", calls + 1).await?;
        if jvm.get_field::<bool>(&this, "failClose", "Z").await? {
            return Err(jvm.exception("java/io/IOException", "close failed").await);
        }
        Ok(())
    }
}

struct StreamOperationRunner;

impl StreamOperationRunner {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "StreamOperationRunner",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/Object;I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("run", "()V", Self::run, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("target", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("operation", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("started", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("done", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Object>,
        operation: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "target", "Ljava/lang/Object;", target).await?;
        jvm.put_field(&mut this, "operation", "I", operation).await?;
        jvm.put_field(&mut this, "started", "Z", false).await?;
        jvm.put_field(&mut this, "done", "Z", false).await
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "started", "Z", true).await?;
        let target: ClassInstanceRef<Object> = jvm.get_field(&this, "target", "Ljava/lang/Object;").await?;
        match jvm.get_field::<i32>(&this, "operation", "I").await? {
            0 => {
                let _: i32 = jvm.invoke_virtual(&target, &target.class_definition().name(), "read", "()I", ()).await?;
            }
            1 => {
                let _: () = jvm
                    .invoke_virtual(&target, &target.class_definition().name(), "write", "(I)V", ('X' as i32,))
                    .await?;
            }
            2 => {
                let _: () = jvm.invoke_virtual(&target, &target.class_definition().name(), "close", "()V", ()).await?;
            }
            _ => {
                let _: () = jvm
                    .invoke_virtual(&target, &target.class_definition().name(), "newLine", "()V", ())
                    .await?;
            }
        }
        jvm.put_field(&mut this, "done", "Z", true).await
    }
}

async fn stream_locking_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            LockCheckingWriter::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            StreamOperationRunner::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;
    Ok(jvm)
}

async fn assert_monitor_released(jvm: &Jvm, lock: &ClassInstanceRef<Object>) -> Result<()> {
    jvm.monitor_enter(lock).await?;
    jvm.monitor_exit(lock).await?;
    let result = jvm.monitor_exit(lock).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("stream leaked a reentrant inherited-lock acquisition");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalMonitorStateException"));
    Ok(())
}

async fn assert_worker_waits_for_lock(jvm: &Jvm, target: ClassInstanceRef<Object>, lock: ClassInstanceRef<Object>, operation: i32) -> Result<()> {
    let runner = jvm
        .new_class("StreamOperationRunner", "(Ljava/lang/Object;I)V", (target, operation))
        .await?;
    let thread = jvm.new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (runner.clone(),)).await?;

    jvm.monitor_enter(&lock).await?;
    let _: () = jvm.invoke_virtual(&thread, &thread.class_definition().name(), "start", "()V", ()).await?;
    let mut started = false;
    for _ in 0..100 {
        started = jvm.get_field::<bool>(&runner, "started", "Z").await?;
        if started {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    tokio::time::sleep(Duration::from_millis(10)).await;
    let completed_while_locked = jvm.get_field::<bool>(&runner, "done", "Z").await?;
    jvm.monitor_exit(&lock).await?;
    let _: () = jvm.invoke_virtual(&thread, &thread.class_definition().name(), "join", "()V", ()).await?;

    assert!(started, "worker thread did not start");
    assert!(!completed_while_locked, "state operation did not synchronize on inherited lock");
    assert!(jvm.get_field::<bool>(&runner, "done", "Z").await?);
    Ok(())
}

#[tokio::test]
async fn buffered_writer_preserves_state_and_closes_after_backing_failures() -> Result<()> {
    let jvm = stream_locking_jvm().await?;
    let mut backing: ClassInstanceRef<LockCheckingWriter> = jvm.new_class("LockCheckingWriter", "(IZ)V", (1, false)).await?.into();
    let backing_writer: ClassInstanceRef<Writer> = backing.instance.clone().into();
    let writer: ClassInstanceRef<BufferedWriter> = jvm
        .new_class("java/io/BufferedWriter", "(Ljava/io/Writer;I)V", (backing_writer, 4))
        .await?
        .into();
    let lock: ClassInstanceRef<Object> = jvm.get_field(&writer, "lock", "Ljava/lang/Object;").await?;
    assert_eq!(lock.identity(), backing.identity());

    let _: () = jvm
        .invoke_virtual(&writer, "java/io/BufferedWriter", "write", "(I)V", ('A' as i32,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&writer, "java/io/BufferedWriter", "write", "(I)V", ('B' as i32,))
        .await?;
    let failed_flush: Result<()> = jvm.invoke_virtual(&writer, "java/io/BufferedWriter", "flush", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = failed_flush else {
        panic!("backing write failure must escape BufferedWriter.flush");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    assert_eq!(jvm.get_field::<i32>(&writer, "nextChar", "I").await?, 2);
    assert_eq!(jvm.get_field::<i32>(&backing, "written", "I").await?, 0);
    assert_monitor_released(&jvm, &lock).await?;

    let _: () = jvm.invoke_virtual(&writer, "java/io/BufferedWriter", "flush", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&writer, "nextChar", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&backing, "written", "I").await?, 2);

    jvm.put_field(&mut backing, "failClose", "Z", true).await?;
    let failed_close: Result<()> = jvm.invoke_virtual(&writer, "java/io/BufferedWriter", "close", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = failed_close else {
        panic!("backing close failure must escape BufferedWriter.close");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    assert_eq!(jvm.get_field::<i32>(&backing, "closeCalls", "I").await?, 1);
    let _: () = jvm.invoke_virtual(&writer, "java/io/BufferedWriter", "close", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&backing, "closeCalls", "I").await?, 1);
    let closed: Result<()> = jvm
        .invoke_virtual(&writer, "java/io/BufferedWriter", "write", "(I)V", ('C' as i32,))
        .await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("BufferedWriter must remain closed after a backing close failure");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    assert_monitor_released(&jvm, &lock).await?;

    let backing: ClassInstanceRef<LockCheckingWriter> = jvm.new_class("LockCheckingWriter", "(IZ)V", (1, false)).await?.into();
    let backing_writer: ClassInstanceRef<Writer> = backing.instance.clone().into();
    let writer: ClassInstanceRef<BufferedWriter> = jvm
        .new_class("java/io/BufferedWriter", "(Ljava/io/Writer;I)V", (backing_writer, 4))
        .await?
        .into();
    let _: () = jvm
        .invoke_virtual(&writer, "java/io/BufferedWriter", "write", "(I)V", ('X' as i32,))
        .await?;
    let failed_close: Result<()> = jvm.invoke_virtual(&writer, "java/io/BufferedWriter", "close", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = failed_close else {
        panic!("close must report a buffered backing write failure");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    assert_eq!(jvm.get_field::<i32>(&writer, "nextChar", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing, "closeCalls", "I").await?, 1);
    let closed: Result<()> = jvm
        .invoke_virtual(&writer, "java/io/BufferedWriter", "write", "(I)V", ('Y' as i32,))
        .await;
    assert!(closed.is_err(), "BufferedWriter must close even when flushing during close fails");

    Ok(())
}

#[tokio::test]
async fn character_streams_release_inherited_locks_after_errors() -> Result<()> {
    let jvm = stream_locking_jvm().await?;

    let value = JavaLangString::from_rust_string(&jvm, "abc").await?;
    let string_reader = jvm.new_class("java/io/StringReader", "(Ljava/lang/String;)V", (value,)).await?;
    let target = jvm.instantiate_array("C", 1).await?;
    let invalid: Result<i32> = jvm
        .invoke_virtual(
            &string_reader,
            &string_reader.class_definition().name(),
            "read",
            "([CII)I",
            (target, -1, 1),
        )
        .await;
    assert!(invalid.is_err());
    let lock: ClassInstanceRef<Object> = jvm.get_field(&string_reader, "lock", "Ljava/lang/Object;").await?;
    assert_eq!(lock.identity(), string_reader.identity());
    assert_monitor_released(&jvm, &lock).await?;

    let chars = jvm.instantiate_array("C", 1).await?;
    let char_reader = jvm.new_class("java/io/CharArrayReader", "([C)V", (chars,)).await?;
    let target = jvm.instantiate_array("C", 1).await?;
    let invalid: Result<i32> = jvm
        .invoke_virtual(&char_reader, &char_reader.class_definition().name(), "read", "([CII)I", (target, 2, 1))
        .await;
    assert!(invalid.is_err());
    let lock: ClassInstanceRef<Object> = jvm.get_field(&char_reader, "lock", "Ljava/lang/Object;").await?;
    assert_eq!(lock.identity(), char_reader.identity());
    assert_monitor_released(&jvm, &lock).await?;

    let char_writer = jvm.new_class("java/io/CharArrayWriter", "()V", ()).await?;
    let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
    let invalid: Result<()> = jvm
        .invoke_virtual(
            &char_writer,
            &char_writer.class_definition().name(),
            "write",
            "([CII)V",
            (null_chars, 0, 1),
        )
        .await;
    assert!(invalid.is_err());
    let lock: ClassInstanceRef<Object> = jvm.get_field(&char_writer, "lock", "Ljava/lang/Object;").await?;
    assert_eq!(lock.identity(), char_writer.identity());
    assert_monitor_released(&jvm, &lock).await?;

    Ok(())
}

#[tokio::test]
async fn character_stream_operations_and_close_wait_for_inherited_locks() -> Result<()> {
    let jvm = stream_locking_jvm().await?;

    let backing: ClassInstanceRef<LockCheckingWriter> = jvm.new_class("LockCheckingWriter", "(IZ)V", (0, false)).await?.into();
    let writer: ClassInstanceRef<Object> = jvm
        .new_class(
            "java/io/BufferedWriter",
            "(Ljava/io/Writer;)V",
            (ClassInstanceRef::<Writer>::from(backing.instance.clone()),),
        )
        .await?
        .into();
    let lock: ClassInstanceRef<Object> = jvm.get_field(&writer, "lock", "Ljava/lang/Object;").await?;
    assert_worker_waits_for_lock(&jvm, writer.instance.clone().into(), lock.clone(), 3).await?;
    assert_worker_waits_for_lock(&jvm, writer.instance.into(), lock, 2).await?;

    let value = JavaLangString::from_rust_string(&jvm, "abc").await?;
    let string_reader: ClassInstanceRef<Object> = jvm.new_class("java/io/StringReader", "(Ljava/lang/String;)V", (value,)).await?.into();
    let lock: ClassInstanceRef<Object> = jvm.get_field(&string_reader, "lock", "Ljava/lang/Object;").await?;
    assert_worker_waits_for_lock(&jvm, string_reader.instance.clone().into(), lock.clone(), 0).await?;
    assert_worker_waits_for_lock(&jvm, string_reader.instance.clone().into(), lock, 2).await?;
    let closed: Result<i32> = jvm
        .invoke_virtual(&string_reader, &string_reader.class_definition().name(), "read", "()I", ())
        .await;
    assert!(closed.is_err());

    let chars = jvm.instantiate_array("C", 1).await?;
    let char_reader: ClassInstanceRef<Object> = jvm.new_class("java/io/CharArrayReader", "([C)V", (chars,)).await?.into();
    let lock: ClassInstanceRef<Object> = jvm.get_field(&char_reader, "lock", "Ljava/lang/Object;").await?;
    assert_worker_waits_for_lock(&jvm, char_reader.instance.clone().into(), lock, 2).await?;
    let closed: Result<i32> = jvm
        .invoke_virtual(&char_reader, &char_reader.class_definition().name(), "read", "()I", ())
        .await;
    assert!(closed.is_err());

    let char_writer: ClassInstanceRef<Object> = jvm.new_class("java/io/CharArrayWriter", "()V", ()).await?.into();
    let lock: ClassInstanceRef<Object> = jvm.get_field(&char_writer, "lock", "Ljava/lang/Object;").await?;
    assert_worker_waits_for_lock(&jvm, char_writer.instance.clone().into(), lock, 1).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&char_writer, &char_writer.class_definition().name(), "size", "()I", ())
            .await?,
        1
    );

    Ok(())
}
