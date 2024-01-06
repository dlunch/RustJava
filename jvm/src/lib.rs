#![no_std]
extern crate alloc;

mod array_class;
mod array_class_instance;
mod as_any;
mod class;
mod class_instance;
mod detail;
mod field;
mod jvm;
mod method;
mod platform;
mod thread;
mod r#type;
mod value;

pub mod runtime;

pub type JvmResult<T> = anyhow::Result<T>;

use alloc::boxed::Box;
#[async_trait::async_trait(?Send)]
pub trait JvmCallback {
    async fn call(&self, jvm: &mut Jvm, args: Box<[JavaValue]>) -> anyhow::Result<JavaValue>;
}

pub use self::{
    array_class::ArrayClass,
    array_class_instance::ArrayClassInstance,
    class::Class,
    class_instance::ClassInstance,
    detail::JvmDetail,
    field::Field,
    jvm::Jvm,
    method::Method,
    platform::Platform,
    r#type::JavaType,
    thread::{ThreadContext, ThreadId},
    value::{JavaChar, JavaValue},
};
