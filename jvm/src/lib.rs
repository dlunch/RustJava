#![no_std]
extern crate alloc;

mod array_class;
mod array_class_instance;
mod class;
mod class_instance;
mod class_loader;
mod field;
mod jvm;
mod method;
mod thread;
mod r#type;
mod value;

pub mod runtime;

pub type JvmResult<T> = anyhow::Result<T>;

pub use self::{
    array_class::ArrayClass,
    array_class_instance::ArrayClassInstance,
    class::Class,
    class_instance::ClassInstance,
    class_loader::ClassLoader,
    field::Field,
    jvm::Jvm,
    method::Method,
    r#type::JavaType,
    thread::{ThreadContext, ThreadContextProvider, ThreadId},
    value::JavaValue,
};
