use alloc::{boxed::Box, format, string::String, vec::Vec};

use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{JavaError, JavaValue, Jvm};

use crate::method::{MethodBody, MethodImpl};

pub struct JavaClassProto<C>
where
    C: ?Sized + Send,
{
    pub name: &'static str,
    pub parent_class: Option<&'static str>,
    pub interfaces: Vec<&'static str>,
    pub methods: Vec<JavaMethodProto<C>>,
    pub fields: Vec<JavaFieldProto>,
}

pub struct JavaFieldProto {
    pub name: String,
    pub descriptor: String,
    pub access_flags: FieldAccessFlags,
}

impl JavaFieldProto {
    pub fn new(name: &str, descriptor: &str, access_flag: FieldAccessFlags) -> Self {
        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            access_flags: access_flag,
        }
    }
}

pub struct JavaMethodProto<C>
where
    C: ?Sized + Send,
{
    pub name: String,
    pub descriptor: String,
    pub body: Box<dyn MethodBody<JavaError, C>>,
    pub access_flags: MethodAccessFlags,
}

impl<C> JavaMethodProto<C>
where
    C: ?Sized + Send,
{
    pub fn new<M, F, R, P>(name: &str, descriptor: &str, method: M, flag: MethodAccessFlags) -> Self
    where
        M: MethodImpl<F, C, R, JavaError, P>,
    {
        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            body: method.into_body(),
            access_flags: flag,
        }
    }

    pub fn new_abstract(name: &str, descriptor: &str, flag: MethodAccessFlags) -> Self {
        struct AbstractCall {
            name: String,
            descriptor: String,
        }

        #[async_trait::async_trait]
        impl<C> MethodBody<JavaError, C> for AbstractCall
        where
            C: ?Sized + Send,
        {
            async fn call(&self, _: &Jvm, _: &mut C, _: Box<[JavaValue]>) -> Result<JavaValue, JavaError> {
                // TODO java.lang.AbstractMethodError
                Err(JavaError::FatalError(format!("Abstract {}{} method called", self.name, self.descriptor)))
            }
        }

        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            body: Box::new(AbstractCall {
                name: name.into(),
                descriptor: descriptor.into(),
            }),
            access_flags: flag,
        }
    }
}
