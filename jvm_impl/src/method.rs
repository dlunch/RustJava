use alloc::{
    boxed::Box,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{self, Debug, Formatter},
    future::Future,
    marker::PhantomData,
};

use classfile::{AttributeInfo, AttributeInfoCode, MethodInfo};
use jvm::{JavaValue, Jvm, JvmResult, Method};

use crate::interpreter::Interpreter;

#[async_trait::async_trait(?Send)]
pub trait RustMethodBody<E, R> {
    async fn call(&self, jvm: &mut Jvm, args: Box<[JavaValue]>) -> Result<R, E>;
}

pub trait FnHelper<'a, E, R> {
    type Output: Future<Output = Result<R, E>> + 'a;
    fn call(&self, jvm: &'a mut Jvm, args: Box<[JavaValue]>) -> Self::Output;
}

impl<'a, E, R, F, Fut> FnHelper<'a, E, R> for F
where
    F: Fn(&'a mut Jvm, Box<[JavaValue]>) -> Fut,
    Fut: Future<Output = Result<R, E>> + 'a,
{
    type Output = Fut;

    fn call(&self, jvm: &'a mut Jvm, args: Box<[JavaValue]>) -> Fut {
        self(jvm, args)
    }
}

struct MethodHolder<F, R>(pub F, PhantomData<R>);

#[async_trait::async_trait(?Send)]
impl<F, R, E> RustMethodBody<E, R> for MethodHolder<F, R>
where
    F: for<'a> FnHelper<'a, E, R>,
{
    async fn call(&self, jvm: &mut Jvm, args: Box<[JavaValue]>) -> Result<R, E> {
        let result = self.0.call(jvm, args).await?;

        Ok(result)
    }
}

pub enum MethodBody {
    ByteCode(AttributeInfoCode),
    Rust(Box<dyn RustMethodBody<anyhow::Error, JavaValue>>),
}

impl MethodBody {
    pub fn from_rust<F>(f: F) -> Self
    where
        F: for<'a> FnHelper<'a, anyhow::Error, JavaValue> + 'static,
    {
        Self::Rust(Box::new(MethodHolder(f, PhantomData)))
    }
}

impl Debug for MethodBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MethodBody::ByteCode(_) => write!(f, "ByteCode"),
            MethodBody::Rust(_) => write!(f, "Rust"),
        }
    }
}

#[derive(Debug)]
struct MethodInner {
    name: String,
    descriptor: String,
    body: MethodBody,
}

#[derive(Clone, Debug)]
pub struct MethodImpl {
    inner: Rc<MethodInner>,
}

impl MethodImpl {
    pub fn new(name: &str, descriptor: &str, body: MethodBody) -> Self {
        Self {
            inner: Rc::new(MethodInner {
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                body,
            }),
        }
    }

    pub fn from_methodinfo(method_info: MethodInfo) -> Self {
        Self {
            inner: Rc::new(MethodInner {
                name: method_info.name.to_string(),
                descriptor: method_info.descriptor.to_string(),
                body: MethodBody::ByteCode(Self::extract_body(method_info.attributes).unwrap()),
            }),
        }
    }

    fn extract_body(attributes: Vec<AttributeInfo>) -> Option<AttributeInfoCode> {
        for attribute in attributes {
            if let AttributeInfo::Code(x) = attribute {
                return Some(x);
            }
        }

        None
    }
}

#[async_trait::async_trait(?Send)]
impl Method for MethodImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn descriptor(&self) -> String {
        self.inner.descriptor.clone()
    }

    async fn run(&self, jvm: &mut Jvm, args: Box<[JavaValue]>) -> JvmResult<JavaValue> {
        Ok(match &self.inner.body {
            MethodBody::ByteCode(x) => Interpreter::run(jvm, x, args).await?,
            MethodBody::Rust(x) => x.call(jvm, args).await?,
        })
    }
}
