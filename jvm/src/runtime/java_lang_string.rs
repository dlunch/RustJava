use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{class_instance::ClassInstance, jvm::Jvm, JavaChar, Result};

pub struct JavaLangString {}

impl JavaLangString {
    pub async fn to_rust_string(jvm: &Jvm, this: Box<dyn ClassInstance>) -> Result<String> {
        let value = jvm.get_field(&this, "value", "[C").await?;

        let length = jvm.array_length(&value).await?;
        let string: Vec<JavaChar> = jvm.load_array(&value, 0, length).await?;

        Ok(String::from_utf16(&string).unwrap())
    }

    pub async fn from_rust_string(jvm: &Jvm, string: &str) -> Result<Box<dyn ClassInstance>> {
        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        Self::from_utf16(jvm, utf16).await
    }

    async fn from_utf16(jvm: &Jvm, data: Vec<u16>) -> Result<Box<dyn ClassInstance>> {
        let mut java_value = jvm.instantiate_array("C", data.len()).await?;

        jvm.store_array(&mut java_value, 0, data.to_vec()).await?;

        let instance = jvm.new_class("java/lang/String", "([C)V", (java_value,)).await?;

        Ok(instance)
    }
}
