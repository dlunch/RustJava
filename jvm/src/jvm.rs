#![allow(clippy::borrowed_box)] // We have get parameter by Box<T> to make ergonomic interface

use alloc::{
    boxed::Box,
    format,
    vec::{self, Vec},
};
use core::{array, fmt::Debug, iter};

use anyhow::Context;
use dyn_clone::clone_box;

use crate::{
    class::Class,
    class_instance::ClassInstance,
    detail::JvmDetail,
    field::Field,
    platform::Platform,
    thread::{ThreadContext, ThreadId},
    value::JavaValue,
    JvmResult,
};

pub struct Jvm {
    platform: Box<dyn Platform>, // normally in java, we access system services via JNI. but we don't have JNI(yet), so we use platform trait
    detail: Box<dyn JvmDetail>,
}

impl Jvm {
    pub fn new<T, U>(detail: T, platform: U) -> Self
    where
        T: JvmDetail + 'static,
        U: Platform + 'static,
    {
        Self {
            detail: Box::new(detail),
            platform: Box::new(platform),
        }
    }

    pub async fn instantiate_class(&mut self, class_name: &str) -> JvmResult<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate {}", class_name);

        let class = self
            .resolve_class(class_name)
            .await?
            .with_context(|| format!("No such class {}", class_name))?;

        let instance = class.instantiate();

        Ok(instance)
    }

    pub async fn new_class<T>(&mut self, class_name: &str, init_descriptor: &str, init_args: T) -> JvmResult<Box<dyn ClassInstance>>
    where
        T: InvokeArg,
    {
        let instance = self.instantiate_class(class_name).await?;

        self.invoke_special(&instance, class_name, "<init>", init_descriptor, init_args).await?;

        Ok(instance)
    }

    pub async fn instantiate_array(&mut self, element_type_name: &str, length: usize) -> JvmResult<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate array of {} with length {}", element_type_name, length);

        let array_class = self.detail.load_array_class(element_type_name).await?.unwrap();

        let instance = array_class.instantiate_array(length);

        Ok(instance)
    }

    pub async fn get_static_field<T>(&mut self, class_name: &str, name: &str, descriptor: &str) -> JvmResult<T>
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

    pub async fn put_static_field<T>(&mut self, class_name: &str, name: &str, descriptor: &str, value: T) -> JvmResult<()>
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

        let field = Self::find_field(&*instance.class(), name, descriptor)?
            .with_context(|| format!("No such field {}.{}:{}", instance.class().name(), name, descriptor))?;

        Ok(instance.get_field(&*field)?.into())
    }

    pub fn put_field<T>(&mut self, instance: &mut Box<dyn ClassInstance>, name: &str, descriptor: &str, value: T) -> JvmResult<()>
    where
        T: Into<JavaValue> + Debug,
    {
        tracing::trace!("Put field {}.{}:{} = {:?}", instance.class().name(), name, descriptor, value);

        let field = Self::find_field(&*instance.class(), name, descriptor)?
            .with_context(|| format!("No such field {}.{}:{}", instance.class().name(), name, descriptor))?;

        instance.put_field(&*field, value.into())
    }

    pub async fn invoke_static<T, U>(&mut self, class_name: &str, name: &str, descriptor: &str, args: T) -> JvmResult<U>
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

        Ok(method.run(self, args.into_arg()).await?.into())
    }

    pub async fn invoke_virtual<T, U>(
        &mut self,
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
        let method = class
            .method(name, descriptor)
            .with_context(|| format!("No such method {}.{}:{}", class_name, name, descriptor))?;

        let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
            .chain(args.into_iter())
            .collect::<Vec<_>>();

        Ok(method.run(self, args.into_boxed_slice()).await?.into())
    }

    // non-virtual
    pub async fn invoke_special<T, U>(
        &mut self,
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

        Ok(method.run(self, args.into_boxed_slice()).await?.into())
    }

    pub fn store_array<T, U>(&mut self, array: &mut Box<dyn ClassInstance>, offset: usize, values: T) -> JvmResult<()>
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

    pub fn store_byte_array(&mut self, array: &mut Box<dyn ClassInstance>, offset: usize, values: Vec<i8>) -> JvmResult<()> {
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

    pub fn current_thread_context(&mut self) -> &mut dyn ThreadContext {
        self.detail.thread_context(Jvm::current_thread_id())
    }

    // temporary until we have working gc
    pub fn destroy(&mut self, instance: Box<dyn ClassInstance>) -> JvmResult<()> {
        tracing::debug!("Destroy {}", instance.class().name());

        instance.destroy();

        Ok(())
    }

    pub fn platform(&self) -> &dyn Platform {
        &*self.platform
    }

    fn get_class(&self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        self.detail.get_class(class_name)
    }

    async fn resolve_class(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        let class = self.get_class(class_name)?;
        if let Some(x) = class {
            return Ok(Some(x));
        }

        if let Some(x) = self.detail.load_class(class_name).await? {
            tracing::debug!("Loaded class {}", class_name);

            let clinit = x.method("<clinit>", "()V");
            drop(class);

            if let Some(x) = clinit {
                tracing::debug!("Calling <clinit> for {}", class_name);

                x.run(self, Box::new([])).await?;
            }

            let class = self.get_class(class_name)?;

            return Ok(class);
        }

        Ok(None)
    }

    fn find_field(class: &dyn Class, name: &str, descriptor: &str) -> JvmResult<Option<Box<dyn Field>>> {
        let field = class.field(name, descriptor, false);

        if let Some(x) = field {
            Ok(Some(x))
        } else if let Some(x) = class.super_class() {
            Self::find_field(&*x, name, descriptor)
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
