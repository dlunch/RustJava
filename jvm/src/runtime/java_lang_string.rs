use alloc::{boxed::Box, format, string::String, vec::Vec};

use crate::{JavaChar, Result, class_instance::ClassInstance, jvm::Jvm};

pub struct JavaLangString;

impl JavaLangString {
    #[allow(clippy::borrowed_box)]
    pub async fn to_utf16(jvm: &Jvm, this: &Box<dyn ClassInstance>) -> Result<Vec<JavaChar>> {
        let value = jvm.get_field(this, "value", "[C").await?;
        let offset: i32 = jvm.get_field(this, "offset", "I").await?;
        let count: i32 = jvm.get_field(this, "count", "I").await?;

        // access flags are not enforced, so bytecode can leave a negative here, which would widen into a huge usize
        if offset < 0 || count < 0 {
            return Err(jvm
                .exception("java/lang/StringIndexOutOfBoundsException", &format!("offset {offset}, count {count}"))
                .await);
        }

        jvm.load_array(&value, offset as _, count as _).await
    }

    #[allow(clippy::borrowed_box)]
    pub async fn to_rust_string(jvm: &Jvm, this: &Box<dyn ClassInstance>) -> Result<String> {
        Ok(String::from_utf16_lossy(&Self::to_utf16(jvm, this).await?))
    }

    pub async fn from_rust_string(jvm: &Jvm, string: &str) -> Result<Box<dyn ClassInstance>> {
        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        Self::from_utf16(jvm, utf16).await
    }

    pub async fn from_utf16(jvm: &Jvm, data: Vec<u16>) -> Result<Box<dyn ClassInstance>> {
        let length = data.len();
        let mut java_value = jvm.instantiate_array("C", length).await?;

        jvm.store_array(&mut java_value, 0, data).await?;

        // the array is fresh, so the package-private sharing constructor is safe and avoids a second copy
        let instance = jvm.new_class("java/lang/String", "(II[C)V", (0, length as i32, java_value)).await?;

        Ok(instance)
    }
}
