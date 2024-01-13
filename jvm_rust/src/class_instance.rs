use alloc::{boxed::Box, collections::BTreeMap, rc::Rc};
use core::cell::RefCell;

use jvm::{Class, ClassInstance, Field, JavaValue, JvmResult};

use crate::{class::ClassImpl, FieldImpl};

#[derive(Debug)]
struct ClassInstanceInner {
    class: Box<dyn Class>,
    storage: RefCell<BTreeMap<FieldImpl, JavaValue>>, // TODO we should use field offset or something
}

#[derive(Debug, Clone)]
pub struct ClassInstanceImpl {
    inner: Rc<ClassInstanceInner>,
}

impl ClassInstanceImpl {
    pub fn new(class: &ClassImpl) -> Self {
        Self {
            inner: Rc::new(ClassInstanceInner {
                class: Box::new(class.clone()),
                storage: RefCell::new(BTreeMap::new()),
            }),
        }
    }
}

impl ClassInstance for ClassInstanceImpl {
    fn destroy(self: Box<Self>) {}

    fn class(&self) -> Box<dyn Class> {
        self.inner.class.clone()
    }

    fn equals(&self, other: &dyn ClassInstance) -> JvmResult<bool> {
        let other = other.as_any().downcast_ref::<ClassInstanceImpl>().unwrap();

        Ok(Rc::ptr_eq(&self.inner, &other.inner))
    }

    fn get_field(&self, field: &dyn Field) -> JvmResult<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        let storage = self.inner.storage.borrow();
        let value = storage.get(field);

        if let Some(x) = value {
            Ok(x.clone())
        } else {
            Ok(field.r#type().default())
        }
    }

    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> JvmResult<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.inner.storage.borrow_mut().insert(field.clone(), value);

        Ok(())
    }
}
