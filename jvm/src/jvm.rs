#![allow(clippy::borrowed_box)] // We have get parameter by Box<T> to make ergonomic interface

use alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, format, string::String, vec::Vec};
use core::{
    cell::RefCell,
    fmt::Debug,
    iter,
    mem::{forget, size_of_val},
};

use anyhow::Context;
use bytemuck::cast_slice;
use dyn_clone::clone_box;

use java_constants::MethodAccessFlags;

use crate::{
    array_class_instance::ArrayClassInstance,
    class_definition::ClassDefinition,
    class_instance::ClassInstance,
    detail::JvmDetail,
    field::Field,
    invoke_arg::InvokeArg,
    method::Method,
    r#type::JavaType,
    runtime::{JavaLangClass, JavaLangClassLoader},
    value::JavaValue,
    JvmResult,
};

#[derive(Clone)]
pub struct Class {
    pub definition: Box<dyn ClassDefinition>,
    java_class: Option<Box<dyn ClassInstance>>,
}

impl Class {
    pub async fn java_class(&mut self, jvm: &Jvm) -> JvmResult<Box<dyn ClassInstance>> {
        if let Some(x) = &self.java_class {
            Ok((*x).clone())
        } else {
            // class registered while bootstrapping might not have java/lang/Class, so instantiate it lazily

            let class_loader = jvm.get_system_class_loader().await?;
            let java_class = JavaLangClass::from_rust_class(jvm, self.definition.clone(), Some(class_loader)).await?;

            self.java_class = Some(java_class.clone());

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
    pub async fn new<T>(detail: T) -> JvmResult<Self>
    where
        T: JvmDetail + 'static,
    {
        Ok(Self {
            classes: RefCell::new(BTreeMap::new()),
            system_class_loader: RefCell::new(None),
            detail: RefCell::new(Box::new(detail)),
        })
    }

    pub async fn instantiate_class(&self, class_name: &str) -> JvmResult<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate {}", class_name);

        let class = self
            .resolve_class(class_name)
            .await?
            .with_context(|| format!("No such class {}", class_name))?;

        let instance = class.definition.instantiate();

        Ok(instance)
    }

    pub async fn new_class<T>(&self, class_name: &str, init_descriptor: &str, init_args: T) -> JvmResult<Box<dyn ClassInstance>>
    where
        T: InvokeArg,
    {
        let instance = self.instantiate_class(class_name).await?;

        self.invoke_special(&instance, class_name, "<init>", init_descriptor, init_args).await?;

        Ok(instance)
    }

    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn instantiate_array(&self, element_type_name: &str, length: usize) -> JvmResult<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate array of {} with length {}", element_type_name, length);

        let class_name = format!("[{}", element_type_name);
        let class = if self.system_class_loader.borrow().is_none() || element_type_name.len() == 1 {
            if self.has_class(&class_name) {
                self.resolve_class(&class_name).await?.unwrap().definition
            } else {
                // bootstrapping or primitive type
                let definition = self.detail.borrow().define_array_class(self, element_type_name).await?;
                self.register_class_internal(definition.clone(), None).await?;

                definition
            }
        } else {
            self.resolve_class(&class_name).await?.unwrap().definition
        };
        let array_class = class.as_array_class_definition().unwrap();

        let instance = array_class.instantiate_array(length);
        Ok(instance)
    }

    pub async fn get_static_field<T>(&self, class_name: &str, name: &str, descriptor: &str) -> JvmResult<T>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Get static field {}.{}:{}", class_name, name, descriptor);

        let class = self
            .resolve_class(class_name)
            .await?
            .with_context(|| format!("No such class {}", class_name))?;

        let field = class
            .definition
            .field(name, descriptor, true)
            .with_context(|| format!("No such field {}.{}:{}", class_name, name, descriptor))?;

        Ok(class.definition.get_static_field(&*field)?.into())
    }

    pub async fn put_static_field<T>(&self, class_name: &str, name: &str, descriptor: &str, value: T) -> JvmResult<()>
    where
        T: Into<JavaValue> + Debug,
    {
        tracing::trace!("Put static field {}.{}:{} = {:?}", class_name, name, descriptor, value);

        let mut class = self
            .resolve_class(class_name)
            .await?
            .with_context(|| format!("No such class {}", class_name))?;

        let field = class
            .definition
            .field(name, descriptor, true)
            .with_context(|| format!("No such field {}.{}:{}", class_name, name, descriptor))?;

        class.definition.put_static_field(&*field, value.into())
    }

    pub fn get_field<T>(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str) -> JvmResult<T>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Get field {}.{}:{}", instance.class_definition().name(), name, descriptor);

        let field = self
            .find_field(&*instance.class_definition(), name, descriptor)?
            .with_context(|| format!("No such field {}.{}:{}", instance.class_definition().name(), name, descriptor))?;

        Ok(instance.get_field(&*field)?.into())
    }

    pub fn put_field<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str, descriptor: &str, value: T) -> JvmResult<()>
    where
        T: Into<JavaValue> + Debug,
    {
        tracing::trace!("Put field {}.{}:{} = {:?}", instance.class_definition().name(), name, descriptor, value);

        let field = self
            .find_field(&*instance.class_definition(), name, descriptor)?
            .with_context(|| format!("No such field {}.{}:{}", instance.class_definition().name(), name, descriptor))?;

        instance.put_field(&*field, value.into())
    }

    pub async fn invoke_static<T, U>(&self, class_name: &str, name: &str, descriptor: &str, args: T) -> JvmResult<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        tracing::trace!("Invoke static {}.{}:{}", class_name, name, descriptor);

        let class = self
            .resolve_class(class_name)
            .await?
            .with_context(|| format!("No such class {}", class_name))?;

        let method = class
            .definition
            .method(name, descriptor)
            .with_context(|| format!("No such method {}.{}:{}", class_name, name, descriptor))?;

        anyhow::ensure!(method.access_flags().contains(MethodAccessFlags::STATIC), "Method is not static");

        Ok(method.run(self, args.into_arg()).await?.into())
    }

    pub async fn invoke_virtual<T, U>(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str, args: T) -> JvmResult<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        tracing::trace!("Invoke virtual {}.{}:{}", instance.class_definition().name(), name, descriptor);

        let class = instance.class_definition();
        let method = self
            .find_virtual_method(&*class, name, descriptor)?
            .with_context(|| format!("No such method {}.{}:{}", instance.class_definition().name(), name, descriptor))?;

        let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
            .chain(args.into_iter())
            .collect::<Vec<_>>();

        anyhow::ensure!(!method.access_flags().contains(MethodAccessFlags::STATIC), "Method is static");

        Ok(method.run(self, args.into_boxed_slice()).await?.into())
    }

    // non-virtual
    pub async fn invoke_special<T, U>(
        &self,
        instance: &Box<dyn ClassInstance>,
        class_name: &str,
        name: &str,
        descriptor: &str,
        args: T,
    ) -> JvmResult<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        tracing::trace!("Invoke special {}.{}:{}", class_name, name, descriptor);

        let class = self.resolve_class(class_name).await?.unwrap();
        let method = class
            .definition
            .method(name, descriptor)
            .with_context(|| format!("No such method {}.{}:{}", class_name, name, descriptor))?;

        let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
            .chain(args.into_iter())
            .collect::<Vec<_>>();

        anyhow::ensure!(!method.access_flags().contains(MethodAccessFlags::STATIC), "Method is static");

        Ok(method.run(self, args.into_boxed_slice()).await?.into())
    }

    pub fn store_array<T, U>(&self, array: &mut Box<dyn ClassInstance>, offset: usize, values: T) -> JvmResult<()>
    where
        T: IntoIterator<Item = U>,
        U: Into<JavaValue>,
    {
        tracing::trace!("Store array {} at offset {}", array.class_definition().name(), offset);

        let array = array.as_array_instance_mut().context("Expected array class instance")?;

        let values = values.into_iter().map(|x| x.into()).collect::<Vec<_>>();
        array.store(offset, values.into_boxed_slice())
    }

    pub fn load_array<T>(&self, array: &Box<dyn ClassInstance>, offset: usize, count: usize) -> JvmResult<Vec<T>>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Load array {} at offset {}", array.class_definition().name(), offset);

        let array = array.as_array_instance().context("Expected array class instance")?;

        let values = array.load(offset, count)?;

        Ok(iter::IntoIterator::into_iter(values).map(|x| x.into()).collect::<Vec<_>>())
    }

    pub fn store_byte_array(&self, array: &mut Box<dyn ClassInstance>, offset: usize, values: Vec<i8>) -> JvmResult<()> {
        tracing::trace!("Store array {} at offset {}", array.class_definition().name(), offset);

        let array = array.as_array_instance_mut().context("Expected array class instance")?;

        array.store_bytes(offset, values.into_boxed_slice())
    }

    pub fn load_byte_array(&self, array: &Box<dyn ClassInstance>, offset: usize, count: usize) -> JvmResult<Vec<i8>> {
        tracing::trace!("Load array {} at offset {}", array.class_definition().name(), offset);

        let array = array.as_array_instance().context("Expected array class instance")?;

        let values = array.load_bytes(offset, count)?;

        Ok(values)
    }

    pub fn array_length(&self, array: &Box<dyn ClassInstance>) -> JvmResult<usize> {
        tracing::trace!("Get array length {}", array.class_definition().name());

        let array = array.as_array_instance().context("Expected array class instance")?;

        Ok(array.length())
    }

    pub fn array_element_type(&self, array: &Box<dyn ClassInstance>) -> JvmResult<JavaType> {
        tracing::trace!("Get array element type {}", array.class_definition().name());

        let array = array.as_array_instance().context("Expected array class instance")?;
        let class = ArrayClassInstance::class_definition(array);

        let type_name = &class.name()[1..]; // TODO can we store JavaType on class?

        Ok(JavaType::parse(type_name))
    }

    // temporary until we have working gc
    pub fn destroy(&self, instance: Box<dyn ClassInstance>) -> JvmResult<()> {
        tracing::debug!("Destroy {}", instance.class_definition().name());

        instance.destroy();

        Ok(())
    }

    pub fn has_class(&self, class_name: &str) -> bool {
        self.classes.borrow().contains_key(class_name)
    }

    #[async_recursion::async_recursion(?Send)]
    pub async fn resolve_class(&self, class_name: &str) -> JvmResult<Option<Class>> {
        let class = self.classes.borrow().get(class_name).cloned();

        if let Some(x) = class {
            return Ok(Some(x));
        }

        // load class
        let class_loader = self.get_system_class_loader().await?;
        let java_class = JavaLangClassLoader::load_class(self, class_loader, class_name).await?;

        anyhow::ensure!(java_class.is_some(), "Class {} not found", class_name);

        if let Some(x) = &java_class {
            tracing::debug!("Loaded class {}", class_name);

            let class = JavaLangClass::to_rust_class(self, x.clone())?;

            self.register_class_internal(class.clone(), java_class).await?;

            let class = self.classes.borrow().get(class_name).unwrap().clone();

            return Ok(Some(class));
        }

        Ok(None)
    }

    pub async fn register_class(&self, class: Box<dyn ClassDefinition>, class_loader: Option<Box<dyn ClassInstance>>) -> JvmResult<()> {
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

    async fn register_class_internal(&self, class_definition: Box<dyn ClassDefinition>, java_class: Option<Box<dyn ClassInstance>>) -> JvmResult<()> {
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
                java_class,
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
    pub async fn define_class(&self, name: &str, data: &[u8], class_loader: Box<dyn ClassInstance>) -> JvmResult<Box<dyn ClassInstance>> {
        let class = self.detail.borrow().define_class(self, name, data).await?;

        self.register_class(class.clone(), Some(class_loader)).await?;

        self.resolve_class(&class.name()).await?.unwrap().java_class(self).await
    }

    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn define_array_class(&self, element_type_name: &str, class_loader: Box<dyn ClassInstance>) -> JvmResult<Box<dyn ClassInstance>> {
        let class = self.detail.borrow().define_array_class(self, element_type_name).await?;

        self.register_class(class.clone(), Some(class_loader)).await?;

        self.resolve_class(&class.name()).await?.unwrap().java_class(self).await
    }

    pub fn set_system_class_loader(&self, class_loader: Box<dyn ClassInstance>) {
        self.system_class_loader.replace(Some(class_loader)); // TODO we need Thread.setContextClassLoader
    }

    pub async fn get_system_class_loader(&self) -> JvmResult<Box<dyn ClassInstance>> {
        if self.system_class_loader.borrow().is_none() {
            let system_class_loader = JavaLangClassLoader::get_system_class_loader(self).await?;

            self.system_class_loader.replace(Some(system_class_loader));
        }

        Ok(self.system_class_loader.borrow().as_ref().unwrap().clone())
    }

    pub fn get_rust_object_field<T>(&self, instance: &Box<dyn ClassInstance>, name: &str) -> JvmResult<T>
    where
        T: Clone,
    {
        let raw_storage = self.get_field(instance, name, "[B")?;
        let raw = self.load_byte_array(&raw_storage, 0, self.array_length(&raw_storage)?)?;

        let rust_raw = usize::from_le_bytes(cast_slice(&raw).try_into().unwrap());

        let rust = unsafe { Box::from_raw(rust_raw as *mut T) };
        let result = (*rust).clone();

        forget(rust); // do not drop box as we still have it in java memory

        Ok(result)
    }

    pub async fn get_rust_object_field_move<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str) -> JvmResult<T> {
        let raw_storage = self.get_field(instance, name, "[B")?;
        let raw = self.load_byte_array(&raw_storage, 0, self.array_length(&raw_storage)?)?;

        let rust_raw = usize::from_le_bytes(cast_slice(&raw).try_into().unwrap());
        let rust = unsafe { Box::from_raw(rust_raw as *mut T) };

        // delete old java data
        let new_raw = self.instantiate_array("B", 0).await?;
        self.put_field(instance, name, "[B", new_raw).unwrap();

        Ok(*rust)
    }

    pub async fn put_rust_object_field<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str, value: T) -> JvmResult<()> {
        let rust_class_raw = Box::into_raw(Box::new(value)) as *const u8 as usize;

        let mut raw_storage = self.instantiate_array("B", size_of_val(&rust_class_raw)).await?;
        self.store_byte_array(&mut raw_storage, 0, cast_slice(&rust_class_raw.to_le_bytes()).to_vec())?;

        self.put_field(instance, name, "[B", raw_storage)?;

        Ok(())
    }

    fn find_field(&self, class: &dyn ClassDefinition, name: &str, descriptor: &str) -> JvmResult<Option<Box<dyn Field>>> {
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

    fn find_virtual_method(&self, class: &dyn ClassDefinition, name: &str, descriptor: &str) -> JvmResult<Option<Box<dyn Method>>> {
        let method = class.method(name, descriptor);

        if let Some(x) = method {
            Ok(Some(x))
        } else if let Some(x) = class.super_class_name() {
            let super_class = self.classes.borrow().get(&x).unwrap().definition.clone();
            self.find_virtual_method(&*super_class, name, descriptor)
        } else {
            Ok(None)
        }
    }
}
