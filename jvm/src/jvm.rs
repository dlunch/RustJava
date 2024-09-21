#![allow(clippy::borrowed_box)] // We have get parameter by Box<T> to make ergonomic interface

use alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, format, string::String, sync::Arc, vec::Vec};
use core::{
    fmt::Debug,
    iter,
    mem::{forget, size_of_val},
    sync::atomic::{AtomicBool, Ordering},
};

use async_lock::RwLock;
use bytemuck::cast_slice;
use dyn_clone::clone_box;

use java_constants::MethodAccessFlags;

use crate::{
    array_class_instance::ArrayClassInstance,
    class_definition::ClassDefinition,
    class_instance::ClassInstance,
    class_loader::{BootstrapClassLoader, BootstrapClassLoaderWrapper, Class, ClassLoaderWrapper, JavaClassLoaderWrapper},
    error::JavaError,
    field::Field,
    invoke_arg::InvokeArg,
    method::Method,
    r#type::JavaType,
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
    thread::JvmThread,
    value::JavaValue,
    Result,
};

struct JvmInner {
    classes: RwLock<BTreeMap<String, Class>>,
    threads: RwLock<BTreeMap<u64, JvmThread>>,
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
                get_current_thread_id: Box::new(get_current_thread_id),
                bootstrap_class_loader: Box::new(bootstrap_class_loader),
                bootstrapping: AtomicBool::new(true),
            }),
        };

        // load bootstrap classes
        let bootstrap_classes = ["java/lang/Object", "java/lang/Thread", "[B", "java/lang/Class"];
        for class_name in bootstrap_classes.iter() {
            let class = jvm.inner.bootstrap_class_loader.load_class(&jvm, class_name).await?.unwrap();
            jvm.register_class(class, None).await?;
        }

        // init startup thread
        jvm.attach_thread().await?;

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

    #[async_recursion::async_recursion]
    pub async fn instantiate_class(&self, class_name: &str) -> Result<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate {}", class_name);

        let class = self.resolve_class(class_name).await?;

        let instance = class.definition.instantiate()?;

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

    #[async_recursion::async_recursion]
    pub async fn instantiate_array(&self, element_type_name: &str, length: usize) -> Result<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate array of {} with length {}", element_type_name, length);

        let class_name = format!("[{}", element_type_name);

        let class = self.resolve_class(&class_name).await?.definition;
        let array_class = class.as_array_class_definition().unwrap();

        let instance = array_class.instantiate_array(length)?;
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
            Ok(class.definition.get_static_field(&*field).await?.into())
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
            class.definition.put_static_field(&*field, value.into()).await
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

        let field = self.find_field(&*instance.class_definition(), name, descriptor).await?;

        if let Some(field) = field {
            Ok(instance.get_field(&*field).await?.into())
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

        let field = self.find_field(&*instance.class_definition(), name, descriptor).await?;

        if let Some(field) = field {
            instance.put_field(&*field, value.into()).await
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
        tracing::trace!("Invoke static {}.{}:{}", class_name, name, descriptor);

        let class = self.resolve_class(class_name).await?;

        let method = class.definition.method(name, descriptor);

        if let Some(method) = method {
            if !method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception(
                        "java/lang/IncompatibleClassChangeError",
                        &format!("{}.{}:{}", class_name, name, descriptor),
                    )
                    .await);
            }

            Ok(self.execute_method(&class, None, &method, args.into_arg()).await?.into())
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
        tracing::trace!("Invoke virtual {}.{}:{}", instance.class_definition().name(), name, descriptor);

        let class = instance.class_definition();
        let method = self.find_virtual_method(&*class, name, descriptor, false).await?;
        if let Some(x) = method {
            let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
                .chain(args.into_iter())
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
        tracing::trace!("Invoke special {}.{}:{}", class_name, name, descriptor);

        let class = self.resolve_class(class_name).await?;
        let method = class.definition.method(name, descriptor);

        if let Some(method) = method {
            let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
                .chain(args.into_iter())
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

        let array = array.as_array_instance_mut();

        if let Some(array) = array {
            let values = values.into_iter().map(|x| x.into()).collect::<Vec<_>>();
            array.store(offset, values.into_boxed_slice()).await?;

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

        let array = array.as_array_instance();

        if let Some(array) = array {
            let values = array.load(offset, count).await?;

            Ok(iter::IntoIterator::into_iter(values).map(|x| x.into()).collect::<Vec<_>>())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn store_byte_array(&self, array: &mut Box<dyn ClassInstance>, offset: usize, values: Vec<i8>) -> Result<()> {
        tracing::trace!("Store array {} at offset {}", array.class_definition().name(), offset);

        let array = array.as_array_instance_mut();

        if let Some(array) = array {
            array.store_bytes(offset, values.into_boxed_slice()).await
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn load_byte_array(&self, array: &Box<dyn ClassInstance>, offset: usize, count: usize) -> Result<Vec<i8>> {
        tracing::trace!("Load array {} at offset {}", array.class_definition().name(), offset);

        let array = array.as_array_instance();

        if let Some(array) = array {
            let values = array.load_bytes(offset, count).await?;

            Ok(values)
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

    // temporary until we have working gc
    pub fn destroy(&self, instance: Box<dyn ClassInstance>) -> Result<()> {
        tracing::debug!("Destroy {}", instance.class_definition().name());

        instance.destroy();

        Ok(())
    }

    pub async fn has_class(&self, class_name: &str) -> bool {
        self.inner.classes.read().await.contains_key(class_name)
    }

    #[async_recursion::async_recursion]
    pub async fn resolve_class(&self, class_name: &str) -> Result<Class> {
        self.resolve_class_internal(class_name, None).await
    }

    #[async_recursion::async_recursion]
    async fn resolve_class_internal(&self, class_name: &str, class_loader_wrapper: Option<&dyn ClassLoaderWrapper>) -> Result<Class> {
        tracing::trace!("Resolving class {}", class_name);
        let class = self.inner.classes.read().await.get(class_name).cloned();

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

    async fn find_calling_class(&self) -> Result<Option<(Class, Option<Box<dyn ClassInstance>>)>> {
        let thread_id = (self.inner.get_current_thread_id)();

        let threads = self.inner.threads.read().await;
        let thread = threads.get(&thread_id).unwrap();

        Ok(thread.top_frame().map(|x| (x.class.clone(), x.class_instance.clone())))
    }

    pub async fn register_class(
        &self,
        class: Box<dyn ClassDefinition>,
        class_loader: Option<Box<dyn ClassInstance>>,
    ) -> Result<Option<Box<dyn ClassInstance>>> {
        tracing::debug!("Register class {}", class.name());

        // delay java/lang/Class construction on bootstrap, as we won't have java/lang/Class yet
        let java_class = if self.has_class("java/lang/Class").await {
            Some(JavaLangClass::from_rust_class(self, class.clone(), class_loader.clone()).await?)
        } else {
            None
        };

        let class = Class::new(class, java_class.clone());

        if let Some(x) = class_loader {
            self.register_class_internal(class, Some(&JavaClassLoaderWrapper::new(x))).await?;
        } else {
            self.register_class_internal(class, None).await?;
        };

        Ok(java_class)
    }

    pub async fn is_instance(&self, instance: &dyn ClassInstance, class_name: &str) -> Result<bool> {
        let instance_class = instance.class_definition();

        self.is_instance_by_name(&instance_class.name(), class_name).await
    }

    #[async_recursion::async_recursion]
    async fn is_a(&self, class: &Class, class_name: &str) -> bool {
        if class.definition.name() == class_name {
            return true;
        }

        if let Some(super_class) = class.definition.super_class_name() {
            self.is_a(self.inner.classes.read().await.get(&super_class).unwrap(), class_name).await
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

    pub async fn stack_trace(&self) -> Vec<String> {
        // TODO we should return in another format

        let thread_id = (self.inner.get_current_thread_id)();
        let threads = self.inner.threads.read().await;
        let thread = threads.get(&thread_id).unwrap();

        let mut result = Vec::with_capacity(thread.stack.len());

        for item in thread.stack.iter().rev() {
            // skip exception classes
            if self.is_a(&item.class, "java/lang/Throwable").await {
                continue;
            }

            result.push(format!("{}.{}", item.class.definition.name(), item.method));
        }

        result
    }

    #[async_recursion::async_recursion]
    async fn is_instance_by_name(&self, instance_class_name: &str, class_name: &str) -> Result<bool> {
        let instance_class = self.resolve_class(instance_class_name).await?.definition;

        if instance_class.name() == class_name {
            return Ok(true);
        }

        if let Some(super_class) = instance_class.super_class_name() {
            return self.is_instance_by_name(&super_class, class_name).await;
        }

        Ok(false)
    }

    async fn register_class_internal(&self, class: Class, class_loader_wrapper: Option<&dyn ClassLoaderWrapper>) -> Result<()> {
        if !class.definition.name().starts_with('[') {
            if let Some(super_class) = class.definition.super_class_name() {
                if !self.has_class(&super_class).await {
                    // ensure superclass is loaded
                    self.resolve_class_internal(&super_class, class_loader_wrapper).await?;
                }
            }
        }

        self.inner.classes.write().await.insert(class.definition.name().to_owned(), class.clone());

        let clinit = class.definition.method("<clinit>", "()V");

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
        let raw = self.load_byte_array(&raw_storage, 0, self.array_length(&raw_storage).await?).await?;

        let rust_raw = usize::from_le_bytes(cast_slice(&raw).try_into().unwrap());

        let rust = unsafe { Box::from_raw(rust_raw as *mut T) };
        let result = (*rust).clone();

        forget(rust); // do not drop box as we still have it in java memory

        Ok(result)
    }

    pub async fn put_rust_object_field<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str, value: T) -> Result<()> {
        let rust_class_raw = Box::into_raw(Box::new(value)) as *const u8 as usize;

        let mut raw_storage = self.instantiate_array("B", size_of_val(&rust_class_raw)).await?;
        self.store_byte_array(&mut raw_storage, 0, cast_slice(&rust_class_raw.to_le_bytes()).to_vec())
            .await?;

        self.put_field(instance, name, "[B", raw_storage).await?;

        Ok(())
    }

    pub async fn attach_thread(&self) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().await.insert(thread_id, JvmThread::new());

        Ok(())
    }

    pub async fn detach_thread(&self) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().await.remove(&thread_id);

        Ok(())
    }

    pub async fn current_class_loader(&self) -> Result<Box<dyn ClassInstance>> {
        let calling_class = self.find_calling_class().await?;

        if let Some((class, class_instance)) = calling_class {
            // called in java

            if self.is_a(&class, "java/lang/ClassLoader").await {
                return Ok(class_instance.unwrap());
            }

            let calling_class_class_loader = JavaLangClass::class_loader(self, &class.java_class(self).await?).await?;
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

    #[async_recursion::async_recursion]
    async fn find_field(&self, class: &dyn ClassDefinition, name: &str, descriptor: &str) -> Result<Option<Box<dyn Field>>> {
        let field = class.field(name, descriptor, false);

        if let Some(x) = field {
            Ok(Some(x))
        } else if let Some(x) = class.super_class_name() {
            let super_class = self.inner.classes.read().await.get(&x).unwrap().definition.clone();
            self.find_field(&*super_class, name, descriptor).await
        } else {
            Ok(None)
        }
    }

    #[async_recursion::async_recursion]
    async fn find_virtual_method(
        &self,
        class: &dyn ClassDefinition,
        name: &str,
        descriptor: &str,
        is_static: bool,
    ) -> Result<Option<Box<dyn Method>>> {
        let method = class.method(name, descriptor);

        if let Some(x) = method {
            if x.access_flags().contains(MethodAccessFlags::STATIC) == is_static {
                return Ok(Some(x));
            }
        } else if let Some(x) = class.super_class_name() {
            let super_class = self.inner.classes.read().await.get(&x).unwrap().definition.clone();
            return self.find_virtual_method(&*super_class, name, descriptor, is_static).await;
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
            .await
            .get_mut(&thread_id)
            .unwrap()
            .push_frame(class, class_instance, &method_str);

        let result = method.run(self, args).await?;

        self.inner.threads.write().await.get_mut(&thread_id).unwrap().pop_frame();

        Ok(result)
    }
}
