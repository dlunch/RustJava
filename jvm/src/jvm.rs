#![allow(clippy::borrowed_box)] // We have get parameter by Box<T> to make ergonomic interface

use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::BTreeMap,
    format,
    string::String,
    vec::{self, Vec},
};
use core::{array, cell::RefCell, fmt::Debug, iter, mem::forget};

use anyhow::Context;
use bytemuck::cast_slice;
use dyn_clone::clone_box;

use java_constants::MethodAccessFlags;

use crate::{
    array_class_instance::ArrayClassInstance,
    class::Class,
    class_instance::ClassInstance,
    detail::JvmDetail,
    field::Field,
    method::Method,
    r#type::JavaType,
    thread::{ThreadContext, ThreadId},
    value::JavaValue,
    JavaChar, JvmResult,
};

pub struct Jvm {
    classes: RefCell<BTreeMap<String, Box<dyn Class>>>,
    system_class_loader: RefCell<Option<Box<dyn ClassInstance>>>,
    detail: RefCell<Box<dyn JvmDetail>>,
}

impl Jvm {
    pub async fn new<T>(detail: T) -> JvmResult<Self>
    where
        T: JvmDetail + 'static,
    {
        let primitive_types = ["Z", "B", "C", "S", "I", "J", "F", "D"];
        let mut array_classes = BTreeMap::new();

        for primitive_type in primitive_types {
            let array_class = detail.define_array_class(primitive_type).await?;

            array_classes.insert(array_class.name().to_owned(), array_class);
        }

        Ok(Self {
            classes: RefCell::new(array_classes),
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

        let instance = class.instantiate();

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

    pub async fn instantiate_array(&self, element_type_name: &str, length: usize) -> JvmResult<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate array of {} with length {}", element_type_name, length);

        let array_class_name = format!("[{}", element_type_name);
        let class = self.resolve_class(&array_class_name).await?.unwrap();
        let array_class = class.as_array_class().unwrap();

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
            .field(name, descriptor, true)
            .with_context(|| format!("No such field {}.{}:{}", class_name, name, descriptor))?;

        Ok(class.get_static_field(&*field)?.into())
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
            .field(name, descriptor, true)
            .with_context(|| format!("No such field {}.{}:{}", class_name, name, descriptor))?;

        class.put_static_field(&*field, value.into())
    }

    pub fn get_field<T>(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str) -> JvmResult<T>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Get field {}.{}:{}", instance.class().name(), name, descriptor);

        let field = self
            .find_field(&*instance.class(), name, descriptor)?
            .with_context(|| format!("No such field {}.{}:{}", instance.class().name(), name, descriptor))?;

        Ok(instance.get_field(&*field)?.into())
    }

    pub fn put_field<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str, descriptor: &str, value: T) -> JvmResult<()>
    where
        T: Into<JavaValue> + Debug,
    {
        tracing::trace!("Put field {}.{}:{} = {:?}", instance.class().name(), name, descriptor, value);

        let field = self
            .find_field(&*instance.class(), name, descriptor)?
            .with_context(|| format!("No such field {}.{}:{}", instance.class().name(), name, descriptor))?;

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
            .method(name, descriptor)
            .with_context(|| format!("No such method {}.{}:{}", class_name, name, descriptor))?;

        anyhow::ensure!(method.access_flags().contains(MethodAccessFlags::STATIC), "Method is not static");

        Ok(method.run(self, args.into_arg()).await?.into())
    }

    pub async fn invoke_virtual<T, U>(
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
        tracing::trace!("Invoke virtual {}.{}:{}", class_name, name, descriptor);

        let class = self.resolve_class(&instance.class().name()).await?.unwrap();
        let method = self
            .find_virtual_method(&*class, name, descriptor)?
            .with_context(|| format!("No such method {}.{}:{}", class_name, name, descriptor))?;

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
        tracing::trace!("Store array {} at offset {}", array.class().name(), offset);

        let array = array.as_array_instance_mut().context("Expected array class instance")?;

        let values = values.into_iter().map(|x| x.into()).collect::<Vec<_>>();
        array.store(offset, values.into_boxed_slice())
    }

    pub fn load_array<T>(&self, array: &Box<dyn ClassInstance>, offset: usize, count: usize) -> JvmResult<Vec<T>>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Load array {} at offset {}", array.class().name(), offset);

        let array = array.as_array_instance().context("Expected array class instance")?;

        let values = array.load(offset, count)?;

        Ok(iter::IntoIterator::into_iter(values).map(|x| x.into()).collect::<Vec<_>>())
    }

    pub fn store_byte_array(&self, array: &mut Box<dyn ClassInstance>, offset: usize, values: Vec<i8>) -> JvmResult<()> {
        tracing::trace!("Store array {} at offset {}", array.class().name(), offset);

        let array = array.as_array_instance_mut().context("Expected array class instance")?;

        array.store_bytes(offset, values.into_boxed_slice())
    }

    pub fn load_byte_array(&self, array: &Box<dyn ClassInstance>, offset: usize, count: usize) -> JvmResult<Vec<i8>> {
        tracing::trace!("Load array {} at offset {}", array.class().name(), offset);

        let array = array.as_array_instance().context("Expected array class instance")?;

        let values = array.load_bytes(offset, count)?;

        Ok(values)
    }

    pub fn array_length(&self, array: &Box<dyn ClassInstance>) -> JvmResult<usize> {
        tracing::trace!("Get array length {}", array.class().name());

        let array = array.as_array_instance().context("Expected array class instance")?;

        Ok(array.length())
    }

    pub fn array_element_type(&self, array: &Box<dyn ClassInstance>) -> JvmResult<JavaType> {
        tracing::trace!("Get array element type {}", array.class().name());

        let array = array.as_array_instance().context("Expected array class instance")?;
        let class = ArrayClassInstance::class(array);

        let type_name = &class.name()[1..]; // TODO can we store JavaType on class?

        Ok(JavaType::parse(type_name))
    }

    pub fn current_thread_context(&self) -> Box<dyn ThreadContext> {
        self.detail.borrow_mut().thread_context(Jvm::current_thread_id())
    }

    // temporary until we have working gc
    pub fn destroy(&self, instance: Box<dyn ClassInstance>) -> JvmResult<()> {
        tracing::debug!("Destroy {}", instance.class().name());

        instance.destroy();

        Ok(())
    }

    pub fn get_class(&self, class_name: &str) -> Option<Box<dyn Class>> {
        self.classes.borrow().get(class_name).cloned()
    }

    #[async_recursion::async_recursion(?Send)]
    pub async fn resolve_class(&self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        let class = self.get_class(class_name);
        if let Some(x) = class {
            return Ok(Some(x));
        }

        if let Some(x) = self.load_class(class_name).await? {
            tracing::debug!("Loaded class {}", class_name);

            if let Some(super_class) = x.super_class_name() {
                self.resolve_class(&super_class).await?;
            }

            self.register_class(x.clone()).await?;
            let class = self.get_class(class_name);

            return Ok(class);
        }

        Ok(None)
    }

    pub async fn register_class(&self, class: Box<dyn Class>) -> JvmResult<()> {
        tracing::debug!("Register class {}", class.name());

        self.classes.borrow_mut().insert(class.name().to_owned(), class.clone());
        self.init_class(&*class).await?;

        Ok(())
    }

    async fn init_class(&self, class: &dyn Class) -> JvmResult<()> {
        let clinit = class.method("<clinit>", "()V");

        if let Some(x) = clinit {
            tracing::debug!("Calling <clinit> for {}", class.name());

            x.run(self, Box::new([])).await?;
        }

        Ok(())
    }

    async fn load_class(&self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        let mut class_name_array = self.instantiate_array("C", class_name.len()).await?;
        self.store_array(&mut class_name_array, 0, class_name.chars().map(|x| x as JavaChar))?;

        let class_name_string = self.new_class("java/lang/String", "([C)V", (class_name_array,)).await?;

        let class_loader = self.get_system_class_loader().await?;
        let java_class = self
            .invoke_virtual(
                &class_loader,
                "java/lang/ClassLoader",
                "loadClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                (class_name_string,),
            )
            .await?;

        let rust_class = self.to_rust_class(java_class)?;

        Ok(Some(rust_class))
    }

    pub async fn define_class(&self, name: &str, data: &[u8]) -> JvmResult<Box<dyn Class>> {
        self.detail.borrow().define_class(name, data).await
    }

    pub async fn define_array_class(&self, element_type_name: &str) -> JvmResult<Box<dyn Class>> {
        self.detail.borrow().define_array_class(element_type_name).await
    }

    pub fn set_system_class_loader(&self, class_loader: Box<dyn ClassInstance>) {
        self.system_class_loader.replace(Some(class_loader)); // TODO we need Thread.setContextClassLoader
    }

    pub async fn get_system_class_loader(&self) -> JvmResult<Box<dyn ClassInstance>> {
        if self.system_class_loader.borrow().is_none() {
            let system_class_loader = self
                .invoke_static("java/lang/ClassLoader", "getSystemClassLoader", "()Ljava/lang/ClassLoader;", ())
                .await?;

            self.system_class_loader.replace(Some(system_class_loader));
        }

        Ok(self.system_class_loader.borrow().as_ref().unwrap().clone())
    }

    // TODO we have same logic on java/lang/Class
    fn to_rust_class(&self, java_class: Box<dyn ClassInstance>) -> JvmResult<Box<dyn Class>> {
        let raw_storage = self.get_field(&java_class, "raw", "[B")?;
        let raw = self.load_byte_array(&raw_storage, 0, self.array_length(&raw_storage)?)?;

        let rust_class_raw = usize::from_le_bytes(cast_slice(&raw).try_into().unwrap());

        let rust_class = unsafe { Box::from_raw(rust_class_raw as *mut Box<dyn Class>) };
        let result = (*rust_class).clone();

        forget(rust_class); // do not drop box as we still have it in java memory

        Ok(result)
    }

    fn find_field(&self, class: &dyn Class, name: &str, descriptor: &str) -> JvmResult<Option<Box<dyn Field>>> {
        let field = class.field(name, descriptor, false);

        if let Some(x) = field {
            Ok(Some(x))
        } else if let Some(x) = class.super_class_name() {
            let super_class = self.get_class(&x).unwrap();
            self.find_field(&*super_class, name, descriptor)
        } else {
            Ok(None)
        }
    }

    fn find_virtual_method(&self, class: &dyn Class, name: &str, descriptor: &str) -> JvmResult<Option<Box<dyn Method>>> {
        let method = class.method(name, descriptor);

        if let Some(x) = method {
            Ok(Some(x))
        } else if let Some(x) = class.super_class_name() {
            let super_class = self.get_class(&x).unwrap();
            self.find_virtual_method(&*super_class, name, descriptor)
        } else {
            Ok(None)
        }
    }

    fn current_thread_id() -> ThreadId {
        0 // TODO
    }
}

pub trait InvokeArg {
    type IntoIter: Iterator<Item = JavaValue>;

    fn into_arg(self) -> Box<[JavaValue]>;
    fn into_iter(self) -> Self::IntoIter;
}

impl InvokeArg for Vec<JavaValue> {
    type IntoIter = vec::IntoIter<JavaValue>;
    fn into_arg(self) -> Box<[JavaValue]> {
        self.into_boxed_slice()
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter(self)
    }
}

impl<const N: usize> InvokeArg for [JavaValue; N] {
    type IntoIter = array::IntoIter<JavaValue, N>;

    fn into_arg(self) -> Box<[JavaValue]> {
        self.into()
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter(self)
    }
}

impl InvokeArg for () {
    type IntoIter = array::IntoIter<JavaValue, 0>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([])
    }
}

impl<T1> InvokeArg for (T1,)
where
    T1: Into<JavaValue>,
{
    type IntoIter = array::IntoIter<JavaValue, 1>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into()])
    }
}

impl<T1, T2> InvokeArg for (T1, T2)
where
    T1: Into<JavaValue>,
    T2: Into<JavaValue>,
{
    type IntoIter = array::IntoIter<JavaValue, 2>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into()])
    }
}

impl<T1, T2, T3> InvokeArg for (T1, T2, T3)
where
    T1: Into<JavaValue>,
    T2: Into<JavaValue>,
    T3: Into<JavaValue>,
{
    type IntoIter = array::IntoIter<JavaValue, 3>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into(), self.2.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into()])
    }
}

impl<T1, T2, T3, T4> InvokeArg for (T1, T2, T3, T4)
where
    T1: Into<JavaValue>,
    T2: Into<JavaValue>,
    T3: Into<JavaValue>,
    T4: Into<JavaValue>,
{
    type IntoIter = array::IntoIter<JavaValue, 4>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into(), self.2.into(), self.3.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into(), self.3.into()])
    }
}

impl<T1, T2, T3, T4, T5> InvokeArg for (T1, T2, T3, T4, T5)
where
    T1: Into<JavaValue>,
    T2: Into<JavaValue>,
    T3: Into<JavaValue>,
    T4: Into<JavaValue>,
    T5: Into<JavaValue>,
{
    type IntoIter = array::IntoIter<JavaValue, 5>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into()])
    }
}

impl<T1, T2, T3, T4, T5, T6> InvokeArg for (T1, T2, T3, T4, T5, T6)
where
    T1: Into<JavaValue>,
    T2: Into<JavaValue>,
    T3: Into<JavaValue>,
    T4: Into<JavaValue>,
    T5: Into<JavaValue>,
    T6: Into<JavaValue>,
{
    type IntoIter = array::IntoIter<JavaValue, 6>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into()])
    }
}
