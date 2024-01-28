use alloc::boxed::Box;

use crate::{ClassDefinition, Jvm, JvmResult};

#[async_trait::async_trait(?Send)]
pub trait JvmDetail {
    async fn define_class(&self, jvm: &Jvm, name: &str, data: &[u8]) -> JvmResult<Box<dyn ClassDefinition>>;
    async fn define_array_class(&self, jvm: &Jvm, element_type_name: &str) -> JvmResult<Box<dyn ClassDefinition>>;
}
