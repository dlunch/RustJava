use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::cell::RefCell;

use jvm::{ArrayClassInstance, Class, ClassInstance, Field, JavaValue};

use crate::{class::ClassImpl, FieldImpl};

#[derive(Debug)]
struct ClassInstanceInner {
    class: Box<dyn Class>,
    storage: RefCell<Vec<JavaValue>>,
}

#[derive(Debug, Clone)]
pub struct ClassInstanceImpl {
    inner: Rc<ClassInstanceInner>,
}

impl ClassInstanceImpl {
    pub fn new(class: &ClassImpl) -> Self {
        let storage = class.fields().iter().filter(|x| !x.is_static()).map(|x| x.r#type().default()).collect();

        Self {
            inner: Rc::new(ClassInstanceInner {
                class: Box::new(class.clone()),
                storage: RefCell::new(storage),
            }),
        }
    }
}

impl ClassInstance for ClassInstanceImpl {
    fn destroy(self: Box<Self>) {}

    fn class(&self) -> Box<dyn Class> {
        self.inner.class.clone()
    }

    fn get_field(&self, field: &dyn Field) -> jvm::JvmResult<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        Ok(self.inner.storage.borrow()[field.index()].clone())
    }

    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> jvm::JvmResult<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.inner.storage.borrow_mut()[field.index()] = value;

        Ok(())
    }

    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        None
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        None
    }
}
