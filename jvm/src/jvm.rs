#![allow(clippy::borrowed_box)] // We have get parameter by Box<T> to make ergonomic interface
#![allow(clippy::double_must_use)] // temporary until https://github.com/rust-lang/rust-clippy/issues/17529 fix lands

use alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, format, string::String, sync::Arc, vec::Vec};
use core::{
    fmt::Debug,
    future::{Future, poll_fn},
    iter,
    pin::pin,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    task::Poll,
};

use dyn_clone::clone_box;
use event_listener::Event;
use hashbrown::HashSet;
use parking_lot::RwLock;

use jvm_types::{ClassAccessFlags, FieldAccessFlags, JavaType, MethodAccessFlags};

use crate::{
    ExternalRoots, Result,
    array_class_instance::{ArrayRawBuffer, ArrayRawBufferMut},
    class_definition::ClassDefinition,
    class_instance::{AsClassInstance, ClassInstance, ClassInstanceRef},
    class_loader::{
        BootstrapClassLoader, BootstrapClassLoaderWrapper, Class, ClassLoaderWrapper, InitState, InitializationAction, JavaClassLoaderWrapper,
    },
    error::JavaError,
    field::Field,
    garbage_collector::determine_garbage,
    global_ref::{GlobalRef, GlobalReferences},
    invoke_arg::InvokeArg,
    method::Method,
    monitor::{Monitor, MonitorWait, MonitorWaitTimeout},
    resolution_cache::{Lookup, MemberKey, ResolutionCache, ResolvedField, ResolvedMethod},
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
    thread::{JvmThread, ThreadInterruptWaiter},
    value::JavaValue,
};

// Fixed frame budget per thread instead of probing the native stack like real JVMs do;
// 512 keeps release builds within tokio's default 2MB worker thread stack.
const MAX_STACK_DEPTH: usize = 512;

struct JvmInner {
    classes: RwLock<BTreeMap<String, Class>>,
    resolution_cache: ResolutionCache,
    threads: RwLock<BTreeMap<u64, JvmThread>>,
    global_references: Arc<GlobalReferences>,
    all_objects: RwLock<HashSet<Box<dyn ClassInstance>>>,
    string_pool: RwLock<BTreeMap<Vec<u16>, Box<dyn ClassInstance>>>,
    monitors: RwLock<BTreeMap<usize, Arc<Monitor>>>,
    get_current_thread_id: Box<dyn Fn() -> u64 + Sync + Send>,
    bootstrap_class_loader: Box<dyn BootstrapClassLoader>,
    external_roots: Option<Box<dyn ExternalRoots>>,
    bootstrapping: AtomicBool,
}

#[derive(Clone)]
pub struct Jvm {
    inner: Arc<JvmInner>,
}

/// Keeps local references alive for the lifetime of a native frame.
/// Scopes must be dropped in stack order, before detaching their originating thread.
#[must_use]
pub struct NativeFrame {
    jvm: Jvm,
    thread_id: u64,
}

impl Drop for NativeFrame {
    fn drop(&mut self) {
        self.jvm.inner.threads.write().get_mut(&self.thread_id).unwrap().pop_frame();
    }
}

impl Jvm {
    pub async fn new<C, F>(
        bootstrap_class_loader: C,
        get_current_thread_id: F,
        properties: BTreeMap<&str, &str>,
        external_roots: Option<Box<dyn ExternalRoots>>,
    ) -> Result<Self>
    where
        C: BootstrapClassLoader + 'static,
        F: Fn() -> u64 + 'static + Sync + Send,
    {
        let jvm = Self {
            inner: Arc::new(JvmInner {
                classes: RwLock::new(BTreeMap::new()),
                resolution_cache: ResolutionCache::default(),
                threads: RwLock::new(BTreeMap::new()),
                global_references: Arc::new(GlobalReferences {
                    next_id: AtomicU64::new(0),
                    objects: RwLock::new(BTreeMap::new()),
                }),
                all_objects: RwLock::new(HashSet::new()),
                string_pool: RwLock::new(BTreeMap::new()),
                monitors: RwLock::new(BTreeMap::new()),
                get_current_thread_id: Box::new(get_current_thread_id),
                bootstrap_class_loader: Box::new(bootstrap_class_loader),
                external_roots,
                bootstrapping: AtomicBool::new(true),
            }),
        };

        // load bootstrap classes
        let bootstrap_classes = [
            "java/lang/Object",
            "java/lang/Runnable",
            "java/lang/Thread",
            "[B",
            "java/io/Serializable",
            "java/lang/Class",
        ];
        for class_name in bootstrap_classes.iter() {
            let class_definition = jvm.inner.bootstrap_class_loader.load_class(&jvm, class_name).await?.unwrap();
            let class = Class::new(class_definition, None, None);

            jvm.register_class_internal(class, None).await?;
        }

        // init startup thread
        jvm.attach_thread(None).await?;

        // set java class for bootstrap classes
        let classes = jvm.inner.classes.read().values().cloned().collect::<Vec<_>>();
        for class in classes {
            let java_class = JavaLangClass::from_rust_class(&jvm, class.definition.clone(), None).await?;
            class.set_java_class(java_class);
        }

        // init properties
        for (key, value) in properties {
            let key = JavaLangString::from_rust_string(&jvm, key).await?;
            let value = JavaLangString::from_rust_string(&jvm, value).await?;

            let _: Option<Box<dyn ClassInstance>> = jvm
                .invoke_static(
                    "java/lang/System",
                    "setProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                    (key, value),
                )
                .await?;
        }

        // load system class loader
        JavaLangClassLoader::get_system_class_loader(&jvm).await?;

        jvm.inner.bootstrapping.store(false, Ordering::Relaxed);

        let thread_id = (jvm.inner.get_current_thread_id)();
        jvm.inner
            .threads
            .write()
            .get_mut(&thread_id)
            .unwrap()
            .top_frame_mut()
            .local_variables_mut()
            .clear();

        Ok(jvm)
    }

    #[async_recursion::async_recursion]
    pub async fn instantiate_class(&self, class_name: &str) -> Result<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate {class_name}");

        let class = self.resolve_class(class_name).await?;

        let access_flags = class.definition.access_flags();
        if access_flags.contains(ClassAccessFlags::INTERFACE) || access_flags.contains(ClassAccessFlags::ABSTRACT) {
            return Err(self
                .exception(
                    "java/lang/InstantiationError",
                    &format!("Cannot instantiate abstract class or interface: {}", class_name),
                )
                .await);
        }

        self.ensure_initialized(&class).await?;

        let instance = class.definition.instantiate(self).await?;

        let thread_id = (self.inner.get_current_thread_id)();
        let mut threads = self.inner.threads.write();
        let thread = threads.get_mut(&thread_id).unwrap();

        thread.top_frame_mut().local_variables_mut().push(instance.clone());
        self.inner.all_objects.write().insert(instance.clone());

        Ok(instance)
    }

    pub async fn new_class<T>(&self, class_name: &str, init_descriptor: &str, init_args: T) -> Result<Box<dyn ClassInstance>>
    where
        T: InvokeArg,
    {
        let instance = self.instantiate_class(class_name).await?;

        let _: () = self.invoke_special(&instance, class_name, "<init>", init_descriptor, init_args).await?;

        Ok(instance)
    }

    pub async fn instantiate_array(&self, element_type_name: &str, length: usize) -> Result<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate array of {element_type_name} with length {length}");

        let class_name = format!("[{element_type_name}");

        let class = self.resolve_class(&class_name).await?.definition;
        let array_class = class.as_array_class_definition().unwrap();

        let instance = array_class.instantiate_array(self, length).await?;

        let thread_id = (self.inner.get_current_thread_id)();
        let mut threads = self.inner.threads.write();
        let thread = threads.get_mut(&thread_id).unwrap();

        thread.top_frame_mut().local_variables_mut().push(instance.clone());
        self.inner.all_objects.write().insert(instance.clone());

        Ok(instance)
    }

    pub async fn get_static_field<T>(&self, class_name: &str, name: &str, descriptor: &str) -> Result<T>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Get static field {class_name}.{name}:{descriptor}");

        if let Some(resolved) = self.resolve_field_ref(class_name, name, descriptor, true).await? {
            let (declaring_class, field) = &*resolved;
            if !field.access_flags().contains(FieldAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            self.ensure_initialized(declaring_class).await?;

            let value = declaring_class.definition.get_static_field(&**field)?;
            if let JavaValue::Object(Some(instance)) = &value {
                let thread_id = (self.inner.get_current_thread_id)();
                self.inner
                    .threads
                    .write()
                    .get_mut(&thread_id)
                    .unwrap()
                    .top_frame_mut()
                    .local_variables_mut()
                    .push(instance.clone());
            }
            Ok(value.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub async fn put_static_field<T>(&self, class_name: &str, name: &str, descriptor: &str, value: T) -> Result<()>
    where
        T: Into<JavaValue> + Debug,
    {
        tracing::trace!("Put static field {class_name}.{name}:{descriptor} = {value:?}");

        if let Some(resolved) = self.resolve_field_ref(class_name, name, descriptor, true).await? {
            let (declaring_class, field) = &*resolved;
            if !field.access_flags().contains(FieldAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            self.ensure_initialized(declaring_class).await?;

            declaring_class.definition.clone().put_static_field(&**field, value.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub async fn get_field<T>(&self, instance: &Box<dyn ClassInstance>, class_name: &str, name: &str, descriptor: &str) -> Result<T>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Get field {class_name}.{name}:{descriptor}");

        if let Some(resolved) = self.resolve_field_ref(class_name, name, descriptor, false).await? {
            let (_, field) = &*resolved;
            if field.access_flags().contains(FieldAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }
            let value = instance.get_field(&**field)?;
            if let JavaValue::Object(Some(instance)) = &value {
                let thread_id = (self.inner.get_current_thread_id)();
                self.inner
                    .threads
                    .write()
                    .get_mut(&thread_id)
                    .unwrap()
                    .top_frame_mut()
                    .local_variables_mut()
                    .push(instance.clone());
            }
            Ok(value.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub async fn put_field<T>(&self, instance: &mut Box<dyn ClassInstance>, class_name: &str, name: &str, descriptor: &str, value: T) -> Result<()>
    where
        T: Into<JavaValue> + Debug,
    {
        tracing::trace!("Put field {class_name}.{name}:{descriptor} = {value:?}");

        if let Some(resolved) = self.resolve_field_ref(class_name, name, descriptor, false).await? {
            let (_, field) = &*resolved;
            if field.access_flags().contains(FieldAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }
            instance.put_field(&**field, value.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub async fn invoke_static<T, U>(&self, class_name: &str, name: &str, descriptor: &str, args: T) -> Result<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        let args = args.into_arg();

        tracing::trace!("Invoke static {class_name}.{name}:{descriptor}({args:?})");

        if let Some(resolved) = self.resolve_method_ref(class_name, name, descriptor, Lookup::Static).await? {
            if !resolved.method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            self.ensure_initialized(&resolved.class).await?;

            Ok(self.execute_method(&resolved, args).await?.into())
        } else {
            tracing::error!("No such method: {class_name}.{name}:{descriptor}");

            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub async fn invoke_virtual<T, U>(&self, instance: &Box<dyn ClassInstance>, class_name: &str, name: &str, descriptor: &str, args: T) -> Result<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        let args = args.into_arg();
        tracing::trace!("Invoke virtual {class_name}.{name}:{descriptor}({args:?})");
        let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
            .chain(args.into_vec())
            .collect::<Box<[_]>>();

        // the method selected for a receiver class (JVMS 5.4.6) is cached per receiver class
        let receiver_definition = instance.class_definition();
        let key = MemberKey {
            lookup: Lookup::Instance,
            receiver: receiver_definition.name(),
            class: class_name.into(),
            name: name.into(),
            descriptor: descriptor.into(),
        };
        if let Some(selected) = self.inner.resolution_cache.method(&key) {
            return Ok(self.execute_method(&selected, args).await?.into());
        }

        let Some(resolved) = self.resolve_method_ref(class_name, name, descriptor, Lookup::Instance).await? else {
            tracing::error!("No such method: {class_name}.{name}:{descriptor}");
            return Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{class_name}.{name}:{descriptor}"))
                .await);
        };

        if resolved.method.access_flags().contains(MethodAccessFlags::STATIC) {
            return Err(self
                .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                .await);
        }

        let mut class = self.get_class(&key.receiver);
        let mut selected = None;
        while let Some(candidate_class) = class {
            if let Some(candidate_method) = candidate_class.definition.method(name, descriptor, false)
                && self.method_overrides(&candidate_class, &*candidate_method, &resolved.class, &*resolved.method)
            {
                selected = Some(ResolvedMethod {
                    class: candidate_class,
                    method: candidate_method,
                });
                break;
            }

            class = candidate_class
                .definition
                .super_class_name()
                .and_then(|super_class| self.get_class(&super_class));
        }

        let Some(selected) = selected else {
            return Err(self
                .exception("java/lang/AbstractMethodError", &format!("{class_name}.{name}:{descriptor}"))
                .await);
        };
        if selected.method.access_flags().contains(MethodAccessFlags::ABSTRACT) {
            return Err(self
                .exception("java/lang/AbstractMethodError", &format!("{class_name}.{name}:{descriptor}"))
                .await);
        }

        let selected = Arc::new(selected);
        self.inner.resolution_cache.insert_method(key, selected.clone());

        Ok(self.execute_method(&selected, args).await?.into())
    }

    // non-virtual
    #[async_recursion::async_recursion]
    pub async fn invoke_special<T, U>(&self, instance: &Box<dyn ClassInstance>, class_name: &str, name: &str, descriptor: &str, args: T) -> Result<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        let args = args.into_arg();
        tracing::trace!("Invoke special {class_name}.{name}:{descriptor}({args:?})");
        let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
            .chain(args.into_vec())
            .collect::<Box<[_]>>();

        let lookup = if name == "<init>" { Lookup::Constructor } else { Lookup::Instance };
        if let Some(resolved) = self.resolve_method_ref(class_name, name, descriptor, lookup).await? {
            if resolved.method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            if resolved.method.access_flags().contains(MethodAccessFlags::ABSTRACT) {
                return Err(self
                    .exception("java/lang/AbstractMethodError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            Ok(self.execute_method(&resolved, args).await?.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    #[async_recursion::async_recursion]
    pub async fn store_array<T, U>(&self, array: &mut Box<dyn ClassInstance>, offset: usize, values: T) -> Result<()>
    where
        T: IntoIterator<Item = U> + Send,
        U: Into<JavaValue>,
    {
        tracing::trace!("Store array {} at offset {offset}", array.class_definition().name());

        let values = values.into_iter().map(|x| x.into()).collect::<Vec<_>>();

        let array_size = self.array_length(array).await?;
        // saturating so an out of range offset reaches the caller as a java exception rather than an overflow panic
        let end = offset.saturating_add(values.len());
        if end > array_size {
            return Err(self
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{end} > {array_size}"))
                .await);
        }

        let array = array.as_array_instance_mut();

        if let Some(array) = array {
            array.store(offset, values.into_boxed_slice())?;

            Ok(())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn load_array<T>(&self, array: &Box<dyn ClassInstance>, offset: usize, count: usize) -> Result<Vec<T>>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Load array {} at offset {offset}", array.class_definition().name());

        let array_size = self.array_length(array).await?;
        // saturating so an out of range offset reaches the caller as a java exception rather than an overflow panic
        let end = offset.saturating_add(count);
        if end > array_size {
            return Err(self
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{end} > {array_size}"))
                .await);
        }

        let array = array.as_array_instance();

        if let Some(array) = array {
            let values = array.load(offset, count)?;

            // primitive arrays have nothing to root, so skip the thread table lock for them
            if values.iter().any(|value| matches!(value, JavaValue::Object(Some(_)))) {
                let thread_id = (self.inner.get_current_thread_id)();
                let mut threads = self.inner.threads.write();
                let local_variables = threads.get_mut(&thread_id).unwrap().top_frame_mut().local_variables_mut();
                values.iter().for_each(|value| {
                    if let JavaValue::Object(Some(instance)) = value {
                        local_variables.push(instance.clone());
                    }
                });
            }

            Ok(iter::IntoIterator::into_iter(values).map(|x| x.into()).collect::<Vec<_>>())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn array_raw_buffer(&self, array: &Box<dyn ClassInstance>) -> Result<Box<dyn ArrayRawBuffer>> {
        let array = array.as_array_instance();

        if let Some(array) = array {
            array.raw_buffer()
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn array_raw_buffer_mut(&self, array: &mut Box<dyn ClassInstance>) -> Result<Box<dyn ArrayRawBufferMut>> {
        let array = array.as_array_instance_mut();

        if let Some(array) = array {
            array.raw_buffer_mut()
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn array_length(&self, array: &Box<dyn ClassInstance>) -> Result<usize> {
        tracing::trace!("Get array length {}", array.class_definition().name());

        let array = array.as_array_instance();

        if let Some(array) = array {
            Ok(array.length())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn array_element_type(&self, array: &Box<dyn ClassInstance>) -> Result<JavaType> {
        tracing::trace!("Get array element type {}", array.class_definition().name());

        let array = array.as_array_instance();

        if let Some(array) = array {
            let class = array.class_definition();

            let type_name = &class.name()[1..]; // TODO can we store JavaType on class?

            Ok(JavaType::parse(type_name))
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub fn destroy(&self, instance: Box<dyn ClassInstance>) -> Result<()> {
        tracing::debug!("Destroy {}", instance.class_definition().name());

        self.inner.monitors.write().remove(&instance.identity());
        self.inner.all_objects.write().remove(&instance);
        instance.destroy();

        Ok(())
    }

    pub fn shallow_clone(&self, instance: &Box<dyn ClassInstance>) -> Result<Box<dyn ClassInstance>> {
        let cloned = instance.shallow_clone()?;
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner
            .threads
            .write()
            .get_mut(&thread_id)
            .unwrap()
            .top_frame_mut()
            .local_variables_mut()
            .push(cloned.clone());
        self.inner.all_objects.write().insert(cloned.clone());
        Ok(cloned)
    }

    // JVMS 5.1 string interning: equal string literals (and String.intern results) share one instance
    pub async fn intern_string(&self, value: &str) -> Result<Box<dyn ClassInstance>> {
        let key = value.encode_utf16().collect::<Vec<_>>();
        if let Some(interned) = self.inner.string_pool.read().get(&key) {
            return Ok(clone_box(&**interned));
        }

        let instance = JavaLangString::from_rust_string(self, value).await?;

        Ok(self.intern_or_get(key, instance))
    }

    // String.intern(): the receiver itself is pooled and returned when it is the first with its value (JVMS String spec)
    pub fn intern_string_instance(&self, instance: Box<dyn ClassInstance>, value: &[u16]) -> Box<dyn ClassInstance> {
        if let Some(interned) = self.inner.string_pool.read().get(value) {
            return clone_box(&**interned);
        }

        self.intern_or_get(value.to_owned(), instance)
    }

    // insert under the write lock, re-checking so concurrent interners converge on one canonical instance
    fn intern_or_get(&self, key: Vec<u16>, instance: Box<dyn ClassInstance>) -> Box<dyn ClassInstance> {
        let mut pool = self.inner.string_pool.write();
        let interned = pool.entry(key).or_insert(instance);

        clone_box(&**interned)
    }

    pub(crate) fn interned_strings(&self) -> Vec<Box<dyn ClassInstance>> {
        self.inner.string_pool.read().values().map(|x| clone_box(&**x)).collect()
    }

    pub fn has_class(&self, class_name: &str) -> bool {
        self.inner.classes.read().contains_key(class_name)
    }

    pub fn get_class(&self, class_name: &str) -> Option<Class> {
        self.inner.classes.read().get(class_name).cloned()
    }

    pub async fn monitor_enter(&self, obj: &(impl AsClassInstance + ?Sized)) -> Result<()> {
        let _frame = self.push_native_frame(&[JavaValue::Object(Some(clone_box(obj.as_class_instance())))]);
        let thread_id = (self.inner.get_current_thread_id)();
        self.get_or_create_monitor(obj.as_class_instance()).enter(thread_id).await;
        Ok(())
    }

    pub async fn monitor_exit(&self, obj: &(impl AsClassInstance + ?Sized)) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        match self.get_or_create_monitor(obj.as_class_instance()).exit(thread_id) {
            Ok(()) => Ok(()),
            Err(_) => Err(self
                .exception("java/lang/IllegalMonitorStateException", "current thread does not own the monitor")
                .await),
        }
    }

    pub async fn object_wait_prepare(&self, obj: &(impl AsClassInstance + ?Sized)) -> Result<(MonitorWait, MonitorWaitTimeout)> {
        let thread_id = (self.inner.get_current_thread_id)();
        match self.get_or_create_monitor(obj.as_class_instance()).prepare_wait(thread_id) {
            Ok((wait, timeout)) => {
                let interrupted = {
                    let mut threads = self.inner.threads.write();
                    let thread = threads.get_mut(&thread_id).expect("current JVM thread must be attached");
                    thread.interrupt_waiter = Some(ThreadInterruptWaiter::Monitor(timeout.clone()));
                    thread.interrupted
                };
                if interrupted {
                    timeout.clone().notify();
                }
                Ok((wait, timeout))
            }
            Err(_) => Err(self
                .exception("java/lang/IllegalMonitorStateException", "current thread does not own the monitor")
                .await),
        }
    }

    pub async fn object_wait(&self, wait: MonitorWait) -> Result<()> {
        wait.wait().await;

        let thread_id = (self.inner.get_current_thread_id)();
        {
            let mut threads = self.inner.threads.write();
            let thread = threads.get_mut(&thread_id).expect("current JVM thread must be attached");
            thread.interrupt_waiter = None;
        }
        let interrupted = self.current_java_thread_interrupted().await?;
        if interrupted {
            Err(self.exception("java/lang/InterruptedException", "thread interrupted").await)
        } else {
            Ok(())
        }
    }

    pub async fn interrupt_java_thread(&self, thread: &mut Box<dyn ClassInstance>) -> Result<()> {
        // Resolve before locking so both interrupt states can be updated atomically without awaiting while the lock is held.
        let field = self.find_field(&*thread.class_definition(), "interrupted", "Z")?.unwrap();
        let identity = thread.identity();
        let waiter = {
            let mut threads = self.inner.threads.write();
            thread.put_field(&*field, JavaValue::from(true))?;
            let target = threads
                .values_mut()
                .find(|candidate| candidate.java_thread().is_some_and(|java_thread| java_thread.identity() == identity));
            if let Some(target) = target {
                target.interrupted = true;
                target.interrupt_waiter.clone()
            } else {
                None
            }
        };
        if let Some(waiter) = waiter {
            waiter.notify();
        }
        Ok(())
    }

    pub async fn is_java_thread_interrupted(&self, thread: &Box<dyn ClassInstance>) -> Result<bool> {
        // The Thread prototype guarantees this field, so avoid the async field API's impossible NoSuchFieldError path.
        let field = self.find_field(&*thread.class_definition(), "interrupted", "Z")?.unwrap();
        let identity = thread.identity();
        let threads = self.inner.threads.read();
        let field_interrupted: bool = thread.get_field(&*field)?.into();
        let jvm_interrupted = threads
            .values()
            .find(|candidate| candidate.java_thread().is_some_and(|java_thread| java_thread.identity() == identity))
            .is_some_and(|target| target.interrupted);
        Ok(field_interrupted || jvm_interrupted)
    }

    pub async fn current_java_thread_interrupted(&self) -> Result<bool> {
        let thread_id = (self.inner.get_current_thread_id)();
        let mut java_thread = self
            .inner
            .threads
            .read()
            .get(&thread_id)
            .and_then(JvmThread::java_thread)
            .expect("attached JVM thread must have a java.lang.Thread")
            .clone();
        // Resolve before locking so both interrupt states can be read and cleared atomically without awaiting while the lock is held.
        let field = self.find_field(&*java_thread.class_definition(), "interrupted", "Z")?.unwrap();
        let mut threads = self.inner.threads.write();
        let thread = threads.get_mut(&thread_id).expect("current JVM thread must be attached");
        let field_interrupted: bool = java_thread.get_field(&*field)?.into();
        let interrupted = thread.interrupted || field_interrupted;
        thread.interrupted = false;
        java_thread.put_field(&*field, JavaValue::from(false))?;
        Ok(interrupted)
    }

    pub async fn sleep_interruptibly<F>(&self, sleep: F) -> Result<()>
    where
        F: Future<Output = ()>,
    {
        let event = Arc::new(Event::new());
        let listener = event.listen();
        let thread_id = (self.inner.get_current_thread_id)();
        let interrupted = {
            let mut threads = self.inner.threads.write();
            let thread = threads.get_mut(&thread_id).expect("current JVM thread must be attached");
            thread.interrupt_waiter = Some(ThreadInterruptWaiter::Event(event.clone()));
            thread.interrupted
        };
        if interrupted {
            event.notify(1);
        }

        let mut sleep = pin!(sleep);
        let mut listener = pin!(listener);
        poll_fn(|context| {
            if listener.as_mut().poll(context).is_ready() || sleep.as_mut().poll(context).is_ready() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        {
            let mut threads = self.inner.threads.write();
            let thread = threads.get_mut(&thread_id).expect("current JVM thread must be attached");
            thread.interrupt_waiter = None;
        }
        let interrupted = self.current_java_thread_interrupted().await?;
        if interrupted {
            Err(self.exception("java/lang/InterruptedException", "thread interrupted").await)
        } else {
            Ok(())
        }
    }

    pub async fn object_notify(&self, obj: &(impl AsClassInstance + ?Sized), count: usize) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        match self.get_or_create_monitor(obj.as_class_instance()).notify(thread_id, count) {
            Ok(()) => Ok(()),
            Err(_) => Err(self
                .exception("java/lang/IllegalMonitorStateException", "current thread does not own the monitor")
                .await),
        }
    }

    pub async fn resolve_class(&self, class_name: &str) -> Result<Class> {
        // a registry hit is synchronous; only a miss pays for the boxed recursive loader future
        if let Some(class) = self.get_class(class_name) {
            tracing::trace!("Resolving class {class_name}");
            return Ok(class);
        }

        self.resolve_class_internal(class_name, None).await
    }

    pub async fn load_bootstrap_class(&self, class_name: &str) -> Result<Option<Box<dyn ClassInstance>>> {
        let class = BootstrapClassLoaderWrapper::new(&*self.inner.bootstrap_class_loader)
            .load_class(self, class_name)
            .await?;
        Ok(class.map(|class| class.java_class()))
    }

    #[async_recursion::async_recursion]
    async fn resolve_class_internal(&self, class_name: &str, class_loader_wrapper: Option<&dyn ClassLoaderWrapper>) -> Result<Class> {
        tracing::trace!("Resolving class {class_name}");
        let class = self.inner.classes.read().get(class_name).cloned();

        if let Some(x) = class {
            return Ok(x);
        }

        if class_name.starts_with('[') {
            let stripped_name = class_name.trim_start_matches('[');
            if stripped_name.starts_with('L') {
                self.resolve_class_internal(&stripped_name[1..stripped_name.len() - 1], class_loader_wrapper)
                    .await?;
                // ensure element type is loaded
            }
        }

        let class_loader_wrapper: &dyn ClassLoaderWrapper = if let Some(x) = class_loader_wrapper {
            x
        } else if self.inner.bootstrapping.load(Ordering::Relaxed) {
            &BootstrapClassLoaderWrapper::new(&*self.inner.bootstrap_class_loader)
        } else {
            &JavaClassLoaderWrapper::new(self.current_class_loader().await?)
        };

        let class = self.load_class(class_name, class_loader_wrapper).await?;

        // loader wrappers may build a fresh Class around an already-registered definition,
        // so return the registry copy to keep init state shared
        if let Some(registered) = self.get_class(class_name) {
            return Ok(registered);
        }

        Ok(class)
    }

    async fn load_class(&self, class_name: &str, class_loader_wrapper: &dyn ClassLoaderWrapper) -> Result<Class> {
        tracing::debug!("Loading class {class_name}");

        let class = class_loader_wrapper.load_class(self, class_name).await?;

        if class.is_none() {
            return Err(self.exception("java/lang/NoClassDefFoundError", class_name).await);
        }

        tracing::debug!("Loaded class {class_name}");

        Ok(class.unwrap())
    }

    #[allow(clippy::type_complexity)]
    fn find_calling_class(&self) -> Result<Option<(Class, Option<Box<dyn ClassInstance>>)>> {
        let thread_id = (self.inner.get_current_thread_id)();

        let threads = self.inner.threads.read();
        let thread = threads.get(&thread_id).unwrap();

        Ok(thread.top_java_frame().map(|x| {
            // an instance method roots its receiver first among the frame's references
            let this = (!x.method.method.access_flags().contains(MethodAccessFlags::STATIC)).then(|| x.local_variables[0].clone());
            (x.method.class.clone(), this)
        }))
    }

    pub async fn register_class(
        &self,
        class: Box<dyn ClassDefinition>,
        class_loader: Option<Box<dyn ClassInstance>>,
    ) -> Result<Option<Box<dyn ClassInstance>>> {
        tracing::debug!("Register class {}", class.name());

        let java_class = Some(JavaLangClass::from_rust_class(self, class.clone(), class_loader.clone()).await?);

        let class = Class::new(class, java_class.clone(), class_loader.clone());

        if let Some(x) = class_loader {
            self.register_class_internal(class, Some(&JavaClassLoaderWrapper::new(x))).await?;
        } else {
            self.register_class_internal(class, None).await?;
        };

        Ok(java_class)
    }

    pub fn is_instance(&self, instance: &dyn ClassInstance, class_name: &str) -> bool {
        // every reference type is a subtype of Object
        if class_name == "java/lang/Object" {
            return true;
        }

        let class = instance.class_definition();
        let name = class.name();
        if name.starts_with('[') || class_name.starts_with('[') {
            return self.is_type_assignable(&JavaType::from_class_name(&name), &JavaType::from_class_name(class_name));
        }

        self.is_inherited_from(&*class, class_name)
    }

    // aastore: whether value may be stored into array (JVMS 6.5 aastore)
    pub fn array_store_allowed(&self, array: &dyn ClassInstance, value: &dyn ClassInstance) -> bool {
        let array_class = array.class_definition();
        let array_class_name = array_class.name();
        let Some(element_type_name) = array_class_name.strip_prefix('[') else {
            return true;
        };

        // class elements are named Lname; while array elements are already class names
        let element_class_name = element_type_name
            .strip_prefix('L')
            .and_then(|x| x.strip_suffix(';'))
            .unwrap_or(element_type_name);

        self.is_instance(value, element_class_name)
    }

    // JVMS 4.10.3 subtyping, including array covariance
    pub fn is_type_assignable(&self, source: &JavaType, target: &JavaType) -> bool {
        if source == target {
            return true;
        }

        match (source, target) {
            (JavaType::Array(source_component), JavaType::Array(target_component)) => self.is_type_assignable(source_component, target_component),
            // every array type is a subtype of Object, Cloneable, and java.io.Serializable (JLS 4.10.3)
            (JavaType::Array(_), JavaType::Class(name)) => {
                name == "java/lang/Object" || name == "java/lang/Cloneable" || name == "java/io/Serializable"
            }
            (JavaType::Class(source), JavaType::Class(target)) => self.is_class_assignable(source, target),
            _ => false,
        }
    }

    fn is_class_assignable(&self, source: &str, target: &str) -> bool {
        match self.get_class(source) {
            Some(class) => self.is_inherited_from(&*class.definition, target),
            None => false,
        }
    }

    pub fn is_inherited_from(&self, class: &dyn ClassDefinition, class_name: &str) -> bool {
        if class.name() == class_name {
            return true;
        }

        for interface in class.interface_names() {
            if interface == class_name {
                return true;
            }

            let interface_class = self.inner.classes.read().get(&interface).unwrap().definition.clone();
            if self.is_inherited_from(&*interface_class, class_name) {
                return true;
            }
        }

        if let Some(super_class) = class.super_class_name() {
            let super_class = self.inner.classes.read().get(&super_class).unwrap().definition.clone();
            self.is_inherited_from(&*super_class, class_name)
        } else {
            false
        }
    }

    pub async fn exception(&self, r#type: &str, message: &str) -> JavaError {
        tracing::info!("throwing java exception: {} {message}", r#type);

        // Construction can itself fail (e.g. StackOverflowError near the depth limit); that error replaces the requested one.
        let message_str = match JavaLangString::from_rust_string(self, message).await {
            Ok(message_str) => message_str,
            Err(error) => return error,
        };
        match self.new_class(r#type, "(Ljava/lang/String;)V", (message_str,)).await {
            Ok(instance) => JavaError::JavaException(instance),
            Err(error) => error,
        }
    }

    pub fn stack_trace(&self) -> Vec<String> {
        // TODO we should return in another format

        let thread_id = (self.inner.get_current_thread_id)();
        let threads = self.inner.threads.read();
        let thread = threads.get(&thread_id).unwrap();

        thread
            .iter_java_frame()
            .rev()
            .filter_map(|x| {
                // skip exception classes
                let class = &x.method.class;
                if self.is_inherited_from(&*class.definition, "java/lang/Throwable") {
                    None
                } else {
                    Some(format!(
                        "{}.{}{}",
                        class.definition.name(),
                        x.method.method.name(),
                        x.method.method.descriptor()
                    ))
                }
            })
            .collect()
    }

    pub fn collect_garbage(&self) -> anyhow::Result<usize> {
        tracing::trace!("Collecting garbage");

        let garbage = {
            let threads = self.inner.threads.read();
            let global_references = self.inner.global_references.objects.read();
            let all_objects = self.inner.all_objects.read();
            let classes = self.inner.classes.read();
            let interned_strings = self.interned_strings();

            let mut root_identities = if let Some(provider) = &self.inner.external_roots {
                let objects = all_objects.iter().map(|object| &**object).collect::<Vec<_>>();
                provider.roots(&objects)?.into_iter().collect::<HashSet<_>>()
            } else {
                HashSet::new()
            };
            root_identities.extend(
                self.inner
                    .monitors
                    .read()
                    .iter()
                    .filter_map(|(identity, monitor)| monitor.is_active().then_some(*identity)),
            );

            determine_garbage(
                self,
                &threads,
                &global_references,
                &all_objects,
                &classes,
                &interned_strings,
                &root_identities,
            )?
        };

        let garbage_count = garbage.len();

        tracing::trace!("Garbage count: {garbage_count}");

        for object in garbage {
            tracing::trace!("Destroying {object:?}({})", object.class_definition().name());

            self.destroy(object).unwrap();
        }

        Ok(garbage_count)
    }

    pub(crate) async fn register_class_internal(&self, class: Class, class_loader_wrapper: Option<&dyn ClassLoaderWrapper>) -> Result<()> {
        if !class.definition.name().starts_with('[') {
            // ensure superclass and superinterfaces are loaded
            if let Some(super_class) = class.definition.super_class_name()
                && !self.has_class(&super_class)
            {
                self.resolve_class_internal(&super_class, class_loader_wrapper).await?;
            }

            for interface in class.definition.interface_names() {
                if !self.has_class(&interface) {
                    self.resolve_class_internal(&interface, class_loader_wrapper).await?;
                }
            }
        }

        self.inner.classes.write().entry(class.definition.name().into_owned()).or_insert(class);

        Ok(())
    }

    pub async fn ensure_initialized(&self, class: &Class) -> Result<()> {
        if class.definition.name().starts_with('[') || class.is_initialized() {
            return Ok(());
        }

        self.initialize_class(class).await
    }

    #[async_recursion::async_recursion]
    async fn initialize_class(&self, class: &Class) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        loop {
            match class.initialization_action(thread_id) {
                InitializationAction::Initialize => break,
                InitializationAction::Recursive | InitializationAction::Initialized => return Ok(()),
                InitializationAction::Wait(listener) => listener.await,
                InitializationAction::Erroneous => {
                    return Err(self
                        .exception(
                            "java/lang/NoClassDefFoundError",
                            &format!("Could not initialize class {}", class.definition.name()),
                        )
                        .await);
                }
            }
        }

        // Before the superclass (JVMS 5.5 steps 6 and 7): its initializer may already create instances of this class.
        if let Err(err) = class.definition.prepare(self).await {
            class.finish_initialization(InitState::Erroneous);
            return Err(err);
        }

        if let Some(super_name) = class.definition.super_class_name() {
            // resolution failure is not an initialization failure, so initialization may be retried
            let super_class = match self.resolve_class(&super_name).await {
                Ok(x) => x,
                Err(err) => {
                    class.finish_initialization(InitState::NotInitialized);
                    return Err(err);
                }
            };

            if let Err(err) = self.ensure_initialized(&super_class).await {
                class.finish_initialization(InitState::Erroneous);
                return Err(err);
            }
        }

        if let Some(clinit) = class.definition.method("<clinit>", "()V", true) {
            tracing::debug!("Calling <clinit> for {}", class.definition.name());

            let clinit = Arc::new(ResolvedMethod {
                class: class.clone(),
                method: clinit,
            });
            if let Err(err) = self.execute_method(&clinit, Box::new([])).await {
                class.finish_initialization(InitState::Erroneous);

                let JavaError::JavaException(exception) = &err;
                if self.is_instance(&**exception, "java/lang/Error") {
                    return Err(err);
                }

                let cause = clone_box(&**exception);
                let wrapped = self
                    .new_class("java/lang/ExceptionInInitializerError", "(Ljava/lang/Throwable;)V", (cause,))
                    .await?;

                return Err(JavaError::JavaException(wrapped));
            }
        }

        class.finish_initialization(InitState::Initialized);

        Ok(())
    }

    // every attached thread owns a java/lang/Thread instance; pass the instance for threads
    // started from java (Thread.start), or None to create one
    pub async fn attach_thread(&self, java_thread: Option<Box<dyn ClassInstance>>) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        let mut thread = JvmThread::new();
        thread.push_native_frame();
        self.inner.threads.write().insert(thread_id, thread);

        let java_thread = match java_thread {
            Some(x) => x,
            None => self.new_class("java/lang/Thread", "(Z)V", (true,)).await?,
        };
        self.inner
            .threads
            .write()
            .get_mut(&thread_id)
            .unwrap()
            .set_java_thread(java_thread.clone());
        let interrupted: bool = self.get_field(&java_thread, "java/lang/Thread", "interrupted", "Z").await?;
        self.inner.threads.write().get_mut(&thread_id).unwrap().interrupted |= interrupted;

        Ok(())
    }

    pub fn new_global_ref<T>(&self, reference: &ClassInstanceRef<T>) -> Option<GlobalRef<T>> {
        let instance = reference.instance.as_ref()?.clone();
        let id = self.inner.global_references.next_id.fetch_add(1, Ordering::Relaxed);
        self.inner.global_references.objects.write().insert(id, instance);

        Some(GlobalRef {
            references: self.inner.global_references.clone(),
            id,
            reference: reference.clone(),
        })
    }

    pub fn detach_thread(&self) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().remove(&thread_id);

        Ok(())
    }

    pub fn current_java_thread(&self) -> Box<dyn ClassInstance> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.read().get(&thread_id).unwrap().java_thread().unwrap().clone()
    }

    pub fn active_thread_count(&self) -> usize {
        self.inner.threads.read().len()
    }

    pub fn push_native_frame(&self, args: &[JavaValue]) -> NativeFrame {
        let thread_id = (self.inner.get_current_thread_id)();
        let mut threads = self.inner.threads.write();
        let thread = threads.get_mut(&thread_id).unwrap();
        thread.push_native_frame();
        thread
            .top_frame_mut()
            .local_variables_mut()
            .extend(args.iter().filter_map(|arg| match arg {
                JavaValue::Object(Some(instance)) => Some(instance.clone()),
                _ => None,
            }));
        NativeFrame {
            jvm: self.clone(),
            thread_id,
        }
    }

    /// Releases frame-local references while preserving the result in its caller.
    fn pop_frame_with_result(&self, result: &Result<JavaValue>) {
        let returned_reference = match result {
            Ok(JavaValue::Object(Some(instance))) => Some(instance.clone()),
            Err(JavaError::JavaException(exception)) => Some(exception.clone()),
            _ => None,
        };
        let thread_id = (self.inner.get_current_thread_id)();
        let mut threads = self.inner.threads.write();
        let thread = threads.get_mut(&thread_id).unwrap();
        thread.pop_frame();
        if let Some(returned_reference) = returned_reference {
            thread.top_frame_mut().local_variables_mut().push(returned_reference);
        }
    }

    async fn current_class_loader(&self) -> Result<Box<dyn ClassInstance>> {
        let calling_class = self.find_calling_class()?;

        if let Some((class, class_instance)) = calling_class {
            // called in java

            if let Some(instance) = class_instance
                && self.is_inherited_from(&*class.definition, "java/lang/ClassLoader")
            {
                return Ok(instance);
            }

            let calling_class_class_loader = JavaLangClass::class_loader(self, &class.java_class()).await?;
            if let Some(x) = calling_class_class_loader {
                Ok(x)
            } else {
                let system_class_loader = JavaLangClassLoader::get_system_class_loader(self).await?;
                Ok(system_class_loader)
            }
        } else {
            // called outside of java

            let system_class_loader = JavaLangClassLoader::get_system_class_loader(self).await?;
            Ok(system_class_loader)
        }
    }

    fn get_or_create_monitor(&self, obj: &dyn ClassInstance) -> Arc<Monitor> {
        let key = obj.identity();

        let monitors = self.inner.monitors.read();
        if let Some(monitor) = monitors.get(&key) {
            return monitor.clone();
        }
        drop(monitors);

        let mut monitors = self.inner.monitors.write();
        monitors.entry(key).or_insert_with(|| Arc::new(Monitor::new())).clone()
    }

    async fn resolve_field_ref(&self, class_name: &str, name: &str, descriptor: &str, expected_static: bool) -> Result<Option<ResolvedField>> {
        let key = MemberKey {
            lookup: if expected_static { Lookup::Static } else { Lookup::Instance },
            receiver: "".into(),
            class: class_name.into(),
            name: name.into(),
            descriptor: descriptor.into(),
        };
        if let Some(resolved) = self.inner.resolution_cache.field(&key) {
            return Ok(Some(resolved));
        }

        let class = self.resolve_class(class_name).await?;
        let Some(resolved) = self.resolve_field(&class, name, descriptor, expected_static) else {
            return Ok(None);
        };
        let resolved = Arc::new(resolved);
        self.inner.resolution_cache.insert_field(key, resolved.clone());

        Ok(Some(resolved))
    }

    // JVMS 5.4.3.2 field resolution: search the class, then its superinterfaces, then its superclass.
    // Matching is by name and descriptor only; the caller checks static-ness afterwards.
    fn resolve_field(&self, class: &Class, name: &str, descriptor: &str, expected_static: bool) -> Option<(Class, Box<dyn Field>)> {
        if let Some(field) = class
            .definition
            .field(name, descriptor, expected_static)
            .or_else(|| class.definition.field(name, descriptor, !expected_static))
        {
            return Some((class.clone(), field));
        }

        for interface in class.definition.interface_names() {
            if let Some(interface_class) = self.get_class(&interface)
                && let Some(found) = self.resolve_field(&interface_class, name, descriptor, expected_static)
            {
                return Some(found);
            }
        }

        let super_class = self.get_class(&class.definition.super_class_name()?)?;
        self.resolve_field(&super_class, name, descriptor, expected_static)
    }

    // Resolves a method symbolic reference once; later invocations reuse the result like a resolved constant pool entry (JVMS 5.4.3).
    async fn resolve_method_ref(&self, class_name: &str, name: &str, descriptor: &str, lookup: Lookup) -> Result<Option<Arc<ResolvedMethod>>> {
        let key = MemberKey {
            lookup,
            receiver: "".into(),
            class: class_name.into(),
            name: name.into(),
            descriptor: descriptor.into(),
        };
        if let Some(resolved) = self.inner.resolution_cache.method(&key) {
            return Ok(Some(resolved));
        }

        let class = self.resolve_class(class_name).await?;
        let resolved = match lookup {
            Lookup::Constructor => class.definition.method(name, descriptor, false).map(|method| (class, method)),
            Lookup::Static | Lookup::Instance => self.resolve_method(&class, name, descriptor, lookup == Lookup::Static),
        };
        let Some((class, method)) = resolved else {
            return Ok(None);
        };

        let resolved = Arc::new(ResolvedMethod { class, method });
        self.inner.resolution_cache.insert_method(key, resolved.clone());

        Ok(Some(resolved))
    }

    // JVMS 5.4.3.3 method resolution searches the class hierarchy before inherited interface methods.
    fn resolve_method(&self, class: &Class, name: &str, descriptor: &str, expected_static: bool) -> Option<(Class, Box<dyn Method>)> {
        let mut candidate = Some(class.clone());
        while let Some(current) = candidate {
            if let Some(method) = current
                .definition
                .method(name, descriptor, expected_static)
                .or_else(|| current.definition.method(name, descriptor, !expected_static))
            {
                return Some((current, method));
            }

            candidate = current.definition.super_class_name().and_then(|super_class| self.get_class(&super_class));
        }

        // superinterfaces are gathered only once the class chain misses, so inherited class methods do not clone interface names
        let mut interfaces = Vec::new();
        let mut candidate = Some(class.clone());
        while let Some(current) = candidate {
            interfaces.extend(current.definition.interface_names());
            candidate = current.definition.super_class_name().and_then(|super_class| self.get_class(&super_class));
        }

        let mut visited = HashSet::new();
        let mut candidates: Vec<(Class, Box<dyn Method>)> = Vec::new();
        while let Some(interface_name) = interfaces.pop() {
            if !visited.insert(interface_name.clone()) {
                continue;
            }
            let interface = self.get_class(&interface_name)?;
            interfaces.extend(interface.definition.interface_names());
            if let Some(method) = interface.definition.method(name, descriptor, false)
                && !method.access_flags().contains(MethodAccessFlags::PRIVATE)
            {
                if candidates
                    .iter()
                    .any(|(candidate, _)| self.is_inherited_from(&*candidate.definition, &interface_name))
                {
                    continue;
                }
                // Keep only declarations not overridden by a more specific interface.
                candidates.retain(|(candidate, _)| !self.is_inherited_from(&*interface.definition, &candidate.definition.name()));
                candidates.push((interface, method));
            }
        }

        let mut concrete = candidates
            .iter()
            .enumerate()
            .filter(|(_, (_, method))| !method.access_flags().contains(MethodAccessFlags::ABSTRACT));
        if let Some((index, _)) = concrete.next()
            && concrete.next().is_none()
        {
            return Some(candidates.swap_remove(index));
        }

        candidates.into_iter().next()
    }

    pub(crate) fn find_field(&self, class: &dyn ClassDefinition, name: &str, descriptor: &str) -> Result<Option<Box<dyn Field>>> {
        let field = class.field(name, descriptor, false);

        if let Some(x) = field {
            Ok(Some(x))
        } else if let Some(x) = class.super_class_name() {
            let super_class = self.inner.classes.read().get(&x).unwrap().definition.clone();
            self.find_field(&*super_class, name, descriptor)
        } else {
            Ok(None)
        }
    }

    fn method_overrides(&self, candidate_class: &Class, candidate_method: &dyn Method, resolved_class: &Class, resolved_method: &dyn Method) -> bool {
        if candidate_class.definition.name() == resolved_class.definition.name() {
            return true;
        }
        if !self.is_inherited_from(&*candidate_class.definition, &resolved_class.definition.name())
            || candidate_method
                .access_flags()
                .intersects(MethodAccessFlags::PRIVATE | MethodAccessFlags::STATIC)
        {
            return false;
        }

        let resolved_access = resolved_method.access_flags();
        if resolved_access.intersects(MethodAccessFlags::PUBLIC | MethodAccessFlags::PROTECTED) {
            return true;
        }
        if resolved_access.contains(MethodAccessFlags::PRIVATE) {
            return false;
        }
        if candidate_class.is_same_runtime_package(resolved_class) {
            return true;
        }

        // JVMS 5.4.5 also permits an override inherited through an intermediate method that overrides the package-private declaration.
        let mut super_class = candidate_class
            .definition
            .super_class_name()
            .and_then(|class_name| self.get_class(&class_name));
        while let Some(intermediate_class) = super_class {
            if intermediate_class.definition.name() == resolved_class.definition.name() {
                break;
            }
            if let Some(intermediate_method) = intermediate_class
                .definition
                .method(&resolved_method.name(), &resolved_method.descriptor(), false)
                && self.method_overrides(candidate_class, candidate_method, &intermediate_class, &*intermediate_method)
                && self.method_overrides(&intermediate_class, &*intermediate_method, resolved_class, resolved_method)
            {
                return true;
            }
            super_class = intermediate_class
                .definition
                .super_class_name()
                .and_then(|class_name| self.get_class(&class_name));
        }

        false
    }

    async fn execute_method(&self, resolved: &Arc<ResolvedMethod>, args: Box<[JavaValue]>) -> Result<JavaValue> {
        let thread_id = (self.inner.get_current_thread_id)();
        let method = &resolved.method;

        let synchronized_object = method
            .access_flags()
            .contains(MethodAccessFlags::SYNCHRONIZED)
            .then(|| match args.first() {
                Some(JavaValue::Object(Some(this))) if !method.access_flags().contains(MethodAccessFlags::STATIC) => this.clone(),
                _ => resolved.class.java_class(),
            });
        let depth = {
            let mut threads = self.inner.threads.write();
            let thread = threads.get_mut(&thread_id).unwrap();
            thread.push_java_frame(resolved.clone(), &args);
            thread.iter_frame().count()
        };

        // Only the exact limit throws: Java code can never pass it, while the error's own constructor frames run above it.
        if depth == MAX_STACK_DEPTH {
            let result = self
                .new_class("java/lang/StackOverflowError", "()V", ())
                .await
                .and_then(|error| Err(JavaError::JavaException(error)));
            self.pop_frame_with_result(&result);
            return result;
        }

        if let Some(object) = &synchronized_object
            && let Err(error) = self.monitor_enter(object).await
        {
            let result = Err(error);
            self.pop_frame_with_result(&result);
            return result;
        }

        let mut result = method.run(self, args).await;

        tracing::trace!("Execute result: {result:?}");

        if let Some(object) = &synchronized_object
            && let Err(error) = self.monitor_exit(object).await
        {
            if result.is_ok() {
                result = Err(error);
            } else {
                tracing::error!(?error, "failed to release synchronized method monitor");
            }
        }

        self.pop_frame_with_result(&result);

        result
    }
}
