#![no_std]
extern crate alloc;

mod array_class_definition;
mod array_class_instance;
mod as_any;
mod class_definition;
mod class_instance;
mod detail;
mod error;
mod field;
mod invoke_arg;
mod jvm;
mod method;
mod r#type;
mod value;

pub mod runtime;

pub type JvmResult<T> = Result<T, error::JvmError>;

use alloc::boxed::Box;
#[async_trait::async_trait(?Send)]
pub trait JvmCallback {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> JvmResult<JavaValue>;
}

pub use self::{
    array_class_definition::ArrayClassDefinition,
    array_class_instance::ArrayClassInstance,
    class_definition::ClassDefinition,
    class_instance::{Array, ClassInstance, ClassInstanceRef},
    detail::JvmDetail,
    error::JvmError,
    field::Field,
    jvm::Jvm,
    method::Method,
    r#type::JavaType,
    value::{JavaChar, JavaValue},
};
