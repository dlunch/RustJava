use alloc::boxed::Box;

use crate::{ClassDefinition, Jvm, Result};

#[async_trait::async_trait(?Send)]
pub trait JvmDetail: Sync + Send {
    async fn define_class(&self, jvm: &Jvm, name: &str, data: &[u8]) -> Result<Box<dyn ClassDefinition>>;
    async fn define_array_class(&self, jvm: &Jvm, element_type_name: &str) -> Result<Box<dyn ClassDefinition>>;
}
