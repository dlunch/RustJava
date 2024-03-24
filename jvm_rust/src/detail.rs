use alloc::boxed::Box;

use jvm::{ClassDefinition, Jvm, JvmDetail, Result};

use crate::{array_class_definition::ArrayClassDefinitionImpl, ClassDefinitionImpl};

pub struct JvmDetailImpl;

#[async_trait::async_trait]
impl JvmDetail for JvmDetailImpl {
    async fn define_class(&self, _jvm: &Jvm, _name: &str, data: &[u8]) -> Result<Box<dyn ClassDefinition>> {
        ClassDefinitionImpl::from_classfile(data).map(|x| Box::new(x) as Box<_>)
    }

    async fn define_array_class(&self, _jvm: &Jvm, element_type_name: &str) -> Result<Box<dyn ClassDefinition>> {
        Ok(Box::new(ArrayClassDefinitionImpl::new(element_type_name)))
    }
}
