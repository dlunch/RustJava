#![no_std]
extern crate alloc;

mod base;
mod handle;
mod method;
mod platform;

pub use {
    base::{JavaClassProto, JavaError, JavaFieldAccessFlag, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaResult},
    handle::{Array, JvmClassInstanceHandle},
    method::{MethodBody, TypeConverter},
    platform::Platform,
};
