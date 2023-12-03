use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use crate::{ClassInstance, Jvm, JvmResult};

pub struct JavaLangString {
    pub instance: Rc<RefCell<Box<dyn ClassInstance>>>,
}

impl JavaLangString {
    pub fn new(jvm: &mut Jvm) -> JvmResult<Self> {
        let instance = jvm.instantiate_class("java/lang/String")?;
        let class = jvm.resolve_class(instance.borrow().class_name())?.unwrap();
        let class = class.borrow();
        let method = class.method("<init>", "()V").unwrap();

        method.run(jvm, &[])?;

        Ok(Self { instance })
    }
}
