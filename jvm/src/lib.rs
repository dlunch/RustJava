#![no_std]
extern crate alloc;

mod array_class_definition;
mod array_class_instance;
mod as_any;
mod class_definition;
mod class_instance;
mod detail;
mod field;
mod jvm;
mod method;
mod runtime;
mod thread;
mod r#type;
mod value;

pub type JvmResult<T> = anyhow::Result<T>;

use alloc::boxed::Box;
#[async_trait::async_trait(?Send)]
pub trait JvmCallback {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> anyhow::Result<JavaValue>;
}

pub use self::{
    array_class_definition::ArrayClassDefinition,
    array_class_instance::ArrayClassInstance,
    class_definition::ClassDefinition,
    class_instance::{Array, ClassInstance, ClassInstanceRef},
    detail::JvmDetail,
    field::Field,
    jvm::Jvm,
    method::Method,
    r#type::JavaType,
    thread::{ThreadContext, ThreadId},
    value::{JavaChar, JavaValue},
};
