#![no_std]
extern crate alloc;

mod array_class_definition;
mod array_class_instance;
mod as_any;
mod class_definition;
mod class_instance;
mod class_loader;
mod detail;
mod error;
mod field;
mod invoke_arg;
mod jvm;
mod method;
mod r#type;
mod value;

pub mod runtime;

use alloc::boxed::Box;
use core::result;

pub type Result<T> = result::Result<T, error::JavaError>;

#[async_trait::async_trait]
pub trait JvmCallback: Sync + Send {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue>;
}

pub use self::{
    array_class_definition::ArrayClassDefinition,
    array_class_instance::ArrayClassInstance,
    class_definition::ClassDefinition,
    class_instance::{Array, ClassInstance, ClassInstanceRef},
    class_loader::BootstrapClassLoader,
    detail::JvmDetail,
    error::JavaError,
    field::Field,
    jvm::Jvm,
    method::Method,
    r#type::JavaType,
    value::{JavaChar, JavaValue},
};
