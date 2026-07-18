#![allow(clippy::borrowed_box)] // We have get parameter by Box<T> to make ergonomic interface

use alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, format, string::String, sync::Arc, vec::Vec};
use core::{
    fmt::Debug,
    iter,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use dyn_clone::clone_box;
use hashbrown::HashSet;
use parking_lot::RwLock;

use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};

use crate::{
    Result,
    array_class_instance::{ArrayRawBuffer, ArrayRawBufferMut},
    class_definition::ClassDefinition,
    class_instance::{ClassInstance, ClassInstanceRef},
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
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
    thread::JvmThread,
    r#type::JavaType,
    value::JavaValue,
};

struct JvmInner {
    classes: RwLock<BTreeMap<String, Class>>,
    threads: RwLock<BTreeMap<u64, JvmThread>>,
    global_references: Arc<GlobalReferences>,
    all_objects: RwLock<HashSet<Box<dyn ClassInstance>>>,
    string_pool: RwLock<BTreeMap<Vec<u16>, Box<dyn ClassInstance>>>,
    monitors: RwLock<BTreeMap<usize, Arc<Monitor>>>,
    get_current_thread_id: Box<dyn Fn() -> u64 + Sync + Send>,
    bootstrap_class_loader: Box<dyn BootstrapClassLoader>,
    bootstrapping: AtomicBool,
}

#[derive(Clone)]
pub struct Jvm {
    inner: Arc<JvmInner>,
}

impl Jvm {
    pub async fn new<C, F>(bootstrap_class_loader: C, get_current_thread_id: F, properties: BTreeMap<&str, &str>) -> Result<Self>
    where
        C: BootstrapClassLoader + 'static,
        F: Fn() -> u64 + 'static + Sync + Send,
    {
        let jvm = Self {
            inner: Arc::new(JvmInner {
                classes: RwLock::new(BTreeMap::new()),
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
                bootstrapping: AtomicBool::new(true),
            }),
        };

        // load bootstrap classes
        let bootstrap_classes = ["java/lang/Object", "java/lang/Runnable", "java/lang/Thread", "[B", "java/lang/Class"];
        for class_name in bootstrap_classes.iter() {
            let class_definition = jvm.inner.bootstrap_class_loader.load_class(&jvm, class_name).await?.unwrap();
            let class = Class::new(class_definition, None);

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

        let class = self.resolve_class(class_name).await?;

        if let Some((declaring_class, field)) = self.resolve_field(&class, name, descriptor) {
            if !field.access_flags().contains(FieldAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            self.ensure_initialized(&declaring_class).await?;

            let value = declaring_class.definition.get_static_field(&*field)?;
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

        let class = self.resolve_class(class_name).await?;

        if let Some((mut declaring_class, field)) = self.resolve_field(&class, name, descriptor) {
            if !field.access_flags().contains(FieldAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            self.ensure_initialized(&declaring_class).await?;

            declaring_class.definition.put_static_field(&*field, value.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub async fn get_field<T>(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str) -> Result<T>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Get field {}.{name}:{descriptor}", instance.class_definition().name());

        let field = self.find_field(&*instance.class_definition(), name, descriptor)?;

        if let Some(field) = field {
            let value = instance.get_field(&*field)?;
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
                .exception(
                    "java/lang/NoSuchFieldError",
                    &format!("{}.{}:{}", instance.class_definition().name(), name, descriptor),
                )
                .await)
        }
    }

    pub async fn put_field<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str, descriptor: &str, value: T) -> Result<()>
    where
        T: Into<JavaValue> + Debug,
    {
        tracing::trace!("Put field {}.{name}:{descriptor} = {value:?}", instance.class_definition().name());

        let field = self.find_field(&*instance.class_definition(), name, descriptor)?;

        if let Some(field) = field {
            instance.put_field(&*field, value.into())
        } else {
            Err(self
                .exception(
                    "java/lang/NoSuchFieldError",
                    &format!("{}.{}:{}", instance.class_definition().name(), name, descriptor),
                )
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

        let class = self.resolve_class(class_name).await?;

        if let Some((declaring_class, method)) = self.resolve_method(&class, name, descriptor) {
            if !method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            self.ensure_initialized(&declaring_class).await?;

            Ok(self.execute_method(&declaring_class, None, &method, args).await?.into())
        } else {
            tracing::error!("No such method: {class_name}.{name}:{descriptor}");

            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub async fn invoke_virtual<T, U>(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str, args: T) -> Result<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        let args = args.into_arg();
        tracing::trace!("Invoke virtual {}.{name}:{descriptor}({args:?})", instance.class_definition().name());

        let class = instance.class_definition();
        let method = self.find_virtual_method(&*class, name, descriptor, false)?;
        if let Some(x) = method {
            let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
                .chain(args.into_vec())
                .collect::<Vec<_>>();

            let class = self.resolve_class(&class.name()).await?; // TODO we're resolving class twice
            Ok(self
                .execute_method(&class, Some(instance.clone()), &x, args.into_boxed_slice())
                .await?
                .into())
        } else {
            tracing::error!("No such method: {}.{name}:{descriptor}", class.name());

            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{}.{}:{}", class.name(), name, descriptor))
                .await)
        }
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

        let class = self.resolve_class(class_name).await?;
        let method = class.definition.method(name, descriptor, false);

        if let Some(method) = method {
            let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
                .chain(args.into_vec())
                .collect::<Vec<_>>();

            if method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            Ok(self
                .execute_method(&class, Some(instance.clone()), &method, args.into_boxed_slice())
                .await?
                .into())
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
        if offset + values.len() > array_size {
            return Err(self
                .exception(
                    "java/lang/ArrayIndexOutOfBoundsException",
                    &format!("{} > {}", offset + values.len(), array_size),
                )
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
        if offset + count > array_size {
            return Err(self
                .exception(
                    "java/lang/ArrayIndexOutOfBoundsException",
                    &format!("{} > {}", offset + count, array_size),
                )
                .await);
        }

        let array = array.as_array_instance();

        if let Some(array) = array {
            let values = array.load(offset, count)?;

            let thread_id = (self.inner.get_current_thread_id)();
            let mut threads = self.inner.threads.write();
            let local_variables = threads.get_mut(&thread_id).unwrap().top_frame_mut().local_variables_mut();
            values.iter().for_each(|value| {
                if let JavaValue::Object(Some(instance)) = value {
                    local_variables.push(instance.clone());
                }
            });

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

    pub async fn monitor_enter(&self, obj: &Box<dyn ClassInstance>) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.get_or_create_monitor(obj).enter(thread_id).await;
        Ok(())
    }

    pub async fn monitor_exit(&self, obj: &Box<dyn ClassInstance>) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        match self.get_or_create_monitor(obj).exit(thread_id) {
            Ok(()) => Ok(()),
            Err(_) => Err(self
                .exception("java/lang/IllegalMonitorStateException", "current thread does not own the monitor")
                .await),
        }
    }

    pub async fn object_wait_prepare(&self, obj: &Box<dyn ClassInstance>) -> Result<(MonitorWait, MonitorWaitTimeout)> {
        let thread_id = (self.inner.get_current_thread_id)();
        match self.get_or_create_monitor(obj).prepare_wait(thread_id) {
            Ok(wait) => Ok(wait),
            Err(_) => Err(self
                .exception("java/lang/IllegalMonitorStateException", "current thread does not own the monitor")
                .await),
        }
    }

    pub async fn object_wait(&self, wait: MonitorWait) -> Result<()> {
        wait.wait().await;
        Ok(())
    }

    pub async fn object_notify(&self, obj: &Box<dyn ClassInstance>, count: usize) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        match self.get_or_create_monitor(obj).notify(thread_id, count) {
            Ok(()) => Ok(()),
            Err(_) => Err(self
                .exception("java/lang/IllegalMonitorStateException", "current thread does not own the monitor")
                .await),
        }
    }

    #[async_recursion::async_recursion]
    pub async fn resolve_class(&self, class_name: &str) -> Result<Class> {
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
                self.resolve_class(&stripped_name[1..stripped_name.len() - 1]).await?;
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
            tracing::error!("No such class: {class_name}");

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

        Ok(thread.top_java_frame().map(|x| (x.class.clone(), x.class_instance.clone())))
    }

    pub async fn register_class(
        &self,
        class: Box<dyn ClassDefinition>,
        class_loader: Option<Box<dyn ClassInstance>>,
    ) -> Result<Option<Box<dyn ClassInstance>>> {
        tracing::debug!("Register class {}", class.name());

        let java_class = Some(JavaLangClass::from_rust_class(self, class.clone(), class_loader.clone()).await?);

        let class = Class::new(class, java_class.clone());

        if let Some(x) = class_loader {
            self.register_class_internal(class, Some(&JavaClassLoaderWrapper::new(x))).await?;
        } else {
            self.register_class_internal(class, None).await?;
        };

        Ok(java_class)
    }

    pub fn is_instance(&self, instance: &dyn ClassInstance, class_name: &str) -> bool {
        self.is_type_assignable(
            &JavaType::from_class_name(&instance.class_definition().name()),
            &JavaType::from_class_name(class_name),
        )
    }

    // aastore: whether value may be stored into array (JVMS 6.5 aastore)
    pub fn array_store_allowed(&self, array: &dyn ClassInstance, value: &dyn ClassInstance) -> bool {
        let JavaType::Array(component) = JavaType::parse(&array.class_definition().name()) else {
            return true;
        };

        self.is_type_assignable(&JavaType::from_class_name(&value.class_definition().name()), &component)
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

        let message_str = JavaLangString::from_rust_string(self, message).await.unwrap();
        let instance = self.new_class(r#type, "(Ljava/lang/String;)V", (message_str,)).await.unwrap();

        JavaError::JavaException(instance)
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
                if self.is_inherited_from(&*x.class.definition, "java/lang/Throwable") {
                    None
                } else {
                    Some(format!("{}.{}", x.class.definition.name(), x.method))
                }
            })
            .collect()
    }

    pub fn collect_garbage(&self) -> Result<usize> {
        tracing::trace!("Collecting garbage");

        let garbage = {
            let threads = self.inner.threads.read();
            let global_references = self.inner.global_references.objects.read();
            let all_objects = self.inner.all_objects.read();
            let classes = self.inner.classes.read();
            let interned_strings = self.interned_strings();

            determine_garbage(self, &threads, &global_references, &all_objects, &classes, &interned_strings)
        };

        let garbage_count = garbage.len();

        tracing::trace!("Garbage count: {garbage_count}");

        for object in garbage {
            let name = object.class_definition().name();
            tracing::trace!("Destroying {object:?}({name})");

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

        self.inner.classes.write().entry(class.definition.name().to_owned()).or_insert(class);

        Ok(())
    }

    #[async_recursion::async_recursion]
    pub async fn ensure_initialized(&self, class: &Class) -> Result<()> {
        if class.definition.name().starts_with('[') {
            return Ok(());
        }

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

        if let Err(err) = class.definition.prepare(self).await {
            class.finish_initialization(InitState::Erroneous);
            return Err(err);
        }

        if let Some(clinit) = class.definition.method("<clinit>", "()V", true) {
            tracing::debug!("Calling <clinit> for {}", class.definition.name());

            if let Err(err) = self.execute_method(class, None, &clinit, Box::new([])).await {
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
        self.inner.threads.write().insert(thread_id, JvmThread::new());
        self.push_native_frame();

        let java_thread = match java_thread {
            Some(x) => x,
            None => self.new_class("java/lang/Thread", "(Z)V", (true,)).await?,
        };
        self.inner.threads.write().get_mut(&thread_id).unwrap().set_java_thread(java_thread);

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

    // TODO we need safe, ergonomic api..
    pub fn push_native_frame(&self) {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().get_mut(&thread_id).unwrap().push_native_frame();
    }

    pub fn pop_frame(&self) {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().get_mut(&thread_id).unwrap().pop_frame();
    }

    pub async fn current_class_loader(&self) -> Result<Box<dyn ClassInstance>> {
        let calling_class = self.find_calling_class()?;

        if let Some((class, class_instance)) = calling_class {
            // called in java

            if self.is_inherited_from(&*class.definition, "java/lang/ClassLoader") {
                return Ok(class_instance.unwrap());
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

    fn get_or_create_monitor(&self, obj: &Box<dyn ClassInstance>) -> Arc<Monitor> {
        let key = obj.identity();

        let monitors = self.inner.monitors.read();
        if let Some(monitor) = monitors.get(&key) {
            return monitor.clone();
        }
        drop(monitors);

        let mut monitors = self.inner.monitors.write();
        monitors.entry(key).or_insert_with(|| Arc::new(Monitor::new())).clone()
    }

    // JVMS 5.4.3.2 field resolution: search the class, then its superinterfaces, then its superclass.
    // Matching is by name and descriptor only; the caller checks static-ness afterwards.
    fn resolve_field(&self, class: &Class, name: &str, descriptor: &str) -> Option<(Class, Box<dyn Field>)> {
        if let Some(field) = class
            .definition
            .field(name, descriptor, true)
            .or_else(|| class.definition.field(name, descriptor, false))
        {
            return Some((class.clone(), field));
        }

        for interface in class.definition.interface_names() {
            if let Some(interface_class) = self.get_class(&interface)
                && let Some(found) = self.resolve_field(&interface_class, name, descriptor)
            {
                return Some(found);
            }
        }

        let super_class = self.get_class(&class.definition.super_class_name()?)?;
        self.resolve_field(&super_class, name, descriptor)
    }

    // JVMS 5.4.3.3 method resolution: search the class, then its superclass (interfaces have no static methods
    // in a 1.2 target). Matching is by name and descriptor only; the caller checks static-ness afterwards.
    fn resolve_method(&self, class: &Class, name: &str, descriptor: &str) -> Option<(Class, Box<dyn Method>)> {
        if let Some(method) = class
            .definition
            .method(name, descriptor, true)
            .or_else(|| class.definition.method(name, descriptor, false))
        {
            return Some((class.clone(), method));
        }

        let super_class = self.get_class(&class.definition.super_class_name()?)?;
        self.resolve_method(&super_class, name, descriptor)
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

    fn find_virtual_method(&self, class: &dyn ClassDefinition, name: &str, descriptor: &str, is_static: bool) -> Result<Option<Box<dyn Method>>> {
        let method = class.method(name, descriptor, false);

        if let Some(x) = method {
            if x.access_flags().contains(MethodAccessFlags::STATIC) == is_static {
                return Ok(Some(x));
            }
        } else if let Some(x) = class.super_class_name() {
            let super_class = self.inner.classes.read().get(&x).unwrap().definition.clone();
            return self.find_virtual_method(&*super_class, name, descriptor, is_static);
        }

        Ok(None)
    }

    async fn execute_method(
        &self,
        class: &Class,
        class_instance: Option<Box<dyn ClassInstance>>,
        method: &Box<dyn Method>,
        args: Box<[JavaValue]>,
    ) -> Result<JavaValue> {
        let thread_id = (self.inner.get_current_thread_id)();
        let method_str = format!("{}{}", method.name(), method.descriptor());

        let synchronized_object = if method.access_flags().contains(MethodAccessFlags::SYNCHRONIZED) {
            Some(class_instance.clone().unwrap_or_else(|| class.java_class()))
        } else {
            None
        };
        if let Some(object) = &synchronized_object {
            self.monitor_enter(object).await?;
        }

        self.inner
            .threads
            .write()
            .get_mut(&thread_id)
            .unwrap()
            .push_java_frame(class, class_instance, &method_str, &args);

        let result = method.run(self, args).await;

        tracing::trace!("Execute result: {result:?}");

        let returned_reference = match &result {
            Ok(JavaValue::Object(Some(instance))) => Some(instance.clone()),
            Err(JavaError::JavaException(exception)) => Some(exception.clone()),
            _ => None,
        };
        {
            let mut threads = self.inner.threads.write();
            let thread = threads.get_mut(&thread_id).unwrap();
            thread.pop_frame();
            if let Some(returned_reference) = returned_reference {
                thread.top_frame_mut().local_variables_mut().push(returned_reference);
            }
        }

        if let Some(object) = &synchronized_object
            && let Err(error) = self.monitor_exit(object).await
        {
            if result.is_ok() {
                return Err(error);
            }
            tracing::error!(?error, "failed to release synchronized method monitor");
        }

        result
    }
}
