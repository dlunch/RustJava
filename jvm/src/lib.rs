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
    detail::JvmDetail,
    field::Field,
    jvm::{ClassInstanceRef, ClassRef, Jvm},
    method::Method,
    r#type::JavaType,
    thread::{ThreadContext, ThreadId},
    value::JavaValue,
};
