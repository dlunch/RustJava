use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use crate::{ClassInstance, Jvm, JvmResult};

pub struct JavaLangString {
    pub instance: Rc<RefCell<Box<dyn ClassInstance>>>,
}

impl JavaLangString {
    pub fn new(jvm: &mut Jvm) -> JvmResult<Self> {
        let instance = jvm.instantiate_class("java/lang/String", "()V", &[])?;

        Ok(Self { instance })
    }
}
