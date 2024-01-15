use alloc::{boxed::Box, string::String, vec::Vec};

use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, JavaChar, JavaValue, Jvm};

use crate::method::{MethodBody, MethodImpl, TypeConverter};

pub struct JavaClassProto<C>
where
    C: ?Sized,
{
    pub parent_class: Option<&'static str>,
    pub interfaces: Vec<&'static str>,
    pub methods: Vec<JavaMethodProto<C>>,
    pub fields: Vec<JavaFieldProto>,
}

pub type JavaError = anyhow::Error;
pub type JavaResult<T> = anyhow::Result<T>;

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
    C: ?Sized,
{
    pub name: String,
    pub descriptor: String,
    pub body: Box<dyn MethodBody<anyhow::Error, C>>,
    pub access_flags: MethodAccessFlags,
}

impl<C> JavaMethodProto<C>
where
    C: ?Sized,
{
    pub fn new<M, F, R, P>(name: &str, descriptor: &str, method: M, flag: MethodAccessFlags) -> Self
    where
        M: MethodImpl<F, C, R, anyhow::Error, P>,
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

        #[async_trait::async_trait(?Send)]
        impl<C> MethodBody<anyhow::Error, C> for AbstractCall
        where
            C: ?Sized,
        {
            async fn call(&self, _: &Jvm, _: &mut C, _: Box<[JavaValue]>) -> Result<JavaValue, JavaError> {
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
            access_flags: flag,
        }
    }
}

impl TypeConverter<i8> for i8 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> i8 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: i8) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<i16> for i16 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> i16 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: i16) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<i32> for i32 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> i32 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: i32) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<JavaChar> for JavaChar {
    fn to_rust(_: &Jvm, raw: JavaValue) -> JavaChar {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: JavaChar) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<i64> for i64 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> i64 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: i64) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<bool> for bool {
    fn to_rust(_: &Jvm, raw: JavaValue) -> bool {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: bool) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<()> for () {
    fn to_rust(_: &Jvm, _: JavaValue) {}

    fn from_rust(_: &Jvm, _: ()) -> JavaValue {
        JavaValue::Void
    }
}

impl<T> TypeConverter<ClassInstanceRef<T>> for ClassInstanceRef<T> {
    fn to_rust(_: &Jvm, raw: JavaValue) -> Self {
        Self::new(raw.into())
    }

    fn from_rust(_: &Jvm, value: Self) -> JavaValue {
        value.instance.into()
    }
}
