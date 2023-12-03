use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::cell::RefCell;

use crate::{ClassInstance, JavaValue, Jvm, JvmResult};

pub struct JavaLangString {
    pub instance: Rc<RefCell<Box<dyn ClassInstance>>>,
}

impl JavaLangString {
    pub fn new(jvm: &mut Jvm, string: &str) -> JvmResult<Self> {
        let chars = string.chars().map(JavaValue::Char).collect::<Vec<_>>();

        let array = jvm.instantiate_array("C", chars.len())?;
        jvm.store_array(&array, 0, &chars)?;

        let instance = jvm.instantiate_class("java/lang/String", "([C)V", &[JavaValue::Object(Some(array))])?;

        Ok(Self { instance })
    }
}
