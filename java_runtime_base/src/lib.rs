#![no_std]
extern crate alloc;

mod base;
mod handle;
mod method;
mod platform;

pub use {
    base::{JavaClassProto, JavaContext, JavaError, JavaFieldAccessFlag, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaResult},
    handle::{Array, JvmClassInstanceHandle},
    method::MethodBody,
    platform::Platform,
};
