use alloc::{boxed::Box, string::String, vec::Vec};

use jvm::{JavaChar, JavaValue, Jvm};

use crate::{
    method::{MethodBody, MethodImpl, TypeConverter},
    platform::Platform,
};

pub struct JavaClassProto {
    pub parent_class: Option<&'static str>,
    pub interfaces: Vec<&'static str>,
    pub methods: Vec<JavaMethodProto>,
    pub fields: Vec<JavaFieldProto>,
}

pub type JavaError = anyhow::Error;
pub type JavaResult<T> = anyhow::Result<T>;

#[derive(Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum JavaMethodFlag {
    // TODO move to jvm
    NONE,
    STATIC = 0x8,
    NATIVE = 0x100,
}

#[derive(Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum JavaFieldAccessFlag {
    // TODO move to jvm
    NONE,
    STATIC = 0x8,
}

pub struct JavaMethodProto {
    pub name: String,
    pub descriptor: String,
    pub body: JavaMethodBody,
    pub flag: JavaMethodFlag,
}

pub struct JavaFieldProto {
    pub name: String,
    pub descriptor: String,
    pub access_flag: JavaFieldAccessFlag,
}

impl JavaFieldProto {
    pub fn new(name: &str, descriptor: &str, access_flag: JavaFieldAccessFlag) -> Self {
        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            access_flag,
        }
    }
}

pub type JavaMethodBody = Box<dyn MethodBody<JavaError>>;

impl JavaMethodProto {
    pub fn new<M, F, R, P>(name: &str, descriptor: &str, method: M, flag: JavaMethodFlag) -> Self
    where
        M: MethodImpl<F, R, JavaError, P>,
    {
        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            body: method.into_body(),
            flag,
        }
    }

    pub fn new_abstract(name: &str, descriptor: &str, flag: JavaMethodFlag) -> Self {
        struct AbstractCall {
            name: String,
            descriptor: String,
        }

        #[async_trait::async_trait(?Send)]
        impl MethodBody<JavaError> for AbstractCall {
            async fn call(&self, _: &mut dyn JavaContext, _: Box<[JavaValue]>) -> Result<JavaValue, JavaError> {
                // TODO throw java.lang.AbstractMethodError
                anyhow::bail!("Call to abstract function {}{}", self.name, self.descriptor)
            }
        }

        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            body: Box::new(AbstractCall {
                name: name.into(),
                descriptor: descriptor.into(),
            }),
            flag,
        }
    }
}

pub trait JavaContext {
    fn jvm(&mut self) -> &mut Jvm;
    fn platform(&mut self) -> &mut dyn Platform;
}

impl TypeConverter<i8> for i8 {
    fn to_rust(_: &mut dyn JavaContext, raw: JavaValue) -> i8 {
        raw.into()
    }

    fn from_rust(_: &mut dyn JavaContext, rust: i8) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<i32> for i32 {
    fn to_rust(_: &mut dyn JavaContext, raw: JavaValue) -> i32 {
        raw.into()
    }

    fn from_rust(_: &mut dyn JavaContext, rust: i32) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<JavaChar> for JavaChar {
    fn to_rust(_: &mut dyn JavaContext, raw: JavaValue) -> JavaChar {
        raw.into()
    }

    fn from_rust(_: &mut dyn JavaContext, rust: JavaChar) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<i64> for i64 {
    fn to_rust(_: &mut dyn JavaContext, raw: JavaValue) -> i64 {
        raw.into()
    }

    fn from_rust(_: &mut dyn JavaContext, rust: i64) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<bool> for bool {
    fn to_rust(_: &mut dyn JavaContext, raw: JavaValue) -> bool {
        raw.into()
    }

    fn from_rust(_: &mut dyn JavaContext, rust: bool) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<()> for () {
    fn to_rust(_: &mut dyn JavaContext, _: JavaValue) {}

    fn from_rust(_: &mut dyn JavaContext, _: ()) -> JavaValue {
        JavaValue::Void
    }
}
