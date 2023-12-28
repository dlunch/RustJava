use alloc::{
    boxed::Box,
    collections::BTreeMap,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{self, Debug, Formatter},
    future::Future,
    marker::PhantomData,
};

use classfile::{AttributeInfo, MethodInfo, Opcode};
use jvm::{JavaValue, Jvm, JvmResult, Method};

use crate::interpreter::Interpreter;

#[async_trait::async_trait(?Send)]
pub trait RustMethodBody<E, R> {
    async fn call(&self, jvm: &mut Jvm, args: &[JavaValue]) -> Result<R, E>;
}

pub trait FnHelper<'a, E, R> {
    type Output: Future<Output = Result<R, E>> + 'a;
    fn call(&self, jvm: &'a mut Jvm, args: &'a [JavaValue]) -> Self::Output;
}

impl<'a, E, R, F, Fut> FnHelper<'a, E, R> for F
where
    F: Fn(&'a mut Jvm, &'a [JavaValue]) -> Fut,
    Fut: Future<Output = Result<R, E>> + 'a,
{
    type Output = Fut;

    fn call(&self, jvm: &'a mut Jvm, args: &'a [JavaValue]) -> Fut {
        self(jvm, args)
    }
}

struct MethodHolder<F, R>(pub F, PhantomData<R>);

#[async_trait::async_trait(?Send)]
impl<F, R, E> RustMethodBody<E, R> for MethodHolder<F, R>
where
    F: for<'a> FnHelper<'a, E, R>,
{
    async fn call(&self, jvm: &mut Jvm, args: &[JavaValue]) -> Result<R, E> {
        let result = self.0.call(jvm, args).await?;

        Ok(result)
    }
}

pub enum MethodBody {
    ByteCode(BTreeMap<u32, Opcode>),
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

#[derive(Clone, Debug)]
pub struct MethodImpl {
    name: String,
    descriptor: String,
    body: Rc<MethodBody>,
}

impl MethodImpl {
    pub fn new(name: &str, descriptor: &str, body: MethodBody) -> Self {
        Self {
            name: name.to_string(),
            descriptor: descriptor.to_string(),
            body: Rc::new(body),
        }
    }

    pub fn from_methodinfo(method_info: MethodInfo) -> Self {
        Self {
            name: method_info.name.to_string(),
            descriptor: method_info.descriptor.to_string(),
            body: Rc::new(MethodBody::ByteCode(Self::extract_body(method_info.attributes).unwrap())),
        }
    }

    fn extract_body(attributes: Vec<AttributeInfo>) -> Option<BTreeMap<u32, Opcode>> {
        for attribute in attributes {
            if let AttributeInfo::Code(x) = attribute {
                return Some(x.code);
            }
        }

        None
    }
}

#[async_trait::async_trait(?Send)]
impl Method for MethodImpl {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    async fn run(&self, jvm: &mut Jvm, args: &[JavaValue]) -> JvmResult<JavaValue> {
        Ok(match self.body.as_ref() {
            MethodBody::ByteCode(x) => Interpreter::run(jvm, x).await?,
            MethodBody::Rust(x) => x.call(jvm, args).await?,
        })
    }
}
