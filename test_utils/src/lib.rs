extern crate alloc;

use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
    format,
    string::String,
    sync::Arc,
    vec::Vec,
};
use core::{
    cmp::min,
    sync::atomic::{AtomicI64, AtomicU32, AtomicU64, Ordering},
    time::Duration,
};
use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::{Notify, oneshot};

use jvm::{ClassDefinition, Jvm, Result};
use jvm_rust::{ArrayClassDefinitionImpl, ClassDefinitionError, ClassDefinitionImpl};

use java_runtime::{
    File, FileDescriptorId, FileOpenOptions, FileSize, FileStat, FileType, IOError, IOResult, RT_RUSTJAR, Runtime, SpawnCallback,
    get_bootstrap_class_loader, get_runtime_class_proto,
};

struct SpawnCallbacks {
    callbacks: Mutex<VecDeque<Box<dyn SpawnCallback>>>,
    available: Notify,
}

struct ManualClock {
    now: AtomicU64,
    sleepers: Mutex<Vec<(u64, oneshot::Sender<()>)>>,
    sleep_registered: Notify,
}

pub struct TestRuntime {
    filesystem: BTreeMap<String, Vec<u8>>,
    stdin: Option<Vec<u8>>,
    file_table: Arc<Mutex<BTreeMap<u32, Box<dyn File>>>>,
    next_fd: Arc<AtomicU32>,
    exit_status: Arc<AtomicI64>,
    spawn_callbacks: Option<Arc<SpawnCallbacks>>,
    manual_clock: Option<Arc<ManualClock>>,
}

impl Clone for TestRuntime {
    fn clone(&self) -> Self {
        Self {
            filesystem: self.filesystem.clone(),
            stdin: self.stdin.clone(),
            file_table: self.file_table.clone(),
            next_fd: self.next_fd.clone(),
            exit_status: self.exit_status.clone(),
            spawn_callbacks: self.spawn_callbacks.clone(),
            manual_clock: self.manual_clock.clone(),
        }
    }
}

impl TestRuntime {
    pub fn new(filesystem: BTreeMap<String, Vec<u8>>) -> Self {
        Self {
            filesystem,
            stdin: None,
            file_table: Arc::new(Mutex::new(BTreeMap::new())),
            next_fd: Arc::new(AtomicU32::new(1)),
            exit_status: Arc::new(AtomicI64::new(i64::MIN)),
            spawn_callbacks: None,
            manual_clock: None,
        }
    }

    pub fn new_with_stdin(filesystem: BTreeMap<String, Vec<u8>>, stdin: Vec<u8>) -> Self {
        Self {
            filesystem,
            stdin: Some(stdin),
            file_table: Arc::new(Mutex::new(BTreeMap::new())),
            next_fd: Arc::new(AtomicU32::new(1)),
            exit_status: Arc::new(AtomicI64::new(i64::MIN)),
            spawn_callbacks: None,
            manual_clock: None,
        }
    }

    pub fn new_with_queued_spawns(filesystem: BTreeMap<String, Vec<u8>>) -> Self {
        Self {
            filesystem,
            stdin: None,
            file_table: Arc::new(Mutex::new(BTreeMap::new())),
            next_fd: Arc::new(AtomicU32::new(1)),
            exit_status: Arc::new(AtomicI64::new(i64::MIN)),
            spawn_callbacks: Some(Arc::new(SpawnCallbacks {
                callbacks: Mutex::new(VecDeque::new()),
                available: Notify::new(),
            })),
            manual_clock: None,
        }
    }

    pub fn new_with_queued_spawns_and_manual_clock(filesystem: BTreeMap<String, Vec<u8>>, now: u64) -> Self {
        Self {
            filesystem,
            stdin: None,
            file_table: Arc::new(Mutex::new(BTreeMap::new())),
            next_fd: Arc::new(AtomicU32::new(1)),
            exit_status: Arc::new(AtomicI64::new(i64::MIN)),
            spawn_callbacks: Some(Arc::new(SpawnCallbacks {
                callbacks: Mutex::new(VecDeque::new()),
                available: Notify::new(),
            })),
            manual_clock: Some(Arc::new(ManualClock {
                now: AtomicU64::new(now),
                sleepers: Mutex::new(Vec::new()),
                sleep_registered: Notify::new(),
            })),
        }
    }

    pub fn take_spawn_callback(&self) -> Option<Box<dyn SpawnCallback>> {
        self.spawn_callbacks.as_ref()?.callbacks.lock().unwrap().pop_front()
    }

    pub async fn next_spawn_callback(&self) -> Option<Box<dyn SpawnCallback>> {
        let spawn_callbacks = self.spawn_callbacks.as_ref()?;
        loop {
            let available = spawn_callbacks.available.notified();
            if let Some(callback) = spawn_callbacks.callbacks.lock().unwrap().pop_front() {
                return Some(callback);
            }
            available.await;
        }
    }

    pub fn advance_time(&self, duration: Duration) {
        let clock = self.manual_clock.as_ref().expect("manual clock is not enabled");
        let now = clock.now.fetch_add(duration.as_millis() as u64, Ordering::SeqCst) + duration.as_millis() as u64;
        let mut sleepers = clock.sleepers.lock().unwrap();
        let mut index = 0;
        while index < sleepers.len() {
            if sleepers[index].0 <= now {
                let (_, sender) = sleepers.swap_remove(index);
                let _ = sender.send(());
            } else {
                index += 1;
            }
        }
    }

    pub async fn next_sleep_deadline(&self) -> u64 {
        let clock = self.manual_clock.as_ref().expect("manual clock is not enabled");
        loop {
            let registered = clock.sleep_registered.notified();
            if let Some(deadline) = clock.sleepers.lock().unwrap().iter().map(|(deadline, _)| *deadline).min() {
                return deadline;
            }
            registered.await;
        }
    }

    pub async fn wait_for_sleep_deadline(&self, expected: u64) {
        let clock = self.manual_clock.as_ref().expect("manual clock is not enabled");
        loop {
            let registered = clock.sleep_registered.notified();
            if clock.sleepers.lock().unwrap().iter().any(|(deadline, _)| *deadline == expected) {
                return;
            }
            registered.await;
        }
    }

    pub fn exit_status(&self) -> Option<i32> {
        let status = self.exit_status.load(Ordering::SeqCst);
        (status != i64::MIN).then_some(status as i32)
    }

    fn register_file(&self, file: Box<dyn File>) -> FileDescriptorId {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
        self.file_table.lock().unwrap().insert(fd, file);
        FileDescriptorId::new(fd)
    }
}

tokio::task_local! {
    static TASK_ID: u64;
}

static LAST_TASK_ID: AtomicU64 = AtomicU64::new(1);

#[async_trait::async_trait]
impl Runtime for TestRuntime {
    async fn sleep(&self, duration: Duration) {
        if let Some(clock) = &self.manual_clock {
            if duration.is_zero() {
                return;
            }
            let deadline = clock.now.load(Ordering::SeqCst).saturating_add(duration.as_millis() as u64);
            let (sender, receiver) = oneshot::channel();
            {
                let mut sleepers = clock.sleepers.lock().unwrap();
                if clock.now.load(Ordering::SeqCst) >= deadline {
                    return;
                }
                sleepers.push((deadline, sender));
            }
            clock.sleep_registered.notify_one();
            let _ = receiver.await;
            return;
        }
        tokio::time::sleep(duration).await;
    }

    async fn r#yield(&self) {
        tokio::task::yield_now().await;
    }

    fn spawn(&self, _jvm: &Jvm, callback: Box<dyn SpawnCallback>) {
        struct TestSpawnCallback {
            task_id: u64,
            callback: Box<dyn SpawnCallback>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for TestSpawnCallback {
            async fn call(&self) -> Result<()> {
                TASK_ID.scope(self.task_id, self.callback.call()).await
            }
        }

        let task_id = LAST_TASK_ID.fetch_add(1, Ordering::SeqCst);
        let callback: Box<dyn SpawnCallback> = Box::new(TestSpawnCallback { task_id, callback });
        if let Some(spawn_callbacks) = &self.spawn_callbacks {
            spawn_callbacks.callbacks.lock().unwrap().push_back(callback);
            spawn_callbacks.available.notify_one();
            return;
        }

        tokio::spawn(async move {
            if let Err(error) = callback.call().await {
                tracing::error!(?error, "spawned Java test task failed");
            }
        });
    }

    fn exit(&self, status: i32) {
        self.exit_status.store(status as i64, Ordering::SeqCst);
    }

    fn now(&self) -> u64 {
        if let Some(clock) = &self.manual_clock {
            return clock.now.load(Ordering::SeqCst);
        }
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_millis() as u64
    }

    fn current_task_id(&self) -> u64 {
        TASK_ID.try_with(|x| *x).unwrap_or(0)
    }

    fn stdin(&self) -> IOResult<FileDescriptorId> {
        match &self.stdin {
            Some(data) => Ok(self.register_file(Box::new(DummyFile::new(data.clone())))),
            None => Err(IOError::NotFound),
        }
    }

    fn stdout(&self) -> IOResult<FileDescriptorId> {
        Err(IOError::NotFound)
    }

    fn stderr(&self) -> IOResult<FileDescriptorId> {
        Err(IOError::NotFound)
    }

    async fn open(&self, path: &str, options: FileOpenOptions) -> IOResult<FileDescriptorId> {
        if !options.read || options.write || options.append || options.truncate || options.create {
            return Err(IOError::Unsupported);
        }

        let entry = self.filesystem.get(path);
        if let Some(data) = entry {
            let file = Box::new(DummyFile::new(data.clone())) as Box<dyn File>;
            Ok(self.register_file(file))
        } else {
            Err(IOError::NotFound)
        }
    }

    fn get_file(&self, fd: FileDescriptorId) -> IOResult<Box<dyn File>> {
        self.file_table.lock().unwrap().get(&fd.id()).cloned().ok_or(IOError::NotFound)
    }

    fn close_file(&self, fd: FileDescriptorId) {
        self.file_table.lock().unwrap().remove(&fd.id());
    }

    async fn unlink(&self, _path: &str) -> IOResult<()> {
        Err(IOError::NotFound)
    }

    async fn metadata(&self, path: &str) -> IOResult<FileStat> {
        let entry = self.filesystem.get(path);
        if let Some(data) = entry {
            Ok(FileStat {
                size: data.len() as FileSize,
                r#type: FileType::File,
            })
        } else {
            Err(IOError::NotFound)
        }
    }

    async fn find_rustjar_class(&self, _jvm: &Jvm, classpath: &str, class: &str) -> jvm::Result<Option<Box<dyn ClassDefinition>>> {
        if classpath == RT_RUSTJAR {
            let proto = get_runtime_class_proto(class);
            if let Some(proto) = proto {
                return Ok(Some(Box::new(ClassDefinitionImpl::from_class_proto(
                    proto,
                    Box::new(self.clone()) as Box<_>,
                ))));
            }
        }

        Ok(None)
    }

    async fn define_class(&self, jvm: &Jvm, data: &[u8]) -> jvm::Result<Box<dyn ClassDefinition>> {
        match ClassDefinitionImpl::from_classfile(data) {
            Ok(class) => Ok(Box::new(class)),
            Err(ClassDefinitionError::InvalidClassFile) => Err(jvm.exception("java/lang/ClassFormatError", "Invalid class file").await),
            Err(ClassDefinitionError::UnsupportedClassVersion(version)) => Err(jvm
                .exception(
                    "java/lang/UnsupportedClassVersionError",
                    &format!("Unsupported class file version {version}"),
                )
                .await),
            Err(ClassDefinitionError::Verification) => Err(jvm.exception("java/lang/VerifyError", "Bytecode verification failed").await),
            Err(ClassDefinitionError::UnsupportedFeature(feature)) => Err(jvm
                .exception(
                    "java/lang/UnsupportedOperationException",
                    &format!("Unsupported class file feature: {feature}"),
                )
                .await),
        }
    }

    async fn define_array_class(&self, _jvm: &Jvm, element_type_name: &str) -> jvm::Result<Box<dyn ClassDefinition>> {
        Ok(Box::new(ArrayClassDefinitionImpl::new(element_type_name)))
    }
}

#[derive(Clone)]
struct DummyFile {
    data: Vec<u8>,
    // shared between clones, as Runtime::get_file hands out cloned handles to the same file
    pos: Arc<Mutex<FileSize>>,
}

impl DummyFile {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            pos: Arc::new(Mutex::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl File for DummyFile {
    async fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        let mut pos = self.pos.lock().unwrap();

        let remaining = &self.data[(*pos).min(self.data.len() as FileSize) as usize..];
        let len = min(buf.len(), remaining.len());
        buf[..len].copy_from_slice(&remaining[..len]);
        *pos += len as FileSize;

        Ok(len)
    }

    async fn write(&mut self, _buf: &[u8]) -> IOResult<usize> {
        Err(IOError::Unsupported)
    }

    async fn seek(&mut self, pos: FileSize) -> IOResult<()> {
        *self.pos.lock().unwrap() = pos;

        Ok(())
    }

    async fn tell(&self) -> IOResult<FileSize> {
        Ok(*self.pos.lock().unwrap())
    }

    async fn set_len(&mut self, _len: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn metadata(&self) -> IOResult<FileStat> {
        Ok(FileStat {
            size: self.data.len() as FileSize,
            r#type: FileType::File,
        })
    }
}

pub async fn create_test_jvm<R>(runtime: R) -> Result<Jvm>
where
    R: Runtime + Clone + 'static,
{
    let bootstrap_class_loader = get_bootstrap_class_loader(Box::new(runtime.clone()));

    let properties = [("java.class.path", ".")].into_iter().collect();

    Jvm::new(bootstrap_class_loader, move || runtime.current_task_id(), properties).await
}

pub async fn test_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    create_test_jvm(runtime).await
}

pub async fn test_jvm_filesystem(filesystem: BTreeMap<String, Vec<u8>>) -> Result<Jvm> {
    let runtime = TestRuntime::new(filesystem);
    create_test_jvm(runtime).await
}
