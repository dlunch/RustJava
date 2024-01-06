use alloc::{
    boxed::Box,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::{self, Debug, Formatter};

use classfile::{AttributeInfo, AttributeInfoCode, MethodInfo};
use java_runtime_base::JavaMethodProto;
use jvm::{JavaValue, Jvm, JvmCallback, JvmResult, Method};

use crate::interpreter::Interpreter;

pub enum MethodBody {
    ByteCode(AttributeInfoCode),
    Rust(Box<dyn JvmCallback>),
}

impl MethodBody {
    pub fn from_rust(callback: Box<dyn JvmCallback>) -> Self {
        Self::Rust(callback)
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

    pub fn from_method_proto<C>(proto: JavaMethodProto<C>, context: C) -> Self
    where
        C: 'static,
    {
        struct MethodProxy<C> {
            body: Box<dyn java_runtime_base::MethodBody<anyhow::Error, C>>,
            context: C,
        }

        #[async_trait::async_trait(?Send)]
        impl<C> JvmCallback for MethodProxy<C> {
            async fn call(&self, jvm: &mut Jvm, args: Box<[JavaValue]>) -> JvmResult<JavaValue> {
                self.body.call(jvm, &self.context, args).await
            }
        }

        Self::new(
            &proto.name,
            &proto.descriptor,
            MethodBody::Rust(Box::new(MethodProxy { body: proto.body, context })),
        )
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
