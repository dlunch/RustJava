#![no_std]
extern crate alloc;

mod class;
mod class_definition;
mod class_instance;
mod class_loader;
mod field;
mod interpreter;
mod jvm;
mod method;
mod runtime;
mod stack_frame;
mod thread;
mod r#type;
mod value;

pub type JvmResult<T> = anyhow::Result<T>;

pub use self::{
    class_definition::ClassDefinition,
    class_loader::ClassLoader,
    field::Field,
    jvm::Jvm,
    method::{Method, MethodBody},
    value::JavaValue,
};
