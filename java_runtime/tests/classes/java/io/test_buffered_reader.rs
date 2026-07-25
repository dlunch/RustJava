use alloc::{boxed::Box, collections::BTreeMap, vec, vec::Vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{BufferedReader, Reader},
        lang::String,
    },
};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

struct ChunkedReader;

impl ChunkedReader {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ChunkedReader",
            parent_class: Some("java/io/Reader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([CI)V", Self::init, Default::default()),
                JavaMethodProto::new("read", "([CII)I", Self::read, Default::default()),
                JavaMethodProto::new("ready", "()Z", Self::ready, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("data", "[C", Default::default()),
                JavaFieldProto::new("position", "I", Default::default()),
                JavaFieldProto::new("chunkSize", "I", Default::default()),
                JavaFieldProto::new("visibleLength", "I", Default::default()),
                JavaFieldProto::new("zeroReads", "I", Default::default()),
                JavaFieldProto::new("closed", "Z", Default::default()),
                JavaFieldProto::new("closeCount", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<JavaChar>>,
        chunk_size: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/io/Reader", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "data", "[C", data).await?;
        jvm.put_field(&mut this, "position", "I", 0).await?;
        jvm.put_field(&mut this, "chunkSize", "I", chunk_size).await?;
        let data: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "data", "[C").await?;
        let data_length = jvm.array_length(&data).await? as i32;
        jvm.put_field(&mut this, "visibleLength", "I", data_length).await?;
        jvm.put_field(&mut this, "zeroReads", "I", 0).await?;
        jvm.put_field(&mut this, "closed", "Z", false).await?;
        jvm.put_field(&mut this, "closeCount", "I", 0).await?;
        Ok(())
    }

    async fn read(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        mut target: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        if jvm.get_field::<bool>(&this, "closed", "Z").await? {
            return Err(jvm.exception("java/io/IOException", "reader is closed").await);
        }

        let target_length = jvm.array_length(&target).await? as i32;
        if offset < 0 || length < 0 || offset > target_length || length > target_length - offset {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid offset or length").await);
        }
        if length == 0 {
            return Ok(0);
        }

        let zero_reads: i32 = jvm.get_field(&this, "zeroReads", "I").await?;
        if zero_reads > 0 {
            jvm.put_field(&mut this, "zeroReads", "I", zero_reads - 1).await?;
            return Ok(0);
        }

        let data: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "data", "[C").await?;
        let position: i32 = jvm.get_field(&this, "position", "I").await?;
        let visible_length: i32 = jvm.get_field(&this, "visibleLength", "I").await?;
        let available = visible_length.min(jvm.array_length(&data).await? as i32) - position;
        if available == 0 {
            return Ok(-1);
        }

        let chunk_size: i32 = jvm.get_field(&this, "chunkSize", "I").await?;
        let count = length.min(available).min(chunk_size);
        let values: Vec<JavaChar> = jvm.load_array(&data, position as usize, count as usize).await?;
        jvm.store_array(&mut target, offset as usize, values).await?;
        jvm.put_field(&mut this, "position", "I", position + count).await?;
        Ok(count)
    }

    async fn ready(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        if jvm.get_field::<bool>(&this, "closed", "Z").await? {
            return Err(jvm.exception("java/io/IOException", "reader is closed").await);
        }

        let data: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "data", "[C").await?;
        let position: i32 = jvm.get_field(&this, "position", "I").await?;
        let visible_length: i32 = jvm.get_field(&this, "visibleLength", "I").await?;
        Ok(position < visible_length.min(jvm.array_length(&data).await? as i32))
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let close_count: i32 = jvm.get_field(&this, "closeCount", "I").await?;
        jvm.put_field(&mut this, "closeCount", "I", close_count + 1).await?;
        jvm.put_field(&mut this, "closed", "Z", true).await?;
        Ok(())
    }
}

struct ReadRunner;

impl ReadRunner {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "BufferedReaderReadRunner",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/BufferedReader;)V", Self::init, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("reader", "Ljava/io/BufferedReader;", Default::default()),
                JavaFieldProto::new("started", "Z", Default::default()),
                JavaFieldProto::new("done", "Z", Default::default()),
                JavaFieldProto::new("value", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, reader: ClassInstanceRef<BufferedReader>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "reader", "Ljava/io/BufferedReader;", reader).await?;
        jvm.put_field(&mut this, "started", "Z", false).await?;
        jvm.put_field(&mut this, "done", "Z", false).await?;
        jvm.put_field(&mut this, "value", "I", -1).await?;
        Ok(())
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "started", "Z", true).await?;
        let reader: ClassInstanceRef<BufferedReader> = jvm.get_field(&this, "reader", "Ljava/io/BufferedReader;").await?;
        let value: i32 = jvm.invoke_virtual(&reader, "read", "()I", ()).await?;
        jvm.put_field(&mut this, "value", "I", value).await?;
        jvm.put_field(&mut this, "done", "Z", true).await?;
        Ok(())
    }
}

async fn buffered_reader(
    value: &str,
    chunk_size: i32,
    buffer_size: i32,
) -> Result<(Jvm, ClassInstanceRef<ChunkedReader>, ClassInstanceRef<BufferedReader>)> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            ChunkedReader::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            ReadRunner::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;

    let utf16: Vec<JavaChar> = value.encode_utf16().collect();
    let mut data = jvm.instantiate_array("C", utf16.len()).await?;
    jvm.store_array(&mut data, 0, utf16).await?;
    let source = jvm.new_class("ChunkedReader", "([CI)V", (data, chunk_size)).await?;
    let reader = jvm
        .new_class("java/io/BufferedReader", "(Ljava/io/Reader;I)V", (source.clone(), buffer_size))
        .await?;

    Ok((jvm, source.into(), reader.into()))
}

#[tokio::test]
async fn test_buffered_reader_constructors_and_read_contract() -> Result<()> {
    let (jvm, _, reader) = buffered_reader("abc", 1, 2).await?;

    let mut chars = jvm.instantiate_array("C", 4).await?;
    jvm.store_array(&mut chars, 0, ['?' as JavaChar; 4]).await?;

    let invalid_range: Result<i32> = jvm.invoke_virtual(&reader, "read", "([CII)I", (chars.clone(), -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = invalid_range else {
        panic!("negative offset must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let overflowing_range: Result<i32> = jvm.invoke_virtual(&reader, "read", "([CII)I", (chars.clone(), 3, 2)).await;
    let Err(JavaError::JavaException(exception)) = overflowing_range else {
        panic!("overflowing range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
    let null_result: Result<i32> = jvm.invoke_virtual(&reader, "read", "([CII)I", (null_chars, 0, 0)).await;
    let Err(JavaError::JavaException(exception)) = null_result else {
        panic!("null destination must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'a' as i32);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (chars.clone(), 1, 2)).await?, 2);
    assert_eq!(
        jvm.load_array::<JavaChar>(&chars, 0, 4).await?,
        ['?' as JavaChar, 'b' as JavaChar, 'c' as JavaChar, '?' as JavaChar]
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, -1);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (chars.clone(), 0, 0)).await?, 0);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (chars.clone(), 0, 1)).await?, -1);

    let empty = jvm.instantiate_array("C", 0).await?;
    let input = jvm.new_class("ChunkedReader", "([CI)V", (empty, 1)).await?;
    let default_reader = jvm.new_class("java/io/BufferedReader", "(Ljava/io/Reader;)V", (input.clone(),)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&default_reader, "read", "()I", ()).await?, -1);

    for size in [0, -1] {
        let invalid_size = jvm
            .new_class("java/io/BufferedReader", "(Ljava/io/Reader;I)V", (input.clone(), size))
            .await;
        let Err(JavaError::JavaException(exception)) = invalid_size else {
            panic!("invalid buffer size must throw IllegalArgumentException");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));
    }

    let null_reader: ClassInstanceRef<Reader> = None.into();
    let null_constructor = jvm
        .new_class("java/io/BufferedReader", "(Ljava/io/Reader;)V", (null_reader.clone(),))
        .await;
    let Err(JavaError::JavaException(exception)) = null_constructor else {
        panic!("null reader must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null_sized_constructor = jvm.new_class("java/io/BufferedReader", "(Ljava/io/Reader;I)V", (null_reader, 2)).await;
    let Err(JavaError::JavaException(exception)) = null_sized_constructor else {
        panic!("null reader with explicit size must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_buffered_reader_read_line_endings_and_long_lines() -> Result<()> {
    let long_line = "0123456789abcdef";
    let input = alloc::format!("first\nsecond\rthird\r\n\n{long_line}");
    let (jvm, _, reader) = buffered_reader(&input, 1, 3).await?;

    for expected in ["first", "second", "third", "", long_line] {
        let line: ClassInstanceRef<String> = jvm.invoke_virtual(&reader, "readLine", "()Ljava/lang/String;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &line).await?, expected);
    }

    let line: ClassInstanceRef<String> = jvm.invoke_virtual(&reader, "readLine", "()Ljava/lang/String;", ()).await?;
    assert!(line.is_null());

    Ok(())
}

#[tokio::test]
async fn test_buffered_reader_mixed_reads_skip_and_ready_share_cursor() -> Result<()> {
    let (jvm, source, reader) = buffered_reader("abc\ndef", 16, 8).await?;

    let negative_skip: Result<i64> = jvm.invoke_virtual(&reader, "skip", "(J)J", (-1i64,)).await;
    let Err(JavaError::JavaException(exception)) = negative_skip else {
        panic!("negative skip must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    assert!(jvm.invoke_virtual::<_, bool>(&reader, "ready", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'a' as i32);
    assert!(!jvm.invoke_virtual::<_, bool>(&source, "ready", "()Z", ()).await?);
    assert!(jvm.invoke_virtual::<_, bool>(&reader, "ready", "()Z", ()).await?);

    let chars = jvm.instantiate_array("C", 2).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (chars.clone(), 0, 2)).await?, 2);
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 2).await?, ['b' as JavaChar, 'c' as JavaChar]);

    let line: ClassInstanceRef<String> = jvm.invoke_virtual(&reader, "readLine", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &line).await?, "");
    assert_eq!(jvm.invoke_virtual::<_, i64>(&reader, "skip", "(J)J", (2i64,)).await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'f' as i32);
    assert!(!jvm.invoke_virtual::<_, bool>(&reader, "ready", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i64>(&reader, "skip", "(J)J", (2i64,)).await?, 0);

    Ok(())
}

#[tokio::test]
async fn test_buffered_reader_mark_reset_preservation_and_invalidation() -> Result<()> {
    let (jvm, _, reader) = buffered_reader("abcdef", 2, 3).await?;

    assert!(jvm.invoke_virtual::<_, bool>(&reader, "markSupported", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'a' as i32);
    let _: () = jvm.invoke_virtual(&reader, "mark", "(I)V", (4,)).await?;

    let negative_mark: Result<()> = jvm.invoke_virtual(&reader, "mark", "(I)V", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = negative_mark else {
        panic!("negative read-ahead limit must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let chars = jvm.instantiate_array("C", 4).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (chars.clone(), 0, 4)).await?, 4);
    assert_eq!(
        jvm.load_array::<JavaChar>(&chars, 0, 4).await?,
        ['b' as JavaChar, 'c' as JavaChar, 'd' as JavaChar, 'e' as JavaChar]
    );
    let _: () = jvm.invoke_virtual(&reader, "reset", "()V", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (chars.clone(), 0, 4)).await?, 4);
    assert_eq!(
        jvm.load_array::<JavaChar>(&chars, 0, 4).await?,
        ['b' as JavaChar, 'c' as JavaChar, 'd' as JavaChar, 'e' as JavaChar]
    );

    let (jvm, _, reader) = buffered_reader("abcd", 2, 2).await?;
    let unset_reset: Result<()> = jvm.invoke_virtual(&reader, "reset", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = unset_reset else {
        panic!("reset without mark must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    let _: () = jvm.invoke_virtual(&reader, "mark", "(I)V", (2,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'a' as i32);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'b' as i32);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'c' as i32);
    let invalidated_reset: Result<()> = jvm.invoke_virtual(&reader, "reset", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = invalidated_reset else {
        panic!("reset beyond read-ahead limit must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    Ok(())
}

#[tokio::test]
async fn test_buffered_reader_mark_restores_pending_crlf_state() -> Result<()> {
    let (jvm, _, reader) = buffered_reader("a\r\nb", 1, 2).await?;

    let line: ClassInstanceRef<String> = jvm.invoke_virtual(&reader, "readLine", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &line).await?, "a");
    let _: () = jvm.invoke_virtual(&reader, "mark", "(I)V", (2,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'b' as i32);
    let _: () = jvm.invoke_virtual(&reader, "reset", "()V", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'b' as i32);

    Ok(())
}

#[tokio::test]
async fn test_buffered_reader_ready_preserves_pending_lf_until_input_is_available() -> Result<()> {
    let (jvm, mut source, reader) = buffered_reader("\r\nX", 3, 2).await?;
    jvm.put_field(&mut source, "visibleLength", "I", 1).await?;

    let line: ClassInstanceRef<String> = jvm.invoke_virtual(&reader, "readLine", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &line).await?, "");
    assert!(!jvm.invoke_virtual::<_, bool>(&reader, "ready", "()Z", ()).await?);

    jvm.put_field(&mut source, "visibleLength", "I", 3).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'X' as i32);

    Ok(())
}

#[tokio::test]
async fn test_buffered_reader_fill_retries_temporary_zero_read() -> Result<()> {
    let (jvm, mut source, reader) = buffered_reader("a", 1, 1).await?;
    jvm.put_field(&mut source, "zeroReads", "I", 1).await?;

    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'a' as i32);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, -1);

    Ok(())
}

#[tokio::test]
async fn test_buffered_reader_serializes_on_inherited_reader_lock() -> Result<()> {
    let (jvm, source, reader) = buffered_reader("a", 1, 1).await?;
    let runner = jvm
        .new_class("BufferedReaderReadRunner", "(Ljava/io/BufferedReader;)V", (reader,))
        .await?;
    let thread = jvm.new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (runner.clone(),)).await?;

    jvm.monitor_enter(&source).await?;
    let _: () = jvm.invoke_virtual(&thread, "start", "()V", ()).await?;

    let mut started = false;
    for _ in 0..100 {
        started = jvm.get_field::<bool>(&runner, "started", "Z").await?;
        if started {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    tokio::time::sleep(Duration::from_millis(10)).await;
    let completed_while_lock_was_held = jvm.get_field::<bool>(&runner, "done", "Z").await?;

    jvm.monitor_exit(&source).await?;
    let _: () = jvm.invoke_virtual(&thread, "join", "()V", ()).await?;

    assert!(started, "worker thread did not start");
    assert!(!completed_while_lock_was_held, "read did not synchronize on Reader.lock");
    assert_eq!(jvm.get_field::<i32>(&runner, "value", "I").await?, 'a' as i32);

    Ok(())
}

#[tokio::test]
async fn test_buffered_reader_close_is_idempotent_and_closes_all_operations() -> Result<()> {
    let (jvm, source, reader) = buffered_reader("abc", 2, 2).await?;

    let _: () = jvm.invoke_virtual(&reader, "close", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&reader, "close", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&source, "closeCount", "I").await?, 1);
    assert!(jvm.invoke_virtual::<_, bool>(&reader, "markSupported", "()Z", ()).await?);

    let chars = jvm.instantiate_array("C", 1).await?;
    let operations: Vec<(&str, Result<()>)> = vec![
        ("read()", jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await.map(|_| ())),
        (
            "read(char[],off,len)",
            jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (chars, 0, 1)).await.map(|_| ()),
        ),
        (
            "readLine",
            jvm.invoke_virtual::<_, ClassInstanceRef<String>>(&reader, "readLine", "()Ljava/lang/String;", ())
                .await
                .map(|_| ()),
        ),
        ("skip", jvm.invoke_virtual::<_, i64>(&reader, "skip", "(J)J", (1i64,)).await.map(|_| ())),
        ("ready", jvm.invoke_virtual::<_, bool>(&reader, "ready", "()Z", ()).await.map(|_| ())),
        ("mark", jvm.invoke_virtual(&reader, "mark", "(I)V", (1,)).await),
        ("reset", jvm.invoke_virtual(&reader, "reset", "()V", ()).await),
    ];

    for (name, result) in operations {
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name} must throw IOException after close");
        };
        assert!(jvm.is_instance(&*exception, "java/io/IOException"), "{name} threw the wrong exception");
    }

    Ok(())
}
