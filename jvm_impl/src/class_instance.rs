use core::cell::RefCell;

use alloc::{boxed::Box, rc::Rc, vec::Vec};

use dyn_clone::clone_box;
use jvm::{ArrayClassInstance, Class, ClassInstance, Field, JavaValue};

use crate::{class::ClassImpl, FieldImpl};

#[derive(Debug, Clone)]
pub struct ClassInstanceImpl {
    class: Box<dyn Class>,
    storage: Rc<RefCell<Vec<JavaValue>>>,
}

impl ClassInstanceImpl {
    pub fn new(class: &ClassImpl) -> Self {
        let storage = class.fields().iter().filter(|x| !x.is_static()).map(|x| x.r#type().default()).collect();

        Self {
            class: clone_box(class),
            storage: Rc::new(RefCell::new(storage)),
        }
    }
}

impl ClassInstance for ClassInstanceImpl {
    fn destroy(self: Box<Self>) {}

    fn class(&self) -> Box<dyn Class> {
        self.class.clone()
    }

    fn get_field(&self, field: &dyn Field) -> jvm::JvmResult<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        Ok(self.storage.borrow()[field.index()].clone())
    }

    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> jvm::JvmResult<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.storage.borrow_mut()[field.index()] = value;

        Ok(())
    }

    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        None
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        None
    }
}
