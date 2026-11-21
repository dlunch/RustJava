use alloc::{boxed::Box, sync::Arc};
use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

use parking_lot::RwLock;

use jvm::{ClassDefinition, ClassInstance, Field, JavaValue, Result};

use crate::{FieldImpl, class_definition::ClassDefinitionImpl};

struct ClassInstanceInner {
    class: ClassDefinitionImpl,
    // instance fields of the class hierarchy; a field's position is its slot in storage
    fields: Arc<[FieldImpl]>,
    storage: RwLock<Box<[JavaValue]>>,
}

#[derive(Clone)]
pub struct ClassInstanceImpl {
    inner: Arc<ClassInstanceInner>,
}

impl ClassInstanceImpl {
    pub fn new(class: &ClassDefinitionImpl) -> Self {
        let (fields, storage) = class.instance_layout();

        Self {
            inner: Arc::new(ClassInstanceInner {
                class: class.clone(),
                fields,
                storage: RwLock::new(storage),
            }),
        }
    }
}

#[async_trait::async_trait]
impl ClassInstance for ClassInstanceImpl {
    fn destroy(self: Box<Self>) {}

    fn identity(&self) -> usize {
        Arc::as_ptr(&self.inner) as usize
    }

    fn shallow_clone(&self) -> Result<Box<dyn ClassInstance>> {
        Ok(Box::new(Self {
            inner: Arc::new(ClassInstanceInner {
                class: self.inner.class.clone(),
                fields: self.inner.fields.clone(),
                storage: RwLock::new(self.inner.storage.read().clone()),
            }),
        }))
    }

    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        Box::new(self.inner.class.clone())
    }

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        let other = other.as_any().downcast_ref::<ClassInstanceImpl>();
        if other.is_none() {
            return Ok(false);
        }
        let other = other.unwrap();

        Ok(Arc::ptr_eq(&self.inner, &other.inner))
    }

    fn get_field(&self, field: &dyn Field) -> Result<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        let slot = self.inner.fields.iter().position(|x| x == field).unwrap();

        Ok(self.inner.storage.read()[slot].clone())
    }

    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> Result<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        let slot = self.inner.fields.iter().position(|x| x == field).unwrap();
        self.inner.storage.write()[slot] = value;

        Ok(())
    }
}

impl Hash for ClassInstanceImpl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state);
    }
}

impl Debug for ClassInstanceImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ClassInstance({})", self.inner.class.name())
    }
}
