use alloc::{boxed::Box, vec::Vec};

use bytemuck::cast_vec;

use crate::{class_instance::ClassInstance, jvm::Jvm, Result};

pub struct JavaIoInputStream;

impl JavaIoInputStream {
    #[allow(clippy::borrowed_box)]
    pub async fn read_until_end(jvm: &Jvm, this: &Box<dyn ClassInstance>) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();

        let java_buffer = jvm.instantiate_array("B", 1024).await?;

        loop {
            let bytes_read: i32 = jvm.invoke_virtual(this, "read", "([B)I", (java_buffer.clone(),)).await?;

            if bytes_read == 0 || bytes_read == -1 {
                break;
            }

            let bytes = jvm.load_byte_array(&java_buffer, 0, bytes_read as _).await?;

            buffer.extend_from_slice(&bytes);
        }

        jvm.destroy(java_buffer)?; // TODO: this should be done automatically

        Ok(cast_vec(buffer))
    }
}
