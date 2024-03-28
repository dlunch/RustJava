use alloc::{boxed::Box, collections::BTreeMap, sync::Arc};
use core::fmt::{self, Debug, Formatter};

use async_lock::RwLock;

use jvm::{ClassDefinition, ClassInstance, Field, JavaValue, Result};

use crate::{class_definition::ClassDefinitionImpl, FieldImpl};

struct ClassInstanceInner {
    class: Box<dyn ClassDefinition>,
    storage: RwLock<BTreeMap<FieldImpl, JavaValue>>, // TODO we should use field offset or something
}

#[derive(Clone)]
pub struct ClassInstanceImpl {
    inner: Arc<ClassInstanceInner>,
}

impl ClassInstanceImpl {
    pub fn new(class: &ClassDefinitionImpl) -> Self {
        Self {
            inner: Arc::new(ClassInstanceInner {
                class: Box::new(class.clone()),
                storage: RwLock::new(BTreeMap::new()),
            }),
        }
    }
}

#[async_trait::async_trait]
impl ClassInstance for ClassInstanceImpl {
    fn destroy(self: Box<Self>) {}

    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        self.inner.class.clone()
    }

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        let other = other.as_any().downcast_ref::<ClassInstanceImpl>().unwrap();

        Ok(Arc::ptr_eq(&self.inner, &other.inner))
    }

    fn hash_code(&self) -> i32 {
        Arc::as_ptr(&self.inner) as i32
    }

    async fn get_field(&self, field: &dyn Field) -> Result<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        let storage = self.inner.storage.read().await;
        let value = storage.get(field);

        if let Some(x) = value {
            Ok(x.clone())
        } else {
            Ok(field.r#type().default())
        }
    }

    async fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> Result<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.inner.storage.write().await.insert(field.clone(), value);

        Ok(())
    }
}

impl Debug for ClassInstanceImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ClassInstance({})", self.inner.class.name())
    }
}
