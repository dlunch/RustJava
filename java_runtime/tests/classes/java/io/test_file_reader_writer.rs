use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec::Vec};
use core::{
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};
use std::sync::Mutex;

use java_runtime::{
    File as RuntimeFile, FileDescriptorId, FileOpenOptions, FileSize, FileStat, FileType, IOError, IOResult, RT_RUSTJAR, Runtime, SpawnCallback,
    classes::java::lang::Object, get_runtime_class_proto,
};
use jvm::{Array, ClassDefinition, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;
use test_utils::{TestRuntime, create_test_jvm, test_jvm_filesystem};

type MemoryFiles = Arc<Mutex<BTreeMap<alloc::string::String, Arc<Mutex<Vec<u8>>>>>>;

#[derive(Clone, Copy)]
struct MemoryFileConfig {
    max_write_size: Option<usize>,
    zero_on_write_call: Option<u32>,
    error_on_write_call: Option<u32>,
    seek_supported: bool,
    set_len_supported: bool,
    metadata_supported: bool,
}

impl Default for MemoryFileConfig {
    fn default() -> Self {
        Self {
            max_write_size: None,
            zero_on_write_call: None,
            error_on_write_call: None,
            seek_supported: true,
            set_len_supported: true,
            metadata_supported: true,
        }
    }
}

#[derive(Clone)]
struct MemoryFile {
    data: Arc<Mutex<Vec<u8>>>,
    position: Arc<Mutex<FileSize>>,
    read: bool,
    write: bool,
    append: bool,
    config: MemoryFileConfig,
    write_calls: Arc<AtomicU32>,
    seek_calls: Arc<AtomicU32>,
    set_len_calls: Arc<AtomicU32>,
    metadata_calls: Arc<AtomicU32>,
}

#[async_trait::async_trait]
impl RuntimeFile for MemoryFile {
    async fn read(&mut self, buffer: &mut [u8]) -> IOResult<usize> {
        if !self.read {
            return Err(IOError::Unsupported);
        }

        let data = self.data.lock().unwrap();
        let mut position = self.position.lock().unwrap();
        let available = &data[(*position).min(data.len() as FileSize) as usize..];
        let length = available.len().min(buffer.len());
        buffer[..length].copy_from_slice(&available[..length]);
        *position += length as FileSize;
        Ok(length)
    }

    async fn write(&mut self, buffer: &[u8]) -> IOResult<usize> {
        if !self.write {
            return Err(IOError::Unsupported);
        }

        let write_call = self.write_calls.fetch_add(1, Ordering::SeqCst) + 1;
        if self.config.error_on_write_call == Some(write_call) {
            return Err(IOError::Io);
        }
        if self.config.zero_on_write_call == Some(write_call) {
            return Ok(0);
        }

        let length = self.config.max_write_size.map_or(buffer.len(), |limit| limit.min(buffer.len()));
        let mut data = self.data.lock().unwrap();
        let mut position = self.position.lock().unwrap();
        let start = if self.append { data.len() } else { *position as usize };
        let end = start + length;
        if data.len() < end {
            data.resize(end, 0);
        }
        data[start..end].copy_from_slice(&buffer[..length]);
        *position = end as FileSize;
        Ok(length)
    }

    async fn seek(&mut self, position: FileSize) -> IOResult<()> {
        self.seek_calls.fetch_add(1, Ordering::SeqCst);
        if !self.config.seek_supported {
            return Err(IOError::Unsupported);
        }
        *self.position.lock().unwrap() = position;
        Ok(())
    }

    async fn tell(&self) -> IOResult<FileSize> {
        Ok(*self.position.lock().unwrap())
    }

    async fn set_len(&mut self, length: FileSize) -> IOResult<()> {
        self.set_len_calls.fetch_add(1, Ordering::SeqCst);
        if !self.write || !self.config.set_len_supported {
            return Err(IOError::Unsupported);
        }
        self.data.lock().unwrap().resize(length as usize, 0);
        let mut position = self.position.lock().unwrap();
        *position = (*position).min(length);
        Ok(())
    }

    async fn metadata(&self) -> IOResult<FileStat> {
        self.metadata_calls.fetch_add(1, Ordering::SeqCst);
        if !self.config.metadata_supported {
            return Err(IOError::Unsupported);
        }
        Ok(FileStat {
            size: self.data.lock().unwrap().len() as FileSize,
            r#type: FileType::File,
        })
    }
}

#[derive(Clone)]
struct MemoryRuntime {
    classes: TestRuntime,
    files: MemoryFiles,
    handles: Arc<Mutex<BTreeMap<u32, Box<dyn RuntimeFile>>>>,
    next_fd: Arc<AtomicU32>,
    open_calls: Arc<Mutex<Vec<(alloc::string::String, FileOpenOptions)>>>,
    seek_calls: Arc<AtomicU32>,
    set_len_calls: Arc<AtomicU32>,
    metadata_calls: Arc<AtomicU32>,
    file_config: MemoryFileConfig,
}

impl MemoryRuntime {
    fn new(files: BTreeMap<alloc::string::String, Vec<u8>>) -> Self {
        Self {
            classes: TestRuntime::new(BTreeMap::new()),
            files: Arc::new(Mutex::new(
                files.into_iter().map(|(path, data)| (path, Arc::new(Mutex::new(data)))).collect(),
            )),
            handles: Arc::new(Mutex::new(BTreeMap::new())),
            next_fd: Arc::new(AtomicU32::new(1)),
            open_calls: Arc::new(Mutex::new(Vec::new())),
            seek_calls: Arc::new(AtomicU32::new(0)),
            set_len_calls: Arc::new(AtomicU32::new(0)),
            metadata_calls: Arc::new(AtomicU32::new(0)),
            file_config: MemoryFileConfig::default(),
        }
    }
}

#[async_trait::async_trait]
impl Runtime for MemoryRuntime {
    async fn sleep(&self, duration: Duration) {
        self.classes.sleep(duration).await;
    }

    async fn r#yield(&self) {
        self.classes.r#yield().await;
    }

    fn spawn(&self, jvm: &Jvm, callback: Box<dyn SpawnCallback>) {
        self.classes.spawn(jvm, callback);
    }

    fn exit(&self, status: i32) {
        self.classes.exit(status);
    }

    fn now(&self) -> u64 {
        self.classes.now()
    }

    fn current_task_id(&self) -> u64 {
        self.classes.current_task_id()
    }

    fn stdin(&self) -> IOResult<FileDescriptorId> {
        Err(IOError::NotFound)
    }

    fn stdout(&self) -> IOResult<FileDescriptorId> {
        Err(IOError::NotFound)
    }

    fn stderr(&self) -> IOResult<FileDescriptorId> {
        Err(IOError::NotFound)
    }

    async fn open(&self, path: &str, options: FileOpenOptions) -> IOResult<FileDescriptorId> {
        self.open_calls.lock().unwrap().push((path.into(), options));
        let write = options.write || options.append;
        if !options.read && !write {
            return Err(IOError::Unsupported);
        }
        if (options.create || options.truncate) && !write {
            return Err(IOError::Unsupported);
        }

        let data = {
            let mut files = self.files.lock().unwrap();
            match files.get(path) {
                Some(data) => data.clone(),
                None if options.create => {
                    let data = Arc::new(Mutex::new(Vec::new()));
                    files.insert(path.into(), data.clone());
                    data
                }
                None => return Err(IOError::NotFound),
            }
        };
        if options.truncate {
            data.lock().unwrap().clear();
        }
        let id = self.next_fd.fetch_add(1, Ordering::SeqCst);
        self.handles.lock().unwrap().insert(
            id,
            Box::new(MemoryFile {
                data,
                position: Arc::new(Mutex::new(0)),
                read: options.read,
                write,
                append: options.append,
                config: self.file_config,
                write_calls: Arc::new(AtomicU32::new(0)),
                seek_calls: self.seek_calls.clone(),
                set_len_calls: self.set_len_calls.clone(),
                metadata_calls: self.metadata_calls.clone(),
            }),
        );
        Ok(FileDescriptorId::new(id))
    }

    fn get_file(&self, fd: FileDescriptorId) -> IOResult<Box<dyn RuntimeFile>> {
        self.handles.lock().unwrap().get(&fd.id()).cloned().ok_or(IOError::NotFound)
    }

    fn close_file(&self, fd: FileDescriptorId) {
        self.handles.lock().unwrap().remove(&fd.id());
    }

    async fn unlink(&self, path: &str) -> IOResult<()> {
        self.files.lock().unwrap().remove(path).map(|_| ()).ok_or(IOError::NotFound)
    }

    async fn metadata(&self, path: &str) -> IOResult<FileStat> {
        let files = self.files.lock().unwrap();
        let data = files.get(path).ok_or(IOError::NotFound)?;
        Ok(FileStat {
            size: data.lock().unwrap().len() as FileSize,
            r#type: FileType::File,
        })
    }

    async fn find_rustjar_class(&self, _jvm: &Jvm, classpath: &str, class: &str) -> Result<Option<Box<dyn ClassDefinition>>> {
        if classpath == RT_RUSTJAR
            && let Some(proto) = get_runtime_class_proto(class)
        {
            return Ok(Some(Box::new(ClassDefinitionImpl::from_class_proto(
                proto,
                Box::new(self.clone()) as Box<_>,
            ))));
        }
        Ok(None)
    }

    async fn define_class(&self, jvm: &Jvm, data: &[u8]) -> Result<Box<dyn ClassDefinition>> {
        self.classes.define_class(jvm, data).await
    }

    async fn define_array_class(&self, jvm: &Jvm, element_type_name: &str) -> Result<Box<dyn ClassDefinition>> {
        self.classes.define_array_class(jvm, element_type_name).await
    }
}

#[tokio::test]
async fn file_01_file_reader_constructor_contracts() -> Result<()> {
    let filesystem = [("input.txt".into(), "A한B".as_bytes().to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;
    let path = JavaLangString::from_rust_string(&jvm, "input.txt").await?;

    let reader = jvm.new_class("java/io/FileReader", "(Ljava/lang/String;)V", (path.clone(),)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'A' as i32);

    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path.clone(),)).await?;
    let reader = jvm.new_class("java/io/FileReader", "(Ljava/io/File;)V", (file.clone(),)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'A' as i32);

    let input = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;
    let descriptor: ClassInstanceRef<java_runtime::classes::java::io::FileDescriptor> =
        jvm.get_field(&input, "fd", "Ljava/io/FileDescriptor;").await?;
    let reader = jvm.new_class("java/io/FileReader", "(Ljava/io/FileDescriptor;)V", (descriptor,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'A' as i32);

    let null_path: ClassInstanceRef<java_runtime::classes::java::lang::String> = None.into();
    let result = jvm.new_class("java/io/FileReader", "(Ljava/lang/String;)V", (null_path,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null path must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn file_02_file_03_file_writer_constructor_and_append_contracts() -> Result<()> {
    let runtime = MemoryRuntime::new([("output.txt".into(), b"old data".to_vec())].into_iter().collect());
    let jvm = create_test_jvm(runtime.clone()).await?;
    let path = JavaLangString::from_rust_string(&jvm, "output.txt").await?;

    let writer = jvm.new_class("java/io/FileWriter", "(Ljava/lang/String;)V", (path.clone(),)).await?;
    let value = JavaLangString::from_rust_string(&jvm, "new").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;
    assert_eq!(&*runtime.files.lock().unwrap()["output.txt"].lock().unwrap(), b"new");
    let value = JavaLangString::from_rust_string(&jvm, "!").await?;
    let closed: Result<()> = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await;
    let Err(JavaError::JavaException(exception)) = closed else {
        panic!("write after close must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path.clone(),)).await?;
    let writer = jvm.new_class("java/io/FileWriter", "(Ljava/io/File;Z)V", (file.clone(), true)).await?;
    let value = JavaLangString::from_rust_string(&jvm, "+file").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;
    assert_eq!(&*runtime.files.lock().unwrap()["output.txt"].lock().unwrap(), b"new+file");

    let writer = jvm
        .new_class("java/io/FileWriter", "(Ljava/lang/String;Z)V", (path.clone(), true))
        .await?;
    let value = JavaLangString::from_rust_string(&jvm, "+path").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;
    assert_eq!(&*runtime.files.lock().unwrap()["output.txt"].lock().unwrap(), b"new+file+path");

    let first = jvm
        .new_class("java/io/FileWriter", "(Ljava/lang/String;Z)V", (path.clone(), true))
        .await?;
    let second = jvm
        .new_class("java/io/FileWriter", "(Ljava/lang/String;Z)V", (path.clone(), true))
        .await?;
    for (writer, text) in [(&first, "+first"), (&second, "+second"), (&first, "+third"), (&second, "+fourth")] {
        let value = JavaLangString::from_rust_string(&jvm, text).await?;
        let _: () = jvm.invoke_virtual(writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    }
    let _: () = jvm.invoke_virtual(&first, "close", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&second, "close", "()V", ()).await?;
    assert_eq!(
        &*runtime.files.lock().unwrap()["output.txt"].lock().unwrap(),
        b"new+file+path+first+second+third+fourth"
    );

    let writer = jvm.new_class("java/io/FileWriter", "(Ljava/io/File;)V", (file.clone(),)).await?;
    let value = JavaLangString::from_rust_string(&jvm, "reset").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;
    assert_eq!(&*runtime.files.lock().unwrap()["output.txt"].lock().unwrap(), b"reset");

    let output = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?;
    let descriptor: ClassInstanceRef<java_runtime::classes::java::io::FileDescriptor> =
        jvm.get_field(&output, "fd", "Ljava/io/FileDescriptor;").await?;
    let writer = jvm.new_class("java/io/FileWriter", "(Ljava/io/FileDescriptor;)V", (descriptor,)).await?;
    let value = JavaLangString::from_rust_string(&jvm, "F").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;
    assert_eq!(&*runtime.files.lock().unwrap()["output.txt"].lock().unwrap(), b"F");

    let null_path: ClassInstanceRef<java_runtime::classes::java::lang::String> = None.into();
    let result = jvm.new_class("java/io/FileWriter", "(Ljava/lang/String;)V", (null_path,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null path must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn file_open_options_and_handle_access_are_enforced() -> Result<()> {
    let runtime = MemoryRuntime::new(
        [
            ("input.txt".into(), b"input".to_vec()),
            ("output.txt".into(), b"output".to_vec()),
            ("random-read.txt".into(), b"read".to_vec()),
            ("random-write.txt".into(), b"write".to_vec()),
        ]
        .into_iter()
        .collect(),
    );
    let jvm = create_test_jvm(runtime.clone()).await?;

    let input_path = JavaLangString::from_rust_string(&jvm, "input.txt").await?;
    let input_file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (input_path,)).await?;
    let _input = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (input_file,)).await?;

    let output_path = JavaLangString::from_rust_string(&jvm, "output.txt").await?;
    let output_file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (output_path,)).await?;
    let _output = jvm
        .new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (output_file.clone(),))
        .await?;
    let _append_output = jvm
        .new_class("java/io/FileOutputStream", "(Ljava/io/File;Z)V", (output_file, true))
        .await?;

    let random_read_path = JavaLangString::from_rust_string(&jvm, "random-read.txt").await?;
    let read_mode = JavaLangString::from_rust_string(&jvm, "r").await?;
    let _random_read = jvm
        .new_class(
            "java/io/RandomAccessFile",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            (random_read_path, read_mode),
        )
        .await?;

    let random_write_path = JavaLangString::from_rust_string(&jvm, "random-write.txt").await?;
    let write_mode = JavaLangString::from_rust_string(&jvm, "rw").await?;
    let _random_write = jvm
        .new_class(
            "java/io/RandomAccessFile",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            (random_write_path, write_mode),
        )
        .await?;

    assert_eq!(
        *runtime.open_calls.lock().unwrap(),
        vec![
            (
                "input.txt".into(),
                FileOpenOptions {
                    read: true,
                    ..Default::default()
                }
            ),
            (
                "output.txt".into(),
                FileOpenOptions {
                    write: true,
                    truncate: true,
                    create: true,
                    ..Default::default()
                }
            ),
            (
                "output.txt".into(),
                FileOpenOptions {
                    write: true,
                    append: true,
                    create: true,
                    ..Default::default()
                }
            ),
            (
                "random-read.txt".into(),
                FileOpenOptions {
                    read: true,
                    ..Default::default()
                }
            ),
            (
                "random-write.txt".into(),
                FileOpenOptions {
                    read: true,
                    write: true,
                    create: true,
                    ..Default::default()
                }
            ),
        ]
    );

    assert!(matches!(
        runtime
            .open(
                "missing.txt",
                FileOpenOptions {
                    read: true,
                    ..Default::default()
                }
            )
            .await,
        Err(IOError::NotFound)
    ));
    let created = runtime
        .open(
            "created.txt",
            FileOpenOptions {
                write: true,
                create: true,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(runtime.files.lock().unwrap().contains_key("created.txt"));

    let read_only = runtime
        .open(
            "input.txt",
            FileOpenOptions {
                read: true,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    let mut read_only = runtime.get_file(read_only).unwrap();
    assert!(matches!(read_only.write(b"x").await, Err(IOError::Unsupported)));
    assert!(matches!(read_only.set_len(0).await, Err(IOError::Unsupported)));

    let mut write_only = runtime.get_file(created).unwrap();
    let mut byte = [0];
    assert!(matches!(write_only.read(&mut byte).await, Err(IOError::Unsupported)));
    assert_eq!(write_only.write(b"x").await.unwrap(), 1);

    let test_runtime = TestRuntime::new([("read-only.txt".into(), b"value".to_vec())].into_iter().collect());
    let descriptor = test_runtime
        .open(
            "read-only.txt",
            FileOpenOptions {
                read: true,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    let mut file = test_runtime.get_file(descriptor).unwrap();
    let mut bytes = [0; 5];
    assert_eq!(file.read(&mut bytes).await.unwrap(), 5);
    assert_eq!(&bytes, b"value");
    for options in [
        FileOpenOptions::default(),
        FileOpenOptions {
            write: true,
            ..Default::default()
        },
        FileOpenOptions {
            append: true,
            ..Default::default()
        },
        FileOpenOptions {
            read: true,
            truncate: true,
            ..Default::default()
        },
        FileOpenOptions {
            read: true,
            create: true,
            ..Default::default()
        },
    ] {
        assert!(matches!(test_runtime.open("read-only.txt", options).await, Err(IOError::Unsupported)));
    }

    Ok(())
}

#[tokio::test]
async fn file_writer_uses_only_opened_handle_write_semantics() -> Result<()> {
    let mut runtime = MemoryRuntime::new([("truncate.txt".into(), b"old".to_vec())].into_iter().collect());
    runtime.file_config.seek_supported = false;
    runtime.file_config.set_len_supported = false;
    let jvm = create_test_jvm(runtime.clone()).await?;
    let path = JavaLangString::from_rust_string(&jvm, "truncate.txt").await?;
    let writer = jvm.new_class("java/io/FileWriter", "(Ljava/lang/String;)V", (path,)).await?;
    let value = JavaLangString::from_rust_string(&jvm, "new").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;

    assert_eq!(&*runtime.files.lock().unwrap()["truncate.txt"].lock().unwrap(), b"new");
    assert_eq!(runtime.seek_calls.load(Ordering::SeqCst), 0);
    assert_eq!(runtime.set_len_calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        runtime.open_calls.lock().unwrap()[0].1,
        FileOpenOptions {
            write: true,
            truncate: true,
            create: true,
            ..Default::default()
        }
    );

    let mut runtime = MemoryRuntime::new([("append.txt".into(), b"start".to_vec())].into_iter().collect());
    runtime.file_config.seek_supported = false;
    runtime.file_config.metadata_supported = false;
    let jvm = create_test_jvm(runtime.clone()).await?;
    let path = JavaLangString::from_rust_string(&jvm, "append.txt").await?;
    let first = jvm
        .new_class("java/io/FileWriter", "(Ljava/lang/String;Z)V", (path.clone(), true))
        .await?;
    let second = jvm.new_class("java/io/FileWriter", "(Ljava/lang/String;Z)V", (path, true)).await?;
    for (writer, text) in [(&first, "-a"), (&second, "-b"), (&first, "-c"), (&second, "-d")] {
        let value = JavaLangString::from_rust_string(&jvm, text).await?;
        let _: () = jvm.invoke_virtual(writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    }
    assert_eq!(&*runtime.files.lock().unwrap()["append.txt"].lock().unwrap(), b"start-a-b-c-d");
    assert_eq!(runtime.seek_calls.load(Ordering::SeqCst), 0);
    assert_eq!(runtime.metadata_calls.load(Ordering::SeqCst), 0);

    Ok(())
}

#[tokio::test]
async fn formatter_file_constructor_truncates_before_writing() -> Result<()> {
    let runtime = MemoryRuntime::new([("formatter.txt".into(), b"old content".to_vec())].into_iter().collect());
    let jvm = create_test_jvm(runtime.clone()).await?;
    let path = JavaLangString::from_rust_string(&jvm, "formatter.txt").await?;
    let formatter = jvm.new_class("java/util/Formatter", "(Ljava/lang/String;)V", (path,)).await?;
    let format = JavaLangString::from_rust_string(&jvm, "new").await?;
    let arguments = ClassInstanceRef::<Array<Object>>::new(None);
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &formatter,
            "format",
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
            (format, arguments),
        )
        .await?;
    let _: () = jvm.invoke_virtual(&formatter, "close", "()V", ()).await?;

    assert_eq!(&*runtime.files.lock().unwrap()["formatter.txt"].lock().unwrap(), b"new");
    assert_eq!(
        runtime.open_calls.lock().unwrap()[0].1,
        FileOpenOptions {
            write: true,
            truncate: true,
            create: true,
            ..Default::default()
        }
    );

    Ok(())
}

#[tokio::test]
async fn file_writers_retry_partial_runtime_writes_until_complete() -> Result<()> {
    let mut runtime = MemoryRuntime::new([("fos.bin".into(), Vec::new())].into_iter().collect());
    runtime.file_config.max_write_size = Some(1);
    let jvm = create_test_jvm(runtime.clone()).await?;
    let path = JavaLangString::from_rust_string(&jvm, "fos.bin").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
    let output = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?;
    let mut bytes = jvm.instantiate_array("B", 4).await?;
    jvm.store_array(&mut bytes, 0, [b'F' as i8, b'O' as i8, b'S' as i8, b'!' as i8]).await?;
    let _: () = jvm.invoke_virtual(&output, "write", "([BII)V", (bytes, 0, 4)).await?;
    assert_eq!(&*runtime.files.lock().unwrap()["fos.bin"].lock().unwrap(), b"FOS!");

    let mut runtime = MemoryRuntime::new([("raf.bin".into(), Vec::new())].into_iter().collect());
    runtime.file_config.max_write_size = Some(1);
    let jvm = create_test_jvm(runtime.clone()).await?;
    let path = JavaLangString::from_rust_string(&jvm, "raf.bin").await?;
    let mode = JavaLangString::from_rust_string(&jvm, "rw").await?;
    let file = jvm
        .new_class("java/io/RandomAccessFile", "(Ljava/lang/String;Ljava/lang/String;)V", (path, mode))
        .await?;
    let mut bytes = jvm.instantiate_array("B", 4).await?;
    jvm.store_array(&mut bytes, 0, [b'R' as i8, b'A' as i8, b'F' as i8, b'!' as i8]).await?;
    let _: () = jvm.invoke_virtual(&file, "write", "([BII)V", (bytes, 0, 4)).await?;
    assert_eq!(&*runtime.files.lock().unwrap()["raf.bin"].lock().unwrap(), b"RAF!");

    let mut runtime = MemoryRuntime::new([("writer.txt".into(), Vec::new())].into_iter().collect());
    runtime.file_config.max_write_size = Some(1);
    let jvm = create_test_jvm(runtime.clone()).await?;
    let path = JavaLangString::from_rust_string(&jvm, "writer.txt").await?;
    let writer = jvm.new_class("java/io/FileWriter", "(Ljava/lang/String;)V", (path,)).await?;
    let value = JavaLangString::from_rust_string(&jvm, "A한B").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    assert_eq!(&*runtime.files.lock().unwrap()["writer.txt"].lock().unwrap(), "A한B".as_bytes());

    Ok(())
}

#[tokio::test]
async fn file_output_stream_reports_zero_and_late_write_failures_with_exact_prefix() -> Result<()> {
    for (path, zero_on_write_call, error_on_write_call) in [("zero.bin", Some(2), None), ("error.bin", None, Some(2))] {
        let mut runtime = MemoryRuntime::new([(path.into(), Vec::new())].into_iter().collect());
        runtime.file_config.max_write_size = Some(1);
        runtime.file_config.zero_on_write_call = zero_on_write_call;
        runtime.file_config.error_on_write_call = error_on_write_call;
        let jvm = create_test_jvm(runtime.clone()).await?;
        let path_string = JavaLangString::from_rust_string(&jvm, path).await?;
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path_string,)).await?;
        let output = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?;
        let mut bytes = jvm.instantiate_array("B", 3).await?;
        jvm.store_array(&mut bytes, 0, [b'a' as i8, b'b' as i8, b'c' as i8]).await?;
        let result: Result<()> = jvm.invoke_virtual(&output, "write", "([BII)V", (bytes, 0, 3)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("incomplete runtime write must throw IOException");
        };
        assert!(jvm.is_instance(&*exception, "java/io/IOException"));
        assert_eq!(&*runtime.files.lock().unwrap()[path].lock().unwrap(), b"a");
    }

    let mut runtime = MemoryRuntime::new([("byte.bin".into(), Vec::new())].into_iter().collect());
    runtime.file_config.zero_on_write_call = Some(1);
    let jvm = create_test_jvm(runtime.clone()).await?;
    let path = JavaLangString::from_rust_string(&jvm, "byte.bin").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
    let output = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?;
    let result: Result<()> = jvm.invoke_virtual(&output, "write", "(I)V", (65,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("zero-byte runtime write must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    assert!(runtime.files.lock().unwrap()["byte.bin"].lock().unwrap().is_empty());

    Ok(())
}

#[tokio::test]
async fn random_access_file_reports_zero_and_late_write_failures_with_exact_prefix() -> Result<()> {
    for (path, zero_on_write_call, error_on_write_call) in [("zero.bin", Some(2), None), ("error.bin", None, Some(2))] {
        let mut runtime = MemoryRuntime::new([(path.into(), Vec::new())].into_iter().collect());
        runtime.file_config.max_write_size = Some(1);
        runtime.file_config.zero_on_write_call = zero_on_write_call;
        runtime.file_config.error_on_write_call = error_on_write_call;
        let jvm = create_test_jvm(runtime.clone()).await?;
        let path_string = JavaLangString::from_rust_string(&jvm, path).await?;
        let mode = JavaLangString::from_rust_string(&jvm, "rw").await?;
        let file = jvm
            .new_class("java/io/RandomAccessFile", "(Ljava/lang/String;Ljava/lang/String;)V", (path_string, mode))
            .await?;
        let mut bytes = jvm.instantiate_array("B", 3).await?;
        jvm.store_array(&mut bytes, 0, [b'a' as i8, b'b' as i8, b'c' as i8]).await?;
        let result: Result<()> = jvm.invoke_virtual(&file, "write", "([BII)V", (bytes, 0, 3)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("incomplete runtime write must throw IOException");
        };
        assert!(jvm.is_instance(&*exception, "java/io/IOException"));
        assert_eq!(&*runtime.files.lock().unwrap()[path].lock().unwrap(), b"a");
    }

    Ok(())
}

#[tokio::test]
async fn file_writer_reports_zero_and_late_write_failures_with_exact_prefix() -> Result<()> {
    for (path, zero_on_write_call, error_on_write_call) in [("zero.txt", Some(2), None), ("error.txt", None, Some(2))] {
        let mut runtime = MemoryRuntime::new([(path.into(), Vec::new())].into_iter().collect());
        runtime.file_config.max_write_size = Some(1);
        runtime.file_config.zero_on_write_call = zero_on_write_call;
        runtime.file_config.error_on_write_call = error_on_write_call;
        let jvm = create_test_jvm(runtime.clone()).await?;
        let path_string = JavaLangString::from_rust_string(&jvm, path).await?;
        let writer = jvm.new_class("java/io/FileWriter", "(Ljava/lang/String;)V", (path_string,)).await?;
        let value = JavaLangString::from_rust_string(&jvm, "abc").await?;
        let result: Result<()> = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("incomplete runtime write must throw IOException");
        };
        assert!(jvm.is_instance(&*exception, "java/io/IOException"));
        assert_eq!(&*runtime.files.lock().unwrap()[path].lock().unwrap(), b"a");
    }

    Ok(())
}
