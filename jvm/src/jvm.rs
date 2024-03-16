#![allow(clippy::borrowed_box)] // We have get parameter by Box<T> to make ergonomic interface

use alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, format, rc::Rc, string::String, vec::Vec};
use core::{
    cell::RefCell,
    fmt::Debug,
    iter,
    mem::{forget, size_of_val},
};

use bytemuck::cast_slice;
use dyn_clone::clone_box;

use java_constants::MethodAccessFlags;

use crate::{
    array_class_instance::ArrayClassInstance,
    class_definition::ClassDefinition,
    class_instance::ClassInstance,
    detail::JvmDetail,
    error::JavaError,
    field::Field,
    invoke_arg::InvokeArg,
    method::Method,
    r#type::JavaType,
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
    value::JavaValue,
    Result,
};

#[derive(Clone)]
pub struct Class {
    pub definition: Box<dyn ClassDefinition>,
    java_class: Rc<RefCell<Option<Box<dyn ClassInstance>>>>,
}

impl Class {
    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn java_class(&mut self, jvm: &Jvm) -> Result<Box<dyn ClassInstance>> {
        let java_class = self.java_class.borrow();
        if let Some(x) = &*java_class {
            Ok(x.clone())
        } else {
            drop(java_class);

            // class registered while bootstrapping might not have java/lang/Class, so instantiate it lazily

            let class_loader = jvm.get_system_class_loader().await?;
            let java_class = JavaLangClass::from_rust_class(jvm, self.definition.clone(), Some(class_loader)).await?;

            self.java_class.replace(Some(java_class.clone()));

            Ok(java_class)
        }
    }
}

pub struct Jvm {
    classes: RefCell<BTreeMap<String, Class>>,
    system_class_loader: RefCell<Option<Box<dyn ClassInstance>>>,
    detail: RefCell<Box<dyn JvmDetail>>,
}

impl Jvm {
    pub async fn new<T>(detail: T) -> Result<Self>
    where
        T: JvmDetail + 'static,
    {
        Ok(Self {
            classes: RefCell::new(BTreeMap::new()),
            system_class_loader: RefCell::new(None),
            detail: RefCell::new(Box::new(detail)),
        })
    }

    pub async fn instantiate_class(&self, class_name: &str) -> Result<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate {}", class_name);

        let class = self.resolve_class(class_name).await?;

        let instance = class.definition.instantiate();

        Ok(instance)
    }

    pub async fn new_class<T>(&self, class_name: &str, init_descriptor: &str, init_args: T) -> Result<Box<dyn ClassInstance>>
    where
        T: InvokeArg,
    {
        let instance = self.instantiate_class(class_name).await?;

        self.invoke_special(&instance, class_name, "<init>", init_descriptor, init_args).await?;

        Ok(instance)
    }

    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn instantiate_array(&self, element_type_name: &str, length: usize) -> Result<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate array of {} with length {}", element_type_name, length);

        let class_name = format!("[{}", element_type_name);
        let class = if self.system_class_loader.borrow().is_none() || element_type_name.len() == 1 {
            if self.has_class(&class_name) {
                self.resolve_class(&class_name).await?.definition
            } else {
                // bootstrapping or primitive type
                let definition = self.detail.borrow().define_array_class(self, element_type_name).await?;
                self.register_class_internal(definition.clone(), None).await?;

                definition
            }
        } else {
            self.resolve_class(&class_name).await?.definition
        };
        let array_class = class.as_array_class_definition().unwrap();

        let instance = array_class.instantiate_array(length);
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

            Ok(method.run(self, args.into_arg()).await?.into())
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

        let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
            .chain(args.into_iter())
            .collect::<Vec<_>>();

        Ok(method.run(self, args.into_boxed_slice()).await?.into())
    }

    // non-virtual
    #[async_recursion::async_recursion(?Send)]
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

            Ok(method.run(self, args.into_boxed_slice()).await?.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{}.{}:{}", class_name, name, descriptor))
                .await)
        }
    }

    #[async_recursion::async_recursion(?Send)]
    pub async fn store_array<T, U>(&self, array: &mut Box<dyn ClassInstance>, offset: usize, values: T) -> Result<()>
    where
        T: IntoIterator<Item = U>,
        U: Into<JavaValue>,
    {
        tracing::trace!("Store array {} at offset {}", array.class_definition().name(), offset);

        let array = array.as_array_instance_mut();

        if let Some(array) = array {
            let values = values.into_iter().map(|x| x.into()).collect::<Vec<_>>();
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

        let array = array.as_array_instance();

        if let Some(array) = array {
            let values = array.load(offset, count)?;

            Ok(iter::IntoIterator::into_iter(values).map(|x| x.into()).collect::<Vec<_>>())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn store_byte_array(&self, array: &mut Box<dyn ClassInstance>, offset: usize, values: Vec<i8>) -> Result<()> {
        tracing::trace!("Store array {} at offset {}", array.class_definition().name(), offset);

        let array = array.as_array_instance_mut();

        if let Some(array) = array {
            array.store_bytes(offset, values.into_boxed_slice())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn load_byte_array(&self, array: &Box<dyn ClassInstance>, offset: usize, count: usize) -> Result<Vec<i8>> {
        tracing::trace!("Load array {} at offset {}", array.class_definition().name(), offset);

        let array = array.as_array_instance();

        if let Some(array) = array {
            let values = array.load_bytes(offset, count)?;

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

    pub fn has_class(&self, class_name: &str) -> bool {
        self.classes.borrow().contains_key(class_name)
    }

    #[async_recursion::async_recursion(?Send)]
    pub async fn resolve_class(&self, class_name: &str) -> Result<Class> {
        let class = self.classes.borrow().get(class_name).cloned();

        if let Some(x) = class {
            return Ok(x);
        }

        // load class
        let class_loader = self.get_system_class_loader().await?;
        let java_class = JavaLangClassLoader::load_class(self, class_loader, class_name).await?;

        if let Some(x) = &java_class {
            tracing::debug!("Loaded class {}", class_name);

            let class = JavaLangClass::to_rust_class(self, x).await?;

            self.register_class_internal(class.clone(), java_class).await?;

            let class = self.classes.borrow().get(class_name).unwrap().clone();

            return Ok(class);
        }

        tracing::error!("No such class: {}", class_name);

        Err(self.exception("java/lang/NoClassDefFoundError", class_name).await)
    }

    pub async fn register_class(&self, class: Box<dyn ClassDefinition>, class_loader: Option<Box<dyn ClassInstance>>) -> Result<()> {
        tracing::debug!("Register class {}", class.name());

        // delay java/lang/Class construction on bootstrap, as we won't have java/lang/Class yet
        let java_class = if class_loader.is_some() {
            Some(JavaLangClass::from_rust_class(self, class.clone(), class_loader).await?)
        } else {
            None
        };

        self.register_class_internal(class, java_class).await?;

        Ok(())
    }

    pub async fn is_instance(&self, instance: &dyn ClassInstance, class_name: &str) -> Result<bool> {
        let instance_class = instance.class_definition();

        self.is_instance_by_name(&instance_class.name(), class_name).await
    }

    async fn exception(&self, r#type: &str, message: &str) -> JavaError {
        let message_str = JavaLangString::from_rust_string(self, message).await.unwrap();
        let instance = self.new_class(r#type, "(Ljava/lang/String;)V", (message_str,)).await.unwrap();

        JavaError::JavaException(instance)
    }

    #[async_recursion::async_recursion(?Send)]
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

    async fn register_class_internal(&self, class_definition: Box<dyn ClassDefinition>, java_class: Option<Box<dyn ClassInstance>>) -> Result<()> {
        if !class_definition.name().starts_with('[') {
            if let Some(super_class) = class_definition.super_class_name() {
                // ensure superclass is loaded
                self.resolve_class(&super_class).await?;
            }
        }

        self.classes.borrow_mut().insert(
            class_definition.name().to_owned(),
            Class {
                definition: class_definition.clone(),
                java_class: Rc::new(RefCell::new(java_class)),
            },
        );

        let clinit = class_definition.method("<clinit>", "()V");

        if let Some(x) = clinit {
            tracing::debug!("Calling <clinit> for {}", class_definition.name());

            x.run(self, Box::new([])).await?;
        }

        Ok(())
    }

    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn define_class(&self, name: &str, data: &[u8], class_loader: Box<dyn ClassInstance>) -> Result<Box<dyn ClassInstance>> {
        let class = self.detail.borrow().define_class(self, name, data).await?;

        self.register_class(class.clone(), Some(class_loader)).await?;

        self.resolve_class(&class.name()).await?.java_class(self).await
    }

    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn define_array_class(&self, element_type_name: &str, class_loader: Box<dyn ClassInstance>) -> Result<Box<dyn ClassInstance>> {
        let class = self.detail.borrow().define_array_class(self, element_type_name).await?;

        self.register_class(class.clone(), Some(class_loader)).await?;

        self.resolve_class(&class.name()).await?.java_class(self).await
    }

    pub fn set_system_class_loader(&self, class_loader: Box<dyn ClassInstance>) {
        self.system_class_loader.replace(Some(class_loader)); // TODO we need Thread.setContextClassLoader
    }

    pub async fn get_system_class_loader(&self) -> Result<Box<dyn ClassInstance>> {
        if self.system_class_loader.borrow().is_none() {
            let system_class_loader = JavaLangClassLoader::get_system_class_loader(self).await?;

            self.system_class_loader.replace(Some(system_class_loader));
        }

        Ok(self.system_class_loader.borrow().as_ref().unwrap().clone())
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

    pub async fn get_rust_object_field_move<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str) -> Result<T> {
        let raw_storage = self.get_field(instance, name, "[B").await?;
        let raw = self.load_byte_array(&raw_storage, 0, self.array_length(&raw_storage).await?).await?;

        let rust_raw = usize::from_le_bytes(cast_slice(&raw).try_into().unwrap());
        let rust = unsafe { Box::from_raw(rust_raw as *mut T) };

        // delete old java data
        let new_raw = self.instantiate_array("B", 0).await?;
        self.put_field(instance, name, "[B", new_raw).await?;

        Ok(*rust)
    }

    pub async fn put_rust_object_field<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str, value: T) -> Result<()> {
        let rust_class_raw = Box::into_raw(Box::new(value)) as *const u8 as usize;

        let mut raw_storage = self.instantiate_array("B", size_of_val(&rust_class_raw)).await?;
        self.store_byte_array(&mut raw_storage, 0, cast_slice(&rust_class_raw.to_le_bytes()).to_vec())
            .await?;

        self.put_field(instance, name, "[B", raw_storage).await?;

        Ok(())
    }

    fn find_field(&self, class: &dyn ClassDefinition, name: &str, descriptor: &str) -> Result<Option<Box<dyn Field>>> {
        let field = class.field(name, descriptor, false);

        if let Some(x) = field {
            Ok(Some(x))
        } else if let Some(x) = class.super_class_name() {
            let super_class = self.classes.borrow().get(&x).unwrap().definition.clone();
            self.find_field(&*super_class, name, descriptor)
        } else {
            Ok(None)
        }
    }

    #[async_recursion::async_recursion(?Send)]
    async fn find_virtual_method(&self, class: &dyn ClassDefinition, name: &str, descriptor: &str, is_static: bool) -> Result<Box<dyn Method>> {
        let method = class.method(name, descriptor);

        if let Some(x) = method {
            if x.access_flags().contains(MethodAccessFlags::STATIC) == is_static {
                return Ok(x);
            }
        } else if let Some(x) = class.super_class_name() {
            let super_class = self.classes.borrow().get(&x).unwrap().definition.clone();
            return self.find_virtual_method(&*super_class, name, descriptor, is_static).await;
        }

        tracing::error!("No such method: {}.{}:{}", class.name(), name, descriptor);

        Err(self
            .exception("java/lang/NoSuchMethodError", &format!("{}.{}:{}", class.name(), name, descriptor))
            .await)
    }
}
