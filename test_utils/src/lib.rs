#![no_std]
extern crate alloc;

mod run_class;
mod test_jvm;

pub use self::{
    run_class::run_class,
    test_jvm::{runtime_test_jvm, test_jvm},
};
