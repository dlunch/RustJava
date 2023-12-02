use alloc::rc::Rc;
use core::cell::RefCell;

use crate::{class_instance::ClassInstance, Jvm, JvmResult};

pub struct JavaLangString {
    pub instance: Rc<RefCell<ClassInstance>>,
}

impl JavaLangString {
    pub fn new(jvm: &mut Jvm) -> JvmResult<Self> {
        let instance = jvm.instantiate_class("java/lang/String")?;
        jvm.invoke_method(&instance, "<init>", "()V")?;

        Ok(Self { instance })
    }
}
