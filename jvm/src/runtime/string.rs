use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use crate::{ClassInstance, JavaValue, Jvm, JvmResult};

pub struct JavaLangString {
    pub instance: Rc<RefCell<Box<dyn ClassInstance>>>,
}

impl JavaLangString {
    pub fn new(jvm: &mut Jvm) -> JvmResult<Self> {
        let array = jvm.instantiate_array("C", 10)?;
        let instance = jvm.instantiate_class("java/lang/String", "([C)V", &[JavaValue::Object(Some(array))])?;

        Ok(Self { instance })
    }
}
