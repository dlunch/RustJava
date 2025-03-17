#![allow(clippy::borrowed_box)] // We have get parameter by Box<T> to make ergonomic interface

use alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, format, string::String, sync::Arc, vec::Vec};
use core::{
    fmt::Debug,
    iter,
    mem::{forget, size_of_val},
    sync::atomic::{AtomicBool, Ordering},
};

use bytemuck::cast_slice;
use dyn_clone::clone_box;
use hashbrown::HashSet;
use parking_lot::RwLock;

use java_constants::MethodAccessFlags;

use crate::{
    Result,
    array_class_instance::{ArrayClassInstance, ArrayRawBuffer, ArrayRawBufferMut},
    class_definition::ClassDefinition,
    class_instance::ClassInstance,
    class_loader::{BootstrapClassLoader, BootstrapClassLoaderWrapper, Class, ClassLoaderWrapper, JavaClassLoaderWrapper},
    error::JavaError,
    field::Field,
    garbage_collector::determine_garbage,
    invoke_arg::InvokeArg,
    method::Method,
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
    thread::JvmThread,
    r#type::JavaType,
    value::JavaValue,
};

struct JvmInner {
    classes: RwLock<BTreeMap<String, Class>>,
    threads: RwLock<BTreeMap<u64, JvmThread>>,
    all_objects: RwLock<HashSet<Box<dyn ClassInstance>>>,
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
                all_objects: RwLock::new(HashSet::new()),
                get_current_thread_id: Box::new(get_current_thread_id),
                bootstrap_class_loader: Box::new(bootstrap_class_loader),
                bootstrapping: AtomicBool::new(true),
            }),
        };

        // load bootstrap classes
        let bootstrap_classes = ["java/lang/Object", "java/lang/Thread", "[B", "java/lang/Class"];
        for class_name in bootstrap_classes.iter() {
            let class_definition = jvm.inner.bootstrap_class_loader.load_class(&jvm, class_name).await?.unwrap();
            let class = Class::new(class_definition, None);

            jvm.register_class_internal(class, None).await?;
        }

        // init startup thread
        jvm.attach_thread()?;

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

        Ok(jvm)
    }

    pub async fn instantiate_class(&self, class_name: &str) -> Result<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate {}", class_name);

        let class = self.resolve_class(class_name).await?;

        let instance = class.definition.instantiate()?;

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
        tracing::trace!("Instantiate array of {} with length {}", element_type_name, length);

        let class_name = format!("[{}", element_type_name);

        let class = self.resolve_class(&class_name).await?.definition;
        let array_class = class.as_array_class_definition().unwrap();

        let instance = array_class.instantiate_array(length)?;

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
        tracing::trace!("Get static field {}.{}:{}", class_name, name, descriptor);

        let class = self.resolve_class(class_name).await?;

        let field = class.definition.field(name, descriptor, true);
        if let Some(field) = field {
            Ok(class.definition.get_static_field(&*field)?.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{}.{}:{}", class_name, name, descriptor))
                .await)
        }
    }

    pub async fn put_static_field<T>(&self, class_name: &str, name: &str, descriptor: &str, value: T) -> Result<()>
    where
        T: Into<JavaValue> + Debug,
    {
        tracing::trace!("Put static field {}.{}:{} = {:?}", class_name, name, descriptor, value);

        let mut class = self.resolve_class(class_name).await?;

        let field = class.definition.field(name, descriptor, true);

        if let Some(field) = field {
            class.definition.put_static_field(&*field, value.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{}.{}:{}", class_name, name, descriptor))
                .await)
        }
    }

    pub async fn get_field<T>(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str) -> Result<T>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Get field {}.{}:{}", instance.class_definition().name(), name, descriptor);

        let field = self.find_field(&*instance.class_definition(), name, descriptor)?;

        if let Some(field) = field {
            Ok(instance.get_field(&*field)?.into())
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
        tracing::trace!("Put field {}.{}:{} = {:?}", instance.class_definition().name(), name, descriptor, value);

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

        tracing::trace!("Invoke static {}.{}:{}({:?})", class_name, name, descriptor, args);

        let class = self.resolve_class(class_name).await?;

        let method = class.definition.method(name, descriptor, true);

        if let Some(method) = method {
            if !method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception(
                        "java/lang/IncompatibleClassChangeError",
                        &format!("{}.{}:{}", class_name, name, descriptor),
                    )
                    .await);
            }

            Ok(self.execute_method(&class, None, &method, args).await?.into())
        } else {
            tracing::error!("No such method: {}.{}:{}", class_name, name, descriptor);

            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{}.{}:{}", class_name, name, descriptor))
                .await)
        }
    }

    pub async fn invoke_virtual<T, U>(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str, args: T) -> Result<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        let args = args.into_arg();
        tracing::trace!(
            "Invoke virtual {}.{}:{}({:?})",
            instance.class_definition().name(),
            name,
            descriptor,
            args
        );

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
            tracing::error!("No such method: {}.{}:{}", class.name(), name, descriptor);

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
        tracing::trace!("Invoke special {}.{}:{}({:?})", class_name, name, descriptor, args);

        let class = self.resolve_class(class_name).await?;
        let method = class.definition.method(name, descriptor, false);

        if let Some(method) = method {
            let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
                .chain(args.into_vec())
                .collect::<Vec<_>>();

            if method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception(
                        "java/lang/IncompatibleClassChangeError",
                        &format!("{}.{}:{}", class_name, name, descriptor),
                    )
                    .await);
            }

            Ok(self
                .execute_method(&class, Some(instance.clone()), &method, args.into_boxed_slice())
                .await?
                .into())
        } else {
            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{}.{}:{}", class_name, name, descriptor))
                .await)
        }
    }

    #[async_recursion::async_recursion]
    pub async fn store_array<T, U>(&self, array: &mut Box<dyn ClassInstance>, offset: usize, values: T) -> Result<()>
    where
        T: IntoIterator<Item = U> + Send,
        U: Into<JavaValue>,
    {
        tracing::trace!("Store array {} at offset {}", array.class_definition().name(), offset);

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
        tracing::trace!("Load array {} at offset {}", array.class_definition().name(), offset);

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
            let class = ArrayClassInstance::class_definition(array);

            let type_name = &class.name()[1..]; // TODO can we store JavaType on class?

            Ok(JavaType::parse(type_name))
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub fn destroy(&self, instance: Box<dyn ClassInstance>) -> Result<()> {
        tracing::debug!("Destroy {}", instance.class_definition().name());

        self.inner.all_objects.write().remove(&instance);
        instance.destroy();

        Ok(())
    }

    pub fn has_class(&self, class_name: &str) -> bool {
        self.inner.classes.read().contains_key(class_name)
    }

    pub fn get_class(&self, class_name: &str) -> Option<Class> {
        self.inner.classes.read().get(class_name).cloned()
    }

    #[async_recursion::async_recursion]
    pub async fn resolve_class(&self, class_name: &str) -> Result<Class> {
        self.resolve_class_internal(class_name, None).await
    }

    #[async_recursion::async_recursion]
    async fn resolve_class_internal(&self, class_name: &str, class_loader_wrapper: Option<&dyn ClassLoaderWrapper>) -> Result<Class> {
        tracing::trace!("Resolving class {}", class_name);
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

        self.load_class(class_name, class_loader_wrapper).await
    }

    async fn load_class(&self, class_name: &str, class_loader_wrapper: &dyn ClassLoaderWrapper) -> Result<Class> {
        tracing::debug!("Loading class {}", class_name);

        let class = class_loader_wrapper.load_class(self, class_name).await?;

        if class.is_none() {
            tracing::error!("No such class: {}", class_name);

            return Err(self.exception("java/lang/NoClassDefFoundError", class_name).await);
        }

        tracing::debug!("Loaded class {}", class_name);

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
        let class = instance.class_definition();

        self.is_inherited_from(&*class, class_name)
    }

    pub fn is_inherited_from(&self, class: &dyn ClassDefinition, class_name: &str) -> bool {
        if class.name() == class_name {
            return true;
        }

        if let Some(super_class) = class.super_class_name() {
            self.is_inherited_from(&*self.inner.classes.read().get(&super_class).unwrap().definition, class_name)
        } else {
            false
        }
    }

    pub async fn exception(&self, r#type: &str, message: &str) -> JavaError {
        tracing::info!("throwing java exception: {} {}", r#type, message);

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
            let all_objects = self.inner.all_objects.read();
            let classes = self.inner.classes.read();

            determine_garbage(self, &threads, &all_objects, &classes)
        };

        let garbage_count = garbage.len();

        tracing::trace!("Garbage count: {}", garbage_count);

        for object in garbage {
            let name = object.class_definition().name();
            tracing::trace!("Destroying {:?}({})", object, name);

            self.destroy(object).unwrap();
        }

        Ok(garbage_count)
    }

    async fn register_class_internal(&self, class: Class, class_loader_wrapper: Option<&dyn ClassLoaderWrapper>) -> Result<()> {
        if !class.definition.name().starts_with('[') {
            if let Some(super_class) = class.definition.super_class_name() {
                if !self.has_class(&super_class) {
                    // ensure superclass is loaded
                    self.resolve_class_internal(&super_class, class_loader_wrapper).await?;
                }
            }
        }

        self.inner.classes.write().insert(class.definition.name().to_owned(), class.clone());

        let clinit = class.definition.method("<clinit>", "()V", true);

        if let Some(x) = clinit {
            tracing::debug!("Calling <clinit> for {}", class.definition.name());

            x.run(self, Box::new([])).await?;
        }

        Ok(())
    }

    pub async fn get_rust_object_field<T>(&self, instance: &Box<dyn ClassInstance>, name: &str) -> Result<T>
    where
        T: Clone,
    {
        let raw_storage = self.get_field(instance, name, "[B").await?;
        let length = self.array_length(&raw_storage).await?;
        let buf: Vec<i8> = self.load_array(&raw_storage, 0, length).await?;

        let rust_raw = usize::from_le_bytes(cast_slice(&buf).try_into().unwrap());

        let rust = unsafe { Box::from_raw(rust_raw as *mut T) };
        let result = (*rust).clone();

        forget(rust); // do not drop box as we still have it in java memory

        Ok(result)
    }

    pub async fn put_rust_object_field<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str, value: T) -> Result<()> {
        let rust_raw = Box::into_raw(Box::new(value)) as *const u8 as usize;

        let length = size_of_val(&rust_raw);
        let mut raw_storage = self.instantiate_array("B", length).await?;
        self.store_array(&mut raw_storage, 0, cast_slice::<u8, i8>(&rust_raw.to_le_bytes()).to_vec())
            .await?;

        self.put_field(instance, name, "[B", raw_storage).await?;

        Ok(())
    }

    pub fn attach_thread(&self) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().insert(thread_id, JvmThread::new());
        self.push_native_frame();

        Ok(())
    }

    pub fn detach_thread(&self) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().remove(&thread_id);

        Ok(())
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

        self.inner
            .threads
            .write()
            .get_mut(&thread_id)
            .unwrap()
            .push_java_frame(class, class_instance, &method_str);

        let result = method.run(self, args).await;

        tracing::trace!("Execute result: {:?}", result);

        self.inner.threads.write().get_mut(&thread_id).unwrap().pop_frame();

        result
    }
}
