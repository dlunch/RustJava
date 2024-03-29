use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use core::fmt::{self, Debug, Formatter};

use async_lock::RwLock;

use jvm::{ArrayClassDefinition, ArrayClassInstance, ClassDefinition, ClassInstance, JavaError, JavaType, JavaValue, Result};

use crate::array_class_definition::ArrayClassDefinitionImpl;

struct ArrayClassInstanceInner {
    class: Box<dyn ClassDefinition>,
    length: usize,
    elements: RwLock<Vec<JavaValue>>,
}

#[derive(Clone)]
pub struct ArrayClassInstanceImpl {
    inner: Arc<ArrayClassInstanceInner>,
}

impl ArrayClassInstanceImpl {
    pub fn new(class: &ArrayClassDefinitionImpl, length: usize) -> Self {
        let element_type = class.element_type_name();
        let default_value = JavaType::parse(&element_type).default();

        Self {
            inner: Arc::new(ArrayClassInstanceInner {
                class: Box::new(class.clone()),
                length,
                elements: RwLock::new(vec![default_value; length]),
            }),
        }
    }
}

#[async_trait::async_trait]
impl ArrayClassInstance for ArrayClassInstanceImpl {
    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        self.inner.class.clone()
    }

    fn destroy(self: Box<Self>) {}

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        let other = other.as_any().downcast_ref::<ArrayClassInstanceImpl>().unwrap();

        Ok(Arc::ptr_eq(&self.inner, &other.inner))
    }

    fn hash_code(&self) -> i32 {
        Arc::as_ptr(&self.inner) as i32
    }

    async fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> Result<()> {
        if offset + values.len() > self.inner.length {
            // TODO real exception
            return Err(JavaError::FatalError("ArrayIndexOutOfBoundsException".into()));
        }

        self.inner.elements.write().await.splice(offset..offset + values.len(), values.into_vec());

        Ok(())
    }

    async fn load(&self, offset: usize, length: usize) -> Result<Vec<JavaValue>> {
        if offset + length > self.inner.length {
            // TODO real exception
            return Err(JavaError::FatalError("ArrayIndexOutOfBoundsException".into()));
        }

        Ok(self.inner.elements.read().await[offset..offset + length].to_vec())
    }

    async fn store_bytes(&mut self, offset: usize, values: Box<[i8]>) -> Result<()> {
        let values = values.into_vec().into_iter().map(JavaValue::Byte).collect::<Vec<_>>();

        self.store(offset, values.into_boxed_slice()).await
    }

    async fn load_bytes(&self, offset: usize, length: usize) -> Result<Vec<i8>> {
        let values = self.load(offset, length).await?;

        Ok(values.into_iter().map(|x| x.into()).collect::<Vec<_>>())
    }

    fn length(&self) -> usize {
        self.inner.length
    }
}

impl Debug for ArrayClassInstanceImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayClassInstance({})", self.inner.class.name())
    }
}
