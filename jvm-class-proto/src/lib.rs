#![no_std]
extern crate alloc;

mod descriptor;
mod method;
mod proto;

pub use {
    descriptor::JavaDescriptor,
    jvm_class_proto_macros::{java_class, java_methods},
    method::{InstanceMethodSignature, MethodBody, StaticMethodSignature, TypeConverter},
    proto::{JavaClassProto, JavaFieldProto, JavaMethodProto},
};

// Paths used by the code `java_class` generates, so that callers do not need matching imports.
#[doc(hidden)]
pub mod __private {
    pub use alloc::{string::String, vec::Vec};
    pub use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
}
